use std::{
    fs::{create_dir_all, exists, read_dir},
    io::Result, path::Path
};

use tabled::{settings::Style, Table};

use crate::{mount::MountedDevice, trash_info::TrashInfo, tree::Tree};

#[derive(Debug)]
pub struct Trash {
    pub device: MountedDevice
}

impl Trash {
    pub fn new(device: &MountedDevice) -> Self {
        Self { device: device.clone() }
    }

    pub fn create_trash_dir_if_not_exists(&self) -> Result<bool> {
        let trash_dir = self.device.trash_dir()?;

        if !exists(&trash_dir)? {
            let (trash_info_dir, trash_files_dir) = (self.device.trash_info_dir()?, self.device.trash_files_dir()?);
            create_dir_all(&trash_info_dir)?;
            create_dir_all(&trash_files_dir)?;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn contents(&self) -> Result<Vec<TrashInfo>> {
        let info_dir = self.device.trash_info_dir()?;

        let mut trash_contents = vec![];

        for entry in read_dir(info_dir)? {
            let path = entry?.path();
            if path.is_file() {
                if let Ok(trash_info) = TrashInfo::from_file(path) {
                    trash_contents.push(trash_info);
                }
            }
        }

        Ok(trash_contents)
    }

    pub fn list(&self, recursive: bool) -> Result<()> {
        self.create_trash_dir_if_not_exists()?;

        if recursive {
            let trash_files_dir = self.device.trash_files_dir()?;
            let tree = files_tree(trash_files_dir, self)?;
            println!("{tree}");
        } else {
            let mut trash_contents = self.contents()?;
            trash_contents.sort_by(|a, b| b.deletion_date.cmp(&a.deletion_date));

            let mut table = Table::new(trash_contents);
            table.with(Style::modern_rounded());

            print!("{}", table.to_string());
        }

        Ok(())
    }
}

fn files_tree_label<P: AsRef<Path>>(p: P, trash: &Trash) -> String {
    let name = p.as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    if name.eq(&String::from("files")) {
        if trash.device.mount_point.eq("/") || trash.device.mount_point.eq("/home")  {
            String::from("Home Trash")
        } else {
            Path::new(&trash.device.mount_point).file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        }
    } else {
        name
    }
}

fn files_tree<P: AsRef<Path>>(p: P, trash: &Trash) -> Result<Tree<String>> {
    let result = read_dir(&p)?.filter_map(|e| e.ok()).fold(
        Tree::new(files_tree_label(p.as_ref().canonicalize()?, trash)),
        |mut root, entry| {
            let dir = entry.metadata().unwrap();
            if dir.is_dir() {
                root.push(files_tree(entry.path(), trash).unwrap());
            } else {
                root.push(Tree::new(files_tree_label(entry.path(), trash)));
            }
            root
        },
    );

    Ok(result)
}
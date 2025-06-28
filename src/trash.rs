use std::{
    fs::{
        create_dir_all, exists, read_dir, remove_dir_all, remove_file, rename
    },
    io::{Error, ErrorKind, Result},
    path::{Path},
};

use inquire::{Confirm, InquireError, Select};
use tabled::{
    settings::Style,
    Table,
};

use crate::{
    common::with_trashinfo_extension, mount::MountedDevice, string_encode::decode_filename, trash_info::TrashInfo, tree::Tree
};

fn restore_from_trash<P: AsRef<Path>>(trash_info_path: P, trash_item_path: P, destination_path: P) -> Result<()> {
    if let Some(parent_path) = destination_path.as_ref().parent() {
        if !exists(parent_path)? {
            create_dir_all(parent_path)?;
        }
    }

    rename(trash_item_path, destination_path)?;
    remove_file(trash_info_path)?;

    Ok(())
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

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> Result<()> {
    if exists(path.as_ref())? {
        for dir_entry in read_dir(path.as_ref())? {
            if let Ok(dir_entry) = dir_entry {
                if dir_entry.path().is_dir() {
                    remove_dir_all(dir_entry.path())?;
                } else {
                    remove_file(dir_entry.path())?;
                }
            }
        }
    }

    Ok(())
}

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

    pub fn restore(&self, overwrite: bool) -> Result<()> {
        let trash_contents = self.contents()?;

        let user_response = Select::new("Select an item from the trash to restore", trash_contents).prompt();
        match user_response {
            Ok(selected_item) => {
                self.create_trash_dir_if_not_exists()?;

                let trash_files_dir = self.device.trash_files_dir()?;
                let trash_info_dir = self.device.trash_info_dir()?;

                let full_trash_file_path = trash_files_dir.join(&selected_item.path);
                let full_trash_info_path = trash_info_dir.join(with_trashinfo_extension(&Path::new(&selected_item.path).to_path_buf()));

                if exists(&full_trash_file_path)? {
                    let destination_path = if selected_item.full_path.starts_with("/") {
                        Path::new(&decode_filename(&selected_item.full_path)).to_path_buf()
                    } else {
                        Path::new(&self.device.mount_point).join(&decode_filename(&selected_item.full_path))
                    };

                    if exists(&destination_path)? {
                        if overwrite {
                            restore_from_trash(&full_trash_info_path, &full_trash_file_path, &destination_path)
                        } else {
                            Err(Error::new(ErrorKind::PermissionDenied, format!("{} already exists", &selected_item.full_path)))
                        }
                    } else {
                        restore_from_trash(&full_trash_info_path, &full_trash_file_path, &destination_path)
                    }
                } else {
                    // If there isn't a corresponding file for the trash info, we have bad data and need to delete the trashinfo file
                    remove_file(full_trash_info_path)?;

                    Ok(())
                }
            },
            Err(error) => {
                match error {
                    InquireError::OperationCanceled => Ok(()),
                    InquireError::OperationInterrupted => Ok(()),
                    _ => Err(Error::new(ErrorKind::Other, error.to_string()))
                }
            }
        }
    }

    fn trash_remove(&self, trash_entry: &TrashInfo) -> Result<()> {
        let (trash_info_dir, trash_files_dir) = (
            self.device.trash_info_dir()?, 
            self.device.trash_files_dir()?
        );

        let full_trash_file_path = trash_files_dir.join(&trash_entry.path);

        if exists(full_trash_file_path)? {
            let full_trash_info_path = with_trashinfo_extension(&trash_info_dir.join(&trash_entry.path));
            let full_trash_item_path = trash_files_dir.join(&trash_entry.path);

            let metadata = full_trash_item_path.metadata()?;

            if metadata.is_dir() {
                remove_dir_all(&full_trash_item_path)?;
                remove_file(&full_trash_info_path)?;

                Ok(())
            } else {
                remove_file(&full_trash_item_path)?;
                remove_file(&full_trash_info_path)?;

                Ok(())
            }
        } else {
            Err(Error::new(ErrorKind::NotFound, format!("{} is not in the trash", &trash_entry.path)))
        }
    }

    pub fn remove(&self, yes: bool) -> Result<()> {
        self.create_trash_dir_if_not_exists()?;

        let trash_contents = self.contents()?;
        let user_response = Select::new("Select an item from the trash to remove", trash_contents).prompt();

        match user_response {
            Ok(selected_item) => {
                if yes {
                    self.trash_remove(&selected_item)
                } else {
                    let message = format!("Are you sure you want to delete {}? This action is irreversible.", selected_item.path.as_str());
                    let should_rm_from_trash_result = Confirm::new(&message.as_str())
                        .with_default(false)
                        .prompt();

                    match should_rm_from_trash_result {
                        Ok(true) => self.trash_remove(&selected_item),
                        Ok(false) => Ok(()),
                        Err(error) => match error {
                            InquireError::OperationCanceled => Ok(()),
                            InquireError::OperationInterrupted => Ok(()),
                            _ => Err(Error::new(ErrorKind::Other, error.to_string()))
                        }
                    }
                }
            },
            Err(error) => {
                match error {
                    InquireError::OperationCanceled => Ok(()),
                    InquireError::OperationInterrupted => Ok(()),
                    _ => Err(Error::new(ErrorKind::Other, error.to_string()))
                }
            }
        }
    }

    fn remove_all_trashinfo_files(&self) -> Result<()> {
        let trash_info_dir = self.device.trash_info_dir()?;
        remove_dir_contents(&trash_info_dir)?;

        Ok(())
    }

    fn remove_all_trash_files(&self) -> Result<()> {
        let trash_files_dir = self.device.trash_files_dir()?;
        remove_dir_contents(&trash_files_dir)?;

        Ok(())
    }

    pub fn empty(&self) -> Result<()> {
        self.remove_all_trash_files()?;
        self.remove_all_trashinfo_files()?;

        Ok(())
    }
}
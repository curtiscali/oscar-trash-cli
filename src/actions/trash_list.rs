use std::{
    fs::read_dir, 
    io::Result, 
    path::Path
};

use tabled::{settings::Style, Table};
use termtree::Tree;
use crate::{common::*, constants::*, string_encode::decode_filename};

fn files_tree_label<P: AsRef<Path>>(p: P) -> String {
    let name = p.as_ref()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    if name.eq(&String::from("files")) {
        String::from("System Trash")
    } else {
        name
    }
}

fn files_tree<P: AsRef<Path>>(p: P) -> Result<Tree<String>> {
    let result = read_dir(&p)?.filter_map(|e| e.ok()).fold(
        Tree::new(files_tree_label(p.as_ref().canonicalize()?)),
        |mut root, entry| {
            let dir = entry.metadata().unwrap();
            if dir.is_dir() {
                root.push(files_tree(entry.path()).unwrap());
            } else {
                root.push(Tree::new(files_tree_label(entry.path())));
            }
            root
        },
    );

    Ok(result)
}

pub fn trash_list(recursive: bool) -> Result<()> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            if recursive {
                let trash_files_dir = freedesktop_home_trash_files_dir().unwrap();
                match files_tree(trash_files_dir) {
                    Ok(tree) => {
                        println!("{tree}");
                        Ok(())
                    },
                    Err(error) => Err(error)
                }
            } else {
                match get_home_trash_contents() {
                    Ok(mut trash_contents) => {
                        trash_contents.sort_by(|a, b| b.deletion_date.cmp(&a.deletion_date));
    
                        let mut table = Table::new(trash_contents);
                        table.with(Style::modern_rounded());
    
                        print!("{}", table.to_string());
                        Ok(())
                    },
                    Err(error) => Err(error)
                }
            }
        },
        Err(error) => Err(error)
    }

}

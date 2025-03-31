use std::{fs::{read_dir, read_to_string}, io::{Error, Result}, path::{Path, PathBuf}};
use chrono::NaiveDateTime;
use configparser::ini::Ini;
use tabled::{settings::Style, Table, Tabled};
use termtree::Tree;
use crate::{common::*, constants::*};

#[derive(Tabled)]
#[tabled(rename_all = "CamelCase")]
pub struct TrashInfo {
    #[tabled(rename = "Path")]
    pub path: String,

    #[tabled(rename = "Deletion Date")]
    pub deletion_date: NaiveDateTime
}

impl TrashInfo {
    fn from_file(path: PathBuf) -> Option<TrashInfo> {
        let (trash_info_header, path_field, deletion_date_field) = (
            String::from(TRASH_INFO_SECTION_HEADER).to_lowercase(),
            String::from(TRASH_INFO_PATH_KEY).to_lowercase(),
            String::from(TRASH_INFO_DELETION_DATE_KEY).to_lowercase()
        );

        match read_to_string(path) {
            Ok(file_contents) => {
                let mut ini = Ini::new();
                match ini.read(file_contents) {
                    Ok(map) => {
                        match map.get(&trash_info_header) {
                            Some(trash_info_section) => {
                                let full_path = match trash_info_section.get(&path_field) {
                                    Some(path_val) => path_val.clone(),
                                    None => None
                                };

                                let deletion_date_str = match trash_info_section.get(&deletion_date_field) {
                                    Some(deletion_date_val) => deletion_date_val.clone(),
                                    None => None
                                };

                                if full_path.is_some() && deletion_date_str.is_some() {
                                    let deletion_date = NaiveDateTime::parse_from_str(
                                        deletion_date_str.unwrap().as_str(),
                                        "%Y-%m-%dT%H:%M:%S"
                                    );

                                    match deletion_date {
                                        Ok(date_result) => Some(TrashInfo { 
                                            path: String::from(Path::new(&full_path.unwrap()).file_name().unwrap().to_str().unwrap()), 
                                            deletion_date: date_result
                                        }) ,
                                        Err(_) => None
                                    }
                                } else {
                                    return None;
                                }
                            },
                            None => None
                        }
                    },
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}

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

pub fn get_home_trash_contents() -> Result<Vec<TrashInfo>> {
    let mut trash_contents = vec![];

    match freedesktop_home_trash_info_dir() {
        Some(info_path) => {
            match read_dir(info_path) {
                Ok(contents) => {
                    for result in contents {
                        match result {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_file() {
                                    match TrashInfo::from_file(path) {
                                        Some(trash_info) => trash_contents.push(trash_info),
                                        None => continue
                                    }
                                }
                            },
                            Err(_) => continue
                        }
                    }

                    Ok(trash_contents)
                },
                Err(error) => Err(error)
            }
        },
        None => Err(
            Error::new(
                std::io::ErrorKind::Other, 
                "Unable to determine the path for the home trash directory."
            )
        )
    }
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

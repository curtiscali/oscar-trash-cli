use std::{fs::read_dir, path::PathBuf, result};

use chrono::{DateTime, NaiveDateTime};
use crate::common::*;

pub struct TrashInfo {
    pub path: &str,
    pub deletion_date: NaiveDateTime
}

impl TrashInfo {
    pub fn from_file(path: PathBuf) -> TrashInfo {
        // TODO: parse
    }
}

pub fn trash_list() -> Result<Vec<TrashInfo>, bool> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            let mut trash_contents = vec![];

            // this shouldn't fail if the call to create_trash_dir_if_not_exists
            // succeeded
            let info_path = freedesktop_home_trash_info_dir().unwrap();

            match read_dir(info_path) {
                Ok(contents) => {
                    for result in contents {
                        match result {
                            Ok(entry) => {
                                // TODO: parse entry as ini file
                            },
                            Err(_) => continue
                        }
                    }

                    Ok(trash_contents)
                },
                Err(_) => {
                    Err(false)
                }
            }
        },
        Err(_) => {
            Err(false)
        }
    }

} 

use std::{
    fs::{canonicalize, exists, rename}, 
    io::{Error, ErrorKind, Result},
    path::PathBuf
};

use chrono::Local;
use configparser::ini::Ini;
use inquire::Confirm;

use crate::{common::*, constants::*};

fn create_trash_info_entry(path: &PathBuf) -> Result<()> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            let trash_info_section_header = String::from(TRASH_INFO_SECTION_HEADER);
            let mut trash_info_ini = Ini::new();

            let now = Local::now();

            trash_info_ini.set(
                &trash_info_section_header, 
                TRASH_INFO_PATH_KEY, 
                Some(String::from(path.to_str().unwrap()))
            );

            trash_info_ini.set(
                &trash_info_section_header, 
                TRASH_INFO_DELETION_DATE_KEY, 
                Some(now.format("%Y-%m-%dT%H:%M:%S").to_string())
            );

            // TODO: create $trash/info/name.trashinfo file
            // write contents of trash_info_ini to the file
            match path.file_name() {
                Some(filename) => {
                    let trash_info_directory = freedesktop_home_trash_info_dir().unwrap();
                    let trash_info_ini_path = trash_info_directory.join(filename).with_extension(TRASH_INFO_FILE_EXTENSION);

                    match trash_info_ini.write(trash_info_ini_path) {
                        Ok(_) => Ok(()),
                        Err(error) => Err(error)
                    }
                },
                None => Err(Error::new(ErrorKind::InvalidInput, format!("Cannot place {} in trash", path.to_str().unwrap())))
            }
        },
        Err(error) => Err(error)
    }
}

pub fn trash_put(path: &String) -> Result<()> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            match canonicalize(path) {
                Ok(os_absolute_path) => {
                    match exists(&os_absolute_path) {
                        Ok(exists) => {
                            if exists {
                                let should_place_in_trash_result = Confirm::new(format!("Are you sure you want to place {} in the trash?", path).as_str())
                                    .with_default(false)
                                    .prompt();

                                match should_place_in_trash_result {
                                    Ok(true) => {
                                        match create_trash_info_entry(&os_absolute_path) {
                                            Ok(_) => {
                                                let trash_files_directory = freedesktop_home_trash_files_dir().unwrap();
                                                match os_absolute_path.file_name() {
                                                    Some(filename) => {
                                                        match rename(path, trash_files_directory.join(filename)) {
                                                            Ok(_) => Ok(()),
                                                            Err(error) => Err(error)
                                                        }
                                                    },
                                                    None => Err(Error::new(ErrorKind::InvalidInput, format!("Cannot place {} in trash", path)))
                                                }
                                            },
                                            Err(error) => Err(error)
                                        }
                                    },
                                    Ok(false) => Ok(()),
                                    Err(error) => Err(Error::new(ErrorKind::Other, error.to_string()))
                                }

                                
                            } else {
                                Err(Error::new(ErrorKind::NotFound, format!("{} does not exist", path)))
                            }
                        },
                        Err(error) => Err(error)
                    }
                },
                Err(error) => Err(error)
            }
            
        },
        Err(error) => Err(error)
    }
}
use std::{
    fs::{
        read_dir, 
        remove_dir_all, 
        remove_file
    }, 
    io::Result
};

use crate::common::{create_trash_dir_if_not_exists, freedesktop_home_trash_files_dir, freedesktop_home_trash_info_dir};

fn remove_all_trashinfo_files() -> Result<Vec<Result<()>>> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            let mut rm_results = vec![];

            let trash_info_dir = freedesktop_home_trash_info_dir().unwrap();
            match read_dir(&trash_info_dir) {
                Ok(dir_contents) => {
                    for result in dir_contents {
                        match result {
                            Ok(entry) => rm_results.push(remove_file(entry.path())),
                            Err(error) => rm_results.push(Err(error))
                        }
                    }
                },
                Err(error) => rm_results.push(Err(error))
            }

            Ok(rm_results)
        },
        Err(error) => Err(error)
    }
}

fn remove_all_trash_files() -> Result<Vec<Result<()>>> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            let mut rm_results = vec![];

            let trash_files_dir = freedesktop_home_trash_files_dir().unwrap();
            match read_dir(&trash_files_dir) {
                Ok(dir_contents) => {
                    for result in dir_contents {
                        match result {
                            Ok(entry) => {
                                match entry.metadata() {
                                    Ok(metadata) => {
                                        if metadata.is_file() {
                                            rm_results.push(remove_file(entry.path()));
                                        } else if metadata.is_dir() {
                                            rm_results.push(remove_dir_all(entry.path()));
                                        }
                                    },
                                    Err(error) => rm_results.push(Err(error))
                                }
                            },
                            Err(error) => rm_results.push(Err(error))
                        }
                    }
                },
                Err(error) => rm_results.push(Err(error))
            }

            Ok(rm_results)
        },
        Err(error) => Err(error)
    }
}

pub fn trash_empty() -> Result<()> {
    match remove_all_trash_files() {
        Ok(_) => {
            match remove_all_trashinfo_files() {
                Ok(_) => Ok(()),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}
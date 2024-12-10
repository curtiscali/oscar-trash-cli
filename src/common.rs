use std::{fs::{create_dir, exists}, path::PathBuf};
use dirs::data_dir;

/// This function gets the home trash directory as defined in the Freedesktop.org spec: https://specifications.freedesktop.org/trash-spec/latest/
pub fn freedesktop_home_trash_dir() -> Option<PathBuf> {
    match data_dir() {
        Some(path) => Some(path.join("Trash")),
        None => None
    }
}

pub fn freedesktop_home_trash_files_dir() -> Option<PathBuf> {
    match freedesktop_home_trash_dir() {
        Some(home_trash_dir) => Some(home_trash_dir.join("files")),
        None => None
    }
}

pub fn freedesktop_home_trash_info_dir() -> Option<PathBuf> {
    match freedesktop_home_trash_dir() {
        Some(home_trash_dir) => Some(home_trash_dir.join("info")),
        None => None
    }
}

pub fn create_trash_dir_if_not_exists() -> Result<bool, bool> {
    match freedesktop_home_trash_dir() {
        Some(home_trash_dir) => {
            match exists(home_trash_dir.clone()) {
                Ok(dir_exists) => {
                    if !dir_exists {
                        match create_dir(home_trash_dir.clone()) {
                            Ok(_) => {
                                match create_dir(freedesktop_home_trash_files_dir().unwrap()) {
                                    Ok(_) => {
                                        match create_dir(freedesktop_home_trash_info_dir().unwrap()) {
                                            Ok(_) => return Ok(true),
                                            Err(_) => return Err(false)
                                        }
                                    },
                                    Err(_) => return Err(false)
                                }
                            },
                            Err(_) => return Err(false)
                        }
                    } else {
                        return Ok(false);
                    }
                },
                Err(_) => {
                    return Err(false);
                }
            }
        },
        None => Err(false)
    }

}

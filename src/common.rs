use std::{
    env::var,
    fs::{create_dir_all, exists, read_dir}, 
    io::{Error, Result}, 
    path::{Path, PathBuf}
};

use crate::constants::TRASH_INFO_FILE_EXTENSION;
use crate::trash_info::TrashInfo;

/// This function gets the home trash directory as defined in the Freedesktop.org spec: https://specifications.freedesktop.org/trash-spec/latest/
pub fn freedesktop_home_trash_dir() -> Option<PathBuf> {
    match var("XDG_DATA_HOME") {
        Ok(xdg_data_home) => Some(Path::new(&xdg_data_home).join("Trash").to_path_buf()),
        Err(_) => {
            match var("HOME") {
                Ok(home) => Some(Path::new(&home).join(".local/share/Trash").to_path_buf()),
                Err(_) => None
            }
        }
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

pub fn with_trashinfo_extension(p: &PathBuf) -> PathBuf {
    p.with_extension(match p.extension() {
        Some(extension) => format!("{}.{}", extension.to_str().unwrap(), TRASH_INFO_FILE_EXTENSION),
        None => String::from(TRASH_INFO_FILE_EXTENSION)
    })
}

pub fn create_home_trash_info_dir_if_not_exists() -> Result<bool>{
    if let Some(home_trash_info_dir) = freedesktop_home_trash_info_dir() {
        return match exists(&home_trash_info_dir) {
            Ok(true) => Ok(false), // dir already exists & no action is needed, so we send false
            Ok(false) => match create_dir_all(&home_trash_info_dir) {
                Ok(_) => Ok(true),
                Err(error) => Err(error)
            },
            Err(error) => Err(error)
        };
    }

    return Err(
        Error::new(
            std::io::ErrorKind::Other, 
            "Unable to determine the path for the home trash info directory."
        )
    );
}

pub fn create_home_trash_files_dir_if_not_exists() -> Result<bool>{
    if let Some(home_trash_files_dir) = freedesktop_home_trash_files_dir() {
        return match exists(&home_trash_files_dir) {
            Ok(true) => Ok(false), // dir already exists & no action is needed, so we send false
            Ok(false) => match create_dir_all(&home_trash_files_dir) {
                Ok(_) => Ok(true),
                Err(error) => Err(error)
            },
            Err(error) => Err(error)
        };
    }

    return Err(
        Error::new(
            std::io::ErrorKind::Other, 
            "Unable to determine the path for the home trash files directory."
        )
    );
}

pub fn create_home_trash_dir_if_not_exists() -> Result<bool> {
    match create_home_trash_info_dir_if_not_exists() {
        Ok(was_info_dir_created) => {
            match create_home_trash_files_dir_if_not_exists() {
                Ok(was_files_dir_created) => Ok(was_info_dir_created && was_files_dir_created),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
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
use std::{
    env::var,
    fs::{create_dir, exists}, 
    io::Error, 
    path::{Path, PathBuf}
};

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

pub fn create_trash_dir_if_not_exists() -> Result<bool, Error> {
    match freedesktop_home_trash_dir() {
        Some(home_trash_dir) => {
            match exists(home_trash_dir.clone()) {
                Ok(dir_exists) => {
                    if dir_exists {
                        return Ok(false);
                    } else {
                        match create_dir(home_trash_dir.clone()) {
                            Ok(_) => {
                                // Unwrap is safe here because the above home_trash_dir()
                                // call succeeded, which means a path can be determined
                                let (files_dir, info_dir) = (
                                    freedesktop_home_trash_files_dir().unwrap(),
                                    freedesktop_home_trash_info_dir().unwrap()
                                );
                               
                                match create_dir(files_dir) {
                                    Ok(_) => {
                                        match create_dir(info_dir) {
                                            Ok(_) => return Ok(true),
                                            Err(error) => return Err(error)
                                        }
                                    },
                                    Err(error) => return Err(error)
                                }
                            },
                            Err(error) => return Err(error)
                        }
                    }
                },
                Err(error) => return Err(error)
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

use std::{
    fs::{canonicalize, exists}, 
    io::{Error, ErrorKind, Result}
};
use crate::common::*;

pub fn trash_restore(path: String, overwrite: bool) -> Result<()> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            let trash_files_dir = freedesktop_home_trash_files_dir().unwrap();
            let full_trash_file_path = trash_files_dir.join(&path);

            match canonicalize(full_trash_file_path) {
                Ok(normalized_full_path) => {
                    match exists(&normalized_full_path) {
                        Ok(true) => {
                            match normalized_full_path.parent() {
                                Some(parent_path) => {
                                    if parent_path.eq(&trash_files_dir) {
                                        match normalized_full_path.file_name() {
                                            Some(filename) => {
                                                Ok(())
                                            },
                                            None => Err(Error::new(ErrorKind::InvalidInput, "not a valid path"))
                                        }
                                    } else {
                                        Err(Error::new(ErrorKind::Other, "file must be at root of trash"))
                                    }
                                },
                                None => Err(Error::new(ErrorKind::NotADirectory, "cannot access /"))
                            }
                        },
                        Ok(false) => Err(Error::new(ErrorKind::NotFound, format!("{} is not in the trash", &path))),
                        Err(error) => Err(error)
                    }
                },
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}
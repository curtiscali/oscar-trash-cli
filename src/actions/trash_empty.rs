use std::{
    fs::{
        exists, 
        remove_dir_all
    }, 
    io::Result
};

use crate::common::{
    create_home_trash_files_dir_if_not_exists, 
    create_home_trash_info_dir_if_not_exists, 
    freedesktop_home_trash_files_dir, 
    freedesktop_home_trash_info_dir
};

fn remove_all_trashinfo_files() -> Result<()> {
    if let Some(home_trash_info_dir) = freedesktop_home_trash_info_dir() {
        match exists(&home_trash_info_dir) {
            Ok(false) => Ok(()),
            Ok(true) => match remove_dir_all(&home_trash_info_dir) {
                Ok(_) => match create_home_trash_info_dir_if_not_exists() {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error)
                },
                Err(error) => Err(error)
            },
            Err(error) => Err(error)
        }
    } else {
        Ok(())
    }
}

fn remove_all_trash_files() -> Result<()> {
    if let Some(home_trash_files_dir) = freedesktop_home_trash_files_dir() {
        match exists(&home_trash_files_dir) {
            Ok(false) => Ok(()),
            Ok(true) => match remove_dir_all(&home_trash_files_dir) {
                Ok(_) => match create_home_trash_files_dir_if_not_exists() {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error)
                },
                Err(error) => Err(error)
            },
            Err(error) => Err(error)
        }
    } else {
        Ok(())
    }
}

pub fn trash_empty() -> Result<()> {
    match remove_all_trash_files() {
        Ok(_) => remove_all_trashinfo_files(),
        Err(error) => Err(error)
    }
}
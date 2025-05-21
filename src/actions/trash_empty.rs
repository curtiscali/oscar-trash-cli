use std::{
    fs::{
        exists, 
        read_dir, 
        remove_dir_all, 
        remove_file
    }, 
    io::Result,
    path::Path
};

use crate::common::{
    freedesktop_home_trash_files_dir, 
    freedesktop_home_trash_info_dir
};

fn rm_dir_contents(path: &Path) -> Result<()> {
    if exists(path)? {
        for dir_entry in read_dir(path)? {
            if let Ok(dir_entry) = dir_entry {
                if dir_entry.path().is_dir() {
                    remove_dir_all(dir_entry.path())?;
                } else {
                    remove_file(dir_entry.path())?;
                }
            }
        }
    }

    Ok(())
}

fn remove_all_trashinfo_files() -> Result<()> {
    if let Some(home_trash_info_dir) = freedesktop_home_trash_info_dir() {
        rm_dir_contents(&home_trash_info_dir)
    } else {
        Ok(())
    }
}

fn remove_all_trash_files() -> Result<()> {
    if let Some(home_trash_files_dir) = freedesktop_home_trash_files_dir() {
        rm_dir_contents(&home_trash_files_dir)
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
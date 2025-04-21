use std::{
    fs::{exists, remove_dir_all, remove_file}, 
    io::{Error, ErrorKind, Result}
};

use inquire::{Confirm, InquireError};
use crate::{common::*, trash_info::TrashInfo};

fn remove_trash_entry(trash_entry: &TrashInfo) -> Result<()> {
    let (trash_info_dir, trash_files_dir) = (
        freedesktop_home_trash_info_dir().unwrap(), 
        freedesktop_home_trash_files_dir().unwrap()
    );

    let full_trash_info_path = with_trashinfo_extension(&trash_info_dir.join(&trash_entry.path));
    let full_trash_item_path = trash_files_dir.join(&trash_entry.path);

    match full_trash_item_path.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                match remove_dir_all(&full_trash_item_path) {
                    Ok(_) => remove_file(&full_trash_info_path),
                    Err(error) => Err(error)
                }
            } else {
                match remove_file(&full_trash_item_path) {
                    Ok(_) => remove_file(&full_trash_info_path),
                    Err(error) => Err(error)
                }
            }
        },
        Err(error) => Err(error)
    }
}

pub fn trash_remove(trash_entry: &TrashInfo) -> Result<()> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            let trash_files_dir = freedesktop_home_trash_files_dir().unwrap();
            let full_trash_file_path = trash_files_dir.join(&trash_entry.path);

            match exists(full_trash_file_path) {
                Ok(true) => {
                    let message = format!("Are you sure you want to delete {}? This action is irreversible.", trash_entry.path.as_str());
                    let should_rm_from_trash_result = Confirm::new(&message.as_str())
                            .with_default(false)
                            .prompt();

                    match should_rm_from_trash_result {
                        Ok(true) => remove_trash_entry(trash_entry),
                        Ok(false) => Ok(()),
                        Err(error) => match error {
                            InquireError::OperationCanceled => Ok(()),
                            InquireError::OperationInterrupted => Ok(()),
                            _ => Err(Error::new(ErrorKind::Other, error.to_string()))
                        }
                    }
                },
                Ok(false) => Err(Error::new(ErrorKind::NotFound, format!("{} is not in the trash", &trash_entry.path))),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}
use std::{
    fs::{canonicalize, exists, remove_file, rename}, 
    io::{Error, ErrorKind, Result}, path::Path
};
use crate::{common::*, constants::TRASH_INFO_FILE_EXTENSION, trash_info::TrashInfo};

fn restore_from_trash(trash_entry: &TrashInfo) -> Result<()> {
    let (trash_info_dir, trash_files_dir) = (
        freedesktop_home_trash_info_dir().unwrap(), 
        freedesktop_home_trash_files_dir().unwrap()
    );

    let original_file = Path::new(&trash_entry.path);

    let full_trash_info_path = trash_info_dir.join(&trash_entry.path)
        .with_extension(match original_file.extension() {
            Some(extension) => format!("{}.{}", extension.to_str().unwrap(), TRASH_INFO_FILE_EXTENSION),
            None => String::from(TRASH_INFO_FILE_EXTENSION)
        });

    let full_trash_item_path = trash_files_dir.join(&trash_entry.path);

    match rename(&full_trash_item_path, &trash_entry.full_path) {
        Ok(_) => remove_file(full_trash_info_path),
        Err(error) => Err(error)
    }
}

pub fn trash_restore(trash_entry: &TrashInfo, overwrite: bool) -> Result<()> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            let trash_files_dir = freedesktop_home_trash_files_dir().unwrap();

            let full_trash_file_path = trash_files_dir.join(&trash_entry.path);

            println!("{}", full_trash_file_path.to_str().unwrap());

            match exists(full_trash_file_path) {
                Ok(true) => {
                    match exists(&trash_entry.full_path) {
                        Ok(true) => {
                            if overwrite {
                                restore_from_trash(trash_entry)
                            } else {
                                Err(Error::new(ErrorKind::PermissionDenied, format!("{} already exists", &trash_entry.full_path)))
                            }
                        },
                        Ok(false) => restore_from_trash(trash_entry),
                        Err(error) => Err(error)
                    }
                },
                Ok(false) => Err(Error::new(ErrorKind::NotFound, format!("{} is not in the trash", &trash_entry.path))),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}
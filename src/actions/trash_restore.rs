use std::{
    fs::{exists, remove_file, rename}, 
    io::{Error, ErrorKind, Result}
};
use crate::{common::*, trash_info::TrashInfo};

fn restore_from_trash(trash_entry: &TrashInfo) -> Result<()> {
    let (trash_info_dir, trash_files_dir) = (
        freedesktop_home_trash_info_dir().unwrap(), 
        freedesktop_home_trash_files_dir().unwrap()
    );

    let full_trash_info_path = with_trashinfo_extension(&trash_info_dir.join(&trash_entry.path));
    let full_trash_item_path = trash_files_dir.join(&trash_entry.path);

    rename(&full_trash_item_path, &trash_entry.full_path)?;
    remove_file(full_trash_info_path)?;

    Ok(())
}

pub fn trash_restore(trash_entry: &TrashInfo, overwrite: bool) -> Result<()> {
    create_home_trash_dir_if_not_exists()?;

    let trash_files_dir = freedesktop_home_trash_files_dir().unwrap();
    let full_trash_file_path = trash_files_dir.join(&trash_entry.path);

    let file_exists_in_trash = exists(full_trash_file_path)?;
    if file_exists_in_trash {
        let does_full_path_exist = exists(&trash_entry.full_path)?;
        if does_full_path_exist {
            if overwrite {
                restore_from_trash(trash_entry)
            } else {
                Err(Error::new(ErrorKind::PermissionDenied, format!("{} already exists", &trash_entry.full_path)))
            }
        } else {
            restore_from_trash(trash_entry)
        }
    } else {
        restore_from_trash(trash_entry)
    }
}
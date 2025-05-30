use std::{
    fs::{
        exists,
        remove_dir_all,
        remove_file,
    }, 
    io::{
        Error,
        ErrorKind,
        Result
    },
};

use crate::{common::*, trash_info::TrashInfo};

fn remove_trash_entry(trash_entry: &TrashInfo) -> Result<()> {
    let (trash_info_dir, trash_files_dir) = (
        freedesktop_home_trash_info_dir().unwrap(), 
        freedesktop_home_trash_files_dir().unwrap()
    );

    let full_trash_info_path = with_trashinfo_extension(&trash_info_dir.join(&trash_entry.path));
    let full_trash_item_path = trash_files_dir.join(&trash_entry.path);

    let metadata = full_trash_item_path.metadata()?;

    if metadata.is_dir() {
        remove_dir_all(&full_trash_item_path)?;
        remove_file(&full_trash_info_path)?;

        Ok(())
    } else {
        remove_file(&full_trash_item_path)?;
        remove_file(&full_trash_info_path)?;

        Ok(())
    }
}

pub fn trash_remove(trash_entry: &TrashInfo) -> Result<()> {
    create_home_trash_dir_if_not_exists()?;

    let trash_files_dir = freedesktop_home_trash_files_dir().unwrap();
    let full_trash_file_path = trash_files_dir.join(&trash_entry.path);

    match exists(full_trash_file_path) {
        Ok(true) => remove_trash_entry(trash_entry),
        Ok(false) => Err(Error::new(ErrorKind::NotFound, format!("{} is not in the trash", &trash_entry.path))),
        Err(error) => Err(error)
    }
}
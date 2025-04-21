use std::io::Result;

use crate::{common::*, constants::TRASH_INFO_FILE_EXTENSION, trash_info::TrashInfo};

pub fn trash_remove(trash_entry: &TrashInfo) -> Result<()> {
    match create_trash_dir_if_not_exists() {
        Ok(_) => {
            Ok(())
        },
        Err(error) => Err(error)
    }
}
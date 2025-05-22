use std::{
    fs::{
        canonicalize,
        exists,
        rename,
    },
    io::{
        Error,
        ErrorKind,
        Result,
    },
    path::PathBuf,
};

use chrono::Local;
use configparser::ini::Ini;

use crate::{
    common::*,
    constants::*,
    string_encode::encode_filename,
};

fn create_trash_info_entry(path: &PathBuf) -> Result<()> {
    create_home_trash_dir_if_not_exists()?;

    let trash_info_section_header = String::from(TRASH_INFO_SECTION_HEADER);
    let mut trash_info_ini = Ini::new();

    let now = Local::now();

    trash_info_ini.set(
        &trash_info_section_header, 
        TRASH_INFO_PATH_KEY, 
        Some(String::from(encode_filename(path.to_str().unwrap())))
    );

    trash_info_ini.set(
        &trash_info_section_header, 
        TRASH_INFO_DELETION_DATE_KEY, 
        Some(now.format("%Y-%m-%dT%H:%M:%S").to_string())
    );

    if let Some(filename) = path.file_name() {
        let trash_info_directory = freedesktop_home_trash_info_dir().unwrap();
        let trash_info_ini_path = with_trashinfo_extension(&trash_info_directory.join(filename));

        trash_info_ini.write(trash_info_ini_path)
    } else {
        Err(Error::new(ErrorKind::InvalidInput, format!("Cannot place {} in trash", path.display())))
    }
}

pub fn trash_put(path: &String) -> Result<()> {
    create_home_trash_dir_if_not_exists()?;

    let os_absolute_path = canonicalize(path)?;
    let os_path_exists = exists(&os_absolute_path)?;

    if os_path_exists {
        match create_trash_info_entry(&os_absolute_path) {
            Ok(_) => {
                let trash_files_directory = freedesktop_home_trash_files_dir().unwrap();
                match os_absolute_path.file_name() {
                    Some(filename) => {
                        match rename(path, trash_files_directory.join(filename)) {
                            Ok(_) => Ok(()),
                            Err(error) => Err(error)
                        }
                    },
                    None => Err(Error::new(ErrorKind::InvalidInput, format!("Cannot place {} in trash", path)))
                }
            },
            Err(error) => Err(error)
        }
    } else {
        Err(Error::new(ErrorKind::NotFound, format!("{} does not exist", path)))
    }
}
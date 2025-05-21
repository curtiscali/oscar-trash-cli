use std::fs::read_dir;
use std::path::Path;
use std::process::Command;
use std::io::{Error, ErrorKind, Result};

use common::setup_xdg_data_home;
use oscar::{
    actions::trash_empty::trash_empty,
    common::*
};

mod common;

fn count_dir_items(path: &Path) -> Result<usize> {
    match read_dir(path) {
        Ok(entries) => Ok(entries.count()),
        Err(error) => Err(error)
    }
}

fn create_home_trash_hierarchy() -> Result<()> {
    setup_xdg_data_home();

    if let Some(trash_info_dir) = freedesktop_home_trash_info_dir() {
        let mk_trash_info_dir_cmd_res = Command::new("mkdir")
            .arg("-p")
            .arg(trash_info_dir)
            .output();

        match mk_trash_info_dir_cmd_res {
            Ok(_) => {
                if let Some(trash_files_dir) = freedesktop_home_trash_files_dir() {
                    let mk_trash_files_dir_cmd_res = Command::new("mkdir")
                        .arg("-p")
                        .arg(trash_files_dir)
                        .output();

                    match mk_trash_files_dir_cmd_res {
                        Ok(_) => Ok(()),
                        Err(error) => Err(error)
                    }
                } else {
                    Err(Error::new(ErrorKind::Other, "Failed to compute trash directory"))
                }
            },
            Err(error) => Err(error)
        }
    } else {
        Err(Error::new(ErrorKind::Other, "Failed to compute trash directory"))
    }
}

#[test]
fn test_trash_empty() -> Result<()> {
    match create_home_trash_hierarchy() {
        Ok(_) => match trash_empty() {
            Ok(_) => {
                if let Some(_) = freedesktop_home_trash_dir() {
                    let home_trash_files_dir = freedesktop_home_trash_files_dir().unwrap();
                    let home_trash_info_dir = freedesktop_home_trash_info_dir().unwrap();

                    let file_items_count = count_dir_items(&home_trash_files_dir)?;
                    let info_items_count = count_dir_items(&home_trash_info_dir)?;

                    if file_items_count == 0 && info_items_count == 0 {
                        Ok(())
                    } else {
                        Err(Error::new(ErrorKind::Other, "Trash directories are not empty"))
                    }
                } else {
                    Err(Error::new(ErrorKind::Other, "Failed to locate the home trash directory"))
                }
            },
            Err(error) => Err(error)
        },
        Err(error) => Err(error)
    }
}
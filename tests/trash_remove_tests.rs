use std::{fs::exists, io::{Error, ErrorKind, Result}, path::Path, process::Command};

use common::{setup_xdg_data_home, test_file_trash_entry};
use configparser::ini::Ini;
use oscar::{actions::trash_remove::trash_remove, common::{freedesktop_home_trash_files_dir, freedesktop_home_trash_info_dir, with_trashinfo_extension}};
use serial_test::serial;

mod common;

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

fn setup_home_trash(is_file: bool) -> Result<()> {
    let test_file = if is_file { 
        String::from("test.txt") 
    } else { 
        String::from("test") 
    };

    match create_home_trash_hierarchy() {
        Ok(_) => {
            let (trash_info_dir, trash_files_dir) = (
                freedesktop_home_trash_info_dir().unwrap(),
                freedesktop_home_trash_files_dir().unwrap()
            );

            let create_trash_test_file = if is_file {
                Command::new("touch")
                    .arg(&trash_files_dir.join(&test_file))
                    .output()
            } else {
                Command::new("mkdir")
                    .arg("-p")
                    .arg(&trash_files_dir.join(&test_file))
                    .output()
            };
            if let Ok(_) = create_trash_test_file {
                let mut trashinfo = Ini::new();
                if let Ok(_) = trashinfo.read(format!("[Trash Info]
                    Path=/tmp/{test_file}
                    DeletionDate=2004-08-31T22:32:08"
                )) {
                    if let Ok(_) = trashinfo.write(
                        &trash_info_dir.join(with_trashinfo_extension(&Path::new(&test_file).to_path_buf()))
                    ) {
                        Ok(())
                    } else {
                        Err(Error::new(ErrorKind::Other, "Failed to create trashinfo"))
                    }
                } else {
                    Err(Error::new(ErrorKind::Other, "Failed to parse"))
                }
            } else {
                Err(Error::new(ErrorKind::Other, "Failed to create test file"))
            }
        },
        Err(error) => Err(error)
    }
}

#[test]
#[serial]
fn test_trash_rm() -> Result<()> {
    const IS_TRASH_ENTRY_FILE: bool = true;

    match setup_home_trash(IS_TRASH_ENTRY_FILE) {
        Ok(_) => {
            let (trash_info_dir, trash_files_dir) = (
                freedesktop_home_trash_info_dir().unwrap(),
                freedesktop_home_trash_files_dir().unwrap()
            );

            let trash_entry_to_rm = test_file_trash_entry(IS_TRASH_ENTRY_FILE);

            match trash_remove(&trash_entry_to_rm) {
                Ok(_) => {
                    match exists(&trash_info_dir.join(with_trashinfo_extension(&Path::new(&trash_entry_to_rm.path).to_path_buf()))) {
                        Ok(true) => Err(Error::new(ErrorKind::Other, "trash info not deleted")),
                        Ok(false) => match exists(&trash_files_dir.join(&trash_entry_to_rm.path)) {
                            Ok(false) => Ok(()),
                            Ok(true) => Err(Error::new(ErrorKind::Other, "trash file not deleted")),
                            Err(err) => Err(err)
                        },
                        Err(err) => Err(err)
                    }
                }
                Err(err) => Err(err)
            }
        },
        Err(error) => Err(error)
    }
}

#[test]
#[serial]
fn test_trash_rmdir() -> Result<()> {
    const IS_TRASH_ENTRY_FILE: bool = false;

    match setup_home_trash(IS_TRASH_ENTRY_FILE) {
        Ok(_) => {
            let (trash_info_dir, trash_files_dir) = (
                freedesktop_home_trash_info_dir().unwrap(),
                freedesktop_home_trash_files_dir().unwrap()
            );

            let trash_entry_to_rm = test_file_trash_entry(IS_TRASH_ENTRY_FILE);

            match trash_remove(&trash_entry_to_rm) {
                Ok(_) => {
                    match exists(&trash_info_dir.join(with_trashinfo_extension(&Path::new(&trash_entry_to_rm.path).to_path_buf()))) {
                        Ok(true) => Err(Error::new(ErrorKind::Other, "trash info not deleted")),
                        Ok(false) => match exists(&trash_files_dir.join(&trash_entry_to_rm.path)) {
                            Ok(false) => Ok(()),
                            Ok(true) => Err(Error::new(ErrorKind::Other, "trash file not deleted")),
                            Err(err) => Err(err)
                        },
                        Err(err) => Err(err)
                    }
                }
                Err(err) => Err(err)
            }
        },
        Err(error) => Err(error)
    }
}
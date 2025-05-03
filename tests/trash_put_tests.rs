use std::{
    fs::exists, 
    io::{
        Error,
        ErrorKind, 
        Result
    },
    process::{
        Command, 
        Output
    }
};

use common::{
    remove_trash_file_hierarchy, 
    setup_xdg_data_home
};
use oscar::{
    actions::trash_put::trash_put, 
    common::{
        freedesktop_home_trash_files_dir, 
        freedesktop_home_trash_info_dir
    }
};
use serial_test::serial;

mod common;

fn create_test_file() -> Result<Output> {
    Command::new("touch")
        .arg("/tmp/test.txt")
        .output()
}

#[test]
#[serial]
fn test_basic_trash_put() -> Result<()> {
    setup_xdg_data_home();
    
    match create_test_file() {
        Ok(_) => {
            match trash_put(&String::from("/tmp/test.txt")) {
                Ok(_) => {
                    match freedesktop_home_trash_files_dir() {
                        Some(trash_files_dir) => {
                            match exists(trash_files_dir.join("test.txt")) {
                                Ok(trashed_file_exists) => {
                                    if trashed_file_exists {
                                        match freedesktop_home_trash_info_dir() {
                                            Some(trash_info_dir) => {
                                                match exists(trash_info_dir.join("test.txt.trashinfo")) {
                                                    Ok(trash_info_exists) => {
                                                        if trash_info_exists {
                                                            remove_trash_file_hierarchy();
                                                            Ok(())
                                                        } else {
                                                            Err(Error::new(ErrorKind::NotFound, "trashinfo file not created as expected"))
                                                        }
                                                    },
                                                    Err(err) => Err(err)
                                                }
                                            },
                                            None => Err(Error::new(ErrorKind::Other, "Could not determine trash path"))
                                        }
                                    } else {
                                        Err(Error::new(ErrorKind::NotFound, "trashed file not in expected path"))
                                    }
                                },
                                Err(err) => Err(err)
                            }
                        },
                        None => Err(Error::new(ErrorKind::Other, "Could not determine trash path"))
                    }
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

#[test]
#[serial]
fn test_trash_put_nonexistent_file() {
    setup_xdg_data_home();

    match trash_put(&String::from("/tmp/does-not-exist.txt")) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
    }
}
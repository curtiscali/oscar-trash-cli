use std::{
    fs::exists, 
    io::{
        Error, 
        ErrorKind, 
        Result
    },
    path::Path, 
    process::Command
};

use common::{remove_trash_file_hierarchy, setup_xdg_data_home, test_file, test_file_trash_entry};
use configparser::ini::Ini;
use oscar::{
    actions::trash_restore::trash_restore, 
    common::{
        create_home_trash_dir_if_not_exists, freedesktop_home_trash_files_dir, freedesktop_home_trash_info_dir, with_trashinfo_extension
    }
};
use serial_test::serial;

mod common;

fn create_home_trash() -> Result<()> {
    setup_xdg_data_home();
    let test_file = test_file(true);

    match create_home_trash_dir_if_not_exists() {
        Ok(_) => {
            let (
                home_trash_files_dir,
                home_trash_info_dir
            ) = (
                freedesktop_home_trash_files_dir().unwrap(),
                freedesktop_home_trash_info_dir().unwrap()
            );

            let create_test_file_cmd = Command::new("touch")
                .arg(&home_trash_files_dir.join(&test_file))
                .output();

            match create_test_file_cmd {
                Ok(_) => {
                    let mut trashinfo = Ini::new();
                    match trashinfo.read(format!("[Trash Info]
                        Path=/tmp/{test_file}
                        DeletionDate=2004-08-31T22:32:08"
                    )) {
                        Ok(_) => {
                            match trashinfo.write(
                                &home_trash_info_dir.join(with_trashinfo_extension(&Path::new(&test_file).to_path_buf()))
                            ) {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err)
                            }
                        },
                        Err(err) => Err(Error::new(ErrorKind::Other, err))
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
fn test_trash_restore_no_file_in_original_path() -> Result<()> {
    match create_home_trash() {
        Ok(_) => {
            let test_trash_entry = test_file_trash_entry(true);

            match trash_restore(&test_trash_entry, false) {
                Ok(_) => {
                    match exists(Path::new(&test_trash_entry.full_path)) {
                        Ok(exists) => {
                            assert!(exists);
                            remove_trash_file_hierarchy();
                            Ok(())
                        },
                        Err(err) => Err(err)
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
fn test_trash_restore_overwrite_file_in_original_path() -> Result<()> {
    let test_trash_entry = test_file_trash_entry(true);

    let create_file_in_original_dir_cmd = Command::new("touch")
        .arg(format!("/tmp/{}", &test_trash_entry.path))
        .output();

    match create_file_in_original_dir_cmd {
        Ok(_) => {
            match create_home_trash() {
                Ok(_) => match trash_restore(&test_trash_entry, true) {
                    Ok(_) => {
                        match exists(Path::new(&test_trash_entry.full_path)) {
                            Ok(exists) => {
                                assert!(exists);
                                remove_trash_file_hierarchy();
                                Ok(())
                            },
                            Err(err) => Err(err)
                        }
                    },
                    Err(err) => Err(err)
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}

#[test]
#[serial]
fn test_trash_restore_no_overwrite_file_in_original_path() -> Result<()> {
    let test_trash_entry = test_file_trash_entry(true);

    let create_file_in_original_dir_cmd = Command::new("touch")
        .arg(format!("/tmp/{}", &test_trash_entry.path))
        .output();

    match create_file_in_original_dir_cmd {
        Ok(_) => {
            match create_home_trash() {
                Ok(_) => match trash_restore(&test_trash_entry, false) {
                    Ok(_) => Err(Error::new(ErrorKind::Other, "Trash restore was not supposed to overwrite")),
                    Err(_) => Ok(())
                },
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(err)
    }
}
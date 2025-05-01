use std::{
    fs::{create_dir_all, exists}, 
    path::Path,
    process::Command
};
use chrono::NaiveDate;
use configparser::ini::Ini;
use oscar::{common::*, trash_info::TrashInfo};
use serial_test::{parallel, serial};
use common::*;

mod common;

#[test]
#[serial(env_var)]
fn test_home_trash_dir_location_with_xdg_data_home() {
    remove_home();
    setup_xdg_data_home();
    assert_eq!(freedesktop_home_trash_dir(), Some(Path::new("/tmp/Trash").to_path_buf()));
}

#[test]
#[serial(env_var)]
fn test_home_trash_info_dir_location_with_xdg_data_home() {
    remove_home();
    setup_xdg_data_home();
    assert_eq!(freedesktop_home_trash_info_dir(), Some(Path::new("/tmp/Trash/info").to_path_buf()));
}

#[test]
#[serial(env_var)]
fn test_home_trash_files_dir_location_with_xdg_data_home() {
    remove_home();
    setup_xdg_data_home();
    assert_eq!(freedesktop_home_trash_files_dir(), Some(Path::new("/tmp/Trash/files").to_path_buf()));
}

#[test]
#[serial(env_var)]
fn test_home_trash_dir_location_without_xdg_data_home() {
    remove_xdg_data_home();
    setup_home();
    assert_eq!(freedesktop_home_trash_dir(), Some(Path::new("/tmp/.local/share/Trash").to_path_buf()));
}

#[test]
#[serial(env_var)]
fn test_home_trash_info_dir_location_without_xdg_data_home() {
    remove_xdg_data_home();
    setup_home();
    assert_eq!(freedesktop_home_trash_info_dir(), Some(Path::new("/tmp/.local/share/Trash/info").to_path_buf()));
}

#[test]
#[serial(env_var)]
fn test_home_trash_files_dir_location_without_xdg_data_home() {
    remove_xdg_data_home();
    setup_home();
    assert_eq!(freedesktop_home_trash_files_dir(), Some(Path::new("/tmp/.local/share/Trash/files").to_path_buf()));
}

#[test]
#[parallel]
fn test_add_trashinfo_extension_to_file_with_extension() {
    let test_file = Path::new("test.txt").to_path_buf();
    assert_eq!(with_trashinfo_extension(&test_file), Path::new("test.txt.trashinfo").to_path_buf());
}

#[test]
#[parallel]
fn test_add_trashinfo_extension_to_file_without_extension() {
    let test_file = Path::new("test").to_path_buf();
    assert_eq!(with_trashinfo_extension(&test_file), Path::new("test.trashinfo").to_path_buf());
}

#[test]
#[serial(fs)]
fn test_create_home_trash_info_dir_if_not_exists() {
    setup_xdg_data_home();

    if let Ok(_) = create_home_trash_info_dir_if_not_exists() {
        if let Some(home_trash_info_dir) = freedesktop_home_trash_info_dir() {
            if let Ok(dir_exists) = exists(&home_trash_info_dir) {
                assert!(dir_exists);
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    } else {
        assert!(false)
    }

    remove_trash_file_hierarchy();
}

#[test]
#[serial(fs)]
fn test_create_home_trash_files_dir_if_not_exists() {
    setup_xdg_data_home();

    if let Ok(_) = create_home_trash_files_dir_if_not_exists() {
        if let Some(home_trash_files_dir) = freedesktop_home_trash_files_dir() {
            if let Ok(dir_exists) = exists(&home_trash_files_dir) {
                assert!(dir_exists);
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    } else {
        assert!(false)
    }

    remove_trash_file_hierarchy();
}

#[test]
#[serial(env_var, fs)]
fn test_get_home_trash_contents() {
    let test_file = String::from("test.txt");

    setup_xdg_data_home();
    if let Some(home_trash_info_dir) = freedesktop_home_trash_info_dir() {
        if let Ok(_) = create_dir_all(&home_trash_info_dir) {
            if let Some(home_trash_files_dir) = freedesktop_home_trash_files_dir() {
                if let Ok(_) = create_dir_all(&home_trash_files_dir) {
                    let _ = Command::new("touch")
                        .arg(&home_trash_files_dir.join(&test_file))
                        .output();

                    let mut trashinfo = Ini::new();
                    if let Ok(_) = trashinfo.read(format!("[Trash Info]
                        Path=/tmp/{test_file}
                        DeletionDate=2004-08-31T22:32:08"
                    )) {
                        if let Ok(_) = trashinfo.write(
                            &home_trash_info_dir.join(with_trashinfo_extension(&Path::new(&test_file).to_path_buf()))
                        ) {
                            if let Ok(trash_contents) = get_home_trash_contents() {
                                assert_eq!(
                                    trash_contents, 
                                    vec![TrashInfo { 
                                        path: test_file.clone(), 
                                        full_path: format!("/tmp/{}", test_file.clone()), 
                                        deletion_date: NaiveDate::from_ymd_opt(2004, 8, 31)
                                            .unwrap()
                                            .and_hms_opt(22, 32, 8)
                                            .unwrap()
                                    }]
                                );
                                remove_trash_file_hierarchy();
                            } else {
                                assert!(false)
                            }
                        } else {
                            assert!(false)
                        }
                    } else {
                        assert!(false)
                    }
                } else {
                    assert!(false)
                }
            }
        } else {
            assert!(false)
        }
    }
}
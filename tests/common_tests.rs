use std::{env::{remove_var, set_var, temp_dir}, fs::exists, path::Path, process::Command};
use oscar::common::*;

fn setup_xdg_data_home() {
    set_var("XDG_DATA_HOME", temp_dir());
}

fn remove_xdg_data_home() {
    remove_var("XDG_DATA_HOME");
}

fn setup_home() {
    set_var("HOME", temp_dir());
}

fn remove_home() {
    remove_var("HOME");
}

fn remove_trash_file_hierarchy() {
    if let Some(home_trash_dir) = freedesktop_home_trash_dir() {
        let _ = Command::new("rm")
            .arg("-rf")
            .arg(home_trash_dir)
            .output();
    }
}

#[test]
fn test_home_trash_dir_location_with_xdg_data_home() {
    remove_home();
    setup_xdg_data_home();
    assert_eq!(freedesktop_home_trash_dir(), Some(Path::new("/tmp/Trash").to_path_buf()));
}

#[test]
fn test_home_trash_info_dir_location_with_xdg_data_home() {
    remove_home();
    setup_xdg_data_home();
    assert_eq!(freedesktop_home_trash_info_dir(), Some(Path::new("/tmp/Trash/info").to_path_buf()));
}

#[test]
fn test_home_trash_files_dir_location_with_xdg_data_home() {
    remove_home();
    setup_xdg_data_home();
    assert_eq!(freedesktop_home_trash_files_dir(), Some(Path::new("/tmp/Trash/files").to_path_buf()));
}

#[test]
fn test_home_trash_dir_location_without_xdg_data_home() {
    remove_xdg_data_home();
    setup_home();
    assert_eq!(freedesktop_home_trash_dir(), Some(Path::new("/tmp/.local/share/Trash").to_path_buf()));
}

#[test]
fn test_home_trash_info_dir_location_without_xdg_data_home() {
    remove_xdg_data_home();
    setup_home();
    assert_eq!(freedesktop_home_trash_info_dir(), Some(Path::new("/tmp/.local/share/Trash/info").to_path_buf()));
}

#[test]
fn test_home_trash_files_dir_location_without_xdg_data_home() {
    remove_xdg_data_home();
    setup_home();
    assert_eq!(freedesktop_home_trash_files_dir(), Some(Path::new("/tmp/.local/share/Trash/files").to_path_buf()));
}

#[test]
fn test_add_trashinfo_extension_to_file_with_extension() {
    let test_file = Path::new("test.txt").to_path_buf();
    assert_eq!(with_trashinfo_extension(&test_file), Path::new("test.txt.trashinfo").to_path_buf());
}

#[test]
fn test_add_trashinfo_extension_to_file_without_extension() {
    let test_file = Path::new("test").to_path_buf();
    assert_eq!(with_trashinfo_extension(&test_file), Path::new("test.trashinfo").to_path_buf());
}

#[test]
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
use std::{
    env::{set_var, temp_dir},
    process::Command
};

use chrono::{NaiveDate, NaiveDateTime};
use oscar::{common::freedesktop_home_trash_dir, trash_info::TrashInfo};

pub fn setup_xdg_data_home() {
    set_var("XDG_DATA_HOME", temp_dir());
}

pub fn remove_trash_file_hierarchy() {
    if let Some(home_trash_dir) = freedesktop_home_trash_dir() {
        let _ = Command::new("rm")
            .arg("-rf")
            .arg(home_trash_dir)
            .output();
    }
}

pub fn test_file_date() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2004, 8, 31)
        .unwrap()
        .and_hms_opt(22, 32, 8)
        .unwrap()
}

pub fn test_file(is_file: bool) -> String {
    format!("test{}", if is_file { ".txt" } else { "" })
    //String::from("test.txt")
}

pub fn test_file_trash_entry(is_file: bool) -> TrashInfo {
    TrashInfo {
        path: test_file(is_file), 
        full_path: format!("/tmp/{}", test_file(is_file)), 
        deletion_date: test_file_date()
    }
}
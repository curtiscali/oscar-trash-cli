use std::{
    env::{remove_var, set_var, temp_dir},
    process::Command
};

use oscar::common::freedesktop_home_trash_dir;

pub fn setup_xdg_data_home() {
    set_var("XDG_DATA_HOME", temp_dir());
}

pub fn remove_xdg_data_home() {
    remove_var("XDG_DATA_HOME");
}

pub fn setup_home() {
    set_var("HOME", temp_dir());
}

pub fn remove_home() {
    remove_var("HOME");
}

pub fn remove_trash_file_hierarchy() {
    if let Some(home_trash_dir) = freedesktop_home_trash_dir() {
        let _ = Command::new("rm")
            .arg("-rf")
            .arg(home_trash_dir)
            .output();
    }
}
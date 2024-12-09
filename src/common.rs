use std::path::PathBuf;
use dirs::data_dir;

/// This function gets the home trash directory as defined in the Freedesktop.org spec: https://specifications.freedesktop.org/trash-spec/latest/
pub fn freedesktop_home_trash_dir() -> Option<PathBuf> {
    match data_dir() {
        Some(path) => Some(path.join("Trash")),
        None => None
    }
}

pub fn freedesktop_home_trash_files_dir() -> Option<PathBuf> {
    match freedesktop_home_trash_dir() {
        Some(home_trash_dir) => Some(home_trash_dir.join("files")),
        None => None
    }
}

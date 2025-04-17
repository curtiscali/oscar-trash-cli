use std::{fmt::Display, fs::read_to_string, path::{Path, PathBuf}};
use chrono::NaiveDateTime;
use configparser::ini::Ini;
use tabled::Tabled;
use crate::{constants::*, string_encode::decode_filename};

#[derive(Tabled)]
#[tabled(rename_all = "CamelCase")]
pub struct TrashInfo {
    #[tabled(rename = "Path")]
    pub path: String,

    #[tabled(rename = "Deletion Date")]
    pub deletion_date: NaiveDateTime
}

impl TrashInfo {
    pub fn from_file(path: PathBuf) -> Option<TrashInfo> {
        let (trash_info_header, path_field, deletion_date_field) = (
            String::from(TRASH_INFO_SECTION_HEADER).to_lowercase(),
            String::from(TRASH_INFO_PATH_KEY).to_lowercase(),
            String::from(TRASH_INFO_DELETION_DATE_KEY).to_lowercase()
        );

        match read_to_string(path) {
            Ok(file_contents) => {
                let mut ini = Ini::new();
                match ini.read(file_contents) {
                    Ok(map) => {
                        match map.get(&trash_info_header) {
                            Some(trash_info_section) => {
                                let full_path = match trash_info_section.get(&path_field) {
                                    Some(path_val) => path_val.clone(),
                                    None => None
                                };

                                let deletion_date_str = match trash_info_section.get(&deletion_date_field) {
                                    Some(deletion_date_val) => deletion_date_val.clone(),
                                    None => None
                                };

                                if full_path.is_some() && deletion_date_str.is_some() {
                                    let deletion_date = NaiveDateTime::parse_from_str(
                                        deletion_date_str.unwrap().as_str(),
                                        "%Y-%m-%dT%H:%M:%S"
                                    );

                                    match deletion_date {
                                        Ok(date_result) => Some(TrashInfo { 
                                            path: decode_filename(
                                                Path::new(&full_path.unwrap())
                                                    .file_name()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap()
                                            ), 
                                            deletion_date: date_result
                                        }) ,
                                        Err(_) => None
                                    }
                                } else {
                                    return None;
                                }
                            },
                            None => None
                        }
                    },
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}

impl Display for TrashInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
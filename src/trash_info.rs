use std::{fmt::Display, fs::read_to_string, io::{Error, ErrorKind, Result}, path::{Path, PathBuf}};
use chrono::NaiveDateTime;
use configparser::ini::Ini;
use tabled::Tabled;
use crate::{constants::*, string_encode::decode_filename};

fn from_option(s: &Option<String>) -> String {
    s.clone().unwrap()
}

#[derive(Tabled, Debug, PartialEq)]
#[tabled(rename_all = "CamelCase")]
pub struct TrashInfo {
    #[tabled(skip)]
    pub full_path: String,

    #[tabled(rename = "Path")]
    pub path: String,

    #[tabled(rename = "Deletion Date")]
    pub deletion_date: NaiveDateTime
}

impl TrashInfo {
    pub fn from_file(path: PathBuf) -> Result<TrashInfo> {
        let (trash_info_header, path_field, deletion_date_field) = (
            String::from(TRASH_INFO_SECTION_HEADER).to_lowercase(),
            String::from(TRASH_INFO_PATH_KEY).to_lowercase(),
            String::from(TRASH_INFO_DELETION_DATE_KEY).to_lowercase()
        );

        let file_contents = read_to_string(path)?;

        let mut ini = Ini::new();
        if let Ok(map) = ini.read(file_contents) {
            if let Some(trash_info_section) = map.get(&trash_info_header) {
                let full_path = match trash_info_section.get(&path_field) {
                    Some(path_val) => path_val.clone(),
                    None => None
                };

                let deletion_date_str = match trash_info_section.get(&deletion_date_field) {
                    Some(deletion_date_val) => deletion_date_val.clone(),
                    None => None
                };

                if full_path.is_some() && deletion_date_str.is_some() {
                    let deletion_date_result = NaiveDateTime::parse_from_str(
                        deletion_date_str.unwrap().as_str(),
                        "%Y-%m-%dT%H:%M:%S"
                    );

                    match deletion_date_result {
                        Ok(date) => Ok(TrashInfo { 
                            full_path: from_option(&full_path),
                            path: decode_filename(
                                Path::new(&from_option(&full_path))
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                            ), 
                            deletion_date: date
                        }),
                        Err(err) => Err(Error::new(ErrorKind::Other, err.to_string()))
                    }
                } else {
                    Err(Error::new(ErrorKind::Other, "Trash info missing full path or date"))
                }
            } else {
                Err(Error::new(ErrorKind::Other, "Trash info file invalid format"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Failed to read trash info file"))
        }
    }
}

impl Display for TrashInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
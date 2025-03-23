// use std::{fs::{read_dir, Metadata, metadata}, io::{Error, ErrorKind}, path::PathBuf};
// use clap::error;
// use crate::common::{create_trash_dir_if_not_exists, freedesktop_home_trash_dir, freedesktop_home_trash_files_dir};

// fn directory_sizes_cache_file_path() -> Option<PathBuf> {
//     match freedesktop_home_trash_dir() {
//         Some(home_trash_dir) => Some(home_trash_dir.join("directorysizes")),
//         None => None
//     }
// }

// fn traverse_trash_dir(show_sizes_only: bool) -> Result<Vec<Result<Metadata, Error>>, Error> {
//     fn traverse_trash_dir_rec(show_sizes_only: bool, dir: PathBuf, trash_contents_data: &mut Vec<Result<Metadata, Error>>) -> Result<Vec<Result<Metadata, Error>>, Error> {
//         match read_dir(dir) {
//             Ok(dir_contents) => {
//                 for rd in dir_contents {
//                     match rd {
//                         Ok(entry) => {
//                             let path = entry.path();
//                             if path.is_dir() {
//                                 traverse_trash_dir_rec(show_sizes_only, path, &mut trash_contents_data);
//                             } else if path.is_file() {
//                                 match metadata(path) {
//                                     Ok(data) => trash_contents_data.push(Ok(data)),
//                                     Err(error) => trash_contents_data.push(Err(error))
//                                 }
//                             }
//                         },
//                         Err(error) => trash_contents_data.push(Err(error)),
//                     }
//                 }

//                 let mut result = vec![];
//                 result.clone_from_slice(&trash_contents_data[0..]);
//                 return Ok(result);
//             },
//             Err(error) => Err(error)
//         }   
//     }

//     match freedesktop_home_trash_files_dir() {
//         Some(home_trash_files_dir) => {
//             let mut metadata = vec![];
//             match traverse_trash_dir_rec(show_sizes_only, home_trash_files_dir, &mut metadata) {
//                 Ok(trash_contents) => Ok(trash_contents),
//                 Err(error) => Err(error)
//             }
//         },
//         None => Err(Error::new(ErrorKind::Other, "Unable to determine home trash files dir"))
//     }
// }

// pub fn trash_info(show_sizes_only: bool) -> Result<Vec<Result<Metadata, Error>>, Error> {
//     match create_trash_dir_if_not_exists() {
//         Ok(_) => match traverse_trash_dir(show_sizes_only) {
//             Ok(trash_contents_metadata) => Ok(trash_contents_metadata),
//             Err(error) => Err(error)
//         },
//         Err(error) => Err(error)
//     }
// }

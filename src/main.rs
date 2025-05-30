use std::error::Error;

use oscar::actions::{
    trash_list::trash_list, 
    trash_put::trash_put, 
    trash_remove::trash_remove, 
    trash_restore::trash_restore,
    trash_empty::trash_empty
};
use clap::{Parser, Subcommand};
use oscar::common::get_home_trash_contents;
use inquire::{Confirm, InquireError, Select};

#[derive(Subcommand, Debug)]
enum OscarCommand {
    /// place a file or directories in the system trash
    #[clap(alias = "p")]
    Put {
        /// path to the file or directory to be placed in the trash
        path: String
    },

    /// empty the system trash
    #[clap(alias = "e")]
    Empty {
        #[arg(short, long, default_value_t=false)]
        yes: bool
    },

    /// list all files or directories in the trash
    #[clap(alias = "ls")]
    List {
        /// List trash contents recursively
        #[arg(short, long, default_value_t=false)]
        recursive: bool
    },

    /// restore a file/directory in the trash to its original location
    #[clap(alias = "rs")]
    Restore {
        /// the path of the file, relative to the system trash, to restore its original location
        //path: Option<String>,

        /// Overwrite the file currently on disk if there is a conflict
        #[arg(long, default_value_t=false)]
        overwrite: bool
    },

    /// remove individual files from the trashcan. 
    #[clap(alias = "rm")]
    Remove {
        #[arg(short, long, default_value_t=false)]
        yes: bool
    },
}

/// Command Line tool to manage your system's Freedesktop.org trash
/// written in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: OscarCommand
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.cmd {
        OscarCommand::Put { path } => {
            let should_place_in_trash_result = Confirm::new(format!("Are you sure you want to place {} in the trash?", path).as_str())
                .with_default(false)
                .prompt();

            match should_place_in_trash_result {
                Ok(true) => match trash_put(&path) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(Box::new(error))
                },
                Ok(false) => Ok(()),
                Err(error) => match error {
                    InquireError::OperationCanceled => Ok(()),
                    InquireError::OperationInterrupted => Ok(()),
                    _ => Err(Box::new(error))
                }
            }
        },
        OscarCommand::Empty { yes } => {
            if yes {
                match trash_empty() {
                    Ok(_) => Ok(()),
                    Err(error) => Err(Box::new(error))
                }
            } else {
                let should_empty_trash_result = Confirm::new("Are you sure you want to empty the trash? This action is irreversible.")
                    .with_default(false)
                    .prompt();

                match should_empty_trash_result {
                    Ok(true) => match trash_empty() {
                        Ok(_) => Ok(()),
                        Err(error) => Err(Box::new(error))
                    },
                    Ok(false) => Ok(()),
                    Err(error) => {
                        match error {
                            InquireError::OperationCanceled => Ok(()),
                            InquireError::OperationInterrupted => Ok(()),
                            _ => Err(Box::new(error))
                        }
                    }
                }
            }
        },
        OscarCommand::List { recursive } => {
            match trash_list(recursive) {
                Ok(_) => Ok(()),
                Err(error) => Err(Box::new(error))
            }
        },
        OscarCommand::Restore { overwrite } => {
            match get_home_trash_contents() {
                Ok(trash_contents) => {
                    let user_response = Select::new("Select an item from the trash to restore", trash_contents).prompt();

                    match user_response {
                        Ok(selected_item) => {
                            match trash_restore(&selected_item, overwrite) {
                                Ok(_) => Ok(()),
                                Err(error) => Err(Box::new(error))
                            }
                        },
                        Err(error) => {
                            match error {
                                InquireError::OperationCanceled => Ok(()),
                                InquireError::OperationInterrupted => Ok(()),
                                _ => Err(Box::new(error))
                            }
                        }
                    }
                },
                Err(error) => Err(Box::new(error))
            }
        },
        OscarCommand::Remove { yes } => {
            match get_home_trash_contents() {
                Ok(trash_contents) => {
                    let user_response = Select::new("Select an item from the trash to remove", trash_contents).prompt();

                    match user_response {
                        Ok(selected_item) => {
                            if yes {
                                match trash_remove(&selected_item) {
                                    Ok(_) => Ok(()),
                                    Err(error) => Err(Box::new(error))
                                }
                            } else {
                                let message = format!("Are you sure you want to delete {}? This action is irreversible.", selected_item.path.as_str());
                                let should_rm_from_trash_result = Confirm::new(&message.as_str())
                                    .with_default(false)
                                    .prompt();

                                match should_rm_from_trash_result {
                                    Ok(true) => match trash_remove(&selected_item) {
                                        Ok(_) => Ok(()),
                                        Err(error) => Err(Box::new(error))
                                    },
                                    Ok(false) => Ok(()),
                                    Err(error) => match error {
                                        InquireError::OperationCanceled => Ok(()),
                                        InquireError::OperationInterrupted => Ok(()),
                                        _ => Err(Box::new(error))
                                    }
                                }
                            }
                        },
                        Err(error) => {
                            match error {
                                InquireError::OperationCanceled => Ok(()),
                                InquireError::OperationInterrupted => Ok(()),
                                _ => Err(Box::new(error))
                            }
                        }
                    }
                },
                Err(error) => Err(Box::new(error))
            }
        }
    }
}

use std::error::Error;

use actions::{
    trash_list::trash_list, 
    trash_put::trash_put, 
    trash_remove::trash_remove, 
    trash_restore::trash_restore
};
use clap::{Parser, Subcommand};
use common::get_home_trash_contents;
use inquire::{InquireError, Select};

mod common;
mod actions;
mod constants;
mod string_encode;
mod trash_info;

fn show_cmd_not_yet_implemented() {
    println!("This command has not yet been implemented");
}

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
    Empty {},

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
    Remove {},
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

    // TODO: implement each of these sub commands
    match args.cmd {
        OscarCommand::Put { path } => {
            match trash_put(&path) {
                Ok(_) => Ok(()),
                Err(error) => Err(Box::new(error))
            }
        },
        OscarCommand::Empty {} => {
            show_cmd_not_yet_implemented();
            Ok(())
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
        OscarCommand::Remove {} => {
            match get_home_trash_contents() {
                Ok(trash_contents) => {
                    let user_response = Select::new("Select an item from the trash to remove", trash_contents).prompt();

                    match user_response {
                        Ok(selected_item) => {
                            match trash_remove(&selected_item) {
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
        }
    }
}

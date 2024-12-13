use actions::trash_list::trash_list;
use clap::{Parser, Subcommand};
use common::create_trash_dir_if_not_exists;

mod common;
mod actions;

#[derive(Subcommand, Debug)]
enum OscarCommand {
    /// trash files and directories.
    Put {},

    /// empty the trashcan(s).
    Empty {},

    /// list trashed files.
    List {},

    /// restore a trashed file. 
    Restore {
        /// Overwrite the file currently on disk if there is a conflict
        #[arg(long, default_value_t=false)]
        overwrite: bool
    },

    /// remove individual files from the trashcan. 
    Remove {}
}

/// Command Line tool to manage your system's Freedesktop.org trash
/// written in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, infer_subcommands = true)]
struct Args {
    #[command(subcommand)]
    cmd: OscarCommand
}

fn main() {
    let args = Args::parse();

    // TODO: implement each of these sub commands
    match args.cmd {
        OscarCommand::Put {} => {},
        OscarCommand::Empty {} => {},
        OscarCommand::List {} => {
            match trash_list() {
                Ok(_) => {},
                Err(error) => println!("Error: {}", error)
            }
        },
        OscarCommand::Restore { overwrite } => {},
        OscarCommand::Remove {} => {}
    }
}

use actions::trash_list::trash_list;
use clap::{Parser, Subcommand};

mod common;
mod actions;

fn show_cmd_not_yet_implemented() {
    println!("This command has not yet been implemented");
}

#[derive(Subcommand, Debug)]
enum OscarCommand {
    /// trash files and directories.
    Put {},

    /// empty the trashcan(s).
    Empty {},

    /// list trashed files.
    #[clap(alias = "ls")]
    List {
        /// List trash contents recursively
        #[arg(short, long, default_value_t=false)]
        recursive: bool
    },

    /// restore a trashed file. 
    Restore {
        /// Overwrite the file currently on disk if there is a conflict
        #[arg(long, default_value_t=false)]
        overwrite: bool
    },

    /// remove individual files from the trashcan. 
    #[clap(alias = "rm")]
    Remove {},

    /// gets info on the contents of the trash, including total size, number of files, number of
    /// directories, etc
    Info {
        #[arg(long, default_value_t=false)]
        sizes_only: bool
    }
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
        OscarCommand::Put {} => {
            show_cmd_not_yet_implemented();
        },
        OscarCommand::Empty {} => {
            show_cmd_not_yet_implemented();
        },
        OscarCommand::List { recursive} => {
            match trash_list(recursive) {
                Ok(_) => {},
                Err(error) => println!("Error: {}", error)
            }
        },
        OscarCommand::Restore { overwrite } => {
            show_cmd_not_yet_implemented();
        },
        OscarCommand::Remove {} => {
            show_cmd_not_yet_implemented();
        },
        OscarCommand::Info { sizes_only } => {
            show_cmd_not_yet_implemented();
        }
    }
}

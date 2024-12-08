use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum OscarCommand {
    /// trash files and directories.
    Put {},

    /// empty the trashcan(s).
    Empty {},

    /// list trashed files.
    List {},

    /// restore a trashed file. 
    Restore {},

    /// remove individual files from the trashcan. 
    Remove {}
}

/// Command Line tool to manage your system's Freedesktop.org trash folder
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
        Put => {},
        Empty => {},
        List => {},
        Restore => {},
        Remove => {}
    }
}

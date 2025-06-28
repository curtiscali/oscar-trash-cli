use std::{
    error::Error,
};

use oscar::{
    actions::{
        trash_put::trash_put
    },
    mount::{get_mounted_devices, MountedDevice},
    trash::Trash,
};
use clap::{Parser, Subcommand};
use inquire::{Confirm, InquireError};
use tabled::{settings::Style, Table};

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
        yes: bool,

        /// Empty the trash of a mounted device (e.g. /dev/sda1). By default will empty the home trash. To empty all trashes, use all
        #[arg(short, long)]
        device: Option<String>
    },

    /// Lists all devices where the user can trash files
    ListDevices {},

    /// list all files or directories in the trash
    #[clap(alias = "ls")]
    List {
        /// List trash contents recursively
        #[arg(short, long, default_value_t=false)]
        recursive: bool,

        /// List contents of a mounted device (e.g. /dev/sda1). By default will list contents of the home trash. To list contents of all trashes, use all
        #[arg(short, long)]
        device: Option<String>
    },

    /// restore a file/directory in the trash to its original location
    #[clap(alias = "rs")]
    Restore {
        /// Overwrite the file currently on disk if there is a conflict
        #[arg(long, default_value_t=false)]
        overwrite: bool,

        /// The device in which an item will be restored, if no device is specified the home trash will be used by default
        #[arg(short, long)]
        device: Option<String>
    },

    /// remove individual files from the trashcan. 
    #[clap(alias = "rm")]
    Remove {
        #[arg(short, long, default_value_t=false)]
        yes: bool,

        /// The device from which an item will be removed, if no device is specified the home trash will be used by default
        #[arg(short, long)]
        device: Option<String>
    }
}

/// Command Line tool to manage your system's Freedesktop.org trash
/// written in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: OscarCommand
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let mounted_devices = get_mounted_devices()?;
    let trashes = mounted_devices.iter()
        .map(|mounted_device| Trash::new(mounted_device))
        .collect::<Vec<Trash>>();

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
        OscarCommand::Empty { yes, device } => {
            let selected_device_opt = if let Some(device_name) = device {
                trashes.iter().find(|t| t.device.name.eq(&device_name))
            } else {
                trashes.iter().find(|t| t.device.mount_point.eq("/") || t.device.mount_point.eq("/home"))
            };

            if let Some(selected_device) = selected_device_opt {
                if yes {
                    match selected_device.empty() {
                        Ok(_) => Ok(()),
                        Err(error) => Err(Box::new(error))
                    }
                } else {
                    let should_empty_trash_result = Confirm::new("Are you sure you want to empty the trash? This action is irreversible.")
                        .with_default(false)
                        .prompt();

                    match should_empty_trash_result {
                        Ok(true) => match selected_device.empty() {
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
            } else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Device not found")))
            }
        },
        OscarCommand::List { recursive, device } => {
            let selected_device_opt = if let Some(device_name) = device {
                trashes.iter().find(|t| t.device.name.eq(&device_name))
            } else {
                trashes.iter().find(|t| t.device.mount_point.eq("/") || t.device.mount_point.eq("/home"))
            };

            if let Some(selected_device) = selected_device_opt {
                match selected_device.list(recursive) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err))
                }
            }else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Device not found")))
            }
        },
        OscarCommand::Restore { overwrite, device } => {
            let selected_device_opt = if let Some(device_name) = device {
                trashes.iter().find(|t| t.device.name.eq(&device_name))
            } else {
                trashes.iter().find(|t| t.device.mount_point.eq("/") || t.device.mount_point.eq("/home"))
            };

            if let Some(selected_device) = selected_device_opt {
                match selected_device.restore(overwrite) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err))
                }
            } else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Device not found")))
            }
        },
        OscarCommand::Remove { yes, device } => {
            let selected_device_opt = if let Some(device_name) = device {
                trashes.iter().find(|t| t.device.name.eq(&device_name))
            } else {
                trashes.iter().find(|t| t.device.mount_point.eq("/") || t.device.mount_point.eq("/home"))
            };

            if let Some(selected_device) = selected_device_opt {
                match selected_device.remove(yes) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(Box::new(err))
                }
            } else {
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Device not found")))
            }
        },
        OscarCommand::ListDevices {} => {
            let mounted_devices = trashes.iter().map(|t| &t.device).collect::<Vec<&MountedDevice>>();
            let mut table = Table::new(mounted_devices);
            table.with(Style::modern_rounded());

            print!("{}", table.to_string());

            Ok(())
        }
    }
}

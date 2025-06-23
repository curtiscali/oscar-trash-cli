use std::{
    env::var,
    fs::{
        exists,
        File,
    },
    io::{
        Error,
        ErrorKind,
        Read,
        Result,
    },
    path::{
        Path,
        PathBuf,
    },
};

use tabled::Tabled;

#[link(name = "c")]
extern "C" {
    fn geteuid() -> u32;
}

#[derive(Debug, Clone, Default, Tabled)]
#[tabled(rename_all = "CamelCase")]
pub struct MountedDevice {
    #[tabled(rename = "Device Name")]
    pub name: String,

    #[tabled(rename = "Mount Point")]
    pub mount_point: String
}

impl MountedDevice {
    pub fn trash_dir(&self) -> Result<PathBuf> {
        if self.mount_point.eq("/home") || self.mount_point.eq("/") {
            match var("XDG_DATA_HOME") {
                Ok(xdg_data_home) => Ok(Path::new(&xdg_data_home).join("Trash").to_path_buf()),
                Err(_) => {
                    match var("HOME") {
                        Ok(home) => Ok(Path::new(&home).join(".local/share/Trash").to_path_buf()),
                        Err(_) => Err(Error::new(ErrorKind::Other, "Could not compute trash directory"))
                    }
                }
            }
        } else {
            let all_user_trash_dir = Path::new(&self.mount_point).join(".Trash");

            if exists(&all_user_trash_dir)? {
                return Ok(all_user_trash_dir);
            }
    
            unsafe {
                let uid = geteuid();
                let user_trash_dir = Path::new(&self.mount_point).join(format!(".Trash-{}", uid));
                
                return Ok(user_trash_dir);
            }
        }
    }

    pub fn trash_info_dir(&self) -> Result<PathBuf> {
        let trash_dir = self.trash_dir()?;
        Ok(trash_dir.join("info"))
    }

    pub fn trash_files_dir(&self) -> Result<PathBuf> {
        let trash_dir = self.trash_dir()?;
        Ok(trash_dir.join("files"))
    }
}

fn get_supported_filesystems() -> Result<Vec<String>> {
    let mut file = File::open("/proc/filesystems")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(
        contents.lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("nodev"))
            .collect()
    )
}

pub fn get_mounted_devices() -> Result<Vec<MountedDevice>> {
    let supported_filesystems = get_supported_filesystems()?;

    let mut mounted_devices = Vec::new();
    let mut file = File::open("/proc/mounts")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let (name, mount_point, filesystem) = (parts[0], parts[1], parts[2]);

        // if it's not a physical drive or it's a boot partition
        // root fs will be managed by user's home trash, which has already been implemented
        if !supported_filesystems.contains(&filesystem.to_string()) || mount_point.starts_with("/boot") {
            continue;
        }

        if !mounted_devices.iter().any(|d: &MountedDevice| d.name.eq(name)) {
            mounted_devices.push(MountedDevice {
                name: name.to_owned(),
                mount_point: mount_point
                    .replace("\\040", " ")
                    .to_owned()
            });
        }
    }

    Ok(mounted_devices)
}

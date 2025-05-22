mod common;

use std::io::Error;

use common::{remove_trash_file_hierarchy, setup_xdg_data_home};
use oscar::actions::trash_list::trash_list;
use serial_test::serial;

#[test]
#[serial]
fn test_list_trash_without_home_trash() -> Result<(), Error> {
    setup_xdg_data_home();

    match trash_list(false)  {
        Ok(_) => {
            let result = trash_list(false);
            remove_trash_file_hierarchy();

            result
        },
        Err(err) => Err(err)
    }
}

#[test]
#[serial]
fn test_list_trash_rec_without_home_trash() -> Result<(), Error> {
    setup_xdg_data_home();

    match trash_list(false)  {
        Ok(_) => {
            let result = trash_list(true);
            remove_trash_file_hierarchy();

            result
        },
        Err(err) => Err(err)
    }
}
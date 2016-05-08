extern crate hdd_standby;

use hdd_standby::{Error, PowerState};
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let mut quiet = false;
    let mut path = None;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--quiet" => {
                quiet = true;
            }
            _ => {
                path = Some(arg);
                break;
            }
        }
    }

    let path = if path.is_some() {
        path.unwrap()
    } else {
        print_error(quiet, "Usage: hdd_status /dev/sda");
        exit(101);
    };

    exit(match hdd_standby::get_power_state(&path) {
        Err(Error::NoAccess) => {
            print_error(quiet, "Cannot open device file");
            102
        }
        Err(Error::InvalidDeviceFile) => {
            print_error(quiet, "Given file is not a device file");
            103
        }
        Ok(mode) => {
            if ! quiet { println!("{}: {:?}", path, mode); }
            match mode {
                PowerMode::Active | PowerMode::Idle => 0,
                PowerMode::Spindown | PowerMode::Spinup => 2,
                PowerMode::Standby => 1,
                PowerMode::Unknown => 3,
            }
        }
    });
}

fn print_error(quiet: bool, msg: &str) {
    if ! quiet {
        writeln!(&mut io::stderr(), "{}", msg).unwrap();
    }
}

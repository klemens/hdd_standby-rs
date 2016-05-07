extern crate libc;

use std::ffi::CString;

#[derive(Debug)]
pub enum PowerMode {
    Standby,
    Spindown,
    Spinup,
    Idle,
    Active,
    Unknown,
}

#[derive(Debug)]
pub enum Error {
    NoAccess,
    InvalidDeviceFile,
}

// Ioctl for special hdd commands
const HDIO_DRIVE_CMD: libc::c_ulong = 0x031F;

// Hdd commands to check the power mode (taken from hdparm)
const ATA_OP_CHECKPOWERMODE1: libc::c_uchar = 0xE5;
const ATA_OP_CHECKPOWERMODE2: libc::c_uchar = 0x98;

pub fn get_power_mode(path: &str) -> Result<PowerMode, Error> {
    let device = try!(DeviceWrapper::open(path));

    let mut query: [libc::c_uchar; 4] = [0; 4];

    unsafe {
        query[0] = ATA_OP_CHECKPOWERMODE1;
        if libc::ioctl(device.fd(), HDIO_DRIVE_CMD, query.as_mut_ptr()) != 0 {
            query[0] = ATA_OP_CHECKPOWERMODE2;
            if libc::ioctl(device.fd(), HDIO_DRIVE_CMD, query.as_mut_ptr()) != 0 {
                return Err(Error::InvalidDeviceFile)
            }
        }
    }

    Ok(match query[2] {
        0x00 => PowerMode::Standby,
        0x40 => PowerMode::Spindown,
        0x41 => PowerMode::Spinup,
        0x80 => PowerMode::Idle,
        0xFF => PowerMode::Active,
        _    => PowerMode::Unknown
    })
}

struct DeviceWrapper(libc::c_int);

impl DeviceWrapper {
    pub fn open(path: &str) -> Result<DeviceWrapper, Error> {
        let path = CString::new(path).unwrap();

        let fd = unsafe {
            libc::open(path.as_ptr(), libc::O_RDONLY | libc::O_NONBLOCK)
        };

        if fd != -1 {
            Ok(DeviceWrapper(fd))
        } else {
            Err(Error::NoAccess)
        }
    }

    pub fn fd(&self) -> libc::c_int {
        self.0
    }
}

impl Drop for DeviceWrapper {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

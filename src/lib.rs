extern crate libc;


#[derive(Debug)]
pub enum PowerState {
    /// The hdd is in the standby state (PM2, usually spun down)
    Standby,
    /// The hdd is in the idle state (PM1)
    Idle,
    /// The hdd is in the active or idle state (PM0 or PM1)
    Active,
    /// The state of the hdd is unknown (invalid ATA response)
    Unknown,
}

#[derive(Debug)]
pub enum Error {
    NoAccess,
    InvalidDeviceFile,
}

// Ioctl for special hdd commands
const IOCTL_DRIVE_CMD: libc::c_ulong = 0x031F;

// Ata commands to check the power state
const ATA_CHECKPOWERMODE: libc::c_uchar = 0xE5;
const ATA_CHECKPOWERMODE_RETIRED: libc::c_uchar = 0x98;

pub fn get_power_state(path: &str) -> Result<PowerState, Error> {
    let device = try!(DeviceWrapper::open(path));

    let mut query: [libc::c_uchar; 4] = [0; 4];

    unsafe {
        query[0] = ATA_CHECKPOWERMODE;
        if libc::ioctl(device.fd(), IOCTL_DRIVE_CMD, query.as_mut_ptr()) != 0 {
            // Try the retired command if the current one failed
            query[0] = ATA_CHECKPOWERMODE_RETIRED;
            if libc::ioctl(device.fd(), IOCTL_DRIVE_CMD, query.as_mut_ptr()) != 0 {
                return Err(Error::InvalidDeviceFile)
            }
        }
    }

    Ok(match query[2] {
        0x00 ... 0x01 => PowerState::Standby,
        0x80 ... 0x83 => PowerState::Idle,
        0xFF => PowerState::Active,
        _    => PowerState::Unknown
    })
}

struct DeviceWrapper(libc::c_int);

impl DeviceWrapper {
    pub fn open(path: &str) -> Result<DeviceWrapper, Error> {
        let path = std::ffi::CString::new(path).unwrap();

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

use crate::sensors::Sensor;
use crate::sensors::Topology;
use windows::Win32::Storage::FileSystem::{CreateFileW, FILE_FLAG_OVERLAPPED, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING, FILE_READ_DATA, FILE_ACCESS_FLAGS};
use windows::Win32::Foundation::{INVALID_HANDLE_VALUE, CloseHandle, GetLastError, HANDLE};
use windows::Win32::System::IO::{DeviceIoControl, OVERLAPPED};
use windows::Win32::System::Ioctl::{FILE_DEVICE_UNKNOWN, METHOD_OUT_DIRECT};
use core::ffi::c_void;
use std::error::Error;
use std::mem::{size_of, size_of_val};

const AGENT_POWER_UNIT_CODE: u16 = 0xBEB;
const AGENT_POWER_LIMIT_CODE: u16 = 0xBEC;
const AGENT_ENERGY_STATUS_CODE: u16 = 0xBED;

pub struct MsrRAPLSensor {
    driver_name: String,

}

impl MsrRAPLSensor {

    pub fn new() -> MsrRAPLSensor {
        MsrRAPLSensor {
            driver_name: String::from("\\\\.\\RAPLDriver"),
            
        }
    }

}

impl Sensor for MsrRAPLSensor {
    fn generate_topology(&mut self) -> Result<Topology, Box<dyn Error>> {

    }

    fn get_topology(&mut self) -> Box<Option<Topology>> {
        let topology = self.generate_topology().ok();
        if topology.is_none() {
            panic!("Couldn't generate the topology !");
        }
        Box::new(topology)
    }
}
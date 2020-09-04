use super::*;
use crate::{UDPMessageBuffer, UDPMessageCode};
use static_assertions::assert_eq_size;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::net::Ipv4Addr;

pub const SIZE_DRIVER_OUTPUT: usize = SIZE_DRIVER_OUTPUT_CARD5;
pub const MESSAGE_CODE_DRIVER_OUTPUT: UDPMessageCode = UDPMessageCode::Output(0x10);

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct DriverOutputStructure {
    pub card5: DriverOutputCard5,
}

assert_eq_size!(DriverOutputStructure, [u8; SIZE_DRIVER_OUTPUT]);

impl Default for DriverOutputStructure {
    fn default() -> Self {
        Self {
            card5: DriverOutputCard5::default(),
        }
    }
}

#[repr(C, packed)]
pub union DriverOutputMessage {
    raw: [u8; SIZE_DRIVER_OUTPUT],
    structured: DriverOutputStructure,
}

assert_eq_size!(DriverOutputMessage, [u8; SIZE_DRIVER_OUTPUT]);

impl Default for DriverOutputMessage {
    fn default() -> Self {
        Self {
            structured: DriverOutputStructure::default(),
        }
    }
}

impl DriverOutputMessage {
    pub fn copy_to_message_buffer(&self, origin_address: Ipv4Addr, message_buffer: &mut UDPMessageBuffer) {
        message_buffer.set_origin(origin_address);
        message_buffer.set_message_code(MESSAGE_CODE_DRIVER_OUTPUT);

        unsafe {
            message_buffer.set_data(Some(&self.raw[..]));
        }
    }

    pub fn copy_from_message_buffer(&mut self, message_buffer: &UDPMessageBuffer) -> IOResult<Ipv4Addr> {
        if message_buffer.get_message_code() != MESSAGE_CODE_DRIVER_OUTPUT {
            return Err(IOError::new(IOErrorKind::InvalidData, "Invalid Message Code!"));
        }

        if message_buffer.get_data_length() != SIZE_DRIVER_OUTPUT {
            return Err(IOError::new(IOErrorKind::InvalidData, "Invalid Data Length!"));
        }

        unsafe {
            self.raw[..].copy_from_slice(message_buffer.get_data_slice().unwrap());
        }

        Ok(message_buffer.get_origin())
    }

    pub fn get_updated_structure(
        &mut self,
        message_buffer: &UDPMessageBuffer,
    ) -> IOResult<(Ipv4Addr, DriverOutputStructure)> {
        let origin_address = self.copy_from_message_buffer(message_buffer)?;

        Ok((origin_address, unsafe { self.structured.clone() }))
    }

    pub fn updated_buffer_from_structure(
        &mut self,
        origin_address: Ipv4Addr,
        message_buffer: &mut UDPMessageBuffer,
        source: DriverOutputStructure,
    ) {
        unsafe {
            self.structured = source;
        }

        self.copy_to_message_buffer(origin_address, message_buffer);
    }
}

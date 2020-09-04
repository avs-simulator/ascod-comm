use super::*;
use crate::{UDPMessageBuffer, UDPMessageCode};
use static_assertions::assert_eq_size;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::net::Ipv4Addr;

pub const SIZE_TURRET_OUTPUT: usize = SIZE_TURRET_OUTPUT_CARD8 + SIZE_TURRET_OUTPUT_CARD9;
pub const MESSAGE_CODE_TURRET_OUTPUT: UDPMessageCode = UDPMessageCode::Output(0x01);

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct TurretOutputStructure {
    pub card8: TurretOutputCard8,
    pub card9: TurretOutputCard9,
}

assert_eq_size!(TurretOutputStructure, [u8; SIZE_TURRET_OUTPUT]);

impl Default for TurretOutputStructure {
    fn default() -> Self {
        Self {
            card8: TurretOutputCard8::default(),
            card9: TurretOutputCard9::default(),
        }
    }
}

#[repr(C, packed)]
pub union TurretOutputMessage {
    raw: [u8; SIZE_TURRET_OUTPUT],
    structured: TurretOutputStructure,
}

assert_eq_size!(TurretOutputMessage, [u8; SIZE_TURRET_OUTPUT]);

impl Default for TurretOutputMessage {
    fn default() -> Self {
        Self {
            structured: TurretOutputStructure::default(),
        }
    }
}

impl TurretOutputMessage {
    pub fn copy_to_message_buffer(&self, origin_address: Ipv4Addr, message_buffer: &mut UDPMessageBuffer) {
        message_buffer.set_origin(origin_address);
        message_buffer.set_message_code(MESSAGE_CODE_TURRET_OUTPUT);

        unsafe {
            message_buffer.set_data(Some(&self.raw[..]));
        }
    }

    pub fn copy_from_message_buffer(&mut self, message_buffer: &UDPMessageBuffer) -> IOResult<Ipv4Addr> {
        if message_buffer.get_message_code() != MESSAGE_CODE_TURRET_OUTPUT {
            return Err(IOError::new(IOErrorKind::InvalidData, "Invalid Message Code!"));
        }

        if message_buffer.get_data_length() != SIZE_TURRET_OUTPUT {
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
    ) -> IOResult<(Ipv4Addr, TurretOutputStructure)> {
        let origin_address = self.copy_from_message_buffer(message_buffer)?;

        Ok((origin_address, unsafe { self.structured.clone() }))
    }

    pub fn updated_buffer_from_structure(
        &mut self,
        origin_address: Ipv4Addr,
        message_buffer: &mut UDPMessageBuffer,
        source: TurretOutputStructure,
    ) {
        unsafe {
            self.structured = source;
        }

        self.copy_to_message_buffer(origin_address, message_buffer);
    }
}

use super::*;
use crate::{UDPMessageBuffer, UDPMessageCode};
use static_assertions::assert_eq_size;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::net::Ipv4Addr;

pub const SIZE_DRIVER_INPUT: usize = 0
    + SIZE_DRIVER_INPUT_CARD0
    + SIZE_DRIVER_INPUT_CARD1
    + SIZE_DRIVER_INPUT_CARD2
    + SIZE_DRIVER_INPUT_CARD3
    + SIZE_DRIVER_INPUT_CARD4;
pub const MESSAGE_CODE_DRIVER_INPUT: UDPMessageCode = UDPMessageCode::Input(0x10);

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct DriverInputStructure {
    pub card0: DriverInputCard0,
    pub card1: DriverInputCard1,
    pub card2: DriverInputCard2,
    pub card3: DriverInputCard3,
    pub card4: DriverInputCard4,
}

assert_eq_size!(DriverInputStructure, [u8; SIZE_DRIVER_INPUT]);

impl Default for DriverInputStructure {
    fn default() -> Self {
        Self {
            card0: DriverInputCard0::default(),
            card1: DriverInputCard1::default(),
            card2: DriverInputCard2::default(),
            card3: DriverInputCard3::default(),
            card4: DriverInputCard4::default(),
        }
    }
}

#[repr(C, packed)]
pub union DriverInputMessage {
    raw: [u8; SIZE_DRIVER_INPUT],
    structured: DriverInputStructure,
}

assert_eq_size!(DriverInputMessage, [u8; SIZE_DRIVER_INPUT]);

impl Default for DriverInputMessage {
    fn default() -> Self {
        Self {
            structured: DriverInputStructure::default(),
        }
    }
}

impl DriverInputMessage {
    pub fn copy_to_message_buffer(&self, origin_address: Ipv4Addr, message_buffer: &mut UDPMessageBuffer) {
        message_buffer.set_origin(origin_address);
        message_buffer.set_message_code(MESSAGE_CODE_DRIVER_INPUT);

        unsafe {
            message_buffer.set_data(Some(&self.raw[..]));
        }
    }

    pub fn copy_from_message_buffer(&mut self, message_buffer: &UDPMessageBuffer) -> IOResult<Ipv4Addr> {
        if message_buffer.get_message_code() != MESSAGE_CODE_DRIVER_INPUT {
            return Err(IOError::new(IOErrorKind::InvalidData, "Invalid Message Code!"));
        }

        if message_buffer.get_data_length() != SIZE_DRIVER_INPUT {
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
    ) -> IOResult<(Ipv4Addr, DriverInputStructure)> {
        let origin_address = self.copy_from_message_buffer(message_buffer)?;

        Ok((origin_address, unsafe { self.structured.clone() }))
    }

    pub fn updated_buffer_from_structure(
        &mut self,
        origin_address: Ipv4Addr,
        message_buffer: &mut UDPMessageBuffer,
        source: DriverInputStructure,
    ) {
        unsafe {
            self.structured = source;
        }

        self.copy_to_message_buffer(origin_address, message_buffer);
    }
}

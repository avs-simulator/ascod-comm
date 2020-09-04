use super::*;
use crate::{UDPMessageBuffer, UDPMessageCode};
use static_assertions::assert_eq_size;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::net::Ipv4Addr;

pub const SIZE_TURRET_INPUT: usize = 0
    + SIZE_TURRET_INPUT_CARD0
    + SIZE_TURRET_INPUT_CARD1
    + SIZE_TURRET_INPUT_CARD2
    + SIZE_TURRET_INPUT_CARD3
    + SIZE_TURRET_INPUT_CARD4
    + SIZE_TURRET_INPUT_CARD5
    + SIZE_TURRET_INPUT_CARD6
    + SIZE_TURRET_INPUT_CARD7;
pub const MESSAGE_CODE_TURRET_INPUT: UDPMessageCode = UDPMessageCode::Input(0x01);

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct TurretInputStructure {
    pub card0: TurretInputCard0,
    pub card1: TurretInputCard1,
    pub card2: TurretInputCard2,
    pub card3: TurretInputCard3,
    pub card4: TurretInputCard4,
    pub card5: TurretInputCard5,
    pub card6: TurretInputCard6,
    pub card7: TurretInputCard7,
}

assert_eq_size!(TurretInputStructure, [u8; SIZE_TURRET_INPUT]);

impl Default for TurretInputStructure {
    fn default() -> Self {
        Self {
            card0: TurretInputCard0::default(),
            card1: TurretInputCard1::default(),
            card2: TurretInputCard2::default(),
            card3: TurretInputCard3::default(),
            card4: TurretInputCard4::default(),
            card5: TurretInputCard5::default(),
            card6: TurretInputCard6::default(),
            card7: TurretInputCard7::default(),
        }
    }
}

#[repr(C, packed)]
pub union TurretInputMessage {
    raw: [u8; SIZE_TURRET_INPUT],
    structured: TurretInputStructure,
}

assert_eq_size!(TurretInputMessage, [u8; SIZE_TURRET_INPUT]);

impl Default for TurretInputMessage {
    fn default() -> Self {
        Self {
            structured: TurretInputStructure::default(),
        }
    }
}

impl TurretInputMessage {
    pub fn copy_to_message_buffer(&self, origin_address: Ipv4Addr, message_buffer: &mut UDPMessageBuffer) {
        message_buffer.set_origin(origin_address);
        message_buffer.set_message_code(MESSAGE_CODE_TURRET_INPUT);

        unsafe {
            message_buffer.set_data(Some(&self.raw[..]));
        }
    }

    pub fn copy_from_message_buffer(&mut self, message_buffer: &UDPMessageBuffer) -> IOResult<Ipv4Addr> {
        if message_buffer.get_message_code() != MESSAGE_CODE_TURRET_INPUT {
            return Err(IOError::new(IOErrorKind::InvalidData, "Invalid Message Code!"));
        }

        if message_buffer.get_data_length() != SIZE_TURRET_INPUT {
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
    ) -> IOResult<(Ipv4Addr, TurretInputStructure)> {
        let origin_address = self.copy_from_message_buffer(message_buffer)?;

        Ok((origin_address, unsafe { self.structured.clone() }))
    }

    pub fn updated_buffer_from_structure(
        &mut self,
        origin_address: Ipv4Addr,
        message_buffer: &mut UDPMessageBuffer,
        source: TurretInputStructure,
    ) {
        unsafe {
            self.structured = source;
        }

        self.copy_to_message_buffer(origin_address, message_buffer);
    }
}

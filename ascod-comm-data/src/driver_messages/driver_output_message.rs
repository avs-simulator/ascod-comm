use super::*;
use crate::{ITransportableMessage, UDPMessageCode};
use static_assertions::assert_eq_size;

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

impl ITransportableMessage<DriverOutputStructure, SIZE_DRIVER_OUTPUT> for DriverOutputMessage {
    fn get_raw_buffer(&self) -> &[u8] {
        unsafe { &self.raw[..] }
    }

    fn get_raw_buffer_mut(&mut self) -> &mut [u8] {
        unsafe { &mut self.raw[..] }
    }

    fn get_cloned_structure(&self) -> DriverOutputStructure {
        unsafe { self.structured.clone() }
    }

    fn get_structure_mut(&mut self) -> &mut DriverOutputStructure {
        unsafe { &mut self.structured }
    }

    fn get_message_code(&self) -> UDPMessageCode {
        MESSAGE_CODE_DRIVER_OUTPUT
    }
}

use super::*;
use crate::{ITransportableMessage, UDPMessageCode};
use static_assertions::assert_eq_size;

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

impl ITransportableMessage<DriverInputStructure, SIZE_DRIVER_INPUT> for DriverInputMessage {
    fn get_raw_buffer(&self) -> &[u8] {
        unsafe { &self.raw[..] }
    }

    fn get_raw_buffer_mut(&mut self) -> &mut [u8] {
        unsafe { &mut self.raw[..] }
    }

    fn get_cloned_structure(&self) -> DriverInputStructure {
        unsafe { self.structured.clone() }
    }

    fn get_structure_mut(&mut self) -> &mut DriverInputStructure {
        unsafe { &mut self.structured }
    }

    fn get_message_code(&self) -> UDPMessageCode {
        MESSAGE_CODE_DRIVER_INPUT
    }
}

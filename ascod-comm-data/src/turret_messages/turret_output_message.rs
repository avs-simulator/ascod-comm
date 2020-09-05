use super::*;
use crate::{ITransportableMessage, UDPMessageCode};
use static_assertions::assert_eq_size;

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

impl ITransportableMessage<TurretOutputStructure, SIZE_TURRET_OUTPUT> for TurretOutputMessage {
    fn get_raw_buffer(&self) -> &[u8] {
        unsafe { &self.raw[..] }
    }

    fn get_raw_buffer_mut(&mut self) -> &mut [u8] {
        unsafe { &mut self.raw[..] }
    }

    fn get_cloned_structure(&self) -> TurretOutputStructure {
        unsafe { self.structured.clone() }
    }

    fn get_structure_mut(&mut self) -> &mut TurretOutputStructure {
        unsafe { &mut self.structured }
    }

    fn get_message_code(&self) -> UDPMessageCode {
        MESSAGE_CODE_TURRET_OUTPUT
    }
}

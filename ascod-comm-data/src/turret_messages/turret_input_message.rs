use super::*;
use crate::{ITransportableMessage, UDPMessageCode};
use static_assertions::assert_eq_size;

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

impl ITransportableMessage<TurretInputStructure, SIZE_TURRET_INPUT> for TurretInputMessage {
    fn get_raw_buffer(&self) -> &[u8] {
        unsafe { &self.raw[..] }
    }

    fn get_raw_buffer_mut(&mut self) -> &mut [u8] {
        unsafe { &mut self.raw[..] }
    }

    fn get_cloned_structure(&self) -> TurretInputStructure {
        unsafe { self.structured.clone() }
    }

    fn get_structure_mut(&mut self) -> &mut TurretInputStructure {
        unsafe { &mut self.structured }
    }

    fn get_message_code(&self) -> UDPMessageCode {
        MESSAGE_CODE_TURRET_INPUT
    }
}

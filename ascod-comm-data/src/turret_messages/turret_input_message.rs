use super::*;
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

#[repr(C, packed)]
pub struct TurretInputStructure {
    card0: TurretInputCard0,
    card1: TurretInputCard1,
    card2: TurretInputCard2,
    card3: TurretInputCard3,
    card4: TurretInputCard4,
    card5: TurretInputCard5,
    card6: TurretInputCard6,
    card7: TurretInputCard7,
}

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

impl Default for TurretInputMessage {
    fn default() -> Self {
        Self {
            structured: TurretInputStructure::default(),
        }
    }
}

assert_eq_size!(TurretInputStructure, [u8; SIZE_TURRET_INPUT]);
assert_eq_size!(TurretInputMessage, [u8; SIZE_TURRET_INPUT]);

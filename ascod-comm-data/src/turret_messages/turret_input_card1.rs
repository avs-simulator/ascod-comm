use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD1: usize = 3;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard1 {
    #[bitfield(name = "manual_elevation", ty = "libc::uint16_t", bits = "0..=9")]
    #[bitfield(name = "volume_radio_gunner", ty = "libc::uint16_t", bits = "10..=19")]
    raw: [u8; SIZE_TURRET_INPUT_CARD1],
}

assert_eq_size!(TurretInputCard1, [u8; SIZE_TURRET_INPUT_CARD1]);

impl Default for TurretInputCard1 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_TURRET_INPUT_CARD1],
        }
    }
}

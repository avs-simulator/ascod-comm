use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD2: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard2 {
    #[bitfield(name = "button_f1", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "button_f2", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "button_f3", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "button_f4", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "button_f5", ty = "libc::uint16_t", bits = "4..=4")]
    #[bitfield(name = "button_f6", ty = "libc::uint16_t", bits = "5..=5")]
    #[bitfield(name = "button_f7", ty = "libc::uint16_t", bits = "6..=6")]
    #[bitfield(name = "button_f8", ty = "libc::uint16_t", bits = "7..=7")]
    #[bitfield(name = "button_f9", ty = "libc::uint16_t", bits = "8..=8")]
    #[bitfield(name = "button_f10", ty = "libc::uint16_t", bits = "9..=9")]
    #[bitfield(name = "button_f11", ty = "libc::uint16_t", bits = "10..=10")]
    #[bitfield(name = "button_f12", ty = "libc::uint16_t", bits = "11..=11")]
    raw: [u8; SIZE_TURRET_INPUT_CARD2],
}

assert_eq_size!(TurretInputCard2, [u8; SIZE_TURRET_INPUT_CARD2]);

impl Default for TurretInputCard2 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_TURRET_INPUT_CARD2],
        }
    }
}

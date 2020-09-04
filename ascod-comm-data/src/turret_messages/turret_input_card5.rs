use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD5: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct)]
pub struct TurretInputCard5 {
    #[bitfield(name = "emergency_button_commander", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "button_f1_gmu", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "button_f2_gmu", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "button_f3_gmu", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "button_f4_gmu", ty = "libc::uint16_t", bits = "4..=4")]
    #[bitfield(name = "button_f5_gmu", ty = "libc::uint16_t", bits = "5..=5")]
    #[bitfield(name = "button_f6_gmu", ty = "libc::uint16_t", bits = "6..=6")]
    #[bitfield(name = "button_f7_gmu", ty = "libc::uint16_t", bits = "7..=7")]
    #[bitfield(name = "button_f8_gmu", ty = "libc::uint16_t", bits = "8..=8")]
    #[bitfield(name = "button_f9_gmu", ty = "libc::uint16_t", bits = "9..=9")]
    #[bitfield(name = "button_f10_gmu", ty = "libc::uint16_t", bits = "10..=10")]
    #[bitfield(name = "button_f11_gmu", ty = "libc::uint16_t", bits = "11..=11")]
    ushort_fields: [u8; SIZE_TURRET_INPUT_CARD5],
}

assert_eq_size!(TurretInputCard5, [u8; SIZE_TURRET_INPUT_CARD5]);

impl Default for TurretInputCard5 {
    fn default() -> Self {
        Self {
            ushort_fields: [0; SIZE_TURRET_INPUT_CARD5],
        }
    }
}

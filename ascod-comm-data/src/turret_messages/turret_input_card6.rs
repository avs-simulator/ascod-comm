use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD6: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard6 {
    #[bitfield(name = "button_f12_gmu", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "button_1_gmu", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "button_2_gmu", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "button_3_gmu", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "button_4_gmu", ty = "libc::uint16_t", bits = "4..=4")]
    #[bitfield(name = "button_5_gmu", ty = "libc::uint16_t", bits = "5..=5")]
    #[bitfield(name = "button_brightness_down_gmu", ty = "libc::uint16_t", bits = "6..=6")]
    #[bitfield(name = "button_brightness_up_gmu", ty = "libc::uint16_t", bits = "7..=7")]
    #[bitfield(name = "button_power_gmu", ty = "libc::uint16_t", bits = "8..=8")]
    #[bitfield(name = "button_lrf_gunner_control", ty = "libc::uint16_t", bits = "9..=9")]
    #[bitfield(name = "button_picu_left_gunner_control", ty = "libc::uint16_t", bits = "10..=10")]
    #[bitfield(name = "button_picu_right_gunner_control", ty = "libc::uint16_t", bits = "11..=11")]
    raw: [u8; SIZE_TURRET_INPUT_CARD6],
}

assert_eq_size!(TurretInputCard6, [u8; SIZE_TURRET_INPUT_CARD6]);

impl Default for TurretInputCard6 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_TURRET_INPUT_CARD6],
        }
    }
}

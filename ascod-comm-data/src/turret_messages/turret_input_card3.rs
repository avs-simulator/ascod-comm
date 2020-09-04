use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD3: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct)]
pub struct TurretInputCard3 {
    #[bitfield(name = "button_1_cmu", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "button_2_cmu", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "button_3_cmu", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "button_4_cmu", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "button_5_cmu", ty = "libc::uint16_t", bits = "4..=4")]
    #[bitfield(name = "button_brightness_down_cmu", ty = "libc::uint16_t", bits = "5..=5")]
    #[bitfield(name = "button_brightness_up_cmu", ty = "libc::uint16_t", bits = "6..=6")]
    #[bitfield(name = "button_power_cmu", ty = "libc::uint16_t", bits = "7..=7")]
    #[bitfield(name = "selector_camera_panoramic_cmu", ty = "libc::uint16_t", bits = "8..=8")]
    #[bitfield(name = "button_lrf_dual_overide", ty = "libc::uint16_t", bits = "9..=9")]
    #[bitfield(name = "button_picu_dual_overide", ty = "libc::uint16_t", bits = "10..=10")]
    #[bitfield(name = "button_palm_grip_dual_overide", ty = "libc::uint16_t", bits = "11..=11")]
    ushort_fields: [u8; SIZE_TURRET_INPUT_CARD3],
}

assert_eq_size!(TurretInputCard3, [u8; SIZE_TURRET_INPUT_CARD3]);

impl Default for TurretInputCard3 {
    fn default() -> Self {
        Self {
            ushort_fields: [0; SIZE_TURRET_INPUT_CARD3],
        }
    }
}

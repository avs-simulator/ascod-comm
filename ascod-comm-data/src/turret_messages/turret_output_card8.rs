use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_OUTPUT_CARD8: usize = 4;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretOutputCard8 {
    #[bitfield(name = "backlight_f1_f6_cmu", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "backlight_f7_f12_cmu", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "backlight_1_power_cmu", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "indicator_ecs_commander_control_box", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "indicator_fcs_commander_control_box", ty = "libc::uint16_t", bits = "4..=4")]
    #[bitfield(name = "indicator_safety_driver_commander_control_box", ty = "libc::uint16_t", bits = "5..=5")]
    #[bitfield(name = "indicator_fan_commander_control_box", ty = "libc::uint16_t", bits = "6..=6")]
    #[bitfield(name = "indicator_pcs_commander_control_box", ty = "libc::uint16_t", bits = "7..=7")]
    #[bitfield(name = "indicator_firing_ready_commander_control_box", ty = "libc::uint16_t", bits = "8..=8")]
    #[bitfield(name = "backlight_f1_f16_gmu", ty = "libc::uint16_t", bits = "9..=9")]
    #[bitfield(name = "interior_light_red_commander", ty = "libc::uint8_t", bits = "16..=23")]
    #[bitfield(name = "interior_light_white_commander", ty = "libc::uint8_t", bits = "24..=31")]
    raw: [u8; SIZE_TURRET_OUTPUT_CARD8],
}

assert_eq_size!(TurretOutputCard8, [u8; SIZE_TURRET_OUTPUT_CARD8]);

impl Default for TurretOutputCard8 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_TURRET_OUTPUT_CARD8],
        }
    }
}

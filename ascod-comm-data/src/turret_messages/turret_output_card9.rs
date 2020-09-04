use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_OUTPUT_CARD9: usize = 3;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretOutputCard9 {
    #[bitfield(name = "backlight_f7_f12_gmu", ty = "libc::uint8_t", bits = "0..=0")]
    #[bitfield(name = "backlight_1_power_gmu", ty = "libc::uint8_t", bits = "1..=1")]
    #[bitfield(name = "indicator_ecs_gunner_control_box", ty = "libc::uint8_t", bits = "2..=2")]
    #[bitfield(name = "indicator_fcs_gunner_control_box", ty = "libc::uint8_t", bits = "3..=3")]
    #[bitfield(name = "indicator_pcs_gunner_control_box", ty = "libc::uint8_t", bits = "4..=4")]
    #[bitfield(
        name = "indicator_firing_ready_gunner_control_box",
        ty = "libc::uint8_t",
        bits = "5..=5"
    )]
    #[bitfield(name = "fan_gunner_control_box", ty = "libc::uint8_t", bits = "6..=6")]
    #[bitfield(name = "safety_laser_gunner_control_box", ty = "libc::uint8_t", bits = "7..=7")]
    #[bitfield(name = "interior_light_red_gunner", ty = "libc::uint8_t", bits = "8..=15")]
    #[bitfield(name = "interior_light_white_gunner", ty = "libc::uint8_t", bits = "16..=23")]
    raw: [u8; SIZE_TURRET_OUTPUT_CARD9],
}

assert_eq_size!(TurretOutputCard9, [u8; SIZE_TURRET_OUTPUT_CARD9]);

impl Default for TurretOutputCard9 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_TURRET_OUTPUT_CARD9],
        }
    }
}

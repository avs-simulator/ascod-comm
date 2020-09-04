use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD7: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard7 {
    #[bitfield(name = "button_palmgrip_left_gunner_control", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "button_palmgrip_right_gunner_control", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "foot_firing", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "channel_radio_gunner", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "selector_interior_light_gunner", ty = "libc::uint16_t", bits = "4..=7")]
    #[bitfield(name = "button_fan_gunner_control_box", ty = "libc::uint16_t", bits = "8..=8")]
    #[bitfield(name = "master_switch_gunner_control_box", ty = "libc::uint16_t", bits = "9..=9")]
    #[bitfield(name = "emergency_button_gunner", ty = "libc::uint16_t", bits = "10..=10")]
    ushort_fields: [u8; SIZE_TURRET_INPUT_CARD7],
}

assert_eq_size!(TurretInputCard7, [u8; SIZE_TURRET_INPUT_CARD7]);

impl Default for TurretInputCard7 {
    fn default() -> Self {
        Self {
            ushort_fields: [0; SIZE_TURRET_INPUT_CARD7],
        }
    }
}

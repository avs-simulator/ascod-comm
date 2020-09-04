use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD4: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard4 {
    #[bitfield(name = "button_fan_commander_control_box", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "saklar_pcs_commander_control_box", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "button_firing_ready_control_box", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "button_grenade_left", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "button_grenade_right", ty = "libc::uint16_t", bits = "4..=4")]
    #[bitfield(name = "saklar_activator_grenade", ty = "libc::uint16_t", bits = "5..=5")]
    #[bitfield(name = "turret_lock", ty = "libc::uint16_t", bits = "6..=6")]
    #[bitfield(name = "channel_radio_commander", ty = "libc::uint16_t", bits = "7..=7")]
    #[bitfield(name = "selector_interior_light_commander", ty = "libc::uint16_t", bits = "8..=11")]
    ushort_fields: [u8; SIZE_TURRET_INPUT_CARD4],
}

assert_eq_size!(TurretInputCard4, [u8; SIZE_TURRET_INPUT_CARD4]);

impl Default for TurretInputCard4 {
    fn default() -> Self {
        Self {
            ushort_fields: [0; SIZE_TURRET_INPUT_CARD4],
        }
    }
}

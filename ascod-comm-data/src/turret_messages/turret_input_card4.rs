use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD4: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard4 {
    #[bitfield(name = "BUTTON_FAN_COMMANDER_CONTROL_BOX", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "SAKLAR_PCS_COMMANDER_CONTROL_BOX", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "BUTTON_FIRING_READY_CONTROL_BOX", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "BUTTON_GRANADE_KIRI", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "BUTTON_GRANADE_KANAN", ty = "libc::uint16_t", bits = "4..=4")]
    #[bitfield(name = "SAKLAR_AKTIVATOR_GRANADE", ty = "libc::uint16_t", bits = "5..=5")]
    #[bitfield(name = "TURRET_LOCK", ty = "libc::uint16_t", bits = "6..=6")]
    #[bitfield(name = "CHANNEL_RADIO_COMMANDER", ty = "libc::uint16_t", bits = "7..=7")]
    #[bitfield(name = "SELECTOR_INTERIOR_LIGHT_COMMANDER", ty = "libc::uint16_t", bits = "8..=11")]
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

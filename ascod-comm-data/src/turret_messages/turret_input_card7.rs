use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD7: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard7 {
    #[bitfield(name = "BUTTON_PALMGRIP_KIRI_GUNNER_CONTROL", ty = "libc::uint16_t", bits = "0..=0")]
    #[bitfield(name = "BUTTON_PALMGRIP_KANAN_GUNNER_CONTROL", ty = "libc::uint16_t", bits = "1..=1")]
    #[bitfield(name = "FOOT_FIRING", ty = "libc::uint16_t", bits = "2..=2")]
    #[bitfield(name = "CHANNEL_RADIO_GUNNER", ty = "libc::uint16_t", bits = "3..=3")]
    #[bitfield(name = "SELECTOR_INTERIOR_LIGHT_GUNNER", ty = "libc::uint16_t", bits = "4..=7")]
    #[bitfield(name = "BUTTON_FAN_GUNNER_CONTROL_BOX", ty = "libc::uint16_t", bits = "8..=8")]
    #[bitfield(name = "MASTER_SWITCH_GUNNER_CONTROL_BOX", ty = "libc::uint16_t", bits = "9..=9")]
    #[bitfield(name = "EMERGENCY_BUTTON_GUNNER", ty = "libc::uint16_t", bits = "10..=10")]
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

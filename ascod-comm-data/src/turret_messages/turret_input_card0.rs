use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD0: usize = 10;

#[repr(C, packed)]
#[derive(BitfieldStruct)]
pub struct TurretInputCard0 {
    #[bitfield(name = "tracking_dual_override", ty = "libc::uint16_t", bits = "0..=9")]
    #[bitfield(name = "dual_overide_x", ty = "libc::uint16_t", bits = "10..=19")]
    #[bitfield(name = "dual_overide_y", ty = "libc::uint16_t", bits = "20..=29")]
    #[bitfield(name = "volume_radio_commander", ty = "libc::uint16_t", bits = "30..=39")]
    #[bitfield(name = "tracking_gunner_control", ty = "libc::uint16_t", bits = "40..=49")]
    #[bitfield(name = "gunner_control_elevation", ty = "libc::uint16_t", bits = "50..=59")]
    #[bitfield(name = "gunner_control_traverse", ty = "libc::uint16_t", bits = "60..=69")]
    #[bitfield(name = "manual_traverse", ty = "libc::uint16_t", bits = "70..=79")]
    ushort_fields: [u8; SIZE_TURRET_INPUT_CARD0],
}

assert_eq_size!(TurretInputCard0, [u8; SIZE_TURRET_INPUT_CARD0]);

impl Default for TurretInputCard0 {
    fn default() -> Self {
        Self {
            ushort_fields: [0; SIZE_TURRET_INPUT_CARD0],
        }
    }
}

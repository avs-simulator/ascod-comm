use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_DRIVER_INPUT_CARD3: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct DriverInputCard3 {
    #[bitfield(name = "headlamp", ty = "libc::uint8_t", bits = "0..=4")]
    #[bitfield(name = "channel_radio", ty = "libc::uint8_t", bits = "5..=5")]
    #[bitfield(name = "level_emergency_gear", ty = "libc::uint8_t", bits = "6..=8")]
    raw: [u8; SIZE_DRIVER_INPUT_CARD3],
}

assert_eq_size!(DriverInputCard3, [u8; SIZE_DRIVER_INPUT_CARD3]);

impl Default for DriverInputCard3 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_DRIVER_INPUT_CARD3],
        }
    }
}

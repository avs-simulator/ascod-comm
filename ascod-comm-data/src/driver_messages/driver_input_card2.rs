use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_DRIVER_INPUT_CARD2: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct DriverInputCard2 {
    #[bitfield(name = "main_switch", ty = "libc::uint8_t", bits = "0..=1")]
    #[bitfield(name = "starter", ty = "libc::uint8_t", bits = "2..=3")]
    #[bitfield(name = "engine_stop", ty = "libc::uint8_t", bits = "4..=4")]
    #[bitfield(name = "horn", ty = "libc::uint8_t", bits = "5..=5")]
    #[bitfield(name = "hazzard", ty = "libc::uint8_t", bits = "6..=6")]
    #[bitfield(name = "wiper", ty = "libc::uint8_t", bits = "7..=8")]
    #[bitfield(name = "sign", ty = "libc::uint8_t", bits = "9..=10")]
    #[bitfield(name = "main_beam", ty = "libc::uint8_t", bits = "11..=11")]
    raw: [u8; SIZE_DRIVER_INPUT_CARD2],
}

assert_eq_size!(DriverInputCard2, [u8; SIZE_DRIVER_INPUT_CARD2]);

impl Default for DriverInputCard2 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_DRIVER_INPUT_CARD2],
        }
    }
}

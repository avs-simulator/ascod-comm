use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_DRIVER_INPUT_CARD4: usize = 1;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct DriverInputCard4 {
    #[bitfield(name = "selector_interior_light", ty = "libc::uint8_t", bits = "0..=3")]
    raw: [u8; SIZE_DRIVER_INPUT_CARD4],
}

assert_eq_size!(DriverInputCard4, [u8; SIZE_DRIVER_INPUT_CARD4]);

impl Default for DriverInputCard4 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_DRIVER_INPUT_CARD4],
        }
    }
}

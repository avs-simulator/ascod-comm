use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_DRIVER_OUTPUT_CARD5: usize = 3;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct DriverOutputCard5 {
    #[bitfield(name = "button_hazard_lamp", ty = "libc::uint8_t", bits = "0..=0")]
    #[bitfield(name = "spark_lamp", ty = "libc::uint8_t", bits = "1..=1")]
    #[bitfield(name = "backlight_button_thermal_monitor", ty = "libc::uint8_t", bits = "2..=2")]
    #[bitfield(name = "warning_lamp", ty = "libc::uint8_t", bits = "3..=3")]
    #[bitfield(name = "interior_light_red", ty = "libc::uint8_t", bits = "8..=15")]
    #[bitfield(name = "interior_light_white", ty = "libc::uint8_t", bits = "16..=23")]
    raw: [u8; SIZE_DRIVER_OUTPUT_CARD5],
}

assert_eq_size!(DriverOutputCard5, [u8; SIZE_DRIVER_OUTPUT_CARD5]);

impl Default for DriverOutputCard5 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_DRIVER_OUTPUT_CARD5],
        }
    }
}

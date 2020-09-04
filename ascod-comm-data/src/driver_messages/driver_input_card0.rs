use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_DRIVER_INPUT_CARD0: usize = 8;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct DriverInputCard0 {
    #[bitfield(name = "steering", ty = "libc::uint32_t", bits = "0..=9")]
    #[bitfield(name = "throttle", ty = "libc::uint32_t", bits = "10..=19")]
    #[bitfield(name = "brake", ty = "libc::uint32_t", bits = "20..=29")]
    #[bitfield(name = "brightness_dip", ty = "libc::uint32_t", bits = "30..=39")]
    #[bitfield(name = "brightness_thermal_monitor", ty = "libc::uint32_t", bits = "40..=49")]
    #[bitfield(name = "volume_control_radio_driver", ty = "libc::uint32_t", bits = "50..=59")]
    raw: [u8; SIZE_DRIVER_INPUT_CARD0],
}

assert_eq_size!(DriverInputCard0, [u8; SIZE_DRIVER_INPUT_CARD0]);

impl Default for DriverInputCard0 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_DRIVER_INPUT_CARD0],
        }
    }
}

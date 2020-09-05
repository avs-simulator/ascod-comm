use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_DRIVER_INPUT_CARD1: usize = 2;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct DriverInputCard1 {
    #[bitfield(name = "parking_brake", ty = "libc::uint8_t", bits = "0..=0")]
    #[bitfield(name = "gear_selector", ty = "libc::uint8_t", bits = "1..=4")]
    #[bitfield(name = "button_release", ty = "libc::uint8_t", bits = "5..=5")]
    #[bitfield(name = "lever_direction", ty = "libc::uint8_t", bits = "6..=8")]
    #[bitfield(name = "power_thermal_monitor", ty = "libc::uint8_t", bits = "9..=9")]
    #[bitfield(name = "selector_day_camera_rear_front", ty = "libc::uint8_t", bits = "10..=10")]
    #[bitfield(name = "selector_thermal_camera_thermal_front", ty = "libc::uint8_t", bits = "11..=11")]
    raw: [u8; SIZE_DRIVER_INPUT_CARD1],
}

assert_eq_size!(DriverInputCard1, [u8; SIZE_DRIVER_INPUT_CARD1]);

impl Default for DriverInputCard1 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_DRIVER_INPUT_CARD1],
        }
    }
}

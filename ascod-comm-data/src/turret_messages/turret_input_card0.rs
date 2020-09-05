use c2rust_bitfields::BitfieldStruct;
use static_assertions::assert_eq_size;

pub const SIZE_TURRET_INPUT_CARD0: usize = 10;

#[repr(C, packed)]
#[derive(BitfieldStruct, Clone, Debug)]
pub struct TurretInputCard0 {
    #[bitfield(name = "tracking_dual_override", ty = "libc::uint16_t", bits = "0..=9")]
    #[bitfield(name = "dual_override_x", ty = "libc::uint16_t", bits = "10..=19")]
    #[bitfield(name = "dual_override_y", ty = "libc::uint16_t", bits = "20..=29")]
    #[bitfield(name = "volume_radio_commander", ty = "libc::uint16_t", bits = "30..=39")]
    #[bitfield(name = "tracking_gunner_control", ty = "libc::uint16_t", bits = "40..=49")]
    #[bitfield(name = "gunner_control_elevation", ty = "libc::uint16_t", bits = "50..=59")]
    #[bitfield(name = "gunner_control_traverse", ty = "libc::uint16_t", bits = "60..=69")]
    #[bitfield(name = "manual_traverse", ty = "libc::uint16_t", bits = "70..=79")]
    raw: [u8; SIZE_TURRET_INPUT_CARD0],
}

assert_eq_size!(TurretInputCard0, [u8; SIZE_TURRET_INPUT_CARD0]);

impl Default for TurretInputCard0 {
    fn default() -> Self {
        Self {
            raw: [0; SIZE_TURRET_INPUT_CARD0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_has_all_zero() {
        let turret_input_card0 = TurretInputCard0::default();
        assert_eq!(turret_input_card0.tracking_dual_override(), 0);
        assert_eq!(turret_input_card0.dual_override_x(), 0);
        assert_eq!(turret_input_card0.dual_override_y(), 0);
        assert_eq!(turret_input_card0.volume_radio_commander(), 0);
        assert_eq!(turret_input_card0.tracking_gunner_control(), 0);
        assert_eq!(turret_input_card0.gunner_control_elevation(), 0);
        assert_eq!(turret_input_card0.gunner_control_traverse(), 0);
        assert_eq!(turret_input_card0.manual_traverse(), 0);
    }

    #[test]
    fn test_set_field_partially() {
        let expected_result = 0x03AB;
        let mut turret_input_card0 = TurretInputCard0::default();
        turret_input_card0.set_dual_override_x(expected_result);
        assert_eq!(turret_input_card0.dual_override_x(), expected_result);
        assert_eq!(turret_input_card0.raw[0], 0x00);
        assert_eq!(turret_input_card0.raw[1], 0xAC);
        assert_eq!(turret_input_card0.raw[2], 0x0E);
    }

    #[test]
    fn test_set_field_partially_with_cropped_overflow() {
        let overflowing_value = 0x07AB;
        let expected_result = 0x03AB;
        let mut turret_input_card0 = TurretInputCard0::default();
        turret_input_card0.set_dual_override_x(overflowing_value);
        assert_eq!(turret_input_card0.dual_override_x(), expected_result);
        assert_eq!(turret_input_card0.raw[0], 0x00);
        assert_eq!(turret_input_card0.raw[1], 0xAC);
        assert_eq!(turret_input_card0.raw[2], 0x0E);
    }

    #[test]
    fn test_set_last_field() {
        let expected_result = 0x03FF;
        let mut turret_input_card0 = TurretInputCard0::default();
        turret_input_card0.set_manual_traverse(expected_result);
        assert_eq!(turret_input_card0.manual_traverse(), expected_result);
        assert_eq!(turret_input_card0.raw[7], 0x00);
        assert_eq!(turret_input_card0.raw[8], 0xC0);
        assert_eq!(turret_input_card0.raw[9], 0xFF);
    }
}

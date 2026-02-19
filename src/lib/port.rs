use super::Bits;

#[allow(dead_code)]
pub trait Port: Bits {
    const PORT_ADDRESS: *mut u8;
    const DDR_ADDRESS: *mut u8;

    fn set_output(&self, pin: u8) {
        Self::set_bits_shift(Self::DDR_ADDRESS, pin);
    }
    fn set_input(pin: u8) {
        Self::unset_bits_shift(Self::DDR_ADDRESS, pin);
    }

    fn set_pin_high(&self, pin: u8) {
        Self::set_bits_shift(Self::PORT_ADDRESS, pin);
    }
    fn set_pin_low(&self, pin: u8) {
        Self::unset_bits_shift(Self::PORT_ADDRESS, pin);
    }
}

mod port_b;

pub use port_b::PortB;

pub trait Port {
    const PORT_ADDRESS: *mut u8;
    const DDR_ADDRESS: *mut u8;

    // bits!();
    fn set_bits(address: *mut u8, current_value: u8, value: u8);
    fn unset_bits(address: *mut u8, current_value: u8, value: u8);
    fn get_address_value(address: *mut u8) -> u8;

    fn get_port_value() -> u8 {
        Self::get_address_value(Self::PORT_ADDRESS)
    }
    fn get_ddr_value() -> u8 {
        Self::get_address_value(Self::DDR_ADDRESS)
    }

    fn set_output(&self, pin: u8) {
        Self::set_bits(
            Self::DDR_ADDRESS,
            Self::get_ddr_value(),
            pin
        );
    }
    fn set_input(pin: u8) {
        Self::unset_bits(
            Self::DDR_ADDRESS,
            Self::get_address_value(Self::DDR_ADDRESS),
            pin
        );
    }

    fn set_pin_high(&self, pin: u8) {
        Self::set_bits(
            Self::PORT_ADDRESS,
            Self::get_port_value(),
            pin
        );
    }
    fn set_pin_low(&self, pin: u8) {
        Self::unset_bits(
            Self::PORT_ADDRESS,
            Self::get_address_value(Self::PORT_ADDRESS),
            pin
        );
    }
}

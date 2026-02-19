mod definitions;

use definitions::PortDefinition;
pub use definitions::PortB;

pub struct Port {}
impl Port {
    pub fn set_output<T: PortDefinition>(pin: u8) {
        unsafe {
            core::ptr::write_volatile(<T>::DDDR_ADDRESS, 1 << pin);
        }
    }
    // unsafe fn set_input(pin: u8) {}

    pub fn set_pin_high<T: PortDefinition>(pin: u8) {
        unsafe {
            core::ptr::write_volatile(<T>::PORT_ADDRESS, 1 << pin);
        }
    }
    pub fn set_pin_low<T: PortDefinition>(pin: u8) {
        unsafe {
            core::ptr::write_volatile(
                <T>::PORT_ADDRESS,
                <T>::get_current_value::<T>() & !(1 << pin)
            );
        }
    }
}

pub trait PortDefinition {
    const PORT_ADDRESS: *mut u8;
    const DDDR_ADDRESS: *mut u8;

    fn get_current_value<T: PortDefinition>() -> u8 {
        unsafe {
            core::ptr::read_volatile(<T>::PORT_ADDRESS)
        }
    }
}

pub struct PortB;

impl PortDefinition for PortB {
    const PORT_ADDRESS: *mut u8 = (0x20 | 0x05) as *mut u8;
    const DDDR_ADDRESS: *mut u8 = (0x20 | 0x04) as *mut u8;
}

#[allow(dead_code)]
impl PortB {
    // PORTB pins start at 8
    pub const PIN_8: u8 = 0; // PB0
    pub const PIN_9: u8 = 1; // PB1
    pub const PIN_10: u8 = 2; // PB2
    pub const PIN_11: u8 = 3; // PB3
    pub const PIN_12: u8 = 4; // PB4
    pub const PIN_13: u8 = 5; // PB5
}

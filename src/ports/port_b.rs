use crate::bits;
use super::Port;

pub struct PortB;

impl Port for PortB {
    bits!();

    const PORT_ADDRESS: *mut u8 = (0x20 | 0x05) as *mut u8;
    const DDR_ADDRESS: *mut u8 = (0x20 | 0x04) as *mut u8;
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

// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72

use super::Port;

pub struct PortB;

impl Port for PortB {
    const PORT_ADDRESS: *mut u8 = (0x20 | 0x05) as *mut u8;
    const DDR_ADDRESS: *mut u8 = (0x20 | 0x04) as *mut u8;
}

#[allow(dead_code)]
impl PortB {
    pub const PB0: u8 = 0; // GPIO 8
    pub const PB1: u8 = 1; // GPIO 9
    pub const PB2: u8 = 2; // GPIO 10
    pub const PB3: u8 = 3; // GPIO 11
    pub const PB4: u8 = 4; // GPIO 12
    pub const PB5: u8 = 5; // GPIO 13 + LED_BUILTIN
}

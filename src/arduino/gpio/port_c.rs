// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=73

use super::Port;

pub struct PortC;

impl Port for PortC {
    const PORT_ADDRESS: *mut u8 = (0x20 | 0x08) as *mut u8;
    const DDR_ADDRESS: *mut u8 = (0x20 | 0x07) as *mut u8;
}

#[allow(dead_code)]
impl PortC {
    pub const PC0: u8 = 0; // GPIO 14
    pub const PC1: u8 = 1; // GPIO 15
    pub const PC2: u8 = 2; // GPIO 16
    pub const PC3: u8 = 3; // GPIO 17
    pub const PC4: u8 = 4; // GPIO 18
    pub const PC5: u8 = 5; // GPIO 19
}

#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::panic::PanicInfo;

mod helpers;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn set_high(port: *mut u8, pin: u8) {
    unsafe {
        core::ptr::write_volatile(port, 1 << pin);
    }
}

fn set_low(port: *mut u8, pin: u8) {
    unsafe {
        let port_value = core::ptr::read_volatile(port);
        core::ptr::write_volatile(port, port_value & !(1 << pin));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    // offset not needed if using e.g asm!("sdi 0x05, $PIN")
    const REGISTER_OFFSET: u8 = 0x20;
    // for pins D8 to D13
    const DDR_B: *mut u8 = (REGISTER_OFFSET | 0x04) as *mut u8;
    // for pins D8 to D13
    const PORT_B: *mut u8 = (REGISTER_OFFSET | 0x05) as *mut u8;
    // PORT B pins start at 8
    const PORT_B_PIN_OFFSET: u8 = 8;
    const PIN_13: u8 = 13 ^ PORT_B_PIN_OFFSET; // 5
    
    // serial
    const UDR0: *mut u8 = 0xC6 as *mut u8;
    const UBRR0H: *mut u8 = 0xC5 as *mut u8;
    const UBRR0L: *mut u8 = 0xC4 as *mut u8;
    // const UCSR0A: *mut u8 = 0xC0 as *mut u8;
    const UCSR0B: *mut u8 = 0xC1 as *mut u8;
    // const UCSR0C: *mut u8 = 0xC2 as *mut u8;
    const TXEN0: u8 = 3; // UCSR0B

    const DELAY_DURATION: u32 = 1000000;

    unsafe {
        // set DDR_B register for PIN_13 input
        // sbi 0x04, 5
        core::ptr::write_volatile(DDR_B, 1 << PIN_13);

        // baud rate
        core::ptr::write_volatile(UBRR0H, 0);
        // ~103 => 9600 | ~16 = 57600
        core::ptr::write_volatile(UBRR0L, 16);

        // enable transmitter
        core::ptr::write_volatile(UCSR0B, 1 << TXEN0);
    }

    loop {
        unsafe {
            // only 2 values allowed by default per frame
            // use ASCI values
            core::ptr::write_volatile(UDR0, 65); // A
            core::ptr::write_volatile(UDR0, 10); // line break
        }

        // set PIN_3 in PORT_B high
        // sbi 0x05, 5
        set_high(PORT_B, PIN_13);
        helpers::delay(DELAY_DURATION);

        // set PIN_13 in PORT_B low 
        // cbi 0x05, 5
        set_low(PORT_B, PIN_13);
        helpers::delay(DELAY_DURATION);
    }
}

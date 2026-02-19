#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::panic::PanicInfo;

mod helpers;
mod ports;

use ports::{ Port, PortB };

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {   
    // serial
    const UDR0: *mut u8 = 0xC6 as *mut u8;
    const UBRR0H: *mut u8 = 0xC5 as *mut u8;
    const UBRR0L: *mut u8 = 0xC4 as *mut u8;
    // const UCSR0A: *mut u8 = 0xC0 as *mut u8;
    const UCSR0B: *mut u8 = 0xC1 as *mut u8;
    // const UCSR0C: *mut u8 = 0xC2 as *mut u8;
    const TXEN0: u8 = 3; // UCSR0B

    const DELAY_DURATION: u32 = 1000000;

    Port::set_output::<PortB>(PortB::PIN_13);

    unsafe {
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

        Port::set_pin_high::<PortB>(PortB::PIN_13);
        helpers::delay(DELAY_DURATION);

        Port::set_pin_low::<PortB>(PortB::PIN_13);
        helpers::delay(DELAY_DURATION);
    }
}

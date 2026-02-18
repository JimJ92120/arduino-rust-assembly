#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::arch::asm;
use core::panic::PanicInfo;

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

fn delay(duration: u32) {
    unsafe {
        for _ in 1..duration {
            asm!("nop");
        }
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
    const DELAY_DURATION: u32 = 1000000;

    unsafe {
        // set DDR_B as input for PIN_13
        // sbi 0x04, 5
        core::ptr::write_volatile(DDR_B, 1 << PIN_13);
    }

    loop {
        // set PIN_3 in PORT_B high
        // sbi 0x05, 5
        set_high(PORT_B, PIN_13);
        delay(DELAY_DURATION);

        // set PIN_13 in PORT_B low 
        // cbi 0x05, 5
        set_low(PORT_B, PIN_13);
        delay(DELAY_DURATION);
    }
}

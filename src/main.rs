#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() { 
    loop {}
}

#![no_std]
#![no_main]
#![allow(special_module_name)]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

mod panic;
mod lib;
mod arduino;

mod uart;
mod gpio;

use lib::helpers;
use arduino::{ Port, PortB, PortD };

static FREQUENCY: u32 = 16_000_000;
static BAUD_RATE: u32 = 57_600;
static ENABLE_TRANSMISSION: bool = true;
static ENABLE_RECEPTION: bool = true;

static REGISTER_OFFSET: u8 = 0x20;

#[unsafe(no_mangle)]
pub extern "C" fn main() {   
    const DELAY_DURATION: u32 = 1_000_000;

    uart::init(BAUD_RATE, FREQUENCY, ENABLE_TRANSMISSION, ENABLE_RECEPTION);

    gpio::set_output(PortB::DDR_ADDRESS, PortB::PB5);
    gpio::set_output(PortD::DDR_ADDRESS, PortD::PD7);

    loop {
        uart::send("high\n");
        gpio::set_high(PortB::PORT_ADDRESS, PortB::PB5);
        gpio::set_high(PortD::PORT_ADDRESS, PortD::PD7);
        helpers::delay(DELAY_DURATION);

        uart::send("low\n");
        gpio::set_low(PortB::PORT_ADDRESS, PortB::PB5);
        gpio::set_low(PortD::PORT_ADDRESS, PortD::PD7);
        helpers::delay(DELAY_DURATION);
    }
}

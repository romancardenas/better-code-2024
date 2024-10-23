//! Basic blinking LED example using CLINT for "sleep" in a loop.

#![no_std]
#![no_main]

use hifive1::{
    hal::{e310x::CLINT, prelude::*, DeviceResources},
    pin, Led,
};
use semihosting::println;

#[riscv_rt::entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let pins = dr.pins;

    // get blue LED pin
    let pin = pin!(pins, led_blue);
    let mut led = pin.into_inverted_output();

    // Get the sleep struct from CLINT
    let mut sleep = CLINT::delay();

    const STEP: u32 = 1000; // 1s
    loop {
        Led::toggle(&mut led);
        println!("LED toggled. New state: {}", led.is_on());
        sleep.delay_ms(STEP);
    }
}

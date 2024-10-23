//! Demonstration on how to configure the GPIO9 interrupt on HiFive boards.

#![no_main]
#![no_std]

use hifive1::{
    hal::DeviceResources,
    hal::{
        e310x::{Gpio0, PLIC},
        prelude::*,
    },
    pin, Led,
};
use semihosting::println;

/* Handler for the GPIO9 interrupt */
#[riscv_rt::external_interrupt(ExternalInterrupt::GPIO9)]
fn gpio9_handler() {
    println!("GPIO9 interrupt!");
    // Clear the GPIO pending interrupt
    let gpio_block = unsafe { Gpio0::steal() };
    gpio_block.fall_ip().write(|w| w.pin9().set_bit());
    gpio_block.rise_ip().write(|w| w.pin9().set_bit());
}

#[riscv_rt::entry]
fn main() -> ! {
    let resources = DeviceResources::take().unwrap();
    let pins = resources.pins;

    println!("Configuring GPIOs...");
    // Configure button pin (GPIO9) as pull-up input
    let mut button = pins.pin9.into_pull_up_input();
    // Configure blue LED pin (GPIO21) as inverted output
    let mut led = pin!(pins, led_blue).into_inverted_output();

    println!("Configuring external interrupts...");
    // Set button interrupt source priority
    let priorities = PLIC::priorities();
    priorities.reset::<ExternalInterrupt>();
    unsafe { priorities.set_priority(ExternalInterrupt::GPIO9, Priority::P1) };

    // Enable GPIO9 interrupt for both edges
    let gpio_block = unsafe { Gpio0::steal() };
    unsafe {
        // Clear pending interrupts from previous states
        gpio_block.fall_ie().write(|w| w.bits(0x00000000));
        gpio_block.rise_ie().write(|w| w.bits(0x00000000));
        gpio_block.fall_ip().write(|w| w.bits(0xffffffff));
        gpio_block.rise_ip().write(|w| w.bits(0xffffffff));
    }
    gpio_block.fall_ie().write(|w| w.pin9().set_bit());
    gpio_block.rise_ie().write(|w| w.pin9().set_bit());

    println!("Enabling external interrupts...");
    // Enable GPIO9 interrupt in PLIC
    let ctx = PLIC::ctx0();
    unsafe {
        ctx.enables().disable_all::<ExternalInterrupt>();
        ctx.threshold().set_threshold(Priority::P0);
        ctx.enables().enable(ExternalInterrupt::GPIO9);
        riscv::interrupt::enable();
        PLIC::enable();
    }

    loop {
        if button.is_low().unwrap() {
            println!("Button pressed");
            led.on();
        } else {
            println!("Button released");
            led.off();
        }
        println!("LED is on: {}", led.is_on());
        riscv::asm::wfi();
    }
}

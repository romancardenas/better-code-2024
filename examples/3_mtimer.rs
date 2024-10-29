//! This example demonstrates how to configure the CLINT to generate
//! periodic interrupts using the machine timer.

#![no_main]
#![no_std]

use hifive1::hal::{e310x::CLINT, prelude::*};
use semihosting::println;

const PERIOD_MS: u64 = 1000;
const FREQUENCY_HZ: u64 = 32768;
const CLINT_TICKS_PER_PERIOD: u64 = PERIOD_MS * FREQUENCY_HZ / 1000;

/// Handler for the machine timer interrupt (handled by the CLINT)
#[riscv_rt::core_interrupt(CoreInterrupt::MachineTimer)]
fn mtimer_handler() {
    println!("MTIMER interrupt!");
    CLINT::mtimecmp0().modify(|f| *f += CLINT_TICKS_PER_PERIOD);
}

#[riscv_rt::entry]
fn main() -> ! {
    println!("Configuring CLINT...");
    CLINT::mtimer_disable();
    let mtimer = CLINT::mtimer();
    let (mtimecmp, mtime) = (mtimer.mtimecmp0, mtimer.mtime);
    mtime.write(0);
    mtimecmp.write(CLINT_TICKS_PER_PERIOD);

    println!("Enabling interrupts...");
    unsafe {
        riscv::interrupt::enable();
        CLINT::mtimer_enable();
    }

    loop {
        println!("Sleeping...");
        riscv::asm::wfi();
    }
}

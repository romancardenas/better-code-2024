#![no_std]
#![no_main]

extern crate riscv_slic;

use hifive1::hal::{
    e310x::{self, CLINT},
    prelude::*,
};
use semihosting::{println, process::exit};

// generate SLIC code for this example
riscv_slic::codegen!(
    pac = e310x,
    swi = [SoftLow, SoftMedium, SoftHigh],
    backend = [hart_id = H0]
);
use slic::SoftwareInterrupt; // Re-export of automatically generated enum of interrupts in previous macro

/// HW handler for MachineTimer interrupts triggered by CLINT.
#[riscv_rt::core_interrupt(CoreInterrupt::MachineTimer)]
fn machine_timer() {
    println!("MTIMER interrupt!");
    let mtimecmp = CLINT::mtimecmp0();
    mtimecmp.modify(|val| *val += CLINT::freq() as u64);
}

/// Handler for SoftLow task (low priority).
#[allow(non_snake_case)]
#[no_mangle]
fn SoftLow() {
    println!("start SoftLow");
    println!("middle SoftLow");
    println!("stop SoftLow");
}

/// Handler for SoftMedium task (medium priority). This task pends both SoftLow and SoftHigh.
#[allow(non_snake_case)]
#[no_mangle]
fn SoftMedium() {
    println!("  start SoftMedium");
    riscv_slic::pend(SoftwareInterrupt::SoftLow);
    println!("  middle SoftMedium");
    riscv_slic::pend(SoftwareInterrupt::SoftHigh);
    println!("  stop SoftMedium");
}

/// Handler for SoftHigh task (high priority).
#[allow(non_snake_case)]
#[no_mangle]
fn SoftHigh() {
    println!("    start SoftHigh");
    println!("    middle SoftHigh");
    println!("    stop SoftHigh");
}

#[riscv_rt::entry]
fn main() -> ! {
    println!("Configuring CLINT...");
    // First, we make sure that all PLIC the interrupts are disabled and set the interrupts priorities
    CLINT::disable();
    let mtimer = CLINT::mtimer();
    mtimer.mtimecmp0.write(CLINT::freq() as u64);
    mtimer.mtime.write(0);

    println!("Configuring SLIC...");
    // make sure that interrupts are off
    riscv_slic::disable();
    riscv_slic::clear_interrupts();
    // Set priorities
    unsafe {
        riscv_slic::set_priority(SoftwareInterrupt::SoftLow, 1); // low priority
        riscv_slic::set_priority(SoftwareInterrupt::SoftMedium, 2); // medium priority
        riscv_slic::set_priority(SoftwareInterrupt::SoftHigh, 3); // high priority
    }

    println!("Enabling interrupts...");
    unsafe {
        riscv_slic::set_interrupts();
        CLINT::mtimer_enable();
        riscv_slic::enable();
    }

    println!("Done!");

    println!("Waiting for interrupts...");
    riscv_slic::riscv::asm::wfi();
    riscv_slic::pend(SoftwareInterrupt::SoftMedium);
    exit(0)
}

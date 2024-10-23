#![no_std]
#![no_main]

extern crate hifive1;
use semihosting::{println, process::exit};

#[riscv_rt::entry]
fn main() -> ! {
    println!("Hello, world!");
    exit(0);
}

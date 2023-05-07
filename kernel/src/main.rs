#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(lazy_cell)]
#![allow(unused)]

// extern crate alloc;
extern crate riscv_rt;

use riscv_rt::entry;
mod panic_handler;
mod utils;

#[entry]
fn main(a0: usize) -> ! {
    println!("Hello world from hart {}\n", a0);

    utils::shutdown();
}

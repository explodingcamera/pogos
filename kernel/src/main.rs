#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(lazy_cell)]
#![allow(unused)]

extern crate alloc;
extern crate riscv_rt;

// enable our custom panic handler
mod panic_handler;

// handle interrupts and exceptions
mod trap;

mod io;
mod ksched;
mod mem;
mod symbols;
mod tasks;
mod util;

use alloc::{boxed::Box, string::String, vec, vec::Vec};
use riscv_rt::entry;
use tasks::Fuse;

#[entry]
fn main(hart_id: usize) -> ! {
    if hart_id != 0 {
        panic!("hart {} is not the boot hart, stopping", hart_id);
    }

    println!();
    println!("== starting pogos kernel on hart {} ==", hart_id);
    println!();

    // Setup everything required for the kernel to run
    unsafe {
        // initialize the kernel heap allocator, alloc is now available
        mem::heap_alloc::init_kernel_heap();
        println!(">>> kernel heap initialized");

        // initialize the frame allocator
        mem::frame_alloc::init_frame_allocator();
        println!(">>> frame allocator initialized");

        // initialize the kernel memory map
        mem::map_kernel::init_kernel_memory_map();
        println!(">>> kernel memory map initialized");

        // todo: initialize mmu here
        mem::init_mmu();
        println!(">>> mmu enabled");
    }

    println!("kernel initialized, starting kschedule\n");

    ksched::KernelScheduler::new()
        .with_task(Box::pin(async { tasks::console().await }.fuse()))
        .block_on_run();

    util::shutdown()
}

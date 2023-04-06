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
mod mem;
mod symbols;
mod util;

use riscv_rt::entry;

#[entry]
fn main(hart_id: usize) -> ! {
    if hart_id != 0 {
        panic!("hart {} is not the boot hart, stopping", hart_id);
    }

    println!("starting kernel on hart {}", hart_id);

    // Setup everything required for the kernel to run
    unsafe {
        // initialize the kernel heap allocator, alloc is now available
        mem::heap_alloc::init_kernel_heap();

        // initialize the frame allocator
        mem::frame_alloc::init_frame_allocator();

        // initialize the kernel memory map
        mem::map_kernel::init_kernel_memory_map();

        // todo: initialize mmu here
        mem::init_mmu();
    }

    println!("kernel initialized, shutting down");
    util::shutdown();
}

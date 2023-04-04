#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(lazy_cell)]

extern crate alloc;
extern crate riscv_rt;

use core::panic::PanicInfo;
use sbi::system_reset::{ResetReason, ResetType};
use util::*;

mod mem;
mod symbols;
mod util;

use riscv_rt::entry;

extern "C" {
    static _kernel_end: u8;
}

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
    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::NoReason);
    unreachable!("System reset failed");
}

#[export_name = "DefaultHandler"]
fn default_handler() {
    print("Default handler called\n");
}

#[export_name = "ExceptionHandler"]
fn custom_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    println!("Exception handler called");
    println!("Trap frame: {:?}", trap_frame);

    let cause = riscv::register::scause::read();
    println!("Exception cause: {:?}", cause.cause());

    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::SystemFailure);
    unreachable!("System reset failed");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print("Panic ");
    if let Some(location) = info.location() {
        println!(
            "at {}:{}:{}: ",
            location.file(),
            location.line(),
            location.column(),
        );
    }
    print(
        info.message()
            .unwrap()
            .as_str()
            .unwrap_or("Unknown panic message\n"),
    );

    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::SystemFailure);
    unreachable!("System reset failed");
}

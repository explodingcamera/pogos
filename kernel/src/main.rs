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
// mod frame_alloc;
mod heap_alloc;
mod mem;
mod symbols;
mod test;
mod util;

use riscv_rt::entry;

extern "C" {
    static _kernel_end: u8;
}

#[entry]
fn main(hart_id: usize) -> ! {
    if hart_id != 0 {
        println!("hart {} is not the boot hart, stopping", hart_id);
        loop {}
    }

    println!("starting kernel on hart {}", hart_id);
    let status = sbi::hsm::hart_status(hart_id).unwrap();
    println!("hart status: {:?}", status);

    symbols::debug();

    // Setup everything required for the kernel to run
    unsafe {
        heap_alloc::init_kernel_heap(); // initialize the kernel heap allocator, alloc is now available
        test::init();
        // mem::init_paging();
        mem::init_mmu();
    }

    // let x = vec![1, 2, 3];
    // let executor = pasts::Executor::default();

    panic!("Stopping kernel");
}

#[export_name = "DefaultHandler"]
fn default_handler() {
    print("Default handler called\n");
}

#[export_name = "ExceptionHandler"]
fn custom_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    println!("Exception handler called");
    println!("Trap frame: {:?}", trap_frame);

    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::SystemFailure);
    loop {}
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
    loop {}
}

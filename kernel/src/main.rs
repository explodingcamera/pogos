#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate riscv_rt;

use core::panic::PanicInfo;
use sbi::system_reset::{ResetReason, ResetType};
use util::{print, print_usize};
mod allocator;
mod util;

use riscv_rt::entry;
static STARTED: &str = "Started PogOs!\n";

#[entry]
fn main() -> ! {
    // do something here
    print(STARTED);

    // initialize heap for kernel
    unsafe {
        allocator::init_heap();
    }

    print_usize(1);
    panic!("Panic test");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("Paniced: ");
    if let Some(message) = _info.message() {
        print(message.as_str().unwrap_or("Unknown panic message"));
        print("\n");
    }
    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::SystemFailure);
    loop {}
}

#![no_std]
#![no_main]
#![feature(panic_info_message)]

extern crate riscv_rt;

use core::panic::PanicInfo;
use util::print;
mod util;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // do something here
    print("Started PogOs!\n");
    panic!("Panic test");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("Paniced: ");
    if let Some(message) = _info.message() {
        print(message.as_str().unwrap_or("Unknown panic message"));
        print("\n");
    }
    sbi::legacy::shutdown()
}

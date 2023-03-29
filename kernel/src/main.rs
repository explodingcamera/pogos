#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::{arch::global_asm, panic::PanicInfo};
use util::print;
mod init;
mod util;

entry!(main);
fn main() -> ! {
    print("Started PogOs!\n");
    sbi::legacy::shutdown()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Nothing here now
    sbi::legacy::shutdown()
}

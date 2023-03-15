#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("entry.asm"));

#[inline]
fn print(t: &str) {
    for c in t.chars() {
        let c: u8 = c.try_into().unwrap_or(0);
        sbi::legacy::console_putchar(c)
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Nothing here now
    sbi::legacy::shutdown()
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
fn start() {
    clear_bss();
    print("Started PogOs!\n");
    sbi::legacy::shutdown()
}

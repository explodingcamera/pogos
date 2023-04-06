use crate::println;
use core::{hint::unreachable_unchecked, panic::PanicInfo};
use sbi::system_reset::{ResetReason, ResetType};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    let _ = sbi::system_reset::system_reset(ResetType::Shutdown, ResetReason::SystemFailure);

    unsafe {
        println!("System reset failed");
        unreachable_unchecked(); // this can pretty much only happen if there is a bug in the sbi implementation or if sbi is not present, unreachable_unchecked so we don't panic again
    }
}

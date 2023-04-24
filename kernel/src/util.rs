use core::{future::Future, pin::Pin, task::Poll};

use riscv::register::medeleg::{clear_machine_env_call, set_supervisor_env_call};
use sbi::ecall3;

#[macro_export]
macro_rules! print {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::util::print_args(format_args!($fmt $(,$($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::print!($fmt $(,$($arg)+)?);
        $crate::util::print("\n");
    };
    () => {
        $crate::util::print("\n");
    }
}

pub fn shutdown() -> ! {
    let _ = sbi::system_reset::system_reset(
        sbi::system_reset::ResetType::Shutdown,
        sbi::system_reset::ResetReason::NoReason,
    );
    unreachable!("System reset failed");
}

struct Writer {}

pub fn print_args(t: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Writer {};
    writer.write_fmt(t).unwrap();
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

pub fn print(t: &str) {
    t.chars().for_each(|c| {
        let c: u8 = c.try_into().unwrap_or(b'?');
        sbi::legacy::console_putchar(c) // TODO: replace with the new SBI debug extension once it's available in all SBI implementations
    });
}

struct DebugConsole();
impl Future for DebugConsole {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<Self::Output> {
        match sbi::legacy::console_getchar() {
            Some(c) => Poll::Ready(c),
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

pub async fn get_char() -> u8 {
    DebugConsole().await
}

// Not supported yet by any SBI implementation
//
// pub fn sbi_debug_write(t: &str) {
//     let num_bytes = t.len();

//     unsafe {
//         // get the address of the string
//         let base_addr = t.as_ptr() as usize;
//         let base_addr_lo = base_addr & 0xFFFFFFFF;
//         let base_addr_hi = (base_addr >> 32);

//         let error: isize;
//         let value: usize;

//         core::arch::asm!(
//             "ecall",
//             inlateout("a0") num_bytes => error,
//             inlateout("a1") base_addr_lo => value,
//             in("a2") base_addr_hi,
//             in("a6") 0x0,
//             in("a7") 0x4442434E,
//         );

//         println!("error: {}, value: {}", error, value);
//     }
// }

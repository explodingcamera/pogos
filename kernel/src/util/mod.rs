use core::{future::Future, pin::Pin, task::Poll};

use riscv::register::medeleg::{clear_machine_env_call, set_supervisor_env_call};
use sbi::ecall3;

mod console;
mod macros;
mod power;
pub mod sstc;
pub use console::*;
pub use power::*;

pub type Result<T> = core::result::Result<T, &'static str>;

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

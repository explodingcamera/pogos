use core::{future::Future, pin::Pin, task::Poll};

mod console;
mod macros;
mod power;
pub mod sbi;

pub mod sstc;
pub use console::*;
pub use power::*;

pub type Result<T> = core::result::Result<T, &'static str>;

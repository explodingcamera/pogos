#![no_std]

extern crate alloc;
pub mod address;
pub mod frame_allocator;
pub mod mem;
pub mod page;

pub const TABLE_ENTRY_CNT: usize = 512;

pub const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_ORDER;

pub const CLINT_BASE: usize = 0x200_0000;
pub const PLIC_BASE: usize = 0x0c00_0000;

pub const SAPT_MODE: riscv::register::satp::Mode = riscv::register::satp::Mode::Sv39;

use alloc::boxed::Box;
use frame_allocator::{FrameAllocator, StackFrameAllocator, WrappedFrameAllocator};
use spin::Once;

pub static GLOBAL_FRAME_ALLOCATOR: Once<Box<dyn WrappedFrameAllocator>> = Once::new();

pub fn register_global_frame_allocator<F: WrappedFrameAllocator>(allocator: F) {
    GLOBAL_FRAME_ALLOCATOR.call_once(|| Box::new(allocator));
}

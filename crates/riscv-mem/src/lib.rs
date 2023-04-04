#![no_std]

extern crate alloc;
pub mod address;
pub mod frame_allocator;
pub mod mem;
pub mod page;

use frame_allocator::StackFrameAllocator;
use riscv::register::satp;
use spin::Once;

pub const TABLE_ENTRY_CNT: usize = 512;
pub const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_ORDER;
pub const CLINT_BASE: usize = 0x200_0000;
pub const PLIC_BASE: usize = 0x0c00_0000;
pub const SAPT_MODE: satp::Mode = satp::Mode::Sv39;

pub type MutexGuard<'a, T> = spin::MutexGuard<'a, T>;
pub type GuardedFrameAllocator = MutexGuard<'static, StackFrameAllocator>;
pub type GetterFn = fn() -> GuardedFrameAllocator;

pub fn get_frame_allocator() -> GuardedFrameAllocator {
    GET_FRAME_ALLOCATOR
        .get()
        .expect("frame allocator not initialized")()
}

static GET_FRAME_ALLOCATOR: Once<GetterFn> = Once::new();

pub fn register_frame_allocator(get: GetterFn) {
    GET_FRAME_ALLOCATOR.call_once(|| get);
}

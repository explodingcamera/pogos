#![no_std]

extern crate alloc;
pub mod address;
pub mod frame_allocator;
pub mod mem;
pub mod page;
use riscv::register::satp;

pub const TABLE_ENTRY_CNT: usize = 512;
pub const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_ORDER;
pub const SAPT_MODE: satp::Mode = satp::Mode::Sv39;

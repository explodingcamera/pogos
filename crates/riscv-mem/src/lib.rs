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
pub const CLINT_BASE: usize = 0x200_0000;
pub const PLIC_BASE: usize = 0x0c00_0000;
pub const SAPT_MODE: satp::Mode = satp::Mode::Sv39;

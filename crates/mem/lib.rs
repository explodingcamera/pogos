#![no_std]

pub struct PhysicalAddress(u64);
pub struct VirtualAddress(u64);

pub struct MemoryArea {
    pub start: PhysicalAddress,
    pub size: usize,
}

pub trait FrameAllocator {
    unsafe fn allocate(&mut self, count: usize) -> Option<PhysicalAddress>;
    unsafe fn free(&mut self, address: PhysicalAddress, count: usize);
}

pub trait Architecture {
    unsafe fn init() -> &'static [MemoryArea];

    fn virt_is_valid(address: VirtualAddress) -> bool;

    #[inline(always)]
    unsafe fn read<T>(address: VirtualAddress) -> T {
        core::ptr::read(address.0 as *const T)
    }

    #[inline(always)]
    unsafe fn write<T>(address: VirtualAddress, value: T) {
        core::ptr::write(address.0 as *mut T, value)
    }

    #[inline(always)]
    unsafe fn write_bytes(address: VirtualAddress, value: u8, count: usize) {
        core::ptr::write_bytes(address.0 as *mut u8, value, count)
    }
}

pub trait VirtualMemory {
    type Allocator: FrameAllocator;
    type Architecture: Architecture;
}

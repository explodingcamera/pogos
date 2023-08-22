#![no_std]

pub struct PhysicalAddress(u64);
pub struct VirtualAddress(u64);

pub struct MemoryArea {
    pub start: PhysicalAddress,
    pub size: usize,
}

pub trait FrameAllocator {
    /// # Safety
    /// The caller must ensure that the returned address is valid.
    unsafe fn allocate(&mut self, count: usize) -> Option<PhysicalAddress>;

    /// # Safety
    /// The caller must ensure that the given address is valid.
    unsafe fn free(&mut self, address: PhysicalAddress, count: usize);
}

pub trait Architecture {
    /// # Safety
    /// This function must be called only once.
    unsafe fn init() -> &'static [MemoryArea];

    fn virt_is_valid(address: VirtualAddress) -> bool;

    /// # Safety
    /// See `core::ptr::read`
    #[inline(always)]
    unsafe fn read<T>(address: VirtualAddress) -> T {
        core::ptr::read(address.0 as *const T)
    }

    /// # Safety
    /// See `core::ptr::write`
    #[inline(always)]
    unsafe fn write<T>(address: VirtualAddress, value: T) {
        core::ptr::write(address.0 as *mut T, value)
    }

    /// # Safety
    /// See `core::ptr::write_bytes`
    #[inline(always)]
    unsafe fn write_bytes(address: VirtualAddress, value: u8, count: usize) {
        core::ptr::write_bytes(address.0 as *mut u8, value, count)
    }
}

pub trait VirtualMemory {
    type Allocator: FrameAllocator;
    type Architecture: Architecture;
}

use buddy_system_allocator::LockedHeap;

extern "C" {
    static _sheap: u8;
    static _heap_size: u8;
}

#[global_allocator]
static KERNEL_HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::new();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!(
        "Allocator failed to allocate {} bytes with {}-byte alignment.",
        layout.size(),
        layout.align()
    );
}

/// Initialize the heap allocator.
pub unsafe fn init_kernel_heap() {
    KERNEL_HEAP_ALLOCATOR.lock().init(
        &_sheap as *const u8 as usize,
        &_heap_size as *const u8 as usize,
    );
}

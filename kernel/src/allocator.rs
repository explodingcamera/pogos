use buddy_system_allocator::LockedHeap;

use crate::{
    println,
    symbols::{HEAP_SIZE, HEAP_START},
};

#[global_allocator]
static KERNEL_HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::new();
// static mut KERNEL_HEAP: [u8; 0x20000] = [0; 0x20000]; //

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    println!(
        "Allocator failed to allocate: {:?} bytes with {:?}-byte alignment.",
        layout.size(),
        layout.align(),
    );

    panic!("Allocator failed to allocate")
}

/// Initialize the heap allocator.
pub unsafe fn init_kernel_heap() {
    // let heap_start = KERNEL_HEAP.as_ptr() as usize;
    // let heap_size = KERNEL_HEAP.len();

    let heap_start = HEAP_START();
    let heap_size = HEAP_SIZE();

    println!(
        "KernelHeap: start: {:#x}, size: {:#x}",
        heap_start, heap_size
    );

    KERNEL_HEAP_ALLOCATOR.lock().init(heap_start, heap_size);
}

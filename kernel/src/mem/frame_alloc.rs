use crate::symbols::{MEMORY_END, STACK_END};
use alloc::{boxed::Box, vec::Vec};
use riscv_mem::frame_allocator::{FrameAllocator, FrameTracker, StackFrameAllocator};

use spin::{lock_api, Mutex, MutexGuard, Once};

// TODO: this might have issues with multiple cores and interrupts
static FRAME_ALLOCATOR: Once<Mutex<StackFrameAllocator>> = Once::new();

pub unsafe fn init_frame_allocator() {
    FRAME_ALLOCATOR.call_once(|| {
        let mut frame_allocator = StackFrameAllocator::new();
        frame_allocator.init(
            riscv_mem::address::PhysAddr::from(STACK_END()).ceil(),
            riscv_mem::address::PhysAddr::from(MEMORY_END()).floor(),
        );

        Mutex::new(frame_allocator)
    });
}

pub fn alloc_fn() -> Option<FrameTracker> {
    frame_allocator().lock().frame_alloc()
}

pub fn frame_allocator<'a>() -> &'a Mutex<StackFrameAllocator> {
    FRAME_ALLOCATOR
        .get()
        .expect("frame allocator not initialized")
}

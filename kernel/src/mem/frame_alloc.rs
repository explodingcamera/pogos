use crate::symbols::{MEMORY_END, STACK_END};
use riscv_mem::{frame_allocator::StackFrameAllocator, GuardedFrameAllocator};
use spin::{Mutex, Once};

pub static FRAME_ALLOCATOR: Once<Mutex<StackFrameAllocator>> = Once::new();

pub unsafe fn init_frame_allocator() {
    FRAME_ALLOCATOR.call_once(|| {
        let mut frame_allocator = StackFrameAllocator::new();
        frame_allocator.init(
            riscv_mem::address::PhysAddr::from(STACK_END()).ceil(),
            riscv_mem::address::PhysAddr::from(MEMORY_END()).floor(),
        );
        Mutex::new(frame_allocator)
    });

    riscv_mem::register_frame_allocator(get_frame_allocator);
}

pub fn get_frame_allocator() -> GuardedFrameAllocator {
    FRAME_ALLOCATOR
        .get()
        .expect("frame allocator not initialized")
        .lock()
}

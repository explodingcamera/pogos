use buddy_system_allocator::FrameAllocator as BuddyFrameAllocator;
use once_cell::sync::OnceCell;
use riscv_mem::{address::PhysPageNum, FrameAllocator};
use spin::mutex::SpinMutex;

pub static FRAME_ALLOCATOR: OnceCell<SpinMutex<Allocator>> = OnceCell::new();

pub struct Allocator(BuddyFrameAllocator<32>);

impl FrameAllocator for Allocator {
    fn new() -> Self {
        Allocator(BuddyFrameAllocator::new())
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        self.0.alloc(1).map(|frame| frame.into())
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        self.0.dealloc(ppn.into(), 1);
    }
}

pub fn init() {
    let allocator = Allocator::new();
    FRAME_ALLOCATOR
        .set(SpinMutex::new(allocator))
        .unwrap_or_else(|_| panic!("FRAME_ALLOCATOR already initialized"));
}

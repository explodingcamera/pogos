use riscv_mem::{
    address::PhysAddr,
    frame_allocator::{FrameAllocator, StackFrameAllocator, WrappedFrameAllocator},
    register_global_frame_allocator,
};
use spin::{Mutex, Once};

use crate::symbols::{MEMORY_END, STACK_END};

struct FrameAllocatorWrapper(&'static Once<Mutex<StackFrameAllocator>>);
pub static FRAME_ALLOCATOR: Once<Mutex<StackFrameAllocator>> = Once::new();

pub unsafe fn init_frame_allocator() {
    FRAME_ALLOCATOR.call_once(|| {
        let mut frame_allocator = StackFrameAllocator::new();
        frame_allocator.init(
            PhysAddr::from(STACK_END()).ceil(),
            PhysAddr::from(MEMORY_END()).floor(),
        );
        Mutex::new(frame_allocator)
    });

    register_global_frame_allocator(FrameAllocatorWrapper(&FRAME_ALLOCATOR));
}

impl WrappedFrameAllocator for FrameAllocatorWrapper {
    fn alloc(&self) -> Option<riscv_mem::address::PhysPageNum> {
        self.0
            .get()
            .expect("frame allocator not initialized")
            .lock()
            .alloc()
    }

    fn alloc_more(&self, pages: usize) -> Option<alloc::vec::Vec<riscv_mem::address::PhysPageNum>> {
        self.0
            .get()
            .expect("frame allocator not initialized")
            .lock()
            .alloc_more(pages)
    }

    fn dealloc(&self, ppn: riscv_mem::address::PhysPageNum) {
        self.0
            .get()
            .expect("frame allocator not initialized")
            .lock()
            .dealloc(ppn)
    }
}

// pub fn frame_alloc() -> Option<FrameTracker<A>> {
//     FRAME_ALLOCATOR
//         .get()
//         .expect("frame allocator not initialized")
//         .lock()
//         .alloc()
//         .map(FrameTracker::new)
// }

// pub fn frame_alloc_more(num: usize) -> Option<Vec<FrameTracker>> {
//     FRAME_ALLOCATOR
//         .get()
//         .expect("frame allocator not initialized")
//         .lock()
//         .alloc_more(num)
//         .map(|x| x.iter().map(|&t| FrameTracker::new(t)).collect())
// }

// pub fn frame_dealloc(ppn: PhysPageNum) {
//     FRAME_ALLOCATOR
//         .get()
//         .expect("frame allocator not initialized")
//         .lock()
//         .dealloc(ppn);
// }

// #[allow(unused)]
// pub fn frame_allocator_test() {
//     let mut v: Vec<FrameTracker> = Vec::new();
//     for i in 0..5 {
//         let frame = frame_alloc().unwrap();
//         v.push(frame);
//     }
//     v.clear();
//     for i in 0..5 {
//         let frame = frame_alloc().unwrap();
//         v.push(frame);
//     }
//     drop(v);
//     println!("frame_allocator_test passed!");
// }

// #[allow(unused)]
// pub fn frame_allocator_alloc_more_test() {
//     let mut v: Vec<FrameTracker> = Vec::new();
//     let frames = frame_alloc_more(5).unwrap();
//     for frame in &frames {
//         println!("{:?}", frame);
//     }
//     v.extend(frames);
//     v.clear();
//     let frames = frame_alloc_more(5).unwrap();
//     for frame in &frames {
//         println!("{:?}", frame);
//     }
//     drop(v);
//     println!("frame_allocator_test passed!");
// }

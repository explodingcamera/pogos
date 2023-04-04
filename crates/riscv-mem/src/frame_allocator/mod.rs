use crate::{address::PhysPageNum, get_frame_allocator};
use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};

mod stack;
pub use stack::StackFrameAllocator;

#[derive(Clone)]
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }

        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        get_frame_allocator().frame_dealloc(self.ppn);
    }
}

pub trait FrameAllocator {
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn alloc_more(&mut self, pages: usize) -> Option<Vec<PhysPageNum>>;
    fn dealloc(&mut self, ppn: PhysPageNum);

    fn frame_alloc(&mut self) -> Option<FrameTracker> {
        self.alloc().map(FrameTracker::new)
    }

    fn frame_alloc_more(&mut self, num: usize) -> Option<Vec<FrameTracker>> {
        self.alloc_more(num)
            .map(|x| x.iter().map(|&t| FrameTracker::new(t)).collect())
    }

    fn frame_dealloc(&mut self, ppn: PhysPageNum) {
        self.dealloc(ppn);
    }
}

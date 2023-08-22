use crate::address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use crate::frame_allocator::{FrameAllocFn, FrameTracker};

use alloc::vec;
use alloc::vec::Vec;

use super::{PTEFlags, PageTableEntry};

#[derive(Clone)]
pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
    allocate_frame: FrameAllocFn,
}

/// Assume that it won't oom when creating/mapping.
impl PageTable {
    pub fn new(allocate_frame: FrameAllocFn) -> Self {
        let frame = allocate_frame().expect("oom when creating page table");

        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
            allocate_frame,
        }
    }

    /// Temporarily used to get arguments from user space.
    pub fn new_from_token(satp: usize, allocate_frame: FrameAllocFn) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
            allocate_frame,
        }
    }

    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = (self.allocate_frame)().expect("failed to allocate frame");
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }
    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.clone().floor()).map(|pte| {
            let aligned_pa: PhysAddr = pte.ppn().into();
            let offset = va.page_offset();
            let aligned_pa_usize: usize = aligned_pa.into();
            (aligned_pa_usize + offset).into()
        })
    }
    pub fn root_token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }

    pub fn root_ppn(&self) -> PhysPageNum {
        self.root_ppn
    }
}

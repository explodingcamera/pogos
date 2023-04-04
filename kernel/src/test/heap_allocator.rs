// use buddy_system_allocator::LockedHeap;

// use crate::println;

// #[global_allocator]
// static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

// #[alloc_error_handler]
// pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
//     panic!("Heap allocation error, layout = {:?}", layout);
// }

// pub const KERNEL_HEAP_SIZE: usize = 0x10000;
// static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

// pub unsafe fn init_heap() {
//     HEAP_ALLOCATOR
//         .lock()
//         .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
// }

// #[allow(unused)]
// pub fn heap_test() {
//     use alloc::boxed::Box;
//     use alloc::vec::Vec;
//     extern "C" {
//         fn sbss();
//         fn ebss();
//     }
//     let bss_range = sbss as usize..ebss as usize;
//     let a = Box::new(5);
//     assert_eq!(*a, 5);
//     assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
//     drop(a);
//     let mut v: Vec<usize> = Vec::new();
//     for i in 0..500 {
//         v.push(i);
//     }
//     for (i, val) in v.iter().take(500).enumerate() {
//         assert_eq!(*val, i);
//     }
//     assert!(bss_range.contains(&(v.as_ptr() as usize)));
//     drop(v);
//     println!("heap_test passed!");
// }

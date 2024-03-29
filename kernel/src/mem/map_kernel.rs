// use crate::{dtb::DEVICE_TREE, io::MMIO_DEVICES, println, symbols::*};
// use alloc::sync::Arc;
// use riscv_mem::{
//     address::VirtAddr,
//     mem::{MapArea, MapPermission, MapType, MemorySet},
//     page::PageTable,
// };
// use spin::{Mutex, Once};

// // TODO: this might have issues with multiple cores and interrupts
// pub static KERNEL_SPACE: Once<Arc<MemorySet>> = Once::new();

// pub fn init_kernel_memory_map() {
//     KERNEL_SPACE.call_once(|| {
//         let kernel_map = new_kernel();
//         kernel_map.activate();
//         Arc::new(kernel_map)
//     });

//     println!(">>> kernel memory map activated");
// }

// pub fn kernel_virt_to_phys(vaddr: usize) -> usize {
//     PageTable::new_from_token(
//         KERNEL_SPACE
//             .get()
//             .expect("kernel memory map not initialized")
//             .root_token(),
//         super::frame_alloc::alloc_fn,
//     )
//     .translate_va(VirtAddr::from(vaddr))
//     .unwrap()
//     .0
// }

// fn new_kernel() -> MemorySet {
//     let mut memory_set = MemorySet::new_bare(super::frame_alloc::alloc_fn);

//     // println!("mapping .text section");
//     memory_set.push(
//         MapArea::new(
//             (TEXT_START()).into(),
//             (TEXT_END() - 1).into(),
//             MapType::Identical,
//             MapPermission::R | MapPermission::X,
//             super::frame_alloc::alloc_fn,
//         ),
//         None,
//     );
//     // println!(
//     //     "mapping .rodata section from {:#x} to {:#x}",
//     //     (RODATA_START()),
//     //     (RODATA_END())
//     // );

//     memory_set.push(
//         MapArea::new(
//             (RODATA_START()).into(),
//             (RODATA_END()).into(),
//             MapType::Identical,
//             MapPermission::R,
//             super::frame_alloc::alloc_fn,
//         ),
//         None,
//     );

//     // println!("mapping .data section");
//     memory_set.push(
//         MapArea::new(
//             (DATA_START()).into(),
//             (DATA_END()).into(),
//             MapType::Identical,
//             MapPermission::R | MapPermission::W,
//             super::frame_alloc::alloc_fn,
//         ),
//         None,
//     );

//     // println!("mapping .bss section");
//     memory_set.push(
//         MapArea::new(
//             // (sbss_with_stack as usize).into(),
//             (BSS_START()).into(),
//             (BSS_END()).into(),
//             MapType::Identical,
//             MapPermission::R | MapPermission::W,
//             super::frame_alloc::alloc_fn,
//         ),
//         None,
//     );

//     // map stack
//     // println!("mapping stack");
//     memory_set.push(
//         MapArea::new(
//             (STACK_END()).into(),
//             (STACK_START()).into(),
//             MapType::Identical,
//             MapPermission::R | MapPermission::W,
//             super::frame_alloc::alloc_fn,
//         ),
//         None,
//     );

//     // println!("mapping physical memory");
//     memory_set.push(
//         MapArea::new(
//             (STACK_START()).into(),
//             (MEMORY_END()).into(),
//             MapType::Identical,
//             MapPermission::R | MapPermission::W,
//             super::frame_alloc::alloc_fn,
//         ),
//         None,
//     );

//     // println!("mapping memory-mapped registers");
//     for (start, end) in MMIO_DEVICES {
//         memory_set.push(
//             MapArea::new(
//                 start.into(),
//                 (start + end).into(),
//                 MapType::Identical,
//                 MapPermission::R | MapPermission::W,
//                 super::frame_alloc::alloc_fn,
//             ),
//             None,
//         );
//     }

//     // println!("mapping device tree");
//     let device_tree = DEVICE_TREE.get().expect("device tree not initialized");
//     memory_set.push(
//         MapArea::new(
//             device_tree.start().into(),
//             device_tree.end().into(),
//             MapType::Identical,
//             MapPermission::R | MapPermission::W,
//             super::frame_alloc::alloc_fn,
//         ),
//         None,
//     );

//     memory_set
// }

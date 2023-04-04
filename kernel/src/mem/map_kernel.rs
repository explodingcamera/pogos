use riscv_mem::mem::{MapArea, MapPermission, MapType, MemorySet};

use crate::{
    println,
    symbols::{
        BSS_END, BSS_START, DATA_END, DATA_START, MEMORY_END, RODATA_END, RODATA_START, STACK_END,
        STACK_START, TEXT_END, TEXT_START,
    },
};

pub fn init_kernel_memory_map() {
    let kernel_map = new_kernel();
    kernel_map.activate();
    println!("kernel memory map activated");
}

fn new_kernel() -> MemorySet {
    let mut memory_set = MemorySet::new_bare();

    println!("mapping .text section");
    memory_set.push(
        MapArea::new(
            (TEXT_START()).into(),
            (TEXT_END() - 1).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::X,
        ),
        None,
    );
    println!(
        "mapping .rodata section from {:#x} to {:#x}",
        (RODATA_START()),
        (RODATA_END())
    );

    memory_set.push(
        MapArea::new(
            (RODATA_START()).into(),
            (RODATA_END()).into(),
            MapType::Identical,
            MapPermission::R,
        ),
        None,
    );

    println!("mapping .data section");
    memory_set.push(
        MapArea::new(
            (DATA_START()).into(),
            (DATA_END()).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ),
        None,
    );

    println!("mapping .bss section");
    memory_set.push(
        MapArea::new(
            // (sbss_with_stack as usize).into(),
            (BSS_START()).into(),
            (BSS_END()).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ),
        None,
    );

    // map stack
    println!("mapping stack");
    memory_set.push(
        MapArea::new(
            (STACK_END()).into(),
            (STACK_START()).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ),
        None,
    );

    println!("mapping physical memory");
    memory_set.push(
        MapArea::new(
            (STACK_START()).into(),
            (MEMORY_END()).into(),
            MapType::Identical,
            MapPermission::R | MapPermission::W,
        ),
        None,
    );

    // //println!("mapping memory-mapped registers");
    // for pair in MMIO {
    //     memory_set.push(
    //         MapArea::new(
    //             (*pair).0.into(),
    //             ((*pair).0 + (*pair).1).into(),
    //             MapType::Identical,
    //             MapPermission::R | MapPermission::W,
    //         ),
    //         None,
    //     );
    // }
    memory_set
}

use crate::println;

extern "C" {
    pub static _stext: u8;
    pub static _etext: u8;

    pub static _srodata: u8;
    pub static _erodata: u8;

    // Kernel data: 0x80210f70 - 0x802111af
    pub static _sdata: u8;
    pub static _edata: u8;

    pub static _sbss: u8;
    pub static _ebss: u8;

    // pub static _sheap: u8;
    // pub static _eheap: u8;
    // pub static _heap_size: u8;

    pub static _sstack: u8;
    pub static _estack: u8;

    pub static _memory_start: u8;
    pub static _memory_end: u8;

    pub static _kernel_end: u8;
}

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;

pub fn MEMORY_START() -> usize {
    unsafe { &_memory_start as *const _ as usize }
}

pub fn MEMORY_END() -> usize {
    unsafe { &_memory_end as *const _ as usize }
}

pub fn TEXT_START() -> usize {
    unsafe { &_stext as *const _ as usize }
}

pub fn TEXT_END() -> usize {
    unsafe { &_etext as *const _ as usize }
}

pub fn RODATA_START() -> usize {
    unsafe { &_srodata as *const _ as usize }
}

pub fn RODATA_END() -> usize {
    unsafe { &_erodata as *const _ as usize }
}

pub fn DATA_START() -> usize {
    unsafe { &_sdata as *const _ as usize }
}

pub fn DATA_END() -> usize {
    unsafe { &_edata as *const _ as usize }
}

pub fn BSS_START() -> usize {
    unsafe { &_sbss as *const _ as usize }
}

pub fn BSS_END() -> usize {
    unsafe { &_ebss as *const _ as usize }
}

// pub fn HEAP_START() -> usize {
//     unsafe { &_sheap as *const _ as usize }
// }

// pub fn HEAP_END() -> usize {
//     unsafe { &_eheap as *const _ as usize }
// }

// pub fn HEAP_SIZE() -> usize {
//     unsafe { &_heap_size as *const _ as usize }
// }

pub fn STACK_START() -> usize {
    unsafe { &_sstack as *const _ as usize }
}

pub fn STACK_END() -> usize {
    unsafe { &_estack as *const _ as usize }
}

pub fn debug() {
    println!("Kernel text: {:#x} - {:#x}", TEXT_START(), TEXT_END() - 1);
    println!(
        "Kernel rodata: {:#x} - {:#x}",
        RODATA_START(),
        RODATA_END() - 1
    );
    println!("Kernel data: {:#x} - {:#x}", DATA_START(), DATA_END() - 1);
    println!("Kernel bss: {:#x} - {:#x}", BSS_START(), BSS_END() - 1);
    // println!(
    //     "Kernel heap: {:#x}  - {:#x} (size: {:#x})",
    //     HEAP_START(),
    //     HEAP_END() - 1,
    //     HEAP_SIZE()
    // );
    println!(
        "Kernel stack: {:#x} - {:#x}",
        STACK_END(),
        STACK_START() - 1
    );
}

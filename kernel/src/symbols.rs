use crate::println;

extern "C" {
    static _stext: u8;
    static _etext: u8;

    static _srodata: u8;
    static _erodata: u8;

    // Kernel data: 0x80210f70 - 0x802111af
    static _sdata: u8;
    static _edata: u8;

    static _sbss: u8;
    static _ebss: u8;

    static _sheap: u8;
    static _heap_size: u8;

    static _sstack: u8;
    static _estack: u8;

}

pub fn TEXT_START() -> usize {
    unsafe { &_stext as *const _ as usize }
}

pub fn TEXT_END() -> usize {
    unsafe { &_etext as *const _ as usize }
}

pub fn BSS_START() -> usize {
    unsafe { &_sbss as *const _ as usize }
}

pub fn BSS_END() -> usize {
    unsafe { &_ebss as *const _ as usize }
}

pub fn HEAP_START() -> usize {
    unsafe { &_sheap as *const _ as usize }
}

pub fn HEAP_SIZE() -> usize {
    unsafe { &_heap_size as *const _ as usize }
}

pub fn STACK_START() -> usize {
    unsafe { &_sstack as *const _ as usize }
}

pub fn STACK_END() -> usize {
    unsafe { &_estack as *const _ as usize }
}

pub fn DATA_START() -> usize {
    unsafe { &_sdata as *const _ as usize }
}

pub fn DATA_END() -> usize {
    unsafe { &_edata as *const _ as usize }
}

pub fn RODATA_START() -> usize {
    unsafe { &_srodata as *const _ as usize }
}

pub fn RODATA_END() -> usize {
    unsafe { &_erodata as *const _ as usize }
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
    println!(
        "Kernel heap: {:#x} - {:#x}",
        HEAP_START(),
        HEAP_START() + HEAP_SIZE() - 1
    );
    println!(
        "Kernel stack: {:#x} - {:#x}",
        STACK_END(),
        STACK_START() - 1
    );
}

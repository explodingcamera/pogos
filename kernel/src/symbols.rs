extern "C" {
    pub static _stext: u8;
    pub static _etext: u8;

    pub static _srodata: u8;
    pub static _erodata: u8;

    pub static _sdata: u8;
    pub static _edata: u8;

    pub static _sbss: u8;
    pub static _ebss: u8;

    pub static _sstack: u8;
    pub static _estack: u8;

    pub static _memory_start: u8;
    pub static _memory_end: u8;

    pub static _end: u8;
}

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

pub fn STACK_START() -> usize {
    unsafe { &_sstack as *const _ as usize }
}

pub fn STACK_END() -> usize {
    unsafe { &_estack as *const _ as usize }
}

pub fn KERNEL_END() -> usize {
    STACK_START()
}

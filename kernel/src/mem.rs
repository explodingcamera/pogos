use crate::println;

pub mod frame_alloc;
pub mod heap_alloc;
pub mod map_kernel;

pub unsafe fn init_mmu() {
    let r = riscv::register::satp::read();
    // println!("satp: {:?}", r);

    // unsafe {
    //     riscv::register::satp::set(satp::Mode::Sv39, 0, root_ppn);
    //     riscv::asm::sfence_vma(0, 0);
    // }
}

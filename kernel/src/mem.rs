use crate::println;

pub unsafe fn init_mmu() {
    let r = riscv::register::satp::read();
    println!("satp: {:?}", r);

    // unsafe {
    //     riscv::register::satp::set(satp::Mode::Sv39, 0, root_ppn);
    //     riscv::asm::sfence_vma(0, 0);
    // }
}
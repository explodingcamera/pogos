use core::arch::asm;

use riscv::asm;

use crate::{
    println,
    util::{shutdown, sstc},
};

#[export_name = "DefaultHandler"]
fn default_handler() {
    println!("Default handler called\n");
}

// #[export_name = "DefaultInterruptHandler"]
// fn default_interrupt_handler() {
//     println!("DefaultInterruptHandler called\n");
// }

#[export_name = "SupervisorTimer"]
fn supervisor_timer_handler() {
    println!("SupervisorTimer called");
    // clear the timer interrupt
    sstc::write(usize::MAX);
}

#[export_name = "ExceptionHandler"]
fn custom_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
    println!("Exception handler called");
    println!("Trap frame: {:?}", trap_frame);

    let cause = riscv::register::scause::read();
    panic!("Exception cause: {:?}", cause.cause());
}

#[export_name = "UserTimer"]
fn custom_user_timer_handler(trap_frame: &riscv_rt::TrapFrame) {
    println!("User timer handler called");
    println!("Trap frame: {:?}", trap_frame);
}

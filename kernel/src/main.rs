#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(async_fn_in_trait)]
#![feature(lazy_cell)]
#![allow(unused)]

extern crate alloc;
extern crate riscv_rt;

mod panic_handler;
mod trap;

mod dtb;
mod io;
mod ksched;
mod mem;
mod symbols;
mod tasks;
mod timer;
mod util;

use alloc::{boxed::Box, string::String, vec, vec::Vec};
use riscv_rt::{entry, pre_init};
use tasks::Fuse;

// this is called by each hart before the kernel is initialized
#[pre_init]
unsafe fn pre_init() {
    // enable supervisor interrupt
    riscv::register::sstatus::set_sie();
    // enable supervisor timer interrupt
    riscv::register::sie::set_stimer();
}

// the entry point of the kernel, this is only called by hart 0.
// the other harts are busy looping
#[entry]
fn main(a0: usize, a1: usize, a2: usize) -> ! {
    let hart_id = a0;

    println!("== starting pogos kernel on hart {} ==", hart_id);
    println!();

    // initialize the device tree
    dtb::init(a1);

    // Setup everything required for the kernel to run
    unsafe {
        // initialize the kernel heap allocator, alloc is now available
        mem::heap_alloc::init_kernel_heap();
        println!(">>> kernel heap initialized");

        // initialize the frame allocator
        mem::frame_alloc::init_frame_allocator();
        println!(">>> frame allocator initialized");

        // initialize the kernel memory map
        mem::map_kernel::init_kernel_memory_map();
        println!(">>> kernel memory map initialized");
    }

    println!("kernel initialized, starting kschedule\n");

    // ksched::KernelScheduler::new()
    //     .with_task(ksched::Task::new(tasks::shell_task(), 0))
    //     .block_on_run();

    // util::shutdown()

    start_shell()
}

pub const ENTER: u8 = 13;
pub const BACKSPACE: u8 = 127;

pub fn start_shell() -> ! {
    print!("> ");

    let mut command = String::new(); // finnaly, we can use heap allocated strings!

    loop {
        match sbi::legacy::console_getchar() {
            Some(ENTER) => {
                println!();
                process_command(&command);
                command.clear();
                print!("> ");
            }
            Some(c) => {
                if c >= 32 && c <= 126 {
                    command.push(c as char);
                    print!("{}", c as char);
                }
            }
            _ => {}
        }
    }
}

fn process_command(command: &str) {
    match command {
        "help" | "?" | "h" => {
            println!("available commands:");
            println!("  help      print this help message  (alias: h, ?)");
            println!("  shutdown  shutdown the machine     (alias: sd, exit)");
        }
        "shutdown" | "sd" | "exit" => util::shutdown(),
        "" => {}
        _ => {
            println!("unknown command: {command}");
        }
    };
}

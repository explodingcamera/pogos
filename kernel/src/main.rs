#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(lazy_cell)]
#![allow(unused)]

extern crate alloc;
extern crate riscv_rt;

// enable our custom panic handler
mod panic_handler;

// handle interrupts and exceptions
mod trap;

mod io;
mod mem;
mod symbols;
mod util;

use alloc::string::String;
use riscv_rt::entry;

#[entry]
fn main(hart_id: usize) -> ! {
    if hart_id != 0 {
        panic!("hart {} is not the boot hart, stopping", hart_id);
    }

    println!();
    println!("== starting kernel on hart {} ==", hart_id);
    println!();

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

        // todo: initialize mmu here
        mem::init_mmu();
        println!(">>> mmu enabled");
    }

    println!("kernel initialized, starting kernel shell\n");
    println!("type 'help' for a list of available commands");

    // prompt
    print!("> ");

    let mut command = String::new();
    loop {
        match sbi::legacy::console_getchar() {
            Some(12) => {
                // clear screen
                print!("\x1b[2J\x1b[1;1H");
                print!("> ");
            }
            Some(13) => {
                println!();
                process_command(&command);
                command.clear();
                print!("> ");
            }
            Some(127) => {
                if command.len() > 0 {
                    command.pop();
                    print!("{}", 127 as char)
                }
            }
            Some(c) => {
                if c >= 32 && c <= 126 {
                    command.push(c as char);
                    print!("{}", c as char);
                } else {
                    print!("\nchar code: {}", c);
                }
            }
            None => {}
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

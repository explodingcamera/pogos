use core::arch::asm;

use alloc::{string::String, vec::Vec};
use riscv::asm;

use crate::{ksched, print, println, util};

pub async fn console_task() -> ksched::TaskResult {
    console().await;
    ksched::TaskResult::Exit
}

async fn console() {
    println!("type 'help' for a list of available commands");

    // prompt
    print!("> ");

    let mut history = Vec::new();
    let mut command = String::new();
    let mut cursor = 0;

    loop {
        match util::get_char().await {
            // ctrl-l
            12 => {
                // clear screen
                print!("\x1b[2J\x1b[1;1H");
                print!("> ");
                print!("{}", command);
                cursor = command.len();
            }
            // enter
            13 => {
                println!();
                process_command(&command);
                history.push(command.clone());
                command.clear();
                cursor = 0;
                print!("> ");
            }
            // backspace
            127 => {
                if cursor > 0 {
                    command.remove(cursor - 1);
                    cursor -= 1;
                    print!("\x08"); // Move cursor left
                    print!("{}", &command[cursor..]); // Print the remaining text
                    print!(" "); // Clear last character
                    print!("\x1b[{}D", command.len() - cursor + 1); // Move cursor to the correct position
                }
            }
            // ctrl-c
            3 => {
                process_command("exit");
            }
            27 => {
                handle_escape(&mut command, &mut cursor).await;
            }
            c => {
                if c >= 32 && c <= 126 {
                    command.insert(cursor, c as char);
                    cursor += 1;
                    // print!("\x1b[K"); // Clear line from cursor position to the end
                    if cursor < command.len() {
                        print!("{}", &command[cursor - 1..]); // Print the remaining text
                        print!("\x1b[{}D", command.len() - cursor); // Move cursor to the correct position
                    } else {
                        print!("{}", c as char);
                    }
                }
            }
        }
    }
}

fn process_command(command: &str) {
    match command {
        "help" | "?" | "h" => {
            println!("available commands:");
            println!("  help        print this help message  (alias: h, ?)");
            println!("  shutdown    shutdown the machine     (alias: sd, exit)");
            println!("  time        print current time       (alias: t)");
            println!("  devicetree  print the device tree    (alias: dt)");
            println!("  panic       panic the kernel         (alias: p)");
            println!("  exception   trigger an exception");
            println!("  interrupt   cause a timer interrupt");
        }
        "time" | "t" => match crate::timer::get_time() {
            Ok(time) => println!("{}ms since boot", time),
            Err(e) => println!("failed to get time: {:?}", e),
        },
        "devicetree" | "dt" => crate::dtb::print_dtb(),
        "shutdown" | "sd" | "exit" => util::shutdown(),
        "panic" | "p" => panic!("panic requested by user"),
        "exception" | "e" => unsafe {
            asm!("ebreak");
        },
        "interrupt" | "i" => crate::timer::set_interrupt(0).unwrap(),
        "" => {}
        _ => {
            println!("unknown command: {command}");
        }
    };
}

async fn handle_escape(command: &mut String, cursor: &mut usize) {
    match util::get_char().await {
        91 => {
            // CSI
            match util::get_char().await {
                65 => {
                    // up
                    // print!("\x1b[1A");
                }
                66 => {
                    // down
                    // print!("\x1b[1B");
                }
                67 => {
                    if cursor < &mut command.len() {
                        // right
                        print!("\x1b[1C");
                        *cursor += 1;
                    }
                }
                68 => {
                    if cursor > &mut 0 {
                        // left
                        print!("\x1b[1D");
                        *cursor -= 1;
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

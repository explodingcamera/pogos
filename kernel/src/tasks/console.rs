use core::arch::asm;

use alloc::{string::String, vec::Vec};
use riscv::asm;

use crate::{ksched, print, println, util};

pub async fn console_task() -> ksched::TaskResult {
    console().await;
    ksched::TaskResult::Exit
}

const CTRL_L: u8 = 12;
const ENTER: u8 = 13;
const BACKSPACE: u8 = 127;
const CTRL_C: u8 = 3;
const ESCAPE: u8 = 27;

fn print_prompt() {
    print!("> ");
}

fn clear_screen() {
    print!("\x1b[2J\x1b[1;1H");
}

async fn console() {
    println!("type 'help' for a list of available commands");
    print_prompt();

    let mut history = Vec::new();
    let mut command = String::new();
    let mut cursor = 0;

    loop {
        match util::get_char().await {
            CTRL_L => {
                // clear screen
                clear_screen();
                print_prompt();
                print!("{command}");
                cursor = command.len();
            }
            ENTER => {
                println!();
                process_command(&command);
                history.push(command.clone());
                command.clear();
                cursor = 0;
                print_prompt();
            }
            BACKSPACE => {
                if cursor > 0 {
                    command.remove(cursor - 1);
                    cursor -= 1;
                    print!("\x08"); // Move cursor left
                    print!("{}", &command[cursor..]); // Print the remaining text
                    print!(" "); // Clear last character
                    print!("\x1b[{}D", command.len() - cursor + 1); // Move cursor to the correct position
                }
            }
            CTRL_C => {
                process_command("exit");
            }
            ESCAPE => {
                handle_escape(&mut command, &mut cursor).await;
            }
            c if (32..=126).contains(&c) => {
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
            _ => {}
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

const CSI: u8 = 91;
const CSI_UP: u8 = 65;
const CSI_DOWN: u8 = 66;
const CSI_RIGHT: u8 = 67;
const CSI_LEFT: u8 = 68;

async fn handle_escape(command: &mut String, cursor: &mut usize) {
    if util::get_char().await != CSI {
        return;
    }

    match util::get_char().await {
        CSI_UP => {}
        CSI_DOWN => {}
        CSI_RIGHT => {
            if cursor < &mut command.len() {
                print!("\x1b[1C");
                *cursor += 1;
            }
        }
        CSI_LEFT => {
            if *cursor > 0 {
                print!("\x1b[1D");
                *cursor -= 1;
            }
        }
        _ => {}
    }
}

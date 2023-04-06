use alloc::string::String;

use crate::{ksched, print, println, util};

pub async fn console_task() -> ksched::TaskResult {
    console().await;
    ksched::TaskResult::Exit
}

async fn console() {
    println!("type 'help' for a list of available commands");

    // prompt
    print!("> ");

    let mut command = String::new();
    loop {
        match util::get_char().await {
            12 => {
                // clear screen
                print!("\x1b[2J\x1b[1;1H");
                print!("> ");
            }
            13 => {
                println!();
                process_command(&command);
                command.clear();
                print!("> ");
            }
            127 => {
                if command.len() > 0 {
                    command.pop();
                    print!("{}", 127 as char)
                }
            }
            3 => {
                process_command("exit");
            }
            c => {
                if c >= 32 && c <= 126 {
                    command.push(c as char);
                    print!("{}", c as char);
                } else {
                    // println!("\nchar code: {}", c);
                }
            }
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

#![no_std]
extern crate alloc;

use core::future::Future;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

mod constants;
pub mod writer;
use constants::*;
use writer::*;

#[derive(Clone, Copy)]
pub struct ShellCommand<'a, F>
where
    F: Future<Output = u8> + 'static,
{
    pub help: &'a str,
    pub func: fn(&[&str], &mut Shell<'a, F>) -> Result<(), &'a str>,
    pub aliases: &'a [&'a str],
}

pub struct Shell<'a, F>
where
    F: Future<Output = u8> + 'static,
{
    pub history: Vec<String>,
    pub commands: BTreeMap<&'a str, ShellCommand<'a, F>>,
    pub command: String,
    pub cursor: usize,
    out: Writer,
    read: fn() -> F,
}

impl<'a, F> Shell<'a, F>
where
    F: Future<Output = u8> + 'static,
{
    pub fn new(write: fn(&str), read: fn() -> F) -> Self
    where
        F: Future<Output = u8> + 'static,
    {
        Self {
            history: Vec::new(),
            commands: BTreeMap::new(),
            command: String::new(),
            cursor: 0,
            out: Writer::new(write),
            read,
        }
    }

    pub fn with_commands(mut self, mut commands: BTreeMap<&'a str, ShellCommand<'a, F>>) -> Self {
        self.commands.append(&mut commands);
        self
    }

    pub async fn run(&mut self) {
        self.print_prompt();

        loop {
            match (self.read)().await {
                CTRL_C => self.process_command("exit".to_string()),
                ESCAPE => self.handle_escape().await,
                CTRL_L => {
                    self.clear_screen();
                    self.print_prompt();
                    print!(self.out, "{}", self.command);
                    self.cursor = self.command.len();
                }
                ENTER => {
                    println!(self.out, "");
                    self.process_command(self.command.clone());
                    self.history.push(self.command.clone());
                    self.command.clear();
                    self.cursor = 0;
                    self.print_prompt();
                }
                BACKSPACE => {
                    if self.cursor > 0 {
                        self.command.remove(self.cursor - 1);
                        self.cursor -= 1;
                        print!(self.out, "\x08"); // Move cursor left
                        print!(self.out, "{}", &self.command[self.cursor..]); // Print the remaining text
                        print!(self.out, " "); // Clear last character
                        print!(self.out, "\x1b[{}D", self.command.len() - self.cursor + 1);
                        // Move cursor to the correct position
                    }
                }
                c if (32..=126).contains(&c) => {
                    self.command.insert(self.cursor, c as char);
                    self.cursor += 1;
                    // print!("\x1b[K"); // Clear line from cursor position to the end
                    if self.cursor < self.command.len() {
                        // Print the remaining text
                        print!(self.out, "{}", &self.command[self.cursor - 1..]);
                        // Move cursor to the correct position
                        print!(self.out, "\x1b[{}D", self.command.len() - self.cursor);
                    } else {
                        print!(self.out, "{}", c as char);
                    }
                }
                _ => {}
            }
        }
    }

    async fn handle_escape(&mut self) {
        if (self.read)().await != CSI {
            return;
        }

        match (self.read)().await {
            CSI_UP => {}
            CSI_DOWN => {}
            CSI_RIGHT => {
                if self.cursor < self.command.len() {
                    print!(self.out, "\x1b[1C");
                    self.cursor += 1;
                }
            }
            CSI_LEFT => {
                if self.cursor > 0 {
                    print!(self.out, "\x1b[1D");
                    self.cursor -= 1;
                }
            }
            _ => {}
        }
    }

    fn process_command(&mut self, command: String) {
        let mut args = command.split_whitespace();
        let command = args.next().unwrap_or("");
        let args = args.collect::<Vec<_>>();

        for (name, shell_command) in &self.commands {
            if shell_command.aliases.contains(&command) || name == &command {
                return (shell_command.func)(&args, self).unwrap_or_else(|err| {
                    println!(self.out, "{}: {}", command, err);
                });
            }
        }

        if command.is_empty() {
            return;
        }

        println!(self.out, "{}: command not found", command);
    }

    pub fn print_help_screen(&mut self) {
        println!(self.out, "available commands:");
        for (name, command) in &self.commands {
            print!(self.out, "  {:<12}{:<25}", name, command.help);
            if !command.aliases.is_empty() {
                print!(self.out, "    aliases: {}", command.aliases.join(", "));
            }
            println!(self.out, "");
        }
    }

    pub fn print_prompt(&mut self) {
        print!(self.out, "> ");
    }

    pub fn clear_screen(&mut self) {
        print!(self.out, "\x1b[2J\x1b[1;1H");
    }
}

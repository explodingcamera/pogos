use core::{arch::asm, cell::OnceCell};

use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};
use riscv::asm;
use simple_shell::{Shell, ShellCommand};

use crate::{
    ksched, print, println,
    util::{self, print, Result},
};

pub async fn shell_task() -> ksched::TaskResult {
    let mut shell = create_shell();
    shell.run().await;
    ksched::TaskResult::Exit
}

pub fn create_shell<'a>() -> Shell<'a, impl core::future::Future<Output = u8>> {
    let mut shell = Shell::new(print, util::get_char);
    let mut commands = BTreeMap::new();

    commands.insert(
        "help",
        ShellCommand {
            help: "print this help message",
            func: |_, shell| Ok(shell.print_help_screen()),
            aliases: &["?", "h"],
        },
    );

    commands.insert(
        "shutdown",
        ShellCommand {
            help: "shutdown the system",
            func: |_, _| {
                println!("\nshutting down the system");
                util::shutdown();
            },
            aliases: &["sd", "exit", "quit"],
        },
    );

    commands.insert(
        "reboot",
        ShellCommand {
            help: "reboot the system",
            func: |_, _| {
                println!("\nrebooting the system");
                util::reboot();
            },
            aliases: &[],
        },
    );

    commands.insert(
        "clear",
        ShellCommand {
            help: "clear the screen",
            func: |_, shell| Ok(shell.clear_screen()),
            aliases: &["cls"],
        },
    );

    commands.insert(
        "echo",
        ShellCommand {
            help: "print the given arguments",
            func: |args, _| {
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
                Ok(())
            },
            aliases: &[],
        },
    );

    commands.insert(
        "panic",
        ShellCommand {
            help: "panic the kernel",
            func: |_, _| panic!("panic command invoked"),
            aliases: &[],
        },
    );

    commands.insert(
        "exception",
        ShellCommand {
            help: "trigger an exception",
            func: |_, _| Ok(unsafe { asm!("ebreak") }),
            aliases: &[],
        },
    );

    commands.insert(
        "interrupt",
        ShellCommand {
            help: "cause a timer interrupt",
            func: |_, _| crate::timer::set_interrupt(0),
            aliases: &[],
        },
    );

    commands.insert(
        "devicetree",
        ShellCommand {
            help: "print the device tree",
            func: |_, _| Ok(crate::dtb::print_dtb()),
            aliases: &["dt"],
        },
    );

    shell.with_commands(commands)
}

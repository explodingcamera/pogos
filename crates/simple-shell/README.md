# `simple-shell`

A simple shell for `no_std` Rust.

## Usage

```rust
fn run_shell() -> ! {
  let (print, read) = (|s: &str| print!(s), || None);
  let mut shell = Shell::new(print, read);

  commands.insert(
      "help",
      ShellCommand {
          help: "print this help message",
          func: |_, shell| {
              shell.print_help_screen();
              Ok(())
          },
          aliases: &["?", "h"],
      },
  );

  // Also supports async
  // shell.run_async().await;

  shell.run()
}
```

    .section .text.entry
    .globl _start
_start:
    # set stack pointer to the top of the stack
    la sp, boot_stack_top
    # call the start function in the kernel (defined in ./main.rs)
    call __main

    # set the memory for the stack
    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
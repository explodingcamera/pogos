    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top # set stack pointer to the top of the stack
    call start # call the start function in the kernel (defined in ./main.rs)

    .section .bss.stack # define the memory for the stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
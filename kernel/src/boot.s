// from osdev barebones

// Multiboot constants
.set ALIGN, 1<<0
.set MEMINFO, 1<<1
.set FLAGS, ALIGN | MEMINFO
.set MAGIC, 0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

// Multiboot header
.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

// Allocate 16kb stack with 16-byte alignment (for System V ABI compat). 
.section .bss
.align 16
stack_bottom:
    .skip 16384 // 16kb stack
stack_top:

// Entry point to the kernel
.section .text
.global _start
.type _start, @function
_start:
    /* 
    Now in 32-bit protected mode. Interrupts and paging are disabled by the
    bootloader by default. 
    */

    // setup the stack
    mov $stack_top, %esp

    // initialise processor state
    // e.g. set up GDT, paging, etc

    // enter the kernel
    call kernel_main

    // put the computer into a loop if kernel exits
    cli
1:
    hlt
    jmp 1b

// set size of _start symbol to current location - starting location
.size _start, . - _start

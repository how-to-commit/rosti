#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod libc;
mod vga_text_mode;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn kernel_main() -> ! {
    vga_text_mode::init_writer();
    println!("Hello world!");
    println!("123");
    loop {}
}

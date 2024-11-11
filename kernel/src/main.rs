#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn kernel_main() -> ! {
    let test: &[u8] = b"Hello world!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in test.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xF;
        }
    }
    loop {}
}

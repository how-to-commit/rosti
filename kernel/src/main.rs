#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

mod libc;
mod multiboot;
mod vga_text_mode;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(magic: u32, info: *const multiboot::BootInfo) -> ! {
    vga_text_mode::init_writer();

    println!("magic number: {:#x}", magic);

    unsafe {
        let entry = (*info).get_mmap_entries();
        println!("mmap length: {}", {entry.length});
        println!("mmap address: {}", {entry.base_addr});
    }

    loop {}
}

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

extern crate alloc;
use alloc::vec::Vec;

mod allocator;
mod libc;
mod multiboot;
mod vga_text_mode;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[global_allocator]
static ALLOC: allocator::Locked<allocator::BumpAlloc> =
    allocator::Locked::new(allocator::BumpAlloc::new());

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main(magic: u32, info: *const multiboot::BootInfo) -> ! {
    vga_text_mode::init_writer();
    if magic != 0x2badb002 {
        panic!("Not booted from multiboot")
    }
    (*info).print_mmap_entries();
    ALLOC.lock().init(info);

    // test
    let mut v: Vec<usize> = Vec::new();
    for i in 0..100 {
        v.push(i);
        println!("write {} to: {:p}", &v[i], &v[i]);
    }

    loop {}
}

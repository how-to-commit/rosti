#![warn(clippy::all)]
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::arch::global_asm;
use core::panic::PanicInfo;

extern crate alloc;
use alloc::vec::Vec;

mod allocator;
mod gdt;
mod interrupt;
mod io;
mod multiboot;
mod utils;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[global_allocator]
static ALLOC: utils::mutex::SpinMutex<allocator::BumpAlloc> =
    utils::mutex::SpinMutex::new(allocator::BumpAlloc::new());

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_main(magic: u32, info: *const multiboot::BootInfo) -> ! {
    io::vga::init_writer();
    if magic != 0x2badb002 {
        panic!("Not booted from multiboot")
    }

    unsafe {
        (*info).print_mmap_entries();
        ALLOC.lock().init(info);
    }

    let mut port_manager = io::ports::PortAllocator::new();

    gdt::init_gdt();
    interrupt::init_idt(&mut port_manager);

    // test alloc
    let mut v: Vec<usize> = Vec::new();
    for i in 0..100 {
        v.push(i);
        println!("write {} to: {:p}", &v[i], &v[i]);
    }
    for i in 0..100 {
        println!("readback {}", &v[i]);
    }

    // test
    // unsafe {
    //     core::arch::asm!("int 13");
    // }

    #[allow(clippy::empty_loop)]
    loop {}
}

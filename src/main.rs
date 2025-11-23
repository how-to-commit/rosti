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

static PORT_MANAGER: utils::mutex::SpinMutex<io::ports::PortAllocator> =
    utils::mutex::SpinMutex::new(io::ports::PortAllocator::new());

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
    println!("hi");

    gdt::init_gdt();
    interrupt::init_idt(&mut PORT_MANAGER.lock());

    // test
    // unsafe {
    //     core::arch::asm!("int 13");
    // }

    #[allow(clippy::empty_loop)]
    loop {}
}

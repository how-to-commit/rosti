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

    // stub: enable ps2 & interrupts
    let pm = &mut PORT_MANAGER.lock();
    let mut ps2_status = pm.allocate(0x64).expect("ps2 status");
    let mut ps2_data = pm.allocate(0x60).expect("ps2 data");
    ps2_status.outb(0xAE);
    ps2_status.outb(0x20);
    let s = ps2_data.inb() | 0x01;
    ps2_status.outb(0x60);
    ps2_data.outb(s);

    println!("ps2 init enable");

    #[allow(clippy::empty_loop)]
    loop {}
}

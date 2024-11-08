#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod framebuffer;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let framebuf_info = boot_info.framebuffer.as_mut().unwrap().info();
    let framebuf_raw = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let mut framewriter = framebuffer::FrameBufferWriter::new(framebuf_raw, framebuf_info);

    for x in 10..50 {
        for y in 10..50 {
            framewriter.draw_pixel(x, y, 68, 75, 110);
        }
    }
    // stop
    loop {}
}

bootloader_api::entry_point!(kernel_main);

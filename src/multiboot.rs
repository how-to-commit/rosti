// See: <https://www.gnu.org/software/grub/manual/multiboot/multiboot.html>
//
// Struct members are named the same as in the multiboot spec, howeer those
// names that are reserved in Rust are postfixed with an underscore.
// Structs look contrived but they are directly from the multiboot spec.

#![allow(dead_code)]

use crate::println;

#[repr(C, packed)]
pub struct BootInfo {
    flags: u32,

    mem_lower: u32,
    mem_upper: u32,

    boot_device: u32,

    cmdline: u32,

    mods_count: u32,
    mods_addr: u32,

    syms: [u32; 4],

    mmap_length: u32,
    mmap_addr: u32,

    drives_length: u32,
    drives_addr: u32,

    config_table: u32,

    boot_loader_name: u32,

    apm_table: u32,

    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u32,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,

    framebuffer_addr: u16,
    framebuffer_pitch: u64,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u32,
    framebuffer_type: u8,
    color_info: [u8; 6],
}

#[repr(C, packed)]
pub struct MmapEntry {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32, // name conflicts with rust keyword
}

impl BootInfo {
    pub unsafe fn get_mmap_entries(&self) -> &[MmapEntry] {
        unsafe {
            core::slice::from_raw_parts(
                self.mmap_addr as *const MmapEntry,
                self.mmap_length as usize / core::mem::size_of::<MmapEntry>(),
            )
        }
    }

    #[allow(clippy::cast_precision_loss)]
    pub unsafe fn print_mmap_entries(&self) {
        println!("----- multiboot mmap -----");
        let num_entries = self.mmap_length as usize / core::mem::size_of::<MmapEntry>();
        println!("num entries: {}", num_entries);

        let mut total_sz: u64 = 0;
        let mmap_entries;
        unsafe {
            mmap_entries = self.get_mmap_entries();
        }
        for entry in mmap_entries {
            println!(
                "size: {}, len: {}K, addr: {:#04x}, typ: {}",
                { entry.size },
                { entry.length } as f64 / 1024.0,
                { entry.base_addr },
                { entry.type_ }
            );
            if entry.type_ == 1 {
                total_sz += entry.length;
            }
        }
        println!("total size: {}K", total_sz as f64 / 1024.0);
    }
}

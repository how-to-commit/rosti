#![allow(dead_code)]
use core::arch::asm;

#[repr(C, packed)]
struct GdtTable {
    limit: u16,
    base: u32,
}

impl GdtTable {
    unsafe fn read_from_sgdt() -> Self {
        let mut ret = core::mem::MaybeUninit::uninit();
        asm!(r#"sgdt ({})"#, in (reg) ret.as_mut_ptr(), options(att_syntax, nostack, preserves_flags));
        ret.assume_init()
    }
}

#[repr(u8)]
enum AccessByte {
    Accessed = 1,
    ReadWrite = 2,
    Direction = 4,
    Executable = 8,
    NotSystemDescriptor = 16,
    Present = 128,

    Privilege0 = 0,
    Privilege1 = 32,
    Privilege2 = 64,
    Privilege3 = 96,
}

struct GdtSegment {
    access_byte: u8,
    flag: u8, // higher nibble is used
    base: u32,
    limit: u32,
}

impl GdtSegment {
    fn new(base: u32, limit: u32) -> Self {
        Self {
            access_byte: 0,
            flag: 0,
            base,
            limit,
        }
    }

    /// modified from: osdev wiki
    /// https://wiki.osdev.org/GDT_Tutorial
    fn as_u64(self) -> u64 {
        let mut desc = (self.limit & 0x000f_0000) as u64;
        desc |= (self.flag as u64) << 16 & 0x00f0_0000;
        desc |= (self.access_byte as u64) << 8 & 0x0000_ff00;
        desc |= (self.base as u64) >> 16 & 0x0000_00ff;
        desc |= self.base as u64 & 0xff00_0000;

        desc = desc << 32;
        desc |= (self.base as u64) << 16;
        desc |= (self.limit & 0x0000_ffff) as u64;

        desc
    }

    fn flag_as_u16(self) -> u16 {
        (self.flag << 8) as u16 | self.access_byte as u16
    }

    fn with_access_byte(mut self, item: AccessByte) -> Self {
        self.access_byte ^= item as u8;
        self
    }

    fn set_privilege(mut self, privilege_level: u8) -> Result<Self, &'static str> {
        if privilege_level > 3 {
            return Err("invalid privilege_level");
        }

        self.access_byte &= !0b0110_0000; // zero privilege level
        self.access_byte |= privilege_level << 5;
        Ok(self)
    }

    fn set_page_granularity(mut self, enable: bool) -> Self {
        const MASK: u8 = 0b1000_0000;
        if enable {
            self.flag |= MASK;
        } else {
            self.flag &= !MASK;
        }

        self
    }

    fn set_32bit_segment_size(mut self, enable: bool) -> Self {
        const MASK: u8 = 0b0100_0000;
        if enable {
            self.flag |= MASK;
        } else {
            self.flag &= !MASK;
        }

        self
    }

    fn set_long_mode(mut self, enable: bool) -> Result<Self, &'static str> {
        const MASK: u8 = 0b0010_0000;
        if enable {
            if self.flag & 0b0100 > 0 {
                return Err("cannot set long mode with 32 bit segment");
            }

            // from osdev wiki:
            // "For any other type of segment (other code types or any data segment), it should be clear (0)."
            // What does "any other type of segment" mean? There are other code types??
            if self.access_byte & (AccessByte::Executable as u8) == 0 {
                return Err("cannot set long mode on non-executable segment");
            }

            self.flag |= MASK;
        } else {
            self.flag &= !MASK;
        }

        Ok(self)
    }
}

fn create_gdt_entries() -> [u64; 3] {
    let code = GdtSegment::new(0, 0xffff_ffff)
        .with_access_byte(AccessByte::Present)
        .with_access_byte(AccessByte::NotSystemDescriptor)
        .with_access_byte(AccessByte::Executable)
        .with_access_byte(AccessByte::Accessed)
        .set_32bit_segment_size(true)
        .set_page_granularity(true)
        .as_u64();

    let data = GdtSegment::new(0, 0xffff_ffff)
        .with_access_byte(AccessByte::Present)
        .with_access_byte(AccessByte::NotSystemDescriptor)
        .with_access_byte(AccessByte::ReadWrite)
        .with_access_byte(AccessByte::Accessed)
        .set_32bit_segment_size(true)
        .set_page_granularity(true)
        .as_u64();

    let blank = GdtSegment::new(0, 0).as_u64();

    [blank, code, data]
}

fn init_gdt() {
    let entries = create_gdt_entries().to_vec().leak();
    let ptr = entries.as_ptr();

    // TODO: init the table, write the address to gdtr
    unimplemented!()
}

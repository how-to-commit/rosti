#[repr(u8)]
enum AccessByteItems {
    Accessed = 1,
    ReadWrite = 2,
    Direction = 4,
    Executable = 8,
    DescriptorType = 16,
    Present = 128,

    Privilege0 = 0,
    Privilege1 = 32,
    Privilege2 = 64,
    Privilege3 = 96,
}

struct Flag {
    access_byte: u8,
    flag: u8,
}

impl Flag {
    fn generate_repr(self) -> u16 {
        (self.flag << 8) as u16 | self.access_byte as u16
    }

    fn with_access_byte(mut self, item: AccessByteItems) -> Self {
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
            if self.access_byte & (AccessByteItems::Executable as u8) == 0 {
                return Err("cannot set long mode on non-executable segment");
            }

            self.flag |= MASK;
        } else {
            self.flag &= !MASK;
        }

        Ok(self)
    }
}
/// modified from: osdev wiki
/// https://wiki.osdev.org/GDT_Tutorial
fn create_gdt_descriptor(base: u32, limit: u32, flag: u16) -> u64 {
    let mut desc = (limit & 0x000f_0000) as u64;
    desc |= (flag << 8) as u64 & 0x00f0_ff00;
    desc |= (base >> 16) as u64 & 0x0000_00ff;
    desc |= base as u64 & 0xff00_0000;

    desc = desc << 32;
    desc |= (base << 16) as u64;
    desc |= (limit & 0x0000_ffff) as u64;

    desc
}

use core::arch::asm;

/// This struct HAS to be 'static OR guaranteed to outlive ANY Port instance it creates.
pub struct PortAllocator {
    used: [bool; 0x10000],
}

impl PortAllocator {
    pub const fn new() -> PortAllocator {
        PortAllocator {
            used: [false; 0x10000],
        }
    }

    pub fn allocate(&mut self, port_id: u16) -> Option<Port> {
        if !self.used[port_id as usize] {
            return Some(Port {
                addr: port_id,
                parent_allocator: self as *const _ as *mut _,
            });
        }

        None
    }

    pub fn release(&mut self, port_id: u16) {
        self.used[port_id as usize] = false;
    }
}

pub struct Port {
    addr: u16,
    parent_allocator: *mut PortAllocator,
}

impl Port {
    pub fn inb(&mut self) -> u8 {
        let mut ret;
        unsafe {
            asm!("in %dx, %al", in("dx") self.addr, out("al") ret);
        }
        ret
    }

    pub fn outb(&mut self, b: u8) {
        unsafe {
            asm!("out %al, %dx", in("dx") self.addr, in("al") b);
        }
    }
}

impl Drop for Port {
    fn drop(&mut self) {
        unsafe { (*self.parent_allocator).release(self.addr) }
    }
}

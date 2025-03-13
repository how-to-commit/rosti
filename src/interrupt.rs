use crate::isr;
use alloc::vec::Vec;
use core::arch::asm;

type InterruptServiceRoutine = extern "x86-interrupt" fn();

struct Entry(u64);

impl Entry {
    fn new(isr: InterruptServiceRoutine, code_segment: u16, attributes: u8) -> Self {
        todo!()
    }
}

struct InterruptTable {
    inner: Vec<Entry>,
}

impl InterruptTable {
    fn new() -> Self {
        Self {
            inner: Vec::with_capacity(256),
        }
    }

    fn set_interrupt(&mut self, entry_id: usize, isr: InterruptServiceRoutine) {
        // TODO: figure out what the "attributes" segment of the descriptor means
        let entry = Entry::new(isr, 0x08, 0);
        self.inner[entry_id] = entry;
    }

    fn load(self) {
        let table = self.inner.leak();

        let idtr = Idtr {
            limit: core::mem::size_of_val(table) as u16,
            base: table.as_ptr() as u32,
        };

        unsafe {
            asm!(r#"lidt ({idtr})"#, idtr = in (reg) &idtr, options(att_syntax));
        }
    }
}

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u32,
}

pub fn init_idt() {
    let mut new_idt = InterruptTable::new();
    new_idt.set_interrupt(11, isr::general_fault);
    new_idt.load();
    todo!();
}

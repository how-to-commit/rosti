use crate::println;
use crate::utils::bits::CanManipulateBits;

use alloc::vec::Vec;
use core::arch::asm;

const IDT_TABLE_SIZE: usize = 256;

type InterruptServiceRoutine = extern "x86-interrupt" fn();

#[allow(dead_code)]
#[repr(u8)]
enum GateType {
    Task = 0b0101,
    Interrupt16Bit = 0b0110,
    Trap16Bit = 0b0111,
    Interrupt32Bit = 0b1110,
    Trap32Bit = 0b1111,
}

struct Entry(u64);

impl Entry {
    fn new_invalid() -> Self {
        Entry(0)
    }

    // create a new initialised interrupt table entry:
    // dpl refers to the cpu priv level which is able to trigger this
    fn new(
        isr: InterruptServiceRoutine,
        segment_selector: u16,
        gate_type: GateType,
        dpl: u8,
    ) -> Self {
        // write the isr address
        let isr_offset = isr as u32;
        Entry(
            0u64.set_bits(0, 16, u64::from(isr_offset))
                .set_bits(48, 16, u64::from(isr_offset >> 16))
                .set_bits(16, 16, u64::from(segment_selector))
                .set_bits(40, 4, u64::from(gate_type as u8))
                .set_bits(45, 2, u64::from(dpl))
                .set_one_bit(47, true), // enable bit
        )
    }
}

struct InterruptTable {
    inner: Vec<Entry>,
}

impl InterruptTable {
    fn new() -> Self {
        let mut v = Vec::with_capacity(IDT_TABLE_SIZE);
        for _ in 0..IDT_TABLE_SIZE {
            v.push(Entry::new_invalid());
        }
        Self { inner: v }
    }

    fn set_interrupt(&mut self, entry_id: usize, isr: InterruptServiceRoutine) {
        // TODO: figure out what the "attributes" segment of the descriptor means
        let entry = Entry::new(isr, 0x08, GateType::Interrupt32Bit, 0);
        self.inner[entry_id] = entry;
    }

    fn load(self) {
        let table = self.inner.leak();

        let idtr = Idtr {
            limit: core::mem::size_of_val(table) as u16,
            base: table.as_ptr() as u32,
        };

        unsafe {
            asm!(r#"
                lidt ({idtr})
                sti
            "#, idtr = in (reg) &idtr, options(att_syntax));
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
    new_idt.set_interrupt(13, isr_general_fault);
    new_idt.load();
    todo!();
}

extern "x86-interrupt" fn isr_general_fault() {
    println!("fault test");
}

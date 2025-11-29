use crate::io::ports::PortAllocator;
use crate::println;
use crate::utils::bits::CanManipulateBits;

use core::arch::asm;

const IDT_TABLE_SIZE: usize = 256;
type InterruptServiceRoutine = extern "x86-interrupt" fn();

static mut INTERRUPT_TABLE: InterruptTable = InterruptTable::new();

#[allow(dead_code)]
#[repr(u8)]
enum GateType {
    Task = 0b0101,
    Interrupt16Bit = 0b0110,
    Trap16Bit = 0b0111,
    Interrupt32Bit = 0b1110,
    Trap32Bit = 0b1111,
}

#[derive(Clone, Copy)]
struct Entry(u64);

impl Entry {
    const fn new_invalid() -> Self {
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
    inner: [Entry; IDT_TABLE_SIZE],
}

impl InterruptTable {
    const fn new() -> Self {
        Self {
            inner: [Entry::new_invalid(); IDT_TABLE_SIZE],
        }
    }

    fn set_interrupt(&mut self, entry_id: usize, isr: InterruptServiceRoutine) {
        let entry = Entry::new(isr, 0x08, GateType::Interrupt32Bit, 0);
        self.inner[entry_id] = entry;
    }

    fn load(&self) {
        let idtr = Idtr {
            limit: (IDT_TABLE_SIZE * 8 - 1) as u16,
            base: self.inner.as_ptr() as u32,
        };

        unsafe {
            asm!(r#"
                lidt ({idtr})
                sti
            "#, idtr = in (reg) &idtr, options(att_syntax));
        }
    }
}

unsafe impl Sync for InterruptTable {}

#[derive(Debug)]
#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u32,
}

pub fn init_idt(palloc: &mut PortAllocator) {
    println!("old idtr: {:?}", get_idtr());

    // mask PIC
    let mut pic_master = palloc.allocate(0x21).expect("Master PIC port not free");
    let mut pic_slave = palloc.allocate(0xA0).expect("Slave PIC port not free");
    pic_master.outb(0xff);
    pic_slave.outb(0xff);

    // load IDT
    unsafe {
        let t = (&raw mut INTERRUPT_TABLE)
            .as_mut()
            .expect("interrupt table is free");

        t.set_interrupt(13, isr_general_fault);
        t.load();
    }

    println!("new idtr: {:?}", get_idtr());
}

fn get_idtr() -> Idtr {
    let mut r = core::mem::MaybeUninit::uninit();
    unsafe {
        asm!("sidt ({})", in (reg) r.as_mut_ptr(), options(att_syntax, nostack, preserves_flags));
        r.assume_init()
    }
}

extern "x86-interrupt" fn isr_general_fault() {
    println!("fault test");
}

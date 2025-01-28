use crate::{multiboot::BootInfo, println, KERNEL_END, KERNEL_START};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use sink::mutex;

/// Align a given address upward to the `align` boundary
///
/// `align` must be a power of 2.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub struct Locked<T> {
    inner: mutex::SpinMutex<T>,
}

impl<T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Locked {
            inner: mutex::SpinMutex::new(inner),
        }
    }

    pub fn lock(&self) -> mutex::SpinMutexGuard<T> {
        self.inner.lock()
    }
}

pub struct BumpAlloc {
    start: usize,
    end: usize,
    next: usize,
    allocs: usize,
}

impl BumpAlloc {
    pub const fn new() -> Self {
        BumpAlloc {
            start: 0,
            end: 0,
            next: 0,
            allocs: 0,
        }
    }

    pub unsafe fn init(&mut self, info: *const BootInfo) {
        let block = (*info)
            .get_mmap_entries()
            .iter()
            .find(|b| b.type_ == 1 && b.base_addr != 0x0)
            .expect("Usable memory required");

        let kstart = &KERNEL_START as *const u32;
        let kend = &KERNEL_END as *const u32;

        let mut end = block.base_addr + block.length;
        let mut start = block.base_addr;

        if block.base_addr == 0 {
            start = kend as u64 + 4;
            end = start + block.length;
        } else {
            if end < kstart as u64 {
                end = kstart as u64 - 4;
            }

            if start < kend as u64 {
                start = kend as u64 + 4;
            }

            if end < start {
                panic!("Bad! No memory other than kernel memory.")
            }
        }

        println!(
            "bumpalloc start segment: {:#4x}, end segment: {:#4x}",
            start, end
        );

        self.start = start as usize;
        self.end = end as usize;
        self.next = start as usize;
        self.allocs = 0;

        println!("a: {:p}, {}", &self, core::mem::size_of_val(self));
        println!("a: {:p}", &self.next);
        println!("a: {:p}", &self.start);
        println!("a: {:p}", &self.end);
        println!("a: {:p}", &self.allocs);
    }
}

unsafe impl GlobalAlloc for Locked<BumpAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = alloc_start + layout.size();

        if alloc_end > bump.end {
            println!(
                "ran out of memory! required {}, end {:#4x}, next {:#4x}",
                layout.size(),
                bump.end,
                bump.next
            );
            return null_mut();
        }

        bump.next = alloc_end;
        bump.allocs += 1;

        println!("alloc: {:#4x}, next: {:#4x}", alloc_start, bump.next);
        return alloc_start as *mut u8;
    }

    /// deallocs the whole thing
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocs -= 1;
        if bump.allocs <= 0 {
            bump.next = bump.start;
            bump.allocs = 0;
        }
    }
}

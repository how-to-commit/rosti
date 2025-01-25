use crate::multiboot::BootInfo;
use crate::println;
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
            .find(|b| b.base_addr == 0)
            .expect("Free RAM required");

        self.start = (block.base_addr + 16386) as usize;
        self.end = (block.base_addr + 16386 + block.length) as usize;
        self.next = (block.base_addr + 16386) as usize;
        self.allocs = 0;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = alloc_start + layout.size();

        if alloc_end > bump.end {
            println!("null here, required {}, had {}", alloc_end, bump.end);
            return null_mut();
        }

        bump.next = alloc_end;
        bump.allocs += 1;

        println!("alloc: {}", alloc_start);
        return alloc_start as *mut u8;
    }

    /// deallocs the whole thing
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocs -= 1;
        if bump.allocs == 0 {
            bump.next = bump.start;
        }
    }
}

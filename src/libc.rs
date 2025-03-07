#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(ptr: *mut u8, value: i32, len: usize) -> *mut u8 {
    for i in 0..len {
        unsafe {
            *ptr.add(i) = value as u8;
        }
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *mut u8, len: usize) -> *mut u8 {
    for i in 0..len {
        unsafe {
            *dst.add(i) = *src.add(i);
        }
    }
    dst
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(lhs: *const u8, rhs: *const u8, count: usize) -> i32 {
    for i in 0..count {
        unsafe {
            let a = *lhs.add(i) as u8;
            let b = *rhs.add(i) as u8;

            if a != b {
                return (a - b) as i32;
            }
        }
    }
    0
}

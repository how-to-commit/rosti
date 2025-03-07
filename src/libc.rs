#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, value: u8, len: usize) -> *mut u8 {
    for i in 0..len {
        *ptr.add(i) = value;
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *mut u8, len: usize) -> *mut u8 {
    for i in 0..len {
        *dst.add(i) = *src.add(i);
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(lhs: *const u8, rhs: *const u8, count: usize) -> i32 {
    if lhs == rhs {
        return 0;
    }

    for i in 0..count {
        let cmp = lhs.cmp(&rhs) as i32;
        if cmp != 0 {
            return cmp;
        }

        let _ = lhs.add(i);
        let _ = rhs.add(i);
    }

    0
}

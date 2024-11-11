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

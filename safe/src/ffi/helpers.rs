use core::ffi::{c_char, c_int, c_void};
use core::panic::AssertUnwindSafe;
use core::ptr;
use std::panic::catch_unwind;
use std::process;
use std::slice;

pub fn abort_on_panic<T>(f: impl FnOnce() -> T) -> T {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(_) => process::abort(),
    }
}

pub unsafe fn set_errno(value: c_int) {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        *libc::__errno_location() = value;
    }

    #[cfg(target_os = "macos")]
    {
        *libc::__error() = value;
    }
}

pub unsafe fn opt_slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        slice::from_raw_parts(ptr, len)
    }
}

pub unsafe fn opt_slice_mut<'a, T>(ptr: *mut T, len: usize) -> &'a mut [T] {
    if len == 0 {
        &mut []
    } else {
        slice::from_raw_parts_mut(ptr, len)
    }
}

pub unsafe fn copy_allow_overlap(dst: *mut u8, src: *const u8, len: usize) {
    if len != 0 {
        ptr::copy(src, dst, len);
    }
}

pub unsafe fn write_opt<T>(ptr: *mut T, value: T) {
    if !ptr.is_null() {
        ptr.write(value);
    }
}

pub fn checked_mul(count: usize, size: usize) -> Option<usize> {
    count.checked_mul(size)
}

pub fn static_cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr().cast()
}

pub unsafe fn cast_handler(ptr: *mut c_void) -> Option<unsafe extern "C" fn()> {
    if ptr.is_null() {
        None
    } else {
        Some(core::mem::transmute::<*mut c_void, unsafe extern "C" fn()>(
            ptr,
        ))
    }
}

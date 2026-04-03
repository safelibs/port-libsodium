use core::ffi::{c_char, c_int, c_void};
use core::panic::AssertUnwindSafe;
use core::ptr;
use std::panic::catch_unwind;
use std::process;
use std::slice;

/// Ensures Rust panics abort rather than unwind across the C ABI boundary.
pub fn abort_on_panic<T>(f: impl FnOnce() -> T) -> T {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(_) => process::abort(),
    }
}

/// SAFETY: the caller must be running on a supported libc platform and may only
/// touch the current thread's errno slot.
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

/// SAFETY: `ptr` must be null or valid for `len` elements for the lifetime of
/// the returned slice.
pub unsafe fn opt_slice<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        slice::from_raw_parts(ptr, len)
    }
}

/// SAFETY: `ptr` must be null or valid for `len` elements and uniquely borrowed
/// for the lifetime of the returned slice.
pub unsafe fn opt_slice_mut<'a, T>(ptr: *mut T, len: usize) -> &'a mut [T] {
    if len == 0 {
        &mut []
    } else {
        slice::from_raw_parts_mut(ptr, len)
    }
}

/// SAFETY: `ptr` must be valid for `N` bytes for the duration of the copy.
pub unsafe fn read_array<const N: usize>(ptr: *const u8) -> [u8; N] {
    let mut out = [0u8; N];
    out.copy_from_slice(opt_slice(ptr, N));
    out
}

/// SAFETY: `ptr` must be valid for `N` bytes and writable for the duration of the copy.
pub unsafe fn write_array<const N: usize>(ptr: *mut u8, bytes: &[u8; N]) {
    opt_slice_mut(ptr, N).copy_from_slice(bytes);
}

/// SAFETY: `src` and `dst` must be valid for `len` bytes. Overlap is allowed.
pub unsafe fn copy_allow_overlap(dst: *mut u8, src: *const u8, len: usize) {
    if len != 0 {
        ptr::copy(src, dst, len);
    }
}

/// SAFETY: `ptr` must be null or valid to receive a single initialized `T`.
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

/// SAFETY: `ptr` must be null or originate from `sodium_set_misuse_handler`.
pub unsafe fn cast_handler(ptr: *mut c_void) -> Option<unsafe extern "C" fn()> {
    if ptr.is_null() {
        None
    } else {
        Some(core::mem::transmute::<*mut c_void, unsafe extern "C" fn()>(
            ptr,
        ))
    }
}

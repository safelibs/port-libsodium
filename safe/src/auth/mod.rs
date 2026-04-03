pub mod hmacsha256;
pub mod hmacsha512;
pub mod hmacsha512256;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_auth(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_auth(out, in_, inlen, k) })
}

#[no_mangle]
pub extern "C" fn crypto_auth_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_auth_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_auth_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_auth_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_auth_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_auth_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_auth_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_auth_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_auth_verify(
    h: *const ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_auth_verify(h, in_, inlen, k) })
}

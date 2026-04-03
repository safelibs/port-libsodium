pub mod sha256;
pub mod sha512;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_hash(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash(out, in_, inlen) })
}

#[no_mangle]
pub extern "C" fn crypto_hash_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_hash_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_primitive() })
}

use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_hash_sha256(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha256(out, in_, inlen) })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha256_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha256_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha256_final(
    state: *mut crypto_hash_sha256_state,
    out: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha256_final(state, out) })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha256_init(
    state: *mut crypto_hash_sha256_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha256_init(state) })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha256_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha256_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha256_update(
    state: *mut crypto_hash_sha256_state,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_hash_sha256_update(state, in_, inlen)
    })
}

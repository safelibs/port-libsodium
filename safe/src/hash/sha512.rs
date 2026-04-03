use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_hash_sha512(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha512(out, in_, inlen) })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha512_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha512_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha512_final(
    state: *mut crypto_hash_sha512_state,
    out: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha512_final(state, out) })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha512_init(
    state: *mut crypto_hash_sha512_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha512_init(state) })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha512_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_hash_sha512_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_hash_sha512_update(
    state: *mut crypto_hash_sha512_state,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_hash_sha512_update(state, in_, inlen)
    })
}

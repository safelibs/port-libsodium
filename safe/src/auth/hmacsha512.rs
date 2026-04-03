use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_auth_hmacsha512(out, in_, inlen, k) })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_auth_hmacsha512_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_final(
    state: *mut crypto_auth_hmacsha512_state,
    out: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_auth_hmacsha512_final(state, out) })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_init(
    state: *mut crypto_auth_hmacsha512_state,
    key: *const ::std::os::raw::c_uchar,
    keylen: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_auth_hmacsha512_init(state, key, keylen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_auth_hmacsha512_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_auth_hmacsha512_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_auth_hmacsha512_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_update(
    state: *mut crypto_auth_hmacsha512_state,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_auth_hmacsha512_update(state, in_, inlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_auth_hmacsha512_verify(
    h: *const ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_auth_hmacsha512_verify(h, in_, inlen, k)
    })
}

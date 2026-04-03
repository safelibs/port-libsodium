pub mod poly1305;
use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_onetimeauth(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_onetimeauth(out, in_, inlen, k) })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_onetimeauth_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_final(
    state: *mut crypto_onetimeauth_state,
    out: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_onetimeauth_final(state, out) })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_init(
    state: *mut crypto_onetimeauth_state,
    key: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_onetimeauth_init(state, key) })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_onetimeauth_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_onetimeauth_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_onetimeauth_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_onetimeauth_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_update(
    state: *mut crypto_onetimeauth_state,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_onetimeauth_update(state, in_, inlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_verify(
    h: *const ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_onetimeauth_verify(h, in_, inlen, k)
    })
}

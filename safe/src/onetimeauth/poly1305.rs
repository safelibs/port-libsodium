use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_onetimeauth_poly1305(out, in_, inlen, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_onetimeauth_poly1305_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_final(
    state: *mut crypto_onetimeauth_poly1305_state,
    out: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_onetimeauth_poly1305_final(state, out)
    })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_init(
    state: *mut crypto_onetimeauth_poly1305_state,
    key: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_onetimeauth_poly1305_init(state, key)
    })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_onetimeauth_poly1305_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_onetimeauth_poly1305_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_onetimeauth_poly1305_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_update(
    state: *mut crypto_onetimeauth_poly1305_state,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_onetimeauth_poly1305_update(state, in_, inlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_onetimeauth_poly1305_verify(
    h: *const ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_onetimeauth_poly1305_verify(h, in_, inlen, k)
    })
}

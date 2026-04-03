use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_generichash(
    out: *mut ::std::os::raw::c_uchar,
    outlen: usize,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    key: *const ::std::os::raw::c_uchar,
    keylen: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash(out, outlen, in_, inlen, key, keylen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b(
    out: *mut ::std::os::raw::c_uchar,
    outlen: usize,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    key: *const ::std::os::raw::c_uchar,
    keylen: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_blake2b(out, outlen, in_, inlen, key, keylen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_bytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_bytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_bytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_bytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_final(
    state: *mut crypto_generichash_blake2b_state,
    out: *mut ::std::os::raw::c_uchar,
    outlen: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_blake2b_final(state, out, outlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_init(
    state: *mut crypto_generichash_blake2b_state,
    key: *const ::std::os::raw::c_uchar,
    keylen: usize,
    outlen: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_blake2b_init(state, key, keylen, outlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_init_salt_personal(
    state: *mut crypto_generichash_blake2b_state,
    key: *const ::std::os::raw::c_uchar,
    keylen: usize,
    outlen: usize,
    salt: *const ::std::os::raw::c_uchar,
    personal: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_blake2b_init_salt_personal(
            state, key, keylen, outlen, salt, personal,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_keybytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_keybytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_keybytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_keybytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_blake2b_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_personalbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_personalbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_salt_personal(
    out: *mut ::std::os::raw::c_uchar,
    outlen: usize,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    key: *const ::std::os::raw::c_uchar,
    keylen: usize,
    salt: *const ::std::os::raw::c_uchar,
    personal: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_blake2b_salt_personal(
            out, outlen, in_, inlen, key, keylen, salt, personal,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_saltbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_saltbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_blake2b_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_blake2b_update(
    state: *mut crypto_generichash_blake2b_state,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_blake2b_update(state, in_, inlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_bytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_bytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_bytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_bytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_final(
    state: *mut crypto_generichash_state,
    out: *mut ::std::os::raw::c_uchar,
    outlen: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_final(state, out, outlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_init(
    state: *mut crypto_generichash_state,
    key: *const ::std::os::raw::c_uchar,
    keylen: usize,
    outlen: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_init(state, key, keylen, outlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_keybytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_keybytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_keybytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_keybytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_generichash_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_generichash_update(
    state: *mut crypto_generichash_state,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_generichash_update(state, in_, inlen)
    })
}

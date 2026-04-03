use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_shorthash(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash(out, in_, inlen, k) })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_shorthash_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_siphash24(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_shorthash_siphash24(out, in_, inlen, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_siphash24_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash_siphash24_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_siphash24_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash_siphash24_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_siphashx24(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    inlen: ::std::os::raw::c_ulonglong,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_shorthash_siphashx24(out, in_, inlen, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_siphashx24_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash_siphashx24_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_shorthash_siphashx24_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_shorthash_siphashx24_keybytes() })
}

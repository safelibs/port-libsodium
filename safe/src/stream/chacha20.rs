use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20(
    c: *mut ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_chacha20(c, clen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_ietf(
    c: *mut ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_chacha20_ietf(c, clen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_ietf_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_chacha20_ietf_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_ietf_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_chacha20_ietf_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_ietf_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_chacha20_ietf_messagebytes_max()
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_ietf_noncebytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_chacha20_ietf_noncebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_ietf_xor(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_chacha20_ietf_xor(c, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_ietf_xor_ic(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    ic: u32,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_chacha20_ietf_xor_ic(c, m, mlen, n, ic, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_chacha20_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_chacha20_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_chacha20_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_noncebytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_chacha20_noncebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_xor(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_chacha20_xor(c, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_chacha20_xor_ic(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    ic: u64,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_chacha20_xor_ic(c, m, mlen, n, ic, k)
    })
}

use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_secretbox(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox(c, m, mlen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_boxzerobytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_boxzerobytes() })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_detached(
    c: *mut ::std::os::raw::c_uchar,
    mac: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_detached(c, mac, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_easy(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_easy(c, m, mlen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_macbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_macbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_noncebytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_noncebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_open(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_open(m, c, clen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_open_detached(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    mac: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_open_detached(m, c, mac, clen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_open_easy(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_open_easy(m, c, clen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305(c, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_boxzerobytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_boxzerobytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_keybytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_keybytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_macbytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_macbytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_messagebytes_max()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_noncebytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_noncebytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_open(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_open(m, c, clen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xsalsa20poly1305_zerobytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xsalsa20poly1305_zerobytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_zerobytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_secretbox_zerobytes() })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_detached(
    c: *mut ::std::os::raw::c_uchar,
    mac: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xchacha20poly1305_detached(c, mac, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_easy(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xchacha20poly1305_easy(c, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_keybytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xchacha20poly1305_keybytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_macbytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xchacha20poly1305_macbytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xchacha20poly1305_messagebytes_max()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_noncebytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xchacha20poly1305_noncebytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_open_detached(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    mac: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_secretbox_xchacha20poly1305_open_detached(m, c, mac, clen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretbox_xchacha20poly1305_open_easy(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretbox_xchacha20poly1305_open_easy(m, c, clen, n, k)
    })
}

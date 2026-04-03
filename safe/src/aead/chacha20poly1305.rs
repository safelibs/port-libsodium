use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_abytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_chacha20poly1305_abytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_decrypt(
    m: *mut ::std::os::raw::c_uchar,
    mlen_p: *mut ::std::os::raw::c_ulonglong,
    nsec: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_aead_chacha20poly1305_decrypt(m, mlen_p, nsec, c, clen, ad, adlen, npub, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_decrypt_detached(
    m: *mut ::std::os::raw::c_uchar,
    nsec: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    mac: *const ::std::os::raw::c_uchar,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_decrypt_detached(
            m, nsec, c, clen, mac, ad, adlen, npub, k,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_encrypt(
    c: *mut ::std::os::raw::c_uchar,
    clen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    nsec: *const ::std::os::raw::c_uchar,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_aead_chacha20poly1305_encrypt(c, clen_p, m, mlen, ad, adlen, nsec, npub, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_encrypt_detached(
    c: *mut ::std::os::raw::c_uchar,
    mac: *mut ::std::os::raw::c_uchar,
    maclen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    nsec: *const ::std::os::raw::c_uchar,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_encrypt_detached(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, k,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_abytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_chacha20poly1305_ietf_abytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_decrypt(
    m: *mut ::std::os::raw::c_uchar,
    mlen_p: *mut ::std::os::raw::c_ulonglong,
    nsec: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_aead_chacha20poly1305_ietf_decrypt(m, mlen_p, nsec, c, clen, ad, adlen, npub, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_decrypt_detached(
    m: *mut ::std::os::raw::c_uchar,
    nsec: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    mac: *const ::std::os::raw::c_uchar,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_ietf_decrypt_detached(
            m, nsec, c, clen, mac, ad, adlen, npub, k,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_encrypt(
    c: *mut ::std::os::raw::c_uchar,
    clen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    nsec: *const ::std::os::raw::c_uchar,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_aead_chacha20poly1305_ietf_encrypt(c, clen_p, m, mlen, ad, adlen, nsec, npub, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_encrypt_detached(
    c: *mut ::std::os::raw::c_uchar,
    mac: *mut ::std::os::raw::c_uchar,
    maclen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    nsec: *const ::std::os::raw::c_uchar,
    npub: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_ietf_encrypt_detached(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, k,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_keybytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_ietf_keybytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_ietf_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_ietf_messagebytes_max()
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_npubbytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_ietf_npubbytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_ietf_nsecbytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_ietf_nsecbytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_chacha20poly1305_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_chacha20poly1305_messagebytes_max()
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_npubbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_chacha20poly1305_npubbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_chacha20poly1305_nsecbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_chacha20poly1305_nsecbytes() })
}

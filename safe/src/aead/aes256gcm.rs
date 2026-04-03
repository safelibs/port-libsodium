use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_abytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_abytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_beforenm(
    ctx_: *mut crypto_aead_aes256gcm_state,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_beforenm(ctx_, k) })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_decrypt(
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
            .crypto_aead_aes256gcm_decrypt(m, mlen_p, nsec, c, clen, ad, adlen, npub, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_decrypt_afternm(
    m: *mut ::std::os::raw::c_uchar,
    mlen_p: *mut ::std::os::raw::c_ulonglong,
    nsec: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    npub: *const ::std::os::raw::c_uchar,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_aead_aes256gcm_decrypt_afternm(m, mlen_p, nsec, c, clen, ad, adlen, npub, ctx_)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_decrypt_detached(
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
        crate::upstream::load()
            .crypto_aead_aes256gcm_decrypt_detached(m, nsec, c, clen, mac, ad, adlen, npub, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_decrypt_detached_afternm(
    m: *mut ::std::os::raw::c_uchar,
    nsec: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    mac: *const ::std::os::raw::c_uchar,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    npub: *const ::std::os::raw::c_uchar,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_aes256gcm_decrypt_detached_afternm(
            m, nsec, c, clen, mac, ad, adlen, npub, ctx_,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_encrypt(
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
            .crypto_aead_aes256gcm_encrypt(c, clen_p, m, mlen, ad, adlen, nsec, npub, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_encrypt_afternm(
    c: *mut ::std::os::raw::c_uchar,
    clen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    nsec: *const ::std::os::raw::c_uchar,
    npub: *const ::std::os::raw::c_uchar,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_aead_aes256gcm_encrypt_afternm(c, clen_p, m, mlen, ad, adlen, nsec, npub, ctx_)
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_encrypt_detached(
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
        crate::upstream::load().crypto_aead_aes256gcm_encrypt_detached(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, k,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_encrypt_detached_afternm(
    c: *mut ::std::os::raw::c_uchar,
    mac: *mut ::std::os::raw::c_uchar,
    maclen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    nsec: *const ::std::os::raw::c_uchar,
    npub: *const ::std::os::raw::c_uchar,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_aes256gcm_encrypt_detached_afternm(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, ctx_,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_is_available() -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_is_available() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_aead_aes256gcm_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_npubbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_npubbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_nsecbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_nsecbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_aead_aes256gcm_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_aead_aes256gcm_statebytes() })
}

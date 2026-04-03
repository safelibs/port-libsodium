use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_seed_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
    seed: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_seed_keypair(pk, sk, seed)
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_keypair(pk, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_beforenm(
    k: *mut ::std::os::raw::c_uchar,
    pk: *const ::std::os::raw::c_uchar,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_beforenm(k, pk, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_afternm(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_afternm(c, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_open_afternm(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_open_afternm(m, c, clen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    pk: *const ::std::os::raw::c_uchar,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305(c, m, mlen, n, pk, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_open(
    m: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    pk: *const ::std::os::raw::c_uchar,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_open(m, c, clen, n, pk, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_seedbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_seedbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_publickeybytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_publickeybytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_secretkeybytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_secretkeybytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_beforenmbytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_beforenmbytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_noncebytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_noncebytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_zerobytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_zerobytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_boxzerobytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_boxzerobytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_macbytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_macbytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_box_curve25519xsalsa20poly1305_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_box_curve25519xsalsa20poly1305_messagebytes_max()
    })
}

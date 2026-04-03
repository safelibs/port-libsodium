use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_kx_seed_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
    seed: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_kx_seed_keypair(pk, sk, seed) })
}

#[no_mangle]
pub extern "C" fn crypto_kx_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_kx_keypair(pk, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_kx_client_session_keys(
    rx: *mut ::std::os::raw::c_uchar,
    tx: *mut ::std::os::raw::c_uchar,
    client_pk: *const ::std::os::raw::c_uchar,
    client_sk: *const ::std::os::raw::c_uchar,
    server_pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_kx_client_session_keys(rx, tx, client_pk, client_sk, server_pk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_kx_server_session_keys(
    rx: *mut ::std::os::raw::c_uchar,
    tx: *mut ::std::os::raw::c_uchar,
    server_pk: *const ::std::os::raw::c_uchar,
    server_sk: *const ::std::os::raw::c_uchar,
    client_pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_kx_server_session_keys(rx, tx, server_pk, server_sk, client_pk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_kx_publickeybytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_kx_publickeybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kx_secretkeybytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_kx_secretkeybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kx_seedbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_kx_seedbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kx_sessionkeybytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_kx_sessionkeybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kx_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_kx_primitive() })
}

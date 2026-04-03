use crate::abi::types::crypto_sign_state;
use crate::ffi::helpers::abort_on_panic;

pub mod ed25519;
pub mod legacy_edwards25519sha512batch;

#[no_mangle]
pub extern "C" fn crypto_sign_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_seedbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_seedbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_publickeybytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_publickeybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_secretkeybytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_secretkeybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_seed_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
    seed: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_seed_keypair(pk, sk, seed) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_keypair(pk, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign(
    sm: *mut ::std::os::raw::c_uchar,
    smlen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign(sm, smlen_p, m, mlen, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_open(
    m: *mut ::std::os::raw::c_uchar,
    mlen_p: *mut ::std::os::raw::c_ulonglong,
    sm: *const ::std::os::raw::c_uchar,
    smlen: ::std::os::raw::c_ulonglong,
    pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_open(m, mlen_p, sm, smlen, pk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_detached(
    sig: *mut ::std::os::raw::c_uchar,
    siglen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_detached(sig, siglen_p, m, mlen, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_verify_detached(
    sig: *const ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_verify_detached(sig, m, mlen, pk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_init(
    state: *mut crypto_sign_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_init(state) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_update(
    state: *mut crypto_sign_state,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_update(state, m, mlen) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_final_create(
    state: *mut crypto_sign_state,
    sig: *mut ::std::os::raw::c_uchar,
    siglen_p: *mut ::std::os::raw::c_ulonglong,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_final_create(state, sig, siglen_p, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_final_verify(
    state: *mut crypto_sign_state,
    sig: *const ::std::os::raw::c_uchar,
    pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_final_verify(state, sig, pk) })
}

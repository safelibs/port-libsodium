use crate::abi::types::crypto_sign_ed25519ph_state;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519ph_statebytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519ph_statebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_seedbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_seedbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_publickeybytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_publickeybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_secretkeybytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_secretkeybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519(
    sm: *mut ::std::os::raw::c_uchar,
    smlen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519(sm, smlen_p, m, mlen, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_open(
    m: *mut ::std::os::raw::c_uchar,
    mlen_p: *mut ::std::os::raw::c_ulonglong,
    sm: *const ::std::os::raw::c_uchar,
    smlen: ::std::os::raw::c_ulonglong,
    pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_open(m, mlen_p, sm, smlen, pk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_detached(
    sig: *mut ::std::os::raw::c_uchar,
    siglen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_ed25519_detached(sig, siglen_p, m, mlen, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_verify_detached(
    sig: *const ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_ed25519_verify_detached(sig, m, mlen, pk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_keypair(pk, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_seed_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
    seed: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_seed_keypair(pk, sk, seed) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_pk_to_curve25519(
    curve25519_pk: *mut ::std::os::raw::c_uchar,
    ed25519_pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_ed25519_pk_to_curve25519(curve25519_pk, ed25519_pk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_sk_to_curve25519(
    curve25519_sk: *mut ::std::os::raw::c_uchar,
    ed25519_sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_ed25519_sk_to_curve25519(curve25519_sk, ed25519_sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_sk_to_seed(
    seed: *mut ::std::os::raw::c_uchar,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_sk_to_seed(seed, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519_sk_to_pk(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519_sk_to_pk(pk, sk) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519ph_init(
    state: *mut crypto_sign_ed25519ph_state,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519ph_init(state) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519ph_update(
    state: *mut crypto_sign_ed25519ph_state,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_sign_ed25519ph_update(state, m, mlen) })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519ph_final_create(
    state: *mut crypto_sign_ed25519ph_state,
    sig: *mut ::std::os::raw::c_uchar,
    siglen_p: *mut ::std::os::raw::c_ulonglong,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_ed25519ph_final_create(state, sig, siglen_p, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_sign_ed25519ph_final_verify(
    state: *mut crypto_sign_ed25519ph_state,
    sig: *const ::std::os::raw::c_uchar,
    pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_ed25519ph_final_verify(state, sig, pk)
    })
}

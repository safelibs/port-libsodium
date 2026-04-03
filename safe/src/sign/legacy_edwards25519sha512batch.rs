use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_sign_edwards25519sha512batch_keypair(
    pk: *mut ::std::os::raw::c_uchar,
    sk: *mut ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_edwards25519sha512batch_keypair(pk, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_sign_edwards25519sha512batch(
    sm: *mut ::std::os::raw::c_uchar,
    smlen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    sk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_edwards25519sha512batch(sm, smlen_p, m, mlen, sk)
    })
}

#[no_mangle]
pub extern "C" fn crypto_sign_edwards25519sha512batch_open(
    m: *mut ::std::os::raw::c_uchar,
    mlen_p: *mut ::std::os::raw::c_ulonglong,
    sm: *const ::std::os::raw::c_uchar,
    smlen: ::std::os::raw::c_ulonglong,
    pk: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_sign_edwards25519sha512batch_open(m, mlen_p, sm, smlen, pk)
    })
}

use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_scalarmult_ristretto255(
    q: *mut ::std::os::raw::c_uchar,
    n: *const ::std::os::raw::c_uchar,
    p: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_ristretto255(q, n, p) })
}

#[no_mangle]
pub extern "C" fn crypto_scalarmult_ristretto255_base(
    q: *mut ::std::os::raw::c_uchar,
    n: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_ristretto255_base(q, n) })
}

#[no_mangle]
pub extern "C" fn crypto_scalarmult_ristretto255_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_ristretto255_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_scalarmult_ristretto255_scalarbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_ristretto255_scalarbytes() })
}

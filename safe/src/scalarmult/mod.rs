use crate::ffi::helpers::abort_on_panic;

pub mod curve25519;
pub mod ed25519;
pub mod ristretto255;

#[no_mangle]
pub extern "C" fn crypto_scalarmult(
    q: *mut ::std::os::raw::c_uchar,
    n: *const ::std::os::raw::c_uchar,
    p: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult(q, n, p) })
}

#[no_mangle]
pub extern "C" fn crypto_scalarmult_base(
    q: *mut ::std::os::raw::c_uchar,
    n: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_base(q, n) })
}

#[no_mangle]
pub extern "C" fn crypto_scalarmult_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_scalarmult_scalarbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_scalarbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_scalarmult_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_scalarmult_primitive() })
}

use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_is_valid_point(
    p: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_is_valid_point(p) })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_add(
    r: *mut ::std::os::raw::c_uchar,
    p: *const ::std::os::raw::c_uchar,
    q: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_add(r, p, q) })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_sub(
    r: *mut ::std::os::raw::c_uchar,
    p: *const ::std::os::raw::c_uchar,
    q: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_sub(r, p, q) })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_from_uniform(
    p: *mut ::std::os::raw::c_uchar,
    r: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_from_uniform(p, r) })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_from_hash(
    p: *mut ::std::os::raw::c_uchar,
    h: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_from_hash(p, h) })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_random(p: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_random(p);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_random(r: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_scalar_random(r);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_invert(
    recip: *mut ::std::os::raw::c_uchar,
    s: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_scalar_invert(recip, s) })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_negate(
    neg: *mut ::std::os::raw::c_uchar,
    s: *const ::std::os::raw::c_uchar,
) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_scalar_negate(neg, s);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_complement(
    comp: *mut ::std::os::raw::c_uchar,
    s: *const ::std::os::raw::c_uchar,
) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_scalar_complement(comp, s);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_add(
    z: *mut ::std::os::raw::c_uchar,
    x: *const ::std::os::raw::c_uchar,
    y: *const ::std::os::raw::c_uchar,
) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_scalar_add(z, x, y);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_sub(
    z: *mut ::std::os::raw::c_uchar,
    x: *const ::std::os::raw::c_uchar,
    y: *const ::std::os::raw::c_uchar,
) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_scalar_sub(z, x, y);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_mul(
    z: *mut ::std::os::raw::c_uchar,
    x: *const ::std::os::raw::c_uchar,
    y: *const ::std::os::raw::c_uchar,
) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_scalar_mul(z, x, y);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalar_reduce(
    r: *mut ::std::os::raw::c_uchar,
    s: *const ::std::os::raw::c_uchar,
) {
    abort_on_panic(|| unsafe {
        crate::public_key_impl::crypto_core_ed25519_scalar_reduce(r, s);
    })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_bytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_bytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_uniformbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_uniformbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_hashbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_hashbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_scalarbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_scalarbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_ed25519_nonreducedscalarbytes() -> usize {
    abort_on_panic(|| unsafe { crate::public_key_impl::crypto_core_ed25519_nonreducedscalarbytes() })
}

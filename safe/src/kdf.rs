use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_kdf_blake2b_bytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_blake2b_bytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_blake2b_bytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_blake2b_bytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_blake2b_contextbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_blake2b_contextbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_blake2b_derive_from_key(
    subkey: *mut ::std::os::raw::c_uchar,
    subkey_len: usize,
    subkey_id: u64,
    ctx: *const ::std::os::raw::c_char,
    key: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_kdf_blake2b_derive_from_key(
            subkey, subkey_len, subkey_id, ctx, key,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_blake2b_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_blake2b_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_bytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_bytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_bytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_bytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_contextbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_contextbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_derive_from_key(
    subkey: *mut ::std::os::raw::c_uchar,
    subkey_len: usize,
    subkey_id: u64,
    ctx: *const ::std::os::raw::c_char,
    key: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_kdf_derive_from_key(subkey, subkey_len, subkey_id, ctx, key)
    })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_kdf_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_kdf_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_kdf_primitive() })
}

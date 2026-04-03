use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_core_salsa20(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa20(out, in_, k, c) })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa20_constbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa20_constbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa20_inputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa20_inputbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa20_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa20_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa20_outputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa20_outputbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa2012(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa2012(out, in_, k, c) })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa2012_constbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa2012_constbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa2012_inputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa2012_inputbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa2012_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa2012_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa2012_outputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa2012_outputbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa208(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa208(out, in_, k, c) })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa208_constbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa208_constbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa208_inputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa208_inputbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa208_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa208_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_salsa208_outputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_core_salsa208_outputbytes() })
}

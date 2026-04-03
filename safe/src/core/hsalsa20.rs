use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_core_hsalsa20(
    out: *mut ::std::os::raw::c_uchar,
    in_: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_core_hsalsa20(out, in_, k, c) })
}

#[no_mangle]
pub extern "C" fn crypto_core_hsalsa20_constbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_core_hsalsa20_constbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_hsalsa20_inputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_core_hsalsa20_inputbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_hsalsa20_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_core_hsalsa20_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_core_hsalsa20_outputbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_core_hsalsa20_outputbytes() })
}

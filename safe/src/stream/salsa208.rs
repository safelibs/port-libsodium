use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_stream_salsa208(
    c: *mut ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_salsa208(c, clen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_stream_salsa208_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_salsa208_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_salsa208_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_stream_salsa208_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_salsa208_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_salsa208_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_salsa208_noncebytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_salsa208_noncebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_salsa208_xor(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_stream_salsa208_xor(c, m, mlen, n, k)
    })
}

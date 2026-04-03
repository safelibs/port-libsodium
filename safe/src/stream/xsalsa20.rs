use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_stream_xsalsa20(
    c: *mut ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_xsalsa20(c, clen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_stream_xsalsa20_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_xsalsa20_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_xsalsa20_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_stream_xsalsa20_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_xsalsa20_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_xsalsa20_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_xsalsa20_noncebytes() -> usize {
    abort_on_panic(|| unsafe { crate::symmetric_impl::crypto_stream_xsalsa20_noncebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_xsalsa20_xor(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_stream_xsalsa20_xor(c, m, mlen, n, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_xsalsa20_xor_ic(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    ic: u64,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::symmetric_impl::crypto_stream_xsalsa20_xor_ic(c, m, mlen, n, ic, k)
    })
}

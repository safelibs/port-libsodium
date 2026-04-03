pub mod chacha20;
pub mod salsa20;
pub mod salsa2012;
pub mod salsa208;
pub mod xchacha20;
pub mod xsalsa20;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_stream(
    c: *mut ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream(c, clen, n, k) })
}

#[no_mangle]
pub extern "C" fn crypto_stream_keybytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_keybytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_stream_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_stream_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_messagebytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_noncebytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_noncebytes() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_stream_xor(
    c: *mut ::std::os::raw::c_uchar,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    n: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_stream_xor(c, m, mlen, n, k) })
}

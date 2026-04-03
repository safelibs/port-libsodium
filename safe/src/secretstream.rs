use crate::abi::types::*;
use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_abytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_abytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_headerbytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_headerbytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_init_pull(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    header: *const ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_init_pull(state, header, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_init_push(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    header: *mut ::std::os::raw::c_uchar,
    k: *const ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_init_push(state, header, k)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_keybytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_keybytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_keygen(k: *mut ::std::os::raw::c_uchar) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_keygen(k);
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_messagebytes_max() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_messagebytes_max()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_pull(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    m: *mut ::std::os::raw::c_uchar,
    mlen_p: *mut ::std::os::raw::c_ulonglong,
    tag_p: *mut ::std::os::raw::c_uchar,
    c: *const ::std::os::raw::c_uchar,
    clen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_secretstream_xchacha20poly1305_pull(state, m, mlen_p, tag_p, c, clen, ad, adlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_push(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    c: *mut ::std::os::raw::c_uchar,
    clen_p: *mut ::std::os::raw::c_ulonglong,
    m: *const ::std::os::raw::c_uchar,
    mlen: ::std::os::raw::c_ulonglong,
    ad: *const ::std::os::raw::c_uchar,
    adlen: ::std::os::raw::c_ulonglong,
    tag: ::std::os::raw::c_uchar,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_secretstream_xchacha20poly1305_push(state, c, clen_p, m, mlen, ad, adlen, tag)
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_rekey(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
) {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_rekey(state);
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_statebytes() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_statebytes()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_tag_final() -> ::std::os::raw::c_uchar {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_tag_final()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_tag_message() -> ::std::os::raw::c_uchar {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_tag_message()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_tag_push() -> ::std::os::raw::c_uchar {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_tag_push()
    })
}

#[no_mangle]
pub extern "C" fn crypto_secretstream_xchacha20poly1305_tag_rekey() -> ::std::os::raw::c_uchar {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_secretstream_xchacha20poly1305_tag_rekey()
    })
}

use crate::ffi::helpers::abort_on_panic;

#[no_mangle]
pub extern "C" fn crypto_pwhash(
    out: *mut ::std::os::raw::c_uchar,
    outlen: ::std::os::raw::c_ulonglong,
    passwd: *const ::std::os::raw::c_char,
    passwdlen: ::std::os::raw::c_ulonglong,
    salt: *const ::std::os::raw::c_uchar,
    opslimit: ::std::os::raw::c_ulonglong,
    memlimit: usize,
    alg: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash(
            out, outlen, passwd, passwdlen, salt, opslimit, memlimit, alg,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_alg_argon2i13() -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_alg_argon2i13() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_alg_argon2id13() -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_alg_argon2id13() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_alg_default() -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_alg_default() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i(
    out: *mut ::std::os::raw::c_uchar,
    outlen: ::std::os::raw::c_ulonglong,
    passwd: *const ::std::os::raw::c_char,
    passwdlen: ::std::os::raw::c_ulonglong,
    salt: *const ::std::os::raw::c_uchar,
    opslimit: ::std::os::raw::c_ulonglong,
    memlimit: usize,
    alg: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_argon2i(
            out, outlen, passwd, passwdlen, salt, opslimit, memlimit, alg,
        )
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_alg_argon2i13() -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_alg_argon2i13() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_bytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_bytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_bytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_bytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_memlimit_interactive() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_argon2i_memlimit_interactive()
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_memlimit_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_memlimit_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_memlimit_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_memlimit_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_memlimit_moderate() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_memlimit_moderate() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_memlimit_sensitive() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_memlimit_sensitive() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_opslimit_interactive() -> usize {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_argon2i_opslimit_interactive()
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_opslimit_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_opslimit_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_opslimit_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_opslimit_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_opslimit_moderate() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_opslimit_moderate() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_opslimit_sensitive() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_opslimit_sensitive() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_passwd_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_passwd_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_passwd_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_passwd_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_saltbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_saltbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_str(
    out: *mut ::std::os::raw::c_char,
    passwd: *const ::std::os::raw::c_char,
    passwdlen: ::std::os::raw::c_ulonglong,
    opslimit: ::std::os::raw::c_ulonglong,
    memlimit: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_pwhash_argon2i_str(out, passwd, passwdlen, opslimit, memlimit)
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_str_needs_rehash(
    str_: *const ::std::os::raw::c_char,
    opslimit: ::std::os::raw::c_ulonglong,
    memlimit: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_argon2i_str_needs_rehash(str_, opslimit, memlimit)
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_str_verify(
    str_: *const ::std::os::raw::c_char,
    passwd: *const ::std::os::raw::c_char,
    passwdlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_argon2i_str_verify(str_, passwd, passwdlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_strbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_strbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_argon2i_strprefix() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_argon2i_strprefix() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_bytes_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_bytes_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_bytes_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_bytes_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_memlimit_interactive() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_memlimit_interactive() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_memlimit_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_memlimit_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_memlimit_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_memlimit_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_memlimit_moderate() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_memlimit_moderate() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_memlimit_sensitive() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_memlimit_sensitive() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_opslimit_interactive() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_opslimit_interactive() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_opslimit_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_opslimit_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_opslimit_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_opslimit_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_opslimit_moderate() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_opslimit_moderate() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_opslimit_sensitive() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_opslimit_sensitive() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_passwd_max() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_passwd_max() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_passwd_min() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_passwd_min() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_primitive() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_primitive() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_saltbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_saltbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_str(
    out: *mut ::std::os::raw::c_char,
    passwd: *const ::std::os::raw::c_char,
    passwdlen: ::std::os::raw::c_ulonglong,
    opslimit: ::std::os::raw::c_ulonglong,
    memlimit: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_str(out, passwd, passwdlen, opslimit, memlimit)
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_str_alg(
    out: *mut ::std::os::raw::c_char,
    passwd: *const ::std::os::raw::c_char,
    passwdlen: ::std::os::raw::c_ulonglong,
    opslimit: ::std::os::raw::c_ulonglong,
    memlimit: usize,
    alg: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load()
            .crypto_pwhash_str_alg(out, passwd, passwdlen, opslimit, memlimit, alg)
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_str_needs_rehash(
    str_: *const ::std::os::raw::c_char,
    opslimit: ::std::os::raw::c_ulonglong,
    memlimit: usize,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_str_needs_rehash(str_, opslimit, memlimit)
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_str_verify(
    str_: *const ::std::os::raw::c_char,
    passwd: *const ::std::os::raw::c_char,
    passwdlen: ::std::os::raw::c_ulonglong,
) -> ::std::os::raw::c_int {
    abort_on_panic(|| unsafe {
        crate::upstream::load().crypto_pwhash_str_verify(str_, passwd, passwdlen)
    })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_strbytes() -> usize {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_strbytes() })
}

#[no_mangle]
pub extern "C" fn crypto_pwhash_strprefix() -> *const ::std::os::raw::c_char {
    abort_on_panic(|| unsafe { crate::upstream::load().crypto_pwhash_strprefix() })
}

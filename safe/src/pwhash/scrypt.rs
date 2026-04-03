use crate::ffi::helpers::abort_on_panic;

macro_rules! export {
    (fn $name:ident($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty;) => {
        #[no_mangle]
        pub extern "C" fn $name($($arg: $ty),*) -> $ret {
            abort_on_panic(|| unsafe { crate::symmetric_impl::$name($($arg),*) })
        }
    };
}

export! {
    fn crypto_pwhash_scryptsalsa208sha256(
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
export! { fn crypto_pwhash_scryptsalsa208sha256_bytes_max() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_bytes_min() -> usize; }
export! {
    fn crypto_pwhash_scryptsalsa208sha256_ll(
        passwd: *const ::std::os::raw::c_uchar,
        passwdlen: usize,
        salt: *const ::std::os::raw::c_uchar,
        saltlen: usize,
        n: u64,
        r: u32,
        p: u32,
        buf: *mut ::std::os::raw::c_uchar,
        buflen: usize,
    ) -> ::std::os::raw::c_int;
}
export! { fn crypto_pwhash_scryptsalsa208sha256_memlimit_interactive() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_memlimit_max() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_memlimit_min() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_memlimit_sensitive() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_opslimit_interactive() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_opslimit_max() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_opslimit_min() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_opslimit_sensitive() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_passwd_max() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_passwd_min() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_saltbytes() -> usize; }
export! {
    fn crypto_pwhash_scryptsalsa208sha256_str(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
export! {
    fn crypto_pwhash_scryptsalsa208sha256_str_needs_rehash(
        str_: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
export! {
    fn crypto_pwhash_scryptsalsa208sha256_str_verify(
        str_: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
export! { fn crypto_pwhash_scryptsalsa208sha256_strbytes() -> usize; }
export! { fn crypto_pwhash_scryptsalsa208sha256_strprefix() -> *const ::std::os::raw::c_char; }

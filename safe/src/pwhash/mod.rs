use crate::ffi::helpers::abort_on_panic;

pub mod argon2;
pub mod scrypt;

macro_rules! export {
    (fn $name:ident($($arg:ident : $ty:ty),* $(,)?) -> $ret:ty;) => {
        #[no_mangle]
        pub extern "C" fn $name($($arg: $ty),*) -> $ret {
            abort_on_panic(|| unsafe { crate::symmetric_impl::$name($($arg),*) })
        }
    };
}

export! {
    fn crypto_pwhash(
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
export! { fn crypto_pwhash_alg_argon2i13() -> ::std::os::raw::c_int; }
export! { fn crypto_pwhash_alg_argon2id13() -> ::std::os::raw::c_int; }
export! { fn crypto_pwhash_alg_default() -> ::std::os::raw::c_int; }
export! { fn crypto_pwhash_bytes_max() -> usize; }
export! { fn crypto_pwhash_bytes_min() -> usize; }
export! { fn crypto_pwhash_memlimit_interactive() -> usize; }
export! { fn crypto_pwhash_memlimit_max() -> usize; }
export! { fn crypto_pwhash_memlimit_min() -> usize; }
export! { fn crypto_pwhash_memlimit_moderate() -> usize; }
export! { fn crypto_pwhash_memlimit_sensitive() -> usize; }
export! { fn crypto_pwhash_opslimit_interactive() -> usize; }
export! { fn crypto_pwhash_opslimit_max() -> usize; }
export! { fn crypto_pwhash_opslimit_min() -> usize; }
export! { fn crypto_pwhash_opslimit_moderate() -> usize; }
export! { fn crypto_pwhash_opslimit_sensitive() -> usize; }
export! { fn crypto_pwhash_passwd_max() -> usize; }
export! { fn crypto_pwhash_passwd_min() -> usize; }
export! { fn crypto_pwhash_primitive() -> *const ::std::os::raw::c_char; }
export! { fn crypto_pwhash_saltbytes() -> usize; }
export! {
    fn crypto_pwhash_str(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
export! {
    fn crypto_pwhash_str_alg(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
export! {
    fn crypto_pwhash_str_needs_rehash(
        str_: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
export! {
    fn crypto_pwhash_str_verify(
        str_: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
export! { fn crypto_pwhash_strbytes() -> usize; }
export! { fn crypto_pwhash_strprefix() -> *const ::std::os::raw::c_char; }

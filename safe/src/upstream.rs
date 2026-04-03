#![allow(non_camel_case_types)]
#![allow(dead_code)]
use crate::abi::types::*;

#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct crypto_aead_aes256gcm_state_ {
    pub opaque: [::std::os::raw::c_uchar; 512usize],
}
pub struct UpstreamLib {
    __library: ::libloading::Library,
    pub sodium_init: Result<unsafe extern "C" fn() -> ::std::os::raw::c_int, ::libloading::Error>,
    pub crypto_aead_aes256gcm_is_available:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_int, ::libloading::Error>,
    pub crypto_aead_aes256gcm_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_aes256gcm_nsecbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_aes256gcm_npubbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_aes256gcm_abytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_aes256gcm_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_aes256gcm_statebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_aes256gcm_encrypt: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_decrypt: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            mlen_p: *mut ::std::os::raw::c_ulonglong,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_encrypt_detached: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            mac: *mut ::std::os::raw::c_uchar,
            maclen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_decrypt_detached: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            mac: *const ::std::os::raw::c_uchar,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_beforenm: Result<
        unsafe extern "C" fn(
            ctx_: *mut crypto_aead_aes256gcm_state,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_encrypt_afternm: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            ctx_: *const crypto_aead_aes256gcm_state,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_decrypt_afternm: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            mlen_p: *mut ::std::os::raw::c_ulonglong,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            ctx_: *const crypto_aead_aes256gcm_state,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_encrypt_detached_afternm: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            mac: *mut ::std::os::raw::c_uchar,
            maclen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            ctx_: *const crypto_aead_aes256gcm_state,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_decrypt_detached_afternm: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            mac: *const ::std::os::raw::c_uchar,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            ctx_: *const crypto_aead_aes256gcm_state,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_aes256gcm_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_ietf_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_ietf_nsecbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_ietf_npubbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_ietf_abytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_ietf_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_ietf_encrypt: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_ietf_decrypt: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            mlen_p: *mut ::std::os::raw::c_ulonglong,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_ietf_encrypt_detached: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            mac: *mut ::std::os::raw::c_uchar,
            maclen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_ietf_decrypt_detached: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            mac: *const ::std::os::raw::c_uchar,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_ietf_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_nsecbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_npubbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_abytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_chacha20poly1305_encrypt: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_decrypt: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            mlen_p: *mut ::std::os::raw::c_ulonglong,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_encrypt_detached: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            mac: *mut ::std::os::raw::c_uchar,
            maclen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_decrypt_detached: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            mac: *const ::std::os::raw::c_uchar,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_chacha20poly1305_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_aead_xchacha20poly1305_ietf_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_xchacha20poly1305_ietf_nsecbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_xchacha20poly1305_ietf_npubbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_xchacha20poly1305_ietf_abytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_xchacha20poly1305_ietf_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_aead_xchacha20poly1305_ietf_encrypt: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_xchacha20poly1305_ietf_decrypt: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            mlen_p: *mut ::std::os::raw::c_ulonglong,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_xchacha20poly1305_ietf_encrypt_detached: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            mac: *mut ::std::os::raw::c_uchar,
            maclen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            nsec: *const ::std::os::raw::c_uchar,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_xchacha20poly1305_ietf_decrypt_detached: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            nsec: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            mac: *const ::std::os::raw::c_uchar,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            npub: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_aead_xchacha20poly1305_ietf_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_hash_sha512_statebytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_hash_sha512_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_hash_sha512: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_hash_sha512_init: Result<
        unsafe extern "C" fn(state: *mut crypto_hash_sha512_state) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_hash_sha512_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_hash_sha512_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_hash_sha512_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_hash_sha512_state,
            out: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha512_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha512: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512_verify: Result<
        unsafe extern "C" fn(
            h: *const ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512_statebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha512_init: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha512_state,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha512_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha512_state,
            out: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_auth_hmacsha512256_bytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha512256_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha512256: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512256_verify: Result<
        unsafe extern "C" fn(
            h: *const ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512256_statebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha512256_init: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha512256_state,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512256_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha512256_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512256_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha512256_state,
            out: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha512256_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_auth_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_auth: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_verify: Result<
        unsafe extern "C" fn(
            h: *const ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_hash_sha256_statebytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_hash_sha256_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_hash_sha256: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_hash_sha256_init: Result<
        unsafe extern "C" fn(state: *mut crypto_hash_sha256_state) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_hash_sha256_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_hash_sha256_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_hash_sha256_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_hash_sha256_state,
            out: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha256_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha256_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha256: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha256_verify: Result<
        unsafe extern "C" fn(
            h: *const ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha256_statebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_auth_hmacsha256_init: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha256_state,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha256_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha256_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha256_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_auth_hmacsha256_state,
            out: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_auth_hmacsha256_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_stream_xsalsa20_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_xsalsa20_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_xsalsa20_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_xsalsa20: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_xsalsa20_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_xsalsa20_xor_ic: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            ic: u64,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_xsalsa20_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_core_hsalsa20_outputbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hsalsa20_inputbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hsalsa20_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hsalsa20_constbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hsalsa20: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_core_hchacha20_outputbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hchacha20_inputbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hchacha20_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hchacha20_constbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_hchacha20: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_core_salsa20_outputbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_salsa20_inputbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_salsa20_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_salsa20_constbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_core_salsa20: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_blake2b_bytes_min:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_bytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_bytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_keybytes_min:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_keybytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_saltbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_personalbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b_statebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_blake2b: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            outlen: usize,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_blake2b_salt_personal: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            outlen: usize,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
            salt: *const ::std::os::raw::c_uchar,
            personal: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_blake2b_init: Result<
        unsafe extern "C" fn(
            state: *mut crypto_generichash_blake2b_state,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
            outlen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_blake2b_init_salt_personal: Result<
        unsafe extern "C" fn(
            state: *mut crypto_generichash_blake2b_state,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
            outlen: usize,
            salt: *const ::std::os::raw::c_uchar,
            personal: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_blake2b_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_generichash_blake2b_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_blake2b_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_generichash_blake2b_state,
            out: *mut ::std::os::raw::c_uchar,
            outlen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_blake2b_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_generichash_bytes_min: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_bytes_max: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_keybytes_min:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_keybytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_generichash_statebytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_generichash: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            outlen: usize,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_init: Result<
        unsafe extern "C" fn(
            state: *mut crypto_generichash_state,
            key: *const ::std::os::raw::c_uchar,
            keylen: usize,
            outlen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_generichash_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_generichash_state,
            out: *mut ::std::os::raw::c_uchar,
            outlen: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_generichash_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_hash_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_hash: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_hash_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_kdf_blake2b_bytes_min: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_blake2b_bytes_max: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_blake2b_contextbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_blake2b_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_blake2b_derive_from_key: Result<
        unsafe extern "C" fn(
            subkey: *mut ::std::os::raw::c_uchar,
            subkey_len: usize,
            subkey_id: u64,
            ctx: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_kdf_bytes_min: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_bytes_max: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_contextbytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_kdf_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_kdf_derive_from_key: Result<
        unsafe extern "C" fn(
            subkey: *mut ::std::os::raw::c_uchar,
            subkey_len: usize,
            subkey_id: u64,
            ctx: *const ::std::os::raw::c_char,
            key: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_kdf_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_onetimeauth_poly1305_statebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_onetimeauth_poly1305_bytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_onetimeauth_poly1305_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_onetimeauth_poly1305: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_poly1305_verify: Result<
        unsafe extern "C" fn(
            h: *const ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_poly1305_init: Result<
        unsafe extern "C" fn(
            state: *mut crypto_onetimeauth_poly1305_state,
            key: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_poly1305_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_onetimeauth_poly1305_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_poly1305_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_onetimeauth_poly1305_state,
            out: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_poly1305_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_onetimeauth_statebytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_onetimeauth_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_onetimeauth_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_onetimeauth_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_onetimeauth: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_verify: Result<
        unsafe extern "C" fn(
            h: *const ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_init: Result<
        unsafe extern "C" fn(
            state: *mut crypto_onetimeauth_state,
            key: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_update: Result<
        unsafe extern "C" fn(
            state: *mut crypto_onetimeauth_state,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_final: Result<
        unsafe extern "C" fn(
            state: *mut crypto_onetimeauth_state,
            out: *mut ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_onetimeauth_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_pwhash_argon2i_alg_argon2i13:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_int, ::libloading::Error>,
    pub crypto_pwhash_argon2i_bytes_min:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_bytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_passwd_min:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_passwd_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_saltbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_strbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_strprefix:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_pwhash_argon2i_opslimit_min:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_opslimit_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_memlimit_min:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_memlimit_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_opslimit_interactive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_memlimit_interactive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_opslimit_moderate:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_memlimit_moderate:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_opslimit_sensitive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i_memlimit_sensitive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_argon2i: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            outlen: ::std::os::raw::c_ulonglong,
            passwd: *const ::std::os::raw::c_char,
            passwdlen: ::std::os::raw::c_ulonglong,
            salt: *const ::std::os::raw::c_uchar,
            opslimit: ::std::os::raw::c_ulonglong,
            memlimit: usize,
            alg: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_argon2i_str: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_char,
            passwd: *const ::std::os::raw::c_char,
            passwdlen: ::std::os::raw::c_ulonglong,
            opslimit: ::std::os::raw::c_ulonglong,
            memlimit: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_argon2i_str_verify: Result<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            passwd: *const ::std::os::raw::c_char,
            passwdlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_argon2i_str_needs_rehash: Result<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            opslimit: ::std::os::raw::c_ulonglong,
            memlimit: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_alg_argon2i13:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_int, ::libloading::Error>,
    pub crypto_pwhash_alg_argon2id13:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_int, ::libloading::Error>,
    pub crypto_pwhash_alg_default:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_int, ::libloading::Error>,
    pub crypto_pwhash_bytes_min: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_bytes_max: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_passwd_min: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_passwd_max: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_saltbytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_strbytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_strprefix:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_pwhash_opslimit_min: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_opslimit_max: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_memlimit_min: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_memlimit_max: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_opslimit_interactive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_memlimit_interactive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_opslimit_moderate:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_memlimit_moderate:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_opslimit_sensitive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash_memlimit_sensitive:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_pwhash: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            outlen: ::std::os::raw::c_ulonglong,
            passwd: *const ::std::os::raw::c_char,
            passwdlen: ::std::os::raw::c_ulonglong,
            salt: *const ::std::os::raw::c_uchar,
            opslimit: ::std::os::raw::c_ulonglong,
            memlimit: usize,
            alg: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_str: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_char,
            passwd: *const ::std::os::raw::c_char,
            passwdlen: ::std::os::raw::c_ulonglong,
            opslimit: ::std::os::raw::c_ulonglong,
            memlimit: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_str_alg: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_char,
            passwd: *const ::std::os::raw::c_char,
            passwdlen: ::std::os::raw::c_ulonglong,
            opslimit: ::std::os::raw::c_ulonglong,
            memlimit: usize,
            alg: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_str_verify: Result<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            passwd: *const ::std::os::raw::c_char,
            passwdlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_str_needs_rehash: Result<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            opslimit: ::std::os::raw::c_ulonglong,
            memlimit: usize,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_pwhash_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_secretbox_xsalsa20poly1305_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xsalsa20poly1305_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xsalsa20poly1305_macbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xsalsa20poly1305_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xsalsa20poly1305: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_xsalsa20poly1305_open: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_xsalsa20poly1305_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_secretbox_xsalsa20poly1305_boxzerobytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xsalsa20poly1305_zerobytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_noncebytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_macbytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_secretbox_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_easy: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_open_easy: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_detached: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            mac: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_open_detached: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            mac: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_secretbox_zerobytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_boxzerobytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_open: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_chacha20_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_chacha20_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_chacha20_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_chacha20: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_chacha20_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_chacha20_xor_ic: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            ic: u64,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_chacha20_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_stream_chacha20_ietf_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_chacha20_ietf_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_chacha20_ietf_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_chacha20_ietf: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_chacha20_ietf_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_chacha20_ietf_xor_ic: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            ic: u32,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_chacha20_ietf_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_abytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_headerbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_tag_message:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_uchar, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_tag_push:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_uchar, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_tag_rekey:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_uchar, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_tag_final:
        Result<unsafe extern "C" fn() -> ::std::os::raw::c_uchar, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_statebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_secretstream_xchacha20poly1305_init_push: Result<
        unsafe extern "C" fn(
            state: *mut crypto_secretstream_xchacha20poly1305_state,
            header: *mut ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretstream_xchacha20poly1305_push: Result<
        unsafe extern "C" fn(
            state: *mut crypto_secretstream_xchacha20poly1305_state,
            c: *mut ::std::os::raw::c_uchar,
            clen_p: *mut ::std::os::raw::c_ulonglong,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
            tag: ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretstream_xchacha20poly1305_init_pull: Result<
        unsafe extern "C" fn(
            state: *mut crypto_secretstream_xchacha20poly1305_state,
            header: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretstream_xchacha20poly1305_pull: Result<
        unsafe extern "C" fn(
            state: *mut crypto_secretstream_xchacha20poly1305_state,
            m: *mut ::std::os::raw::c_uchar,
            mlen_p: *mut ::std::os::raw::c_ulonglong,
            tag_p: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            ad: *const ::std::os::raw::c_uchar,
            adlen: ::std::os::raw::c_ulonglong,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretstream_xchacha20poly1305_rekey: Result<
        unsafe extern "C" fn(state: *mut crypto_secretstream_xchacha20poly1305_state),
        ::libloading::Error,
    >,
    pub crypto_shorthash_siphash24_bytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_shorthash_siphash24_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_shorthash_siphash24: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_shorthash_siphashx24_bytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_shorthash_siphashx24_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_shorthash_siphashx24: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_shorthash_bytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_shorthash_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_shorthash_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_shorthash: Result<
        unsafe extern "C" fn(
            out: *mut ::std::os::raw::c_uchar,
            in_: *const ::std::os::raw::c_uchar,
            inlen: ::std::os::raw::c_ulonglong,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_shorthash_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_stream_keybytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_noncebytes: Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_primitive:
        Result<unsafe extern "C" fn() -> *const ::std::os::raw::c_char, ::libloading::Error>,
    pub crypto_stream: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_stream_salsa20_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa20_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa20_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa20: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa20_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa20_xor_ic: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            ic: u64,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa20_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_stream_xchacha20_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_xchacha20_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_xchacha20_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_xchacha20: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_xchacha20_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_xchacha20_xor_ic: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            ic: u64,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_xchacha20_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_secretbox_xchacha20poly1305_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xchacha20poly1305_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xchacha20poly1305_macbytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xchacha20poly1305_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_secretbox_xchacha20poly1305_easy: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_xchacha20poly1305_open_easy: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_xchacha20poly1305_detached: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            mac: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_secretbox_xchacha20poly1305_open_detached: Result<
        unsafe extern "C" fn(
            m: *mut ::std::os::raw::c_uchar,
            c: *const ::std::os::raw::c_uchar,
            mac: *const ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa2012_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa2012_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa2012_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa2012: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa2012_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa2012_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
    pub crypto_stream_salsa208_keybytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa208_noncebytes:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa208_messagebytes_max:
        Result<unsafe extern "C" fn() -> usize, ::libloading::Error>,
    pub crypto_stream_salsa208: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            clen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa208_xor: Result<
        unsafe extern "C" fn(
            c: *mut ::std::os::raw::c_uchar,
            m: *const ::std::os::raw::c_uchar,
            mlen: ::std::os::raw::c_ulonglong,
            n: *const ::std::os::raw::c_uchar,
            k: *const ::std::os::raw::c_uchar,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub crypto_stream_salsa208_keygen:
        Result<unsafe extern "C" fn(k: *mut ::std::os::raw::c_uchar), ::libloading::Error>,
}
impl UpstreamLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let sodium_init = __library.get(b"sodium_init\0").map(|sym| *sym);
        let crypto_aead_aes256gcm_is_available = __library
            .get(b"crypto_aead_aes256gcm_is_available\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_keybytes = __library
            .get(b"crypto_aead_aes256gcm_keybytes\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_nsecbytes = __library
            .get(b"crypto_aead_aes256gcm_nsecbytes\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_npubbytes = __library
            .get(b"crypto_aead_aes256gcm_npubbytes\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_abytes = __library
            .get(b"crypto_aead_aes256gcm_abytes\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_messagebytes_max = __library
            .get(b"crypto_aead_aes256gcm_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_statebytes = __library
            .get(b"crypto_aead_aes256gcm_statebytes\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_encrypt = __library
            .get(b"crypto_aead_aes256gcm_encrypt\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_decrypt = __library
            .get(b"crypto_aead_aes256gcm_decrypt\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_encrypt_detached = __library
            .get(b"crypto_aead_aes256gcm_encrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_decrypt_detached = __library
            .get(b"crypto_aead_aes256gcm_decrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_beforenm = __library
            .get(b"crypto_aead_aes256gcm_beforenm\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_encrypt_afternm = __library
            .get(b"crypto_aead_aes256gcm_encrypt_afternm\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_decrypt_afternm = __library
            .get(b"crypto_aead_aes256gcm_decrypt_afternm\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_encrypt_detached_afternm = __library
            .get(b"crypto_aead_aes256gcm_encrypt_detached_afternm\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_decrypt_detached_afternm = __library
            .get(b"crypto_aead_aes256gcm_decrypt_detached_afternm\0")
            .map(|sym| *sym);
        let crypto_aead_aes256gcm_keygen = __library
            .get(b"crypto_aead_aes256gcm_keygen\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_keybytes = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_keybytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_nsecbytes = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_nsecbytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_npubbytes = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_npubbytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_abytes = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_abytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_messagebytes_max = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_encrypt = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_encrypt\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_decrypt = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_decrypt\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_encrypt_detached = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_encrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_decrypt_detached = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_decrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_ietf_keygen = __library
            .get(b"crypto_aead_chacha20poly1305_ietf_keygen\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_keybytes = __library
            .get(b"crypto_aead_chacha20poly1305_keybytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_nsecbytes = __library
            .get(b"crypto_aead_chacha20poly1305_nsecbytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_npubbytes = __library
            .get(b"crypto_aead_chacha20poly1305_npubbytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_abytes = __library
            .get(b"crypto_aead_chacha20poly1305_abytes\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_messagebytes_max = __library
            .get(b"crypto_aead_chacha20poly1305_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_encrypt = __library
            .get(b"crypto_aead_chacha20poly1305_encrypt\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_decrypt = __library
            .get(b"crypto_aead_chacha20poly1305_decrypt\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_encrypt_detached = __library
            .get(b"crypto_aead_chacha20poly1305_encrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_decrypt_detached = __library
            .get(b"crypto_aead_chacha20poly1305_decrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_chacha20poly1305_keygen = __library
            .get(b"crypto_aead_chacha20poly1305_keygen\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_keybytes = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_keybytes\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_nsecbytes = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_nsecbytes\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_npubbytes = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_npubbytes\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_abytes = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_abytes\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_messagebytes_max = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_encrypt = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_encrypt\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_decrypt = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_decrypt\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_encrypt_detached = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_encrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_decrypt_detached = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_decrypt_detached\0")
            .map(|sym| *sym);
        let crypto_aead_xchacha20poly1305_ietf_keygen = __library
            .get(b"crypto_aead_xchacha20poly1305_ietf_keygen\0")
            .map(|sym| *sym);
        let crypto_hash_sha512_statebytes = __library
            .get(b"crypto_hash_sha512_statebytes\0")
            .map(|sym| *sym);
        let crypto_hash_sha512_bytes = __library.get(b"crypto_hash_sha512_bytes\0").map(|sym| *sym);
        let crypto_hash_sha512 = __library.get(b"crypto_hash_sha512\0").map(|sym| *sym);
        let crypto_hash_sha512_init = __library.get(b"crypto_hash_sha512_init\0").map(|sym| *sym);
        let crypto_hash_sha512_update = __library
            .get(b"crypto_hash_sha512_update\0")
            .map(|sym| *sym);
        let crypto_hash_sha512_final = __library.get(b"crypto_hash_sha512_final\0").map(|sym| *sym);
        let crypto_auth_hmacsha512_bytes = __library
            .get(b"crypto_auth_hmacsha512_bytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512_keybytes = __library
            .get(b"crypto_auth_hmacsha512_keybytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512 = __library.get(b"crypto_auth_hmacsha512\0").map(|sym| *sym);
        let crypto_auth_hmacsha512_verify = __library
            .get(b"crypto_auth_hmacsha512_verify\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512_statebytes = __library
            .get(b"crypto_auth_hmacsha512_statebytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512_init = __library
            .get(b"crypto_auth_hmacsha512_init\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512_update = __library
            .get(b"crypto_auth_hmacsha512_update\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512_final = __library
            .get(b"crypto_auth_hmacsha512_final\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512_keygen = __library
            .get(b"crypto_auth_hmacsha512_keygen\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_bytes = __library
            .get(b"crypto_auth_hmacsha512256_bytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_keybytes = __library
            .get(b"crypto_auth_hmacsha512256_keybytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256 = __library
            .get(b"crypto_auth_hmacsha512256\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_verify = __library
            .get(b"crypto_auth_hmacsha512256_verify\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_statebytes = __library
            .get(b"crypto_auth_hmacsha512256_statebytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_init = __library
            .get(b"crypto_auth_hmacsha512256_init\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_update = __library
            .get(b"crypto_auth_hmacsha512256_update\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_final = __library
            .get(b"crypto_auth_hmacsha512256_final\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha512256_keygen = __library
            .get(b"crypto_auth_hmacsha512256_keygen\0")
            .map(|sym| *sym);
        let crypto_auth_bytes = __library.get(b"crypto_auth_bytes\0").map(|sym| *sym);
        let crypto_auth_keybytes = __library.get(b"crypto_auth_keybytes\0").map(|sym| *sym);
        let crypto_auth_primitive = __library.get(b"crypto_auth_primitive\0").map(|sym| *sym);
        let crypto_auth = __library.get(b"crypto_auth\0").map(|sym| *sym);
        let crypto_auth_verify = __library.get(b"crypto_auth_verify\0").map(|sym| *sym);
        let crypto_auth_keygen = __library.get(b"crypto_auth_keygen\0").map(|sym| *sym);
        let crypto_hash_sha256_statebytes = __library
            .get(b"crypto_hash_sha256_statebytes\0")
            .map(|sym| *sym);
        let crypto_hash_sha256_bytes = __library.get(b"crypto_hash_sha256_bytes\0").map(|sym| *sym);
        let crypto_hash_sha256 = __library.get(b"crypto_hash_sha256\0").map(|sym| *sym);
        let crypto_hash_sha256_init = __library.get(b"crypto_hash_sha256_init\0").map(|sym| *sym);
        let crypto_hash_sha256_update = __library
            .get(b"crypto_hash_sha256_update\0")
            .map(|sym| *sym);
        let crypto_hash_sha256_final = __library.get(b"crypto_hash_sha256_final\0").map(|sym| *sym);
        let crypto_auth_hmacsha256_bytes = __library
            .get(b"crypto_auth_hmacsha256_bytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha256_keybytes = __library
            .get(b"crypto_auth_hmacsha256_keybytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha256 = __library.get(b"crypto_auth_hmacsha256\0").map(|sym| *sym);
        let crypto_auth_hmacsha256_verify = __library
            .get(b"crypto_auth_hmacsha256_verify\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha256_statebytes = __library
            .get(b"crypto_auth_hmacsha256_statebytes\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha256_init = __library
            .get(b"crypto_auth_hmacsha256_init\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha256_update = __library
            .get(b"crypto_auth_hmacsha256_update\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha256_final = __library
            .get(b"crypto_auth_hmacsha256_final\0")
            .map(|sym| *sym);
        let crypto_auth_hmacsha256_keygen = __library
            .get(b"crypto_auth_hmacsha256_keygen\0")
            .map(|sym| *sym);
        let crypto_stream_xsalsa20_keybytes = __library
            .get(b"crypto_stream_xsalsa20_keybytes\0")
            .map(|sym| *sym);
        let crypto_stream_xsalsa20_noncebytes = __library
            .get(b"crypto_stream_xsalsa20_noncebytes\0")
            .map(|sym| *sym);
        let crypto_stream_xsalsa20_messagebytes_max = __library
            .get(b"crypto_stream_xsalsa20_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_xsalsa20 = __library.get(b"crypto_stream_xsalsa20\0").map(|sym| *sym);
        let crypto_stream_xsalsa20_xor = __library
            .get(b"crypto_stream_xsalsa20_xor\0")
            .map(|sym| *sym);
        let crypto_stream_xsalsa20_xor_ic = __library
            .get(b"crypto_stream_xsalsa20_xor_ic\0")
            .map(|sym| *sym);
        let crypto_stream_xsalsa20_keygen = __library
            .get(b"crypto_stream_xsalsa20_keygen\0")
            .map(|sym| *sym);
        let crypto_core_hsalsa20_outputbytes = __library
            .get(b"crypto_core_hsalsa20_outputbytes\0")
            .map(|sym| *sym);
        let crypto_core_hsalsa20_inputbytes = __library
            .get(b"crypto_core_hsalsa20_inputbytes\0")
            .map(|sym| *sym);
        let crypto_core_hsalsa20_keybytes = __library
            .get(b"crypto_core_hsalsa20_keybytes\0")
            .map(|sym| *sym);
        let crypto_core_hsalsa20_constbytes = __library
            .get(b"crypto_core_hsalsa20_constbytes\0")
            .map(|sym| *sym);
        let crypto_core_hsalsa20 = __library.get(b"crypto_core_hsalsa20\0").map(|sym| *sym);
        let crypto_core_hchacha20_outputbytes = __library
            .get(b"crypto_core_hchacha20_outputbytes\0")
            .map(|sym| *sym);
        let crypto_core_hchacha20_inputbytes = __library
            .get(b"crypto_core_hchacha20_inputbytes\0")
            .map(|sym| *sym);
        let crypto_core_hchacha20_keybytes = __library
            .get(b"crypto_core_hchacha20_keybytes\0")
            .map(|sym| *sym);
        let crypto_core_hchacha20_constbytes = __library
            .get(b"crypto_core_hchacha20_constbytes\0")
            .map(|sym| *sym);
        let crypto_core_hchacha20 = __library.get(b"crypto_core_hchacha20\0").map(|sym| *sym);
        let crypto_core_salsa20_outputbytes = __library
            .get(b"crypto_core_salsa20_outputbytes\0")
            .map(|sym| *sym);
        let crypto_core_salsa20_inputbytes = __library
            .get(b"crypto_core_salsa20_inputbytes\0")
            .map(|sym| *sym);
        let crypto_core_salsa20_keybytes = __library
            .get(b"crypto_core_salsa20_keybytes\0")
            .map(|sym| *sym);
        let crypto_core_salsa20_constbytes = __library
            .get(b"crypto_core_salsa20_constbytes\0")
            .map(|sym| *sym);
        let crypto_core_salsa20 = __library.get(b"crypto_core_salsa20\0").map(|sym| *sym);
        let crypto_generichash_blake2b_bytes_min = __library
            .get(b"crypto_generichash_blake2b_bytes_min\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_bytes_max = __library
            .get(b"crypto_generichash_blake2b_bytes_max\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_bytes = __library
            .get(b"crypto_generichash_blake2b_bytes\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_keybytes_min = __library
            .get(b"crypto_generichash_blake2b_keybytes_min\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_keybytes_max = __library
            .get(b"crypto_generichash_blake2b_keybytes_max\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_keybytes = __library
            .get(b"crypto_generichash_blake2b_keybytes\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_saltbytes = __library
            .get(b"crypto_generichash_blake2b_saltbytes\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_personalbytes = __library
            .get(b"crypto_generichash_blake2b_personalbytes\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_statebytes = __library
            .get(b"crypto_generichash_blake2b_statebytes\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b = __library
            .get(b"crypto_generichash_blake2b\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_salt_personal = __library
            .get(b"crypto_generichash_blake2b_salt_personal\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_init = __library
            .get(b"crypto_generichash_blake2b_init\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_init_salt_personal = __library
            .get(b"crypto_generichash_blake2b_init_salt_personal\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_update = __library
            .get(b"crypto_generichash_blake2b_update\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_final = __library
            .get(b"crypto_generichash_blake2b_final\0")
            .map(|sym| *sym);
        let crypto_generichash_blake2b_keygen = __library
            .get(b"crypto_generichash_blake2b_keygen\0")
            .map(|sym| *sym);
        let crypto_generichash_bytes_min = __library
            .get(b"crypto_generichash_bytes_min\0")
            .map(|sym| *sym);
        let crypto_generichash_bytes_max = __library
            .get(b"crypto_generichash_bytes_max\0")
            .map(|sym| *sym);
        let crypto_generichash_bytes = __library.get(b"crypto_generichash_bytes\0").map(|sym| *sym);
        let crypto_generichash_keybytes_min = __library
            .get(b"crypto_generichash_keybytes_min\0")
            .map(|sym| *sym);
        let crypto_generichash_keybytes_max = __library
            .get(b"crypto_generichash_keybytes_max\0")
            .map(|sym| *sym);
        let crypto_generichash_keybytes = __library
            .get(b"crypto_generichash_keybytes\0")
            .map(|sym| *sym);
        let crypto_generichash_primitive = __library
            .get(b"crypto_generichash_primitive\0")
            .map(|sym| *sym);
        let crypto_generichash_statebytes = __library
            .get(b"crypto_generichash_statebytes\0")
            .map(|sym| *sym);
        let crypto_generichash = __library.get(b"crypto_generichash\0").map(|sym| *sym);
        let crypto_generichash_init = __library.get(b"crypto_generichash_init\0").map(|sym| *sym);
        let crypto_generichash_update = __library
            .get(b"crypto_generichash_update\0")
            .map(|sym| *sym);
        let crypto_generichash_final = __library.get(b"crypto_generichash_final\0").map(|sym| *sym);
        let crypto_generichash_keygen = __library
            .get(b"crypto_generichash_keygen\0")
            .map(|sym| *sym);
        let crypto_hash_bytes = __library.get(b"crypto_hash_bytes\0").map(|sym| *sym);
        let crypto_hash = __library.get(b"crypto_hash\0").map(|sym| *sym);
        let crypto_hash_primitive = __library.get(b"crypto_hash_primitive\0").map(|sym| *sym);
        let crypto_kdf_blake2b_bytes_min = __library
            .get(b"crypto_kdf_blake2b_bytes_min\0")
            .map(|sym| *sym);
        let crypto_kdf_blake2b_bytes_max = __library
            .get(b"crypto_kdf_blake2b_bytes_max\0")
            .map(|sym| *sym);
        let crypto_kdf_blake2b_contextbytes = __library
            .get(b"crypto_kdf_blake2b_contextbytes\0")
            .map(|sym| *sym);
        let crypto_kdf_blake2b_keybytes = __library
            .get(b"crypto_kdf_blake2b_keybytes\0")
            .map(|sym| *sym);
        let crypto_kdf_blake2b_derive_from_key = __library
            .get(b"crypto_kdf_blake2b_derive_from_key\0")
            .map(|sym| *sym);
        let crypto_kdf_bytes_min = __library.get(b"crypto_kdf_bytes_min\0").map(|sym| *sym);
        let crypto_kdf_bytes_max = __library.get(b"crypto_kdf_bytes_max\0").map(|sym| *sym);
        let crypto_kdf_contextbytes = __library.get(b"crypto_kdf_contextbytes\0").map(|sym| *sym);
        let crypto_kdf_keybytes = __library.get(b"crypto_kdf_keybytes\0").map(|sym| *sym);
        let crypto_kdf_primitive = __library.get(b"crypto_kdf_primitive\0").map(|sym| *sym);
        let crypto_kdf_derive_from_key = __library
            .get(b"crypto_kdf_derive_from_key\0")
            .map(|sym| *sym);
        let crypto_kdf_keygen = __library.get(b"crypto_kdf_keygen\0").map(|sym| *sym);
        let crypto_onetimeauth_poly1305_statebytes = __library
            .get(b"crypto_onetimeauth_poly1305_statebytes\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305_bytes = __library
            .get(b"crypto_onetimeauth_poly1305_bytes\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305_keybytes = __library
            .get(b"crypto_onetimeauth_poly1305_keybytes\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305 = __library
            .get(b"crypto_onetimeauth_poly1305\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305_verify = __library
            .get(b"crypto_onetimeauth_poly1305_verify\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305_init = __library
            .get(b"crypto_onetimeauth_poly1305_init\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305_update = __library
            .get(b"crypto_onetimeauth_poly1305_update\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305_final = __library
            .get(b"crypto_onetimeauth_poly1305_final\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_poly1305_keygen = __library
            .get(b"crypto_onetimeauth_poly1305_keygen\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_statebytes = __library
            .get(b"crypto_onetimeauth_statebytes\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_bytes = __library.get(b"crypto_onetimeauth_bytes\0").map(|sym| *sym);
        let crypto_onetimeauth_keybytes = __library
            .get(b"crypto_onetimeauth_keybytes\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_primitive = __library
            .get(b"crypto_onetimeauth_primitive\0")
            .map(|sym| *sym);
        let crypto_onetimeauth = __library.get(b"crypto_onetimeauth\0").map(|sym| *sym);
        let crypto_onetimeauth_verify = __library
            .get(b"crypto_onetimeauth_verify\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_init = __library.get(b"crypto_onetimeauth_init\0").map(|sym| *sym);
        let crypto_onetimeauth_update = __library
            .get(b"crypto_onetimeauth_update\0")
            .map(|sym| *sym);
        let crypto_onetimeauth_final = __library.get(b"crypto_onetimeauth_final\0").map(|sym| *sym);
        let crypto_onetimeauth_keygen = __library
            .get(b"crypto_onetimeauth_keygen\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_alg_argon2i13 = __library
            .get(b"crypto_pwhash_argon2i_alg_argon2i13\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_bytes_min = __library
            .get(b"crypto_pwhash_argon2i_bytes_min\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_bytes_max = __library
            .get(b"crypto_pwhash_argon2i_bytes_max\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_passwd_min = __library
            .get(b"crypto_pwhash_argon2i_passwd_min\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_passwd_max = __library
            .get(b"crypto_pwhash_argon2i_passwd_max\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_saltbytes = __library
            .get(b"crypto_pwhash_argon2i_saltbytes\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_strbytes = __library
            .get(b"crypto_pwhash_argon2i_strbytes\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_strprefix = __library
            .get(b"crypto_pwhash_argon2i_strprefix\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_opslimit_min = __library
            .get(b"crypto_pwhash_argon2i_opslimit_min\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_opslimit_max = __library
            .get(b"crypto_pwhash_argon2i_opslimit_max\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_memlimit_min = __library
            .get(b"crypto_pwhash_argon2i_memlimit_min\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_memlimit_max = __library
            .get(b"crypto_pwhash_argon2i_memlimit_max\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_opslimit_interactive = __library
            .get(b"crypto_pwhash_argon2i_opslimit_interactive\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_memlimit_interactive = __library
            .get(b"crypto_pwhash_argon2i_memlimit_interactive\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_opslimit_moderate = __library
            .get(b"crypto_pwhash_argon2i_opslimit_moderate\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_memlimit_moderate = __library
            .get(b"crypto_pwhash_argon2i_memlimit_moderate\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_opslimit_sensitive = __library
            .get(b"crypto_pwhash_argon2i_opslimit_sensitive\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_memlimit_sensitive = __library
            .get(b"crypto_pwhash_argon2i_memlimit_sensitive\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i = __library.get(b"crypto_pwhash_argon2i\0").map(|sym| *sym);
        let crypto_pwhash_argon2i_str = __library
            .get(b"crypto_pwhash_argon2i_str\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_str_verify = __library
            .get(b"crypto_pwhash_argon2i_str_verify\0")
            .map(|sym| *sym);
        let crypto_pwhash_argon2i_str_needs_rehash = __library
            .get(b"crypto_pwhash_argon2i_str_needs_rehash\0")
            .map(|sym| *sym);
        let crypto_pwhash_alg_argon2i13 = __library
            .get(b"crypto_pwhash_alg_argon2i13\0")
            .map(|sym| *sym);
        let crypto_pwhash_alg_argon2id13 = __library
            .get(b"crypto_pwhash_alg_argon2id13\0")
            .map(|sym| *sym);
        let crypto_pwhash_alg_default = __library
            .get(b"crypto_pwhash_alg_default\0")
            .map(|sym| *sym);
        let crypto_pwhash_bytes_min = __library.get(b"crypto_pwhash_bytes_min\0").map(|sym| *sym);
        let crypto_pwhash_bytes_max = __library.get(b"crypto_pwhash_bytes_max\0").map(|sym| *sym);
        let crypto_pwhash_passwd_min = __library.get(b"crypto_pwhash_passwd_min\0").map(|sym| *sym);
        let crypto_pwhash_passwd_max = __library.get(b"crypto_pwhash_passwd_max\0").map(|sym| *sym);
        let crypto_pwhash_saltbytes = __library.get(b"crypto_pwhash_saltbytes\0").map(|sym| *sym);
        let crypto_pwhash_strbytes = __library.get(b"crypto_pwhash_strbytes\0").map(|sym| *sym);
        let crypto_pwhash_strprefix = __library.get(b"crypto_pwhash_strprefix\0").map(|sym| *sym);
        let crypto_pwhash_opslimit_min = __library
            .get(b"crypto_pwhash_opslimit_min\0")
            .map(|sym| *sym);
        let crypto_pwhash_opslimit_max = __library
            .get(b"crypto_pwhash_opslimit_max\0")
            .map(|sym| *sym);
        let crypto_pwhash_memlimit_min = __library
            .get(b"crypto_pwhash_memlimit_min\0")
            .map(|sym| *sym);
        let crypto_pwhash_memlimit_max = __library
            .get(b"crypto_pwhash_memlimit_max\0")
            .map(|sym| *sym);
        let crypto_pwhash_opslimit_interactive = __library
            .get(b"crypto_pwhash_opslimit_interactive\0")
            .map(|sym| *sym);
        let crypto_pwhash_memlimit_interactive = __library
            .get(b"crypto_pwhash_memlimit_interactive\0")
            .map(|sym| *sym);
        let crypto_pwhash_opslimit_moderate = __library
            .get(b"crypto_pwhash_opslimit_moderate\0")
            .map(|sym| *sym);
        let crypto_pwhash_memlimit_moderate = __library
            .get(b"crypto_pwhash_memlimit_moderate\0")
            .map(|sym| *sym);
        let crypto_pwhash_opslimit_sensitive = __library
            .get(b"crypto_pwhash_opslimit_sensitive\0")
            .map(|sym| *sym);
        let crypto_pwhash_memlimit_sensitive = __library
            .get(b"crypto_pwhash_memlimit_sensitive\0")
            .map(|sym| *sym);
        let crypto_pwhash = __library.get(b"crypto_pwhash\0").map(|sym| *sym);
        let crypto_pwhash_str = __library.get(b"crypto_pwhash_str\0").map(|sym| *sym);
        let crypto_pwhash_str_alg = __library.get(b"crypto_pwhash_str_alg\0").map(|sym| *sym);
        let crypto_pwhash_str_verify = __library.get(b"crypto_pwhash_str_verify\0").map(|sym| *sym);
        let crypto_pwhash_str_needs_rehash = __library
            .get(b"crypto_pwhash_str_needs_rehash\0")
            .map(|sym| *sym);
        let crypto_pwhash_primitive = __library.get(b"crypto_pwhash_primitive\0").map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_keybytes = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_keybytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_noncebytes = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_noncebytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_macbytes = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_macbytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_messagebytes_max = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305 = __library
            .get(b"crypto_secretbox_xsalsa20poly1305\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_open = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_open\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_keygen = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_keygen\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_boxzerobytes = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_boxzerobytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_xsalsa20poly1305_zerobytes = __library
            .get(b"crypto_secretbox_xsalsa20poly1305_zerobytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_keybytes = __library
            .get(b"crypto_secretbox_keybytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_noncebytes = __library
            .get(b"crypto_secretbox_noncebytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_macbytes = __library
            .get(b"crypto_secretbox_macbytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_primitive = __library
            .get(b"crypto_secretbox_primitive\0")
            .map(|sym| *sym);
        let crypto_secretbox_messagebytes_max = __library
            .get(b"crypto_secretbox_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_secretbox_easy = __library.get(b"crypto_secretbox_easy\0").map(|sym| *sym);
        let crypto_secretbox_open_easy = __library
            .get(b"crypto_secretbox_open_easy\0")
            .map(|sym| *sym);
        let crypto_secretbox_detached = __library
            .get(b"crypto_secretbox_detached\0")
            .map(|sym| *sym);
        let crypto_secretbox_open_detached = __library
            .get(b"crypto_secretbox_open_detached\0")
            .map(|sym| *sym);
        let crypto_secretbox_keygen = __library.get(b"crypto_secretbox_keygen\0").map(|sym| *sym);
        let crypto_secretbox_zerobytes = __library
            .get(b"crypto_secretbox_zerobytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_boxzerobytes = __library
            .get(b"crypto_secretbox_boxzerobytes\0")
            .map(|sym| *sym);
        let crypto_secretbox = __library.get(b"crypto_secretbox\0").map(|sym| *sym);
        let crypto_secretbox_open = __library.get(b"crypto_secretbox_open\0").map(|sym| *sym);
        let crypto_stream_chacha20_keybytes = __library
            .get(b"crypto_stream_chacha20_keybytes\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_noncebytes = __library
            .get(b"crypto_stream_chacha20_noncebytes\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_messagebytes_max = __library
            .get(b"crypto_stream_chacha20_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20 = __library.get(b"crypto_stream_chacha20\0").map(|sym| *sym);
        let crypto_stream_chacha20_xor = __library
            .get(b"crypto_stream_chacha20_xor\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_xor_ic = __library
            .get(b"crypto_stream_chacha20_xor_ic\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_keygen = __library
            .get(b"crypto_stream_chacha20_keygen\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_ietf_keybytes = __library
            .get(b"crypto_stream_chacha20_ietf_keybytes\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_ietf_noncebytes = __library
            .get(b"crypto_stream_chacha20_ietf_noncebytes\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_ietf_messagebytes_max = __library
            .get(b"crypto_stream_chacha20_ietf_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_ietf = __library
            .get(b"crypto_stream_chacha20_ietf\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_ietf_xor = __library
            .get(b"crypto_stream_chacha20_ietf_xor\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_ietf_xor_ic = __library
            .get(b"crypto_stream_chacha20_ietf_xor_ic\0")
            .map(|sym| *sym);
        let crypto_stream_chacha20_ietf_keygen = __library
            .get(b"crypto_stream_chacha20_ietf_keygen\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_abytes = __library
            .get(b"crypto_secretstream_xchacha20poly1305_abytes\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_headerbytes = __library
            .get(b"crypto_secretstream_xchacha20poly1305_headerbytes\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_keybytes = __library
            .get(b"crypto_secretstream_xchacha20poly1305_keybytes\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_messagebytes_max = __library
            .get(b"crypto_secretstream_xchacha20poly1305_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_tag_message = __library
            .get(b"crypto_secretstream_xchacha20poly1305_tag_message\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_tag_push = __library
            .get(b"crypto_secretstream_xchacha20poly1305_tag_push\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_tag_rekey = __library
            .get(b"crypto_secretstream_xchacha20poly1305_tag_rekey\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_tag_final = __library
            .get(b"crypto_secretstream_xchacha20poly1305_tag_final\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_statebytes = __library
            .get(b"crypto_secretstream_xchacha20poly1305_statebytes\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_keygen = __library
            .get(b"crypto_secretstream_xchacha20poly1305_keygen\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_init_push = __library
            .get(b"crypto_secretstream_xchacha20poly1305_init_push\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_push = __library
            .get(b"crypto_secretstream_xchacha20poly1305_push\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_init_pull = __library
            .get(b"crypto_secretstream_xchacha20poly1305_init_pull\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_pull = __library
            .get(b"crypto_secretstream_xchacha20poly1305_pull\0")
            .map(|sym| *sym);
        let crypto_secretstream_xchacha20poly1305_rekey = __library
            .get(b"crypto_secretstream_xchacha20poly1305_rekey\0")
            .map(|sym| *sym);
        let crypto_shorthash_siphash24_bytes = __library
            .get(b"crypto_shorthash_siphash24_bytes\0")
            .map(|sym| *sym);
        let crypto_shorthash_siphash24_keybytes = __library
            .get(b"crypto_shorthash_siphash24_keybytes\0")
            .map(|sym| *sym);
        let crypto_shorthash_siphash24 = __library
            .get(b"crypto_shorthash_siphash24\0")
            .map(|sym| *sym);
        let crypto_shorthash_siphashx24_bytes = __library
            .get(b"crypto_shorthash_siphashx24_bytes\0")
            .map(|sym| *sym);
        let crypto_shorthash_siphashx24_keybytes = __library
            .get(b"crypto_shorthash_siphashx24_keybytes\0")
            .map(|sym| *sym);
        let crypto_shorthash_siphashx24 = __library
            .get(b"crypto_shorthash_siphashx24\0")
            .map(|sym| *sym);
        let crypto_shorthash_bytes = __library.get(b"crypto_shorthash_bytes\0").map(|sym| *sym);
        let crypto_shorthash_keybytes = __library
            .get(b"crypto_shorthash_keybytes\0")
            .map(|sym| *sym);
        let crypto_shorthash_primitive = __library
            .get(b"crypto_shorthash_primitive\0")
            .map(|sym| *sym);
        let crypto_shorthash = __library.get(b"crypto_shorthash\0").map(|sym| *sym);
        let crypto_shorthash_keygen = __library.get(b"crypto_shorthash_keygen\0").map(|sym| *sym);
        let crypto_stream_keybytes = __library.get(b"crypto_stream_keybytes\0").map(|sym| *sym);
        let crypto_stream_noncebytes = __library.get(b"crypto_stream_noncebytes\0").map(|sym| *sym);
        let crypto_stream_messagebytes_max = __library
            .get(b"crypto_stream_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_primitive = __library.get(b"crypto_stream_primitive\0").map(|sym| *sym);
        let crypto_stream = __library.get(b"crypto_stream\0").map(|sym| *sym);
        let crypto_stream_xor = __library.get(b"crypto_stream_xor\0").map(|sym| *sym);
        let crypto_stream_keygen = __library.get(b"crypto_stream_keygen\0").map(|sym| *sym);
        let crypto_stream_salsa20_keybytes = __library
            .get(b"crypto_stream_salsa20_keybytes\0")
            .map(|sym| *sym);
        let crypto_stream_salsa20_noncebytes = __library
            .get(b"crypto_stream_salsa20_noncebytes\0")
            .map(|sym| *sym);
        let crypto_stream_salsa20_messagebytes_max = __library
            .get(b"crypto_stream_salsa20_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_salsa20 = __library.get(b"crypto_stream_salsa20\0").map(|sym| *sym);
        let crypto_stream_salsa20_xor = __library
            .get(b"crypto_stream_salsa20_xor\0")
            .map(|sym| *sym);
        let crypto_stream_salsa20_xor_ic = __library
            .get(b"crypto_stream_salsa20_xor_ic\0")
            .map(|sym| *sym);
        let crypto_stream_salsa20_keygen = __library
            .get(b"crypto_stream_salsa20_keygen\0")
            .map(|sym| *sym);
        let crypto_stream_xchacha20_keybytes = __library
            .get(b"crypto_stream_xchacha20_keybytes\0")
            .map(|sym| *sym);
        let crypto_stream_xchacha20_noncebytes = __library
            .get(b"crypto_stream_xchacha20_noncebytes\0")
            .map(|sym| *sym);
        let crypto_stream_xchacha20_messagebytes_max = __library
            .get(b"crypto_stream_xchacha20_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_xchacha20 = __library.get(b"crypto_stream_xchacha20\0").map(|sym| *sym);
        let crypto_stream_xchacha20_xor = __library
            .get(b"crypto_stream_xchacha20_xor\0")
            .map(|sym| *sym);
        let crypto_stream_xchacha20_xor_ic = __library
            .get(b"crypto_stream_xchacha20_xor_ic\0")
            .map(|sym| *sym);
        let crypto_stream_xchacha20_keygen = __library
            .get(b"crypto_stream_xchacha20_keygen\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_keybytes = __library
            .get(b"crypto_secretbox_xchacha20poly1305_keybytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_noncebytes = __library
            .get(b"crypto_secretbox_xchacha20poly1305_noncebytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_macbytes = __library
            .get(b"crypto_secretbox_xchacha20poly1305_macbytes\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_messagebytes_max = __library
            .get(b"crypto_secretbox_xchacha20poly1305_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_easy = __library
            .get(b"crypto_secretbox_xchacha20poly1305_easy\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_open_easy = __library
            .get(b"crypto_secretbox_xchacha20poly1305_open_easy\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_detached = __library
            .get(b"crypto_secretbox_xchacha20poly1305_detached\0")
            .map(|sym| *sym);
        let crypto_secretbox_xchacha20poly1305_open_detached = __library
            .get(b"crypto_secretbox_xchacha20poly1305_open_detached\0")
            .map(|sym| *sym);
        let crypto_stream_salsa2012_keybytes = __library
            .get(b"crypto_stream_salsa2012_keybytes\0")
            .map(|sym| *sym);
        let crypto_stream_salsa2012_noncebytes = __library
            .get(b"crypto_stream_salsa2012_noncebytes\0")
            .map(|sym| *sym);
        let crypto_stream_salsa2012_messagebytes_max = __library
            .get(b"crypto_stream_salsa2012_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_salsa2012 = __library.get(b"crypto_stream_salsa2012\0").map(|sym| *sym);
        let crypto_stream_salsa2012_xor = __library
            .get(b"crypto_stream_salsa2012_xor\0")
            .map(|sym| *sym);
        let crypto_stream_salsa2012_keygen = __library
            .get(b"crypto_stream_salsa2012_keygen\0")
            .map(|sym| *sym);
        let crypto_stream_salsa208_keybytes = __library
            .get(b"crypto_stream_salsa208_keybytes\0")
            .map(|sym| *sym);
        let crypto_stream_salsa208_noncebytes = __library
            .get(b"crypto_stream_salsa208_noncebytes\0")
            .map(|sym| *sym);
        let crypto_stream_salsa208_messagebytes_max = __library
            .get(b"crypto_stream_salsa208_messagebytes_max\0")
            .map(|sym| *sym);
        let crypto_stream_salsa208 = __library.get(b"crypto_stream_salsa208\0").map(|sym| *sym);
        let crypto_stream_salsa208_xor = __library
            .get(b"crypto_stream_salsa208_xor\0")
            .map(|sym| *sym);
        let crypto_stream_salsa208_keygen = __library
            .get(b"crypto_stream_salsa208_keygen\0")
            .map(|sym| *sym);
        Ok(UpstreamLib {
            __library,
            sodium_init,
            crypto_aead_aes256gcm_is_available,
            crypto_aead_aes256gcm_keybytes,
            crypto_aead_aes256gcm_nsecbytes,
            crypto_aead_aes256gcm_npubbytes,
            crypto_aead_aes256gcm_abytes,
            crypto_aead_aes256gcm_messagebytes_max,
            crypto_aead_aes256gcm_statebytes,
            crypto_aead_aes256gcm_encrypt,
            crypto_aead_aes256gcm_decrypt,
            crypto_aead_aes256gcm_encrypt_detached,
            crypto_aead_aes256gcm_decrypt_detached,
            crypto_aead_aes256gcm_beforenm,
            crypto_aead_aes256gcm_encrypt_afternm,
            crypto_aead_aes256gcm_decrypt_afternm,
            crypto_aead_aes256gcm_encrypt_detached_afternm,
            crypto_aead_aes256gcm_decrypt_detached_afternm,
            crypto_aead_aes256gcm_keygen,
            crypto_aead_chacha20poly1305_ietf_keybytes,
            crypto_aead_chacha20poly1305_ietf_nsecbytes,
            crypto_aead_chacha20poly1305_ietf_npubbytes,
            crypto_aead_chacha20poly1305_ietf_abytes,
            crypto_aead_chacha20poly1305_ietf_messagebytes_max,
            crypto_aead_chacha20poly1305_ietf_encrypt,
            crypto_aead_chacha20poly1305_ietf_decrypt,
            crypto_aead_chacha20poly1305_ietf_encrypt_detached,
            crypto_aead_chacha20poly1305_ietf_decrypt_detached,
            crypto_aead_chacha20poly1305_ietf_keygen,
            crypto_aead_chacha20poly1305_keybytes,
            crypto_aead_chacha20poly1305_nsecbytes,
            crypto_aead_chacha20poly1305_npubbytes,
            crypto_aead_chacha20poly1305_abytes,
            crypto_aead_chacha20poly1305_messagebytes_max,
            crypto_aead_chacha20poly1305_encrypt,
            crypto_aead_chacha20poly1305_decrypt,
            crypto_aead_chacha20poly1305_encrypt_detached,
            crypto_aead_chacha20poly1305_decrypt_detached,
            crypto_aead_chacha20poly1305_keygen,
            crypto_aead_xchacha20poly1305_ietf_keybytes,
            crypto_aead_xchacha20poly1305_ietf_nsecbytes,
            crypto_aead_xchacha20poly1305_ietf_npubbytes,
            crypto_aead_xchacha20poly1305_ietf_abytes,
            crypto_aead_xchacha20poly1305_ietf_messagebytes_max,
            crypto_aead_xchacha20poly1305_ietf_encrypt,
            crypto_aead_xchacha20poly1305_ietf_decrypt,
            crypto_aead_xchacha20poly1305_ietf_encrypt_detached,
            crypto_aead_xchacha20poly1305_ietf_decrypt_detached,
            crypto_aead_xchacha20poly1305_ietf_keygen,
            crypto_hash_sha512_statebytes,
            crypto_hash_sha512_bytes,
            crypto_hash_sha512,
            crypto_hash_sha512_init,
            crypto_hash_sha512_update,
            crypto_hash_sha512_final,
            crypto_auth_hmacsha512_bytes,
            crypto_auth_hmacsha512_keybytes,
            crypto_auth_hmacsha512,
            crypto_auth_hmacsha512_verify,
            crypto_auth_hmacsha512_statebytes,
            crypto_auth_hmacsha512_init,
            crypto_auth_hmacsha512_update,
            crypto_auth_hmacsha512_final,
            crypto_auth_hmacsha512_keygen,
            crypto_auth_hmacsha512256_bytes,
            crypto_auth_hmacsha512256_keybytes,
            crypto_auth_hmacsha512256,
            crypto_auth_hmacsha512256_verify,
            crypto_auth_hmacsha512256_statebytes,
            crypto_auth_hmacsha512256_init,
            crypto_auth_hmacsha512256_update,
            crypto_auth_hmacsha512256_final,
            crypto_auth_hmacsha512256_keygen,
            crypto_auth_bytes,
            crypto_auth_keybytes,
            crypto_auth_primitive,
            crypto_auth,
            crypto_auth_verify,
            crypto_auth_keygen,
            crypto_hash_sha256_statebytes,
            crypto_hash_sha256_bytes,
            crypto_hash_sha256,
            crypto_hash_sha256_init,
            crypto_hash_sha256_update,
            crypto_hash_sha256_final,
            crypto_auth_hmacsha256_bytes,
            crypto_auth_hmacsha256_keybytes,
            crypto_auth_hmacsha256,
            crypto_auth_hmacsha256_verify,
            crypto_auth_hmacsha256_statebytes,
            crypto_auth_hmacsha256_init,
            crypto_auth_hmacsha256_update,
            crypto_auth_hmacsha256_final,
            crypto_auth_hmacsha256_keygen,
            crypto_stream_xsalsa20_keybytes,
            crypto_stream_xsalsa20_noncebytes,
            crypto_stream_xsalsa20_messagebytes_max,
            crypto_stream_xsalsa20,
            crypto_stream_xsalsa20_xor,
            crypto_stream_xsalsa20_xor_ic,
            crypto_stream_xsalsa20_keygen,
            crypto_core_hsalsa20_outputbytes,
            crypto_core_hsalsa20_inputbytes,
            crypto_core_hsalsa20_keybytes,
            crypto_core_hsalsa20_constbytes,
            crypto_core_hsalsa20,
            crypto_core_hchacha20_outputbytes,
            crypto_core_hchacha20_inputbytes,
            crypto_core_hchacha20_keybytes,
            crypto_core_hchacha20_constbytes,
            crypto_core_hchacha20,
            crypto_core_salsa20_outputbytes,
            crypto_core_salsa20_inputbytes,
            crypto_core_salsa20_keybytes,
            crypto_core_salsa20_constbytes,
            crypto_core_salsa20,
            crypto_generichash_blake2b_bytes_min,
            crypto_generichash_blake2b_bytes_max,
            crypto_generichash_blake2b_bytes,
            crypto_generichash_blake2b_keybytes_min,
            crypto_generichash_blake2b_keybytes_max,
            crypto_generichash_blake2b_keybytes,
            crypto_generichash_blake2b_saltbytes,
            crypto_generichash_blake2b_personalbytes,
            crypto_generichash_blake2b_statebytes,
            crypto_generichash_blake2b,
            crypto_generichash_blake2b_salt_personal,
            crypto_generichash_blake2b_init,
            crypto_generichash_blake2b_init_salt_personal,
            crypto_generichash_blake2b_update,
            crypto_generichash_blake2b_final,
            crypto_generichash_blake2b_keygen,
            crypto_generichash_bytes_min,
            crypto_generichash_bytes_max,
            crypto_generichash_bytes,
            crypto_generichash_keybytes_min,
            crypto_generichash_keybytes_max,
            crypto_generichash_keybytes,
            crypto_generichash_primitive,
            crypto_generichash_statebytes,
            crypto_generichash,
            crypto_generichash_init,
            crypto_generichash_update,
            crypto_generichash_final,
            crypto_generichash_keygen,
            crypto_hash_bytes,
            crypto_hash,
            crypto_hash_primitive,
            crypto_kdf_blake2b_bytes_min,
            crypto_kdf_blake2b_bytes_max,
            crypto_kdf_blake2b_contextbytes,
            crypto_kdf_blake2b_keybytes,
            crypto_kdf_blake2b_derive_from_key,
            crypto_kdf_bytes_min,
            crypto_kdf_bytes_max,
            crypto_kdf_contextbytes,
            crypto_kdf_keybytes,
            crypto_kdf_primitive,
            crypto_kdf_derive_from_key,
            crypto_kdf_keygen,
            crypto_onetimeauth_poly1305_statebytes,
            crypto_onetimeauth_poly1305_bytes,
            crypto_onetimeauth_poly1305_keybytes,
            crypto_onetimeauth_poly1305,
            crypto_onetimeauth_poly1305_verify,
            crypto_onetimeauth_poly1305_init,
            crypto_onetimeauth_poly1305_update,
            crypto_onetimeauth_poly1305_final,
            crypto_onetimeauth_poly1305_keygen,
            crypto_onetimeauth_statebytes,
            crypto_onetimeauth_bytes,
            crypto_onetimeauth_keybytes,
            crypto_onetimeauth_primitive,
            crypto_onetimeauth,
            crypto_onetimeauth_verify,
            crypto_onetimeauth_init,
            crypto_onetimeauth_update,
            crypto_onetimeauth_final,
            crypto_onetimeauth_keygen,
            crypto_pwhash_argon2i_alg_argon2i13,
            crypto_pwhash_argon2i_bytes_min,
            crypto_pwhash_argon2i_bytes_max,
            crypto_pwhash_argon2i_passwd_min,
            crypto_pwhash_argon2i_passwd_max,
            crypto_pwhash_argon2i_saltbytes,
            crypto_pwhash_argon2i_strbytes,
            crypto_pwhash_argon2i_strprefix,
            crypto_pwhash_argon2i_opslimit_min,
            crypto_pwhash_argon2i_opslimit_max,
            crypto_pwhash_argon2i_memlimit_min,
            crypto_pwhash_argon2i_memlimit_max,
            crypto_pwhash_argon2i_opslimit_interactive,
            crypto_pwhash_argon2i_memlimit_interactive,
            crypto_pwhash_argon2i_opslimit_moderate,
            crypto_pwhash_argon2i_memlimit_moderate,
            crypto_pwhash_argon2i_opslimit_sensitive,
            crypto_pwhash_argon2i_memlimit_sensitive,
            crypto_pwhash_argon2i,
            crypto_pwhash_argon2i_str,
            crypto_pwhash_argon2i_str_verify,
            crypto_pwhash_argon2i_str_needs_rehash,
            crypto_pwhash_alg_argon2i13,
            crypto_pwhash_alg_argon2id13,
            crypto_pwhash_alg_default,
            crypto_pwhash_bytes_min,
            crypto_pwhash_bytes_max,
            crypto_pwhash_passwd_min,
            crypto_pwhash_passwd_max,
            crypto_pwhash_saltbytes,
            crypto_pwhash_strbytes,
            crypto_pwhash_strprefix,
            crypto_pwhash_opslimit_min,
            crypto_pwhash_opslimit_max,
            crypto_pwhash_memlimit_min,
            crypto_pwhash_memlimit_max,
            crypto_pwhash_opslimit_interactive,
            crypto_pwhash_memlimit_interactive,
            crypto_pwhash_opslimit_moderate,
            crypto_pwhash_memlimit_moderate,
            crypto_pwhash_opslimit_sensitive,
            crypto_pwhash_memlimit_sensitive,
            crypto_pwhash,
            crypto_pwhash_str,
            crypto_pwhash_str_alg,
            crypto_pwhash_str_verify,
            crypto_pwhash_str_needs_rehash,
            crypto_pwhash_primitive,
            crypto_secretbox_xsalsa20poly1305_keybytes,
            crypto_secretbox_xsalsa20poly1305_noncebytes,
            crypto_secretbox_xsalsa20poly1305_macbytes,
            crypto_secretbox_xsalsa20poly1305_messagebytes_max,
            crypto_secretbox_xsalsa20poly1305,
            crypto_secretbox_xsalsa20poly1305_open,
            crypto_secretbox_xsalsa20poly1305_keygen,
            crypto_secretbox_xsalsa20poly1305_boxzerobytes,
            crypto_secretbox_xsalsa20poly1305_zerobytes,
            crypto_secretbox_keybytes,
            crypto_secretbox_noncebytes,
            crypto_secretbox_macbytes,
            crypto_secretbox_primitive,
            crypto_secretbox_messagebytes_max,
            crypto_secretbox_easy,
            crypto_secretbox_open_easy,
            crypto_secretbox_detached,
            crypto_secretbox_open_detached,
            crypto_secretbox_keygen,
            crypto_secretbox_zerobytes,
            crypto_secretbox_boxzerobytes,
            crypto_secretbox,
            crypto_secretbox_open,
            crypto_stream_chacha20_keybytes,
            crypto_stream_chacha20_noncebytes,
            crypto_stream_chacha20_messagebytes_max,
            crypto_stream_chacha20,
            crypto_stream_chacha20_xor,
            crypto_stream_chacha20_xor_ic,
            crypto_stream_chacha20_keygen,
            crypto_stream_chacha20_ietf_keybytes,
            crypto_stream_chacha20_ietf_noncebytes,
            crypto_stream_chacha20_ietf_messagebytes_max,
            crypto_stream_chacha20_ietf,
            crypto_stream_chacha20_ietf_xor,
            crypto_stream_chacha20_ietf_xor_ic,
            crypto_stream_chacha20_ietf_keygen,
            crypto_secretstream_xchacha20poly1305_abytes,
            crypto_secretstream_xchacha20poly1305_headerbytes,
            crypto_secretstream_xchacha20poly1305_keybytes,
            crypto_secretstream_xchacha20poly1305_messagebytes_max,
            crypto_secretstream_xchacha20poly1305_tag_message,
            crypto_secretstream_xchacha20poly1305_tag_push,
            crypto_secretstream_xchacha20poly1305_tag_rekey,
            crypto_secretstream_xchacha20poly1305_tag_final,
            crypto_secretstream_xchacha20poly1305_statebytes,
            crypto_secretstream_xchacha20poly1305_keygen,
            crypto_secretstream_xchacha20poly1305_init_push,
            crypto_secretstream_xchacha20poly1305_push,
            crypto_secretstream_xchacha20poly1305_init_pull,
            crypto_secretstream_xchacha20poly1305_pull,
            crypto_secretstream_xchacha20poly1305_rekey,
            crypto_shorthash_siphash24_bytes,
            crypto_shorthash_siphash24_keybytes,
            crypto_shorthash_siphash24,
            crypto_shorthash_siphashx24_bytes,
            crypto_shorthash_siphashx24_keybytes,
            crypto_shorthash_siphashx24,
            crypto_shorthash_bytes,
            crypto_shorthash_keybytes,
            crypto_shorthash_primitive,
            crypto_shorthash,
            crypto_shorthash_keygen,
            crypto_stream_keybytes,
            crypto_stream_noncebytes,
            crypto_stream_messagebytes_max,
            crypto_stream_primitive,
            crypto_stream,
            crypto_stream_xor,
            crypto_stream_keygen,
            crypto_stream_salsa20_keybytes,
            crypto_stream_salsa20_noncebytes,
            crypto_stream_salsa20_messagebytes_max,
            crypto_stream_salsa20,
            crypto_stream_salsa20_xor,
            crypto_stream_salsa20_xor_ic,
            crypto_stream_salsa20_keygen,
            crypto_stream_xchacha20_keybytes,
            crypto_stream_xchacha20_noncebytes,
            crypto_stream_xchacha20_messagebytes_max,
            crypto_stream_xchacha20,
            crypto_stream_xchacha20_xor,
            crypto_stream_xchacha20_xor_ic,
            crypto_stream_xchacha20_keygen,
            crypto_secretbox_xchacha20poly1305_keybytes,
            crypto_secretbox_xchacha20poly1305_noncebytes,
            crypto_secretbox_xchacha20poly1305_macbytes,
            crypto_secretbox_xchacha20poly1305_messagebytes_max,
            crypto_secretbox_xchacha20poly1305_easy,
            crypto_secretbox_xchacha20poly1305_open_easy,
            crypto_secretbox_xchacha20poly1305_detached,
            crypto_secretbox_xchacha20poly1305_open_detached,
            crypto_stream_salsa2012_keybytes,
            crypto_stream_salsa2012_noncebytes,
            crypto_stream_salsa2012_messagebytes_max,
            crypto_stream_salsa2012,
            crypto_stream_salsa2012_xor,
            crypto_stream_salsa2012_keygen,
            crypto_stream_salsa208_keybytes,
            crypto_stream_salsa208_noncebytes,
            crypto_stream_salsa208_messagebytes_max,
            crypto_stream_salsa208,
            crypto_stream_salsa208_xor,
            crypto_stream_salsa208_keygen,
        })
    }
    pub unsafe fn sodium_init(&self) -> ::std::os::raw::c_int {
        (self
            .sodium_init
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_is_available(&self) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_is_available
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_keybytes(&self) -> usize {
        (self
            .crypto_aead_aes256gcm_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_nsecbytes(&self) -> usize {
        (self
            .crypto_aead_aes256gcm_nsecbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_npubbytes(&self) -> usize {
        (self
            .crypto_aead_aes256gcm_npubbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_abytes(&self) -> usize {
        (self
            .crypto_aead_aes256gcm_abytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_messagebytes_max(&self) -> usize {
        (self
            .crypto_aead_aes256gcm_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_statebytes(&self) -> usize {
        (self
            .crypto_aead_aes256gcm_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_aes256gcm_encrypt(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_encrypt
            .as_ref()
            .expect("Expected function, got error."))(
            c, clen_p, m, mlen, ad, adlen, nsec, npub, k
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_decrypt(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_decrypt
            .as_ref()
            .expect("Expected function, got error."))(
            m, mlen_p, nsec, c, clen, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_encrypt_detached(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_encrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, k,
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_decrypt_detached(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_decrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            m, nsec, c, clen, mac, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_beforenm(
        &self,
        ctx_: *mut crypto_aead_aes256gcm_state,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_beforenm
            .as_ref()
            .expect("Expected function, got error."))(ctx_, k)
    }
    pub unsafe fn crypto_aead_aes256gcm_encrypt_afternm(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_encrypt_afternm
            .as_ref()
            .expect("Expected function, got error."))(
            c, clen_p, m, mlen, ad, adlen, nsec, npub, ctx_,
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_decrypt_afternm(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_decrypt_afternm
            .as_ref()
            .expect("Expected function, got error."))(
            m, mlen_p, nsec, c, clen, ad, adlen, npub, ctx_,
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_encrypt_detached_afternm(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_encrypt_detached_afternm
            .as_ref()
            .expect("Expected function, got error."))(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, ctx_,
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_decrypt_detached_afternm(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        ctx_: *const crypto_aead_aes256gcm_state,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_aes256gcm_decrypt_detached_afternm
            .as_ref()
            .expect("Expected function, got error."))(
            m, nsec, c, clen, mac, ad, adlen, npub, ctx_
        )
    }
    pub unsafe fn crypto_aead_aes256gcm_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_aead_aes256gcm_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_keybytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_ietf_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_nsecbytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_ietf_nsecbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_npubbytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_ietf_npubbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_abytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_ietf_abytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_messagebytes_max(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_ietf_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_encrypt(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_ietf_encrypt
            .as_ref()
            .expect("Expected function, got error."))(
            c, clen_p, m, mlen, ad, adlen, nsec, npub, k
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_decrypt(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_ietf_decrypt
            .as_ref()
            .expect("Expected function, got error."))(
            m, mlen_p, nsec, c, clen, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_encrypt_detached(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_ietf_encrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, k,
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_decrypt_detached(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_ietf_decrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            m, nsec, c, clen, mac, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_ietf_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_aead_chacha20poly1305_ietf_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_aead_chacha20poly1305_keybytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_nsecbytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_nsecbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_npubbytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_npubbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_abytes(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_abytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_messagebytes_max(&self) -> usize {
        (self
            .crypto_aead_chacha20poly1305_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_chacha20poly1305_encrypt(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_encrypt
            .as_ref()
            .expect("Expected function, got error."))(
            c, clen_p, m, mlen, ad, adlen, nsec, npub, k
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_decrypt(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_decrypt
            .as_ref()
            .expect("Expected function, got error."))(
            m, mlen_p, nsec, c, clen, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_encrypt_detached(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_encrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, k,
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_decrypt_detached(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_chacha20poly1305_decrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            m, nsec, c, clen, mac, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_chacha20poly1305_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_aead_chacha20poly1305_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_keybytes(&self) -> usize {
        (self
            .crypto_aead_xchacha20poly1305_ietf_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_nsecbytes(&self) -> usize {
        (self
            .crypto_aead_xchacha20poly1305_ietf_nsecbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_npubbytes(&self) -> usize {
        (self
            .crypto_aead_xchacha20poly1305_ietf_npubbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_abytes(&self) -> usize {
        (self
            .crypto_aead_xchacha20poly1305_ietf_abytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_messagebytes_max(&self) -> usize {
        (self
            .crypto_aead_xchacha20poly1305_ietf_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_encrypt(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_xchacha20poly1305_ietf_encrypt
            .as_ref()
            .expect("Expected function, got error."))(
            c, clen_p, m, mlen, ad, adlen, nsec, npub, k
        )
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_decrypt(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_xchacha20poly1305_ietf_decrypt
            .as_ref()
            .expect("Expected function, got error."))(
            m, mlen_p, nsec, c, clen, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_encrypt_detached(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        maclen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_xchacha20poly1305_ietf_encrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            c, mac, maclen_p, m, mlen, ad, adlen, nsec, npub, k,
        )
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        mac: *const ::std::os::raw::c_uchar,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_aead_xchacha20poly1305_ietf_decrypt_detached
            .as_ref()
            .expect("Expected function, got error."))(
            m, nsec, c, clen, mac, ad, adlen, npub, k
        )
    }
    pub unsafe fn crypto_aead_xchacha20poly1305_ietf_keygen(
        &self,
        k: *mut ::std::os::raw::c_uchar,
    ) {
        (self
            .crypto_aead_xchacha20poly1305_ietf_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_hash_sha512_statebytes(&self) -> usize {
        (self
            .crypto_hash_sha512_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_hash_sha512_bytes(&self) -> usize {
        (self
            .crypto_hash_sha512_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_hash_sha512(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha512
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen)
    }
    pub unsafe fn crypto_hash_sha512_init(
        &self,
        state: *mut crypto_hash_sha512_state,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha512_init
            .as_ref()
            .expect("Expected function, got error."))(state)
    }
    pub unsafe fn crypto_hash_sha512_update(
        &self,
        state: *mut crypto_hash_sha512_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha512_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_hash_sha512_final(
        &self,
        state: *mut crypto_hash_sha512_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha512_final
            .as_ref()
            .expect("Expected function, got error."))(state, out)
    }
    pub unsafe fn crypto_auth_hmacsha512_bytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha512_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha512_keybytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha512_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha512(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_hmacsha512_verify(
        &self,
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512_verify
            .as_ref()
            .expect("Expected function, got error."))(h, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_hmacsha512_statebytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha512_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha512_init(
        &self,
        state: *mut crypto_auth_hmacsha512_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512_init
            .as_ref()
            .expect("Expected function, got error."))(state, key, keylen)
    }
    pub unsafe fn crypto_auth_hmacsha512_update(
        &self,
        state: *mut crypto_auth_hmacsha512_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_auth_hmacsha512_final(
        &self,
        state: *mut crypto_auth_hmacsha512_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512_final
            .as_ref()
            .expect("Expected function, got error."))(state, out)
    }
    pub unsafe fn crypto_auth_hmacsha512_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_auth_hmacsha512_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_auth_hmacsha512256_bytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha512256_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha512256_keybytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha512256_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha512256(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512256
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_hmacsha512256_verify(
        &self,
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512256_verify
            .as_ref()
            .expect("Expected function, got error."))(h, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_hmacsha512256_statebytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha512256_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha512256_init(
        &self,
        state: *mut crypto_auth_hmacsha512256_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512256_init
            .as_ref()
            .expect("Expected function, got error."))(state, key, keylen)
    }
    pub unsafe fn crypto_auth_hmacsha512256_update(
        &self,
        state: *mut crypto_auth_hmacsha512256_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512256_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_auth_hmacsha512256_final(
        &self,
        state: *mut crypto_auth_hmacsha512256_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha512256_final
            .as_ref()
            .expect("Expected function, got error."))(state, out)
    }
    pub unsafe fn crypto_auth_hmacsha512256_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_auth_hmacsha512256_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_auth_bytes(&self) -> usize {
        (self
            .crypto_auth_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_keybytes(&self) -> usize {
        (self
            .crypto_auth_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_auth_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_verify(
        &self,
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_verify
            .as_ref()
            .expect("Expected function, got error."))(h, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_auth_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_hash_sha256_statebytes(&self) -> usize {
        (self
            .crypto_hash_sha256_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_hash_sha256_bytes(&self) -> usize {
        (self
            .crypto_hash_sha256_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_hash_sha256(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha256
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen)
    }
    pub unsafe fn crypto_hash_sha256_init(
        &self,
        state: *mut crypto_hash_sha256_state,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha256_init
            .as_ref()
            .expect("Expected function, got error."))(state)
    }
    pub unsafe fn crypto_hash_sha256_update(
        &self,
        state: *mut crypto_hash_sha256_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha256_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_hash_sha256_final(
        &self,
        state: *mut crypto_hash_sha256_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash_sha256_final
            .as_ref()
            .expect("Expected function, got error."))(state, out)
    }
    pub unsafe fn crypto_auth_hmacsha256_bytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha256_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha256_keybytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha256_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha256(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha256
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_hmacsha256_verify(
        &self,
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha256_verify
            .as_ref()
            .expect("Expected function, got error."))(h, in_, inlen, k)
    }
    pub unsafe fn crypto_auth_hmacsha256_statebytes(&self) -> usize {
        (self
            .crypto_auth_hmacsha256_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_auth_hmacsha256_init(
        &self,
        state: *mut crypto_auth_hmacsha256_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha256_init
            .as_ref()
            .expect("Expected function, got error."))(state, key, keylen)
    }
    pub unsafe fn crypto_auth_hmacsha256_update(
        &self,
        state: *mut crypto_auth_hmacsha256_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha256_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_auth_hmacsha256_final(
        &self,
        state: *mut crypto_auth_hmacsha256_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_auth_hmacsha256_final
            .as_ref()
            .expect("Expected function, got error."))(state, out)
    }
    pub unsafe fn crypto_auth_hmacsha256_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_auth_hmacsha256_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_stream_xsalsa20_keybytes(&self) -> usize {
        (self
            .crypto_stream_xsalsa20_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_xsalsa20_noncebytes(&self) -> usize {
        (self
            .crypto_stream_xsalsa20_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_xsalsa20_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_xsalsa20_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_xsalsa20(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_xsalsa20
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_xsalsa20_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_xsalsa20_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_xsalsa20_xor_ic(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_xsalsa20_xor_ic
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, ic, k)
    }
    pub unsafe fn crypto_stream_xsalsa20_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_xsalsa20_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_core_hsalsa20_outputbytes(&self) -> usize {
        (self
            .crypto_core_hsalsa20_outputbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hsalsa20_inputbytes(&self) -> usize {
        (self
            .crypto_core_hsalsa20_inputbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hsalsa20_keybytes(&self) -> usize {
        (self
            .crypto_core_hsalsa20_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hsalsa20_constbytes(&self) -> usize {
        (self
            .crypto_core_hsalsa20_constbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hsalsa20(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_core_hsalsa20
            .as_ref()
            .expect("Expected function, got error."))(out, in_, k, c)
    }
    pub unsafe fn crypto_core_hchacha20_outputbytes(&self) -> usize {
        (self
            .crypto_core_hchacha20_outputbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hchacha20_inputbytes(&self) -> usize {
        (self
            .crypto_core_hchacha20_inputbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hchacha20_keybytes(&self) -> usize {
        (self
            .crypto_core_hchacha20_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hchacha20_constbytes(&self) -> usize {
        (self
            .crypto_core_hchacha20_constbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_hchacha20(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_core_hchacha20
            .as_ref()
            .expect("Expected function, got error."))(out, in_, k, c)
    }
    pub unsafe fn crypto_core_salsa20_outputbytes(&self) -> usize {
        (self
            .crypto_core_salsa20_outputbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_salsa20_inputbytes(&self) -> usize {
        (self
            .crypto_core_salsa20_inputbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_salsa20_keybytes(&self) -> usize {
        (self
            .crypto_core_salsa20_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_salsa20_constbytes(&self) -> usize {
        (self
            .crypto_core_salsa20_constbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_core_salsa20(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_core_salsa20
            .as_ref()
            .expect("Expected function, got error."))(out, in_, k, c)
    }
    pub unsafe fn crypto_generichash_blake2b_bytes_min(&self) -> usize {
        (self
            .crypto_generichash_blake2b_bytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_bytes_max(&self) -> usize {
        (self
            .crypto_generichash_blake2b_bytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_bytes(&self) -> usize {
        (self
            .crypto_generichash_blake2b_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_keybytes_min(&self) -> usize {
        (self
            .crypto_generichash_blake2b_keybytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_keybytes_max(&self) -> usize {
        (self
            .crypto_generichash_blake2b_keybytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_keybytes(&self) -> usize {
        (self
            .crypto_generichash_blake2b_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_saltbytes(&self) -> usize {
        (self
            .crypto_generichash_blake2b_saltbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_personalbytes(&self) -> usize {
        (self
            .crypto_generichash_blake2b_personalbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b_statebytes(&self) -> usize {
        (self
            .crypto_generichash_blake2b_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_blake2b(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_blake2b
            .as_ref()
            .expect("Expected function, got error."))(out, outlen, in_, inlen, key, keylen)
    }
    pub unsafe fn crypto_generichash_blake2b_salt_personal(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        salt: *const ::std::os::raw::c_uchar,
        personal: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_blake2b_salt_personal
            .as_ref()
            .expect("Expected function, got error."))(
            out, outlen, in_, inlen, key, keylen, salt, personal,
        )
    }
    pub unsafe fn crypto_generichash_blake2b_init(
        &self,
        state: *mut crypto_generichash_blake2b_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_blake2b_init
            .as_ref()
            .expect("Expected function, got error."))(state, key, keylen, outlen)
    }
    pub unsafe fn crypto_generichash_blake2b_init_salt_personal(
        &self,
        state: *mut crypto_generichash_blake2b_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        outlen: usize,
        salt: *const ::std::os::raw::c_uchar,
        personal: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_blake2b_init_salt_personal
            .as_ref()
            .expect("Expected function, got error."))(
            state, key, keylen, outlen, salt, personal
        )
    }
    pub unsafe fn crypto_generichash_blake2b_update(
        &self,
        state: *mut crypto_generichash_blake2b_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_blake2b_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_generichash_blake2b_final(
        &self,
        state: *mut crypto_generichash_blake2b_state,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_blake2b_final
            .as_ref()
            .expect("Expected function, got error."))(state, out, outlen)
    }
    pub unsafe fn crypto_generichash_blake2b_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_generichash_blake2b_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_generichash_bytes_min(&self) -> usize {
        (self
            .crypto_generichash_bytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_bytes_max(&self) -> usize {
        (self
            .crypto_generichash_bytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_bytes(&self) -> usize {
        (self
            .crypto_generichash_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_keybytes_min(&self) -> usize {
        (self
            .crypto_generichash_keybytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_keybytes_max(&self) -> usize {
        (self
            .crypto_generichash_keybytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_keybytes(&self) -> usize {
        (self
            .crypto_generichash_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_generichash_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash_statebytes(&self) -> usize {
        (self
            .crypto_generichash_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_generichash(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash
            .as_ref()
            .expect("Expected function, got error."))(out, outlen, in_, inlen, key, keylen)
    }
    pub unsafe fn crypto_generichash_init(
        &self,
        state: *mut crypto_generichash_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_init
            .as_ref()
            .expect("Expected function, got error."))(state, key, keylen, outlen)
    }
    pub unsafe fn crypto_generichash_update(
        &self,
        state: *mut crypto_generichash_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_generichash_final(
        &self,
        state: *mut crypto_generichash_state,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_generichash_final
            .as_ref()
            .expect("Expected function, got error."))(state, out, outlen)
    }
    pub unsafe fn crypto_generichash_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_generichash_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_hash_bytes(&self) -> usize {
        (self
            .crypto_hash_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_hash(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_hash
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen)
    }
    pub unsafe fn crypto_hash_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_hash_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_blake2b_bytes_min(&self) -> usize {
        (self
            .crypto_kdf_blake2b_bytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_blake2b_bytes_max(&self) -> usize {
        (self
            .crypto_kdf_blake2b_bytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_blake2b_contextbytes(&self) -> usize {
        (self
            .crypto_kdf_blake2b_contextbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_blake2b_keybytes(&self) -> usize {
        (self
            .crypto_kdf_blake2b_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_blake2b_derive_from_key(
        &self,
        subkey: *mut ::std::os::raw::c_uchar,
        subkey_len: usize,
        subkey_id: u64,
        ctx: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_kdf_blake2b_derive_from_key
            .as_ref()
            .expect("Expected function, got error."))(
            subkey, subkey_len, subkey_id, ctx, key
        )
    }
    pub unsafe fn crypto_kdf_bytes_min(&self) -> usize {
        (self
            .crypto_kdf_bytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_bytes_max(&self) -> usize {
        (self
            .crypto_kdf_bytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_contextbytes(&self) -> usize {
        (self
            .crypto_kdf_contextbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_keybytes(&self) -> usize {
        (self
            .crypto_kdf_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_kdf_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_kdf_derive_from_key(
        &self,
        subkey: *mut ::std::os::raw::c_uchar,
        subkey_len: usize,
        subkey_id: u64,
        ctx: *const ::std::os::raw::c_char,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_kdf_derive_from_key
            .as_ref()
            .expect("Expected function, got error."))(
            subkey, subkey_len, subkey_id, ctx, key
        )
    }
    pub unsafe fn crypto_kdf_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_kdf_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_onetimeauth_poly1305_statebytes(&self) -> usize {
        (self
            .crypto_onetimeauth_poly1305_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_onetimeauth_poly1305_bytes(&self) -> usize {
        (self
            .crypto_onetimeauth_poly1305_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_onetimeauth_poly1305_keybytes(&self) -> usize {
        (self
            .crypto_onetimeauth_poly1305_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_onetimeauth_poly1305(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_poly1305
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_onetimeauth_poly1305_verify(
        &self,
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_poly1305_verify
            .as_ref()
            .expect("Expected function, got error."))(h, in_, inlen, k)
    }
    pub unsafe fn crypto_onetimeauth_poly1305_init(
        &self,
        state: *mut crypto_onetimeauth_poly1305_state,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_poly1305_init
            .as_ref()
            .expect("Expected function, got error."))(state, key)
    }
    pub unsafe fn crypto_onetimeauth_poly1305_update(
        &self,
        state: *mut crypto_onetimeauth_poly1305_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_poly1305_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_onetimeauth_poly1305_final(
        &self,
        state: *mut crypto_onetimeauth_poly1305_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_poly1305_final
            .as_ref()
            .expect("Expected function, got error."))(state, out)
    }
    pub unsafe fn crypto_onetimeauth_poly1305_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_onetimeauth_poly1305_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_onetimeauth_statebytes(&self) -> usize {
        (self
            .crypto_onetimeauth_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_onetimeauth_bytes(&self) -> usize {
        (self
            .crypto_onetimeauth_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_onetimeauth_keybytes(&self) -> usize {
        (self
            .crypto_onetimeauth_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_onetimeauth_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_onetimeauth_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_onetimeauth(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_onetimeauth_verify(
        &self,
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_verify
            .as_ref()
            .expect("Expected function, got error."))(h, in_, inlen, k)
    }
    pub unsafe fn crypto_onetimeauth_init(
        &self,
        state: *mut crypto_onetimeauth_state,
        key: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_init
            .as_ref()
            .expect("Expected function, got error."))(state, key)
    }
    pub unsafe fn crypto_onetimeauth_update(
        &self,
        state: *mut crypto_onetimeauth_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_update
            .as_ref()
            .expect("Expected function, got error."))(state, in_, inlen)
    }
    pub unsafe fn crypto_onetimeauth_final(
        &self,
        state: *mut crypto_onetimeauth_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_onetimeauth_final
            .as_ref()
            .expect("Expected function, got error."))(state, out)
    }
    pub unsafe fn crypto_onetimeauth_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_onetimeauth_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_pwhash_argon2i_alg_argon2i13(&self) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_argon2i_alg_argon2i13
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_bytes_min(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_bytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_bytes_max(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_bytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_passwd_min(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_passwd_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_passwd_max(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_passwd_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_saltbytes(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_saltbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_strbytes(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_strbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_strprefix(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_pwhash_argon2i_strprefix
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_opslimit_min(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_opslimit_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_opslimit_max(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_opslimit_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_memlimit_min(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_memlimit_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_memlimit_max(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_memlimit_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_opslimit_interactive(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_opslimit_interactive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_memlimit_interactive(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_memlimit_interactive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_opslimit_moderate(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_opslimit_moderate
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_memlimit_moderate(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_memlimit_moderate
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_opslimit_sensitive(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_opslimit_sensitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i_memlimit_sensitive(&self) -> usize {
        (self
            .crypto_pwhash_argon2i_memlimit_sensitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_argon2i(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_argon2i
            .as_ref()
            .expect("Expected function, got error."))(
            out, outlen, passwd, passwdlen, salt, opslimit, memlimit, alg,
        )
    }
    pub unsafe fn crypto_pwhash_argon2i_str(
        &self,
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_argon2i_str
            .as_ref()
            .expect("Expected function, got error."))(
            out, passwd, passwdlen, opslimit, memlimit
        )
    }
    pub unsafe fn crypto_pwhash_argon2i_str_verify(
        &self,
        str_: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_argon2i_str_verify
            .as_ref()
            .expect("Expected function, got error."))(str_, passwd, passwdlen)
    }
    pub unsafe fn crypto_pwhash_argon2i_str_needs_rehash(
        &self,
        str_: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_argon2i_str_needs_rehash
            .as_ref()
            .expect("Expected function, got error."))(str_, opslimit, memlimit)
    }
    pub unsafe fn crypto_pwhash_alg_argon2i13(&self) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_alg_argon2i13
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_alg_argon2id13(&self) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_alg_argon2id13
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_alg_default(&self) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_alg_default
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_bytes_min(&self) -> usize {
        (self
            .crypto_pwhash_bytes_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_bytes_max(&self) -> usize {
        (self
            .crypto_pwhash_bytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_passwd_min(&self) -> usize {
        (self
            .crypto_pwhash_passwd_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_passwd_max(&self) -> usize {
        (self
            .crypto_pwhash_passwd_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_saltbytes(&self) -> usize {
        (self
            .crypto_pwhash_saltbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_strbytes(&self) -> usize {
        (self
            .crypto_pwhash_strbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_strprefix(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_pwhash_strprefix
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_opslimit_min(&self) -> usize {
        (self
            .crypto_pwhash_opslimit_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_opslimit_max(&self) -> usize {
        (self
            .crypto_pwhash_opslimit_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_memlimit_min(&self) -> usize {
        (self
            .crypto_pwhash_memlimit_min
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_memlimit_max(&self) -> usize {
        (self
            .crypto_pwhash_memlimit_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_opslimit_interactive(&self) -> usize {
        (self
            .crypto_pwhash_opslimit_interactive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_memlimit_interactive(&self) -> usize {
        (self
            .crypto_pwhash_memlimit_interactive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_opslimit_moderate(&self) -> usize {
        (self
            .crypto_pwhash_opslimit_moderate
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_memlimit_moderate(&self) -> usize {
        (self
            .crypto_pwhash_memlimit_moderate
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_opslimit_sensitive(&self) -> usize {
        (self
            .crypto_pwhash_opslimit_sensitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash_memlimit_sensitive(&self) -> usize {
        (self
            .crypto_pwhash_memlimit_sensitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_pwhash(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        outlen: ::std::os::raw::c_ulonglong,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        salt: *const ::std::os::raw::c_uchar,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash
            .as_ref()
            .expect("Expected function, got error."))(
            out, outlen, passwd, passwdlen, salt, opslimit, memlimit, alg,
        )
    }
    pub unsafe fn crypto_pwhash_str(
        &self,
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_str
            .as_ref()
            .expect("Expected function, got error."))(
            out, passwd, passwdlen, opslimit, memlimit
        )
    }
    pub unsafe fn crypto_pwhash_str_alg(
        &self,
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
        alg: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_str_alg
            .as_ref()
            .expect("Expected function, got error."))(
            out, passwd, passwdlen, opslimit, memlimit, alg,
        )
    }
    pub unsafe fn crypto_pwhash_str_verify(
        &self,
        str_: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_str_verify
            .as_ref()
            .expect("Expected function, got error."))(str_, passwd, passwdlen)
    }
    pub unsafe fn crypto_pwhash_str_needs_rehash(
        &self,
        str_: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_pwhash_str_needs_rehash
            .as_ref()
            .expect("Expected function, got error."))(str_, opslimit, memlimit)
    }
    pub unsafe fn crypto_pwhash_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_pwhash_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_keybytes(&self) -> usize {
        (self
            .crypto_secretbox_xsalsa20poly1305_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_noncebytes(&self) -> usize {
        (self
            .crypto_secretbox_xsalsa20poly1305_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_macbytes(&self) -> usize {
        (self
            .crypto_secretbox_xsalsa20poly1305_macbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_messagebytes_max(&self) -> usize {
        (self
            .crypto_secretbox_xsalsa20poly1305_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_xsalsa20poly1305
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_open(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_xsalsa20poly1305_open
            .as_ref()
            .expect("Expected function, got error."))(m, c, clen, n, k)
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_secretbox_xsalsa20poly1305_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_boxzerobytes(&self) -> usize {
        (self
            .crypto_secretbox_xsalsa20poly1305_boxzerobytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xsalsa20poly1305_zerobytes(&self) -> usize {
        (self
            .crypto_secretbox_xsalsa20poly1305_zerobytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_keybytes(&self) -> usize {
        (self
            .crypto_secretbox_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_noncebytes(&self) -> usize {
        (self
            .crypto_secretbox_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_macbytes(&self) -> usize {
        (self
            .crypto_secretbox_macbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_secretbox_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_messagebytes_max(&self) -> usize {
        (self
            .crypto_secretbox_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_easy(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_easy
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_secretbox_open_easy(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_open_easy
            .as_ref()
            .expect("Expected function, got error."))(m, c, clen, n, k)
    }
    pub unsafe fn crypto_secretbox_detached(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_detached
            .as_ref()
            .expect("Expected function, got error."))(c, mac, m, mlen, n, k)
    }
    pub unsafe fn crypto_secretbox_open_detached(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_open_detached
            .as_ref()
            .expect("Expected function, got error."))(m, c, mac, clen, n, k)
    }
    pub unsafe fn crypto_secretbox_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_secretbox_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_secretbox_zerobytes(&self) -> usize {
        (self
            .crypto_secretbox_zerobytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_boxzerobytes(&self) -> usize {
        (self
            .crypto_secretbox_boxzerobytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_secretbox_open(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_open
            .as_ref()
            .expect("Expected function, got error."))(m, c, clen, n, k)
    }
    pub unsafe fn crypto_stream_chacha20_keybytes(&self) -> usize {
        (self
            .crypto_stream_chacha20_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_chacha20_noncebytes(&self) -> usize {
        (self
            .crypto_stream_chacha20_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_chacha20_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_chacha20_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_chacha20(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_chacha20
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_chacha20_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_chacha20_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_chacha20_xor_ic(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_chacha20_xor_ic
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, ic, k)
    }
    pub unsafe fn crypto_stream_chacha20_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_chacha20_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_stream_chacha20_ietf_keybytes(&self) -> usize {
        (self
            .crypto_stream_chacha20_ietf_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_chacha20_ietf_noncebytes(&self) -> usize {
        (self
            .crypto_stream_chacha20_ietf_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_chacha20_ietf_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_chacha20_ietf_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_chacha20_ietf(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_chacha20_ietf
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_chacha20_ietf_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_chacha20_ietf_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_chacha20_ietf_xor_ic(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u32,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_chacha20_ietf_xor_ic
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, ic, k)
    }
    pub unsafe fn crypto_stream_chacha20_ietf_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_chacha20_ietf_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_abytes(&self) -> usize {
        (self
            .crypto_secretstream_xchacha20poly1305_abytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_headerbytes(&self) -> usize {
        (self
            .crypto_secretstream_xchacha20poly1305_headerbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_keybytes(&self) -> usize {
        (self
            .crypto_secretstream_xchacha20poly1305_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_messagebytes_max(&self) -> usize {
        (self
            .crypto_secretstream_xchacha20poly1305_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_message(
        &self,
    ) -> ::std::os::raw::c_uchar {
        (self
            .crypto_secretstream_xchacha20poly1305_tag_message
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_push(&self) -> ::std::os::raw::c_uchar {
        (self
            .crypto_secretstream_xchacha20poly1305_tag_push
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_rekey(
        &self,
    ) -> ::std::os::raw::c_uchar {
        (self
            .crypto_secretstream_xchacha20poly1305_tag_rekey
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_final(
        &self,
    ) -> ::std::os::raw::c_uchar {
        (self
            .crypto_secretstream_xchacha20poly1305_tag_final
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_statebytes(&self) -> usize {
        (self
            .crypto_secretstream_xchacha20poly1305_statebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_keygen(
        &self,
        k: *mut ::std::os::raw::c_uchar,
    ) {
        (self
            .crypto_secretstream_xchacha20poly1305_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_init_push(
        &self,
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        header: *mut ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretstream_xchacha20poly1305_init_push
            .as_ref()
            .expect("Expected function, got error."))(state, header, k)
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_push(
        &self,
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        tag: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretstream_xchacha20poly1305_push
            .as_ref()
            .expect("Expected function, got error."))(
            state, c, clen_p, m, mlen, ad, adlen, tag
        )
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_init_pull(
        &self,
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        header: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretstream_xchacha20poly1305_init_pull
            .as_ref()
            .expect("Expected function, got error."))(state, header, k)
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_pull(
        &self,
        state: *mut crypto_secretstream_xchacha20poly1305_state,
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        tag_p: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretstream_xchacha20poly1305_pull
            .as_ref()
            .expect("Expected function, got error."))(
            state, m, mlen_p, tag_p, c, clen, ad, adlen
        )
    }
    pub unsafe fn crypto_secretstream_xchacha20poly1305_rekey(
        &self,
        state: *mut crypto_secretstream_xchacha20poly1305_state,
    ) {
        (self
            .crypto_secretstream_xchacha20poly1305_rekey
            .as_ref()
            .expect("Expected function, got error."))(state)
    }
    pub unsafe fn crypto_shorthash_siphash24_bytes(&self) -> usize {
        (self
            .crypto_shorthash_siphash24_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_shorthash_siphash24_keybytes(&self) -> usize {
        (self
            .crypto_shorthash_siphash24_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_shorthash_siphash24(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_shorthash_siphash24
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_shorthash_siphashx24_bytes(&self) -> usize {
        (self
            .crypto_shorthash_siphashx24_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_shorthash_siphashx24_keybytes(&self) -> usize {
        (self
            .crypto_shorthash_siphashx24_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_shorthash_siphashx24(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_shorthash_siphashx24
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_shorthash_bytes(&self) -> usize {
        (self
            .crypto_shorthash_bytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_shorthash_keybytes(&self) -> usize {
        (self
            .crypto_shorthash_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_shorthash_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_shorthash_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_shorthash(
        &self,
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_shorthash
            .as_ref()
            .expect("Expected function, got error."))(out, in_, inlen, k)
    }
    pub unsafe fn crypto_shorthash_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_shorthash_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_stream_keybytes(&self) -> usize {
        (self
            .crypto_stream_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_noncebytes(&self) -> usize {
        (self
            .crypto_stream_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_primitive(&self) -> *const ::std::os::raw::c_char {
        (self
            .crypto_stream_primitive
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_stream_salsa20_keybytes(&self) -> usize {
        (self
            .crypto_stream_salsa20_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa20_noncebytes(&self) -> usize {
        (self
            .crypto_stream_salsa20_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa20_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_salsa20_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa20(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_salsa20
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_salsa20_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_salsa20_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_salsa20_xor_ic(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_salsa20_xor_ic
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, ic, k)
    }
    pub unsafe fn crypto_stream_salsa20_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_salsa20_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_stream_xchacha20_keybytes(&self) -> usize {
        (self
            .crypto_stream_xchacha20_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_xchacha20_noncebytes(&self) -> usize {
        (self
            .crypto_stream_xchacha20_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_xchacha20_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_xchacha20_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_xchacha20(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_xchacha20
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_xchacha20_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_xchacha20_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_xchacha20_xor_ic(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        ic: u64,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_xchacha20_xor_ic
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, ic, k)
    }
    pub unsafe fn crypto_stream_xchacha20_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_xchacha20_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_keybytes(&self) -> usize {
        (self
            .crypto_secretbox_xchacha20poly1305_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_noncebytes(&self) -> usize {
        (self
            .crypto_secretbox_xchacha20poly1305_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_macbytes(&self) -> usize {
        (self
            .crypto_secretbox_xchacha20poly1305_macbytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_messagebytes_max(&self) -> usize {
        (self
            .crypto_secretbox_xchacha20poly1305_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_easy(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_xchacha20poly1305_easy
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_open_easy(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_xchacha20poly1305_open_easy
            .as_ref()
            .expect("Expected function, got error."))(m, c, clen, n, k)
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_detached(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        mac: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_xchacha20poly1305_detached
            .as_ref()
            .expect("Expected function, got error."))(c, mac, m, mlen, n, k)
    }
    pub unsafe fn crypto_secretbox_xchacha20poly1305_open_detached(
        &self,
        m: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        mac: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_secretbox_xchacha20poly1305_open_detached
            .as_ref()
            .expect("Expected function, got error."))(m, c, mac, clen, n, k)
    }
    pub unsafe fn crypto_stream_salsa2012_keybytes(&self) -> usize {
        (self
            .crypto_stream_salsa2012_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa2012_noncebytes(&self) -> usize {
        (self
            .crypto_stream_salsa2012_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa2012_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_salsa2012_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa2012(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_salsa2012
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_salsa2012_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_salsa2012_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_salsa2012_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_salsa2012_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
    pub unsafe fn crypto_stream_salsa208_keybytes(&self) -> usize {
        (self
            .crypto_stream_salsa208_keybytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa208_noncebytes(&self) -> usize {
        (self
            .crypto_stream_salsa208_noncebytes
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa208_messagebytes_max(&self) -> usize {
        (self
            .crypto_stream_salsa208_messagebytes_max
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn crypto_stream_salsa208(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_salsa208
            .as_ref()
            .expect("Expected function, got error."))(c, clen, n, k)
    }
    pub unsafe fn crypto_stream_salsa208_xor(
        &self,
        c: *mut ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        n: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int {
        (self
            .crypto_stream_salsa208_xor
            .as_ref()
            .expect("Expected function, got error."))(c, m, mlen, n, k)
    }
    pub unsafe fn crypto_stream_salsa208_keygen(&self, k: *mut ::std::os::raw::c_uchar) {
        (self
            .crypto_stream_salsa208_keygen
            .as_ref()
            .expect("Expected function, got error."))(k)
    }
}

use core::ptr;
use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();
static mut LIB: *const UpstreamLib = ptr::null();

fn upstream_library_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("safe crate must live under the repository root")
        .join("original/src/libsodium/.libs/libsodium.so")
}

unsafe fn open_library(path: &PathBuf) -> Result<UpstreamLib, libloading::Error> {
    #[cfg(all(unix, target_os = "linux"))]
    {
        let flags = libc::RTLD_NOW | libc::RTLD_LOCAL | libc::RTLD_DEEPBIND;
        let library = libloading::os::unix::Library::open(Some(path), flags)?;
        UpstreamLib::from_library(library)
    }

    #[cfg(not(all(unix, target_os = "linux")))]
    {
        UpstreamLib::new(path)
    }
}

pub(crate) unsafe fn load() -> &'static UpstreamLib {
    INIT.call_once(|| {
        let path = upstream_library_path();
        let lib = unsafe { open_library(&path) }.unwrap_or_else(|err| {
            panic!(
                "failed to load upstream libsodium at {}: {err}",
                path.display()
            )
        });
        let lib = Box::new(lib);
        let ptr = Box::into_raw(lib);
        unsafe {
            (*ptr).sodium_init();
            LIB = ptr;
        }
    });

    unsafe {
        LIB.as_ref()
            .expect("upstream libsodium loader did not initialize")
    }
}

impl UpstreamLib {
    pub unsafe fn symbol<T: Copy>(&self, name: &[u8]) -> T {
        let symbol_name = String::from_utf8_lossy(name);
        *self.__library.get::<T>(name).unwrap_or_else(|err| {
            panic!(
                "failed to load upstream symbol {}: {err}",
                symbol_name.trim_end_matches('\0')
            )
        })
    }
}

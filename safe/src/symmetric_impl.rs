use crate::abi::types::*;
use crate::ffi::helpers::{opt_slice, opt_slice_mut, set_errno, static_cstr, write_opt};
use crate::foundation::randombytes::fill_random_bytes;
use argon2::{
    password_hash::{
        phc::PasswordHash as PhcPasswordHash, PasswordHasher as _, PasswordVerifier as _,
    },
    Algorithm as Argon2Algorithm, Argon2, Params as Argon2Params, Version as Argon2Version,
};
use blake2::Blake2bVarCore;
use blake2b_simd::Params as Blake2bParams;
use chacha20::cipher::{KeyIvInit as _, StreamCipher as _, StreamCipherSeek as _};
use chacha20::{ChaCha20, ChaCha20Legacy};
use core::cmp;
use core::ffi::{c_char, c_int};
use core::hash::Hasher;
use core::mem::size_of;
use core::ptr;
use salsa20::{Salsa12, Salsa20, Salsa8, XSalsa20};
use scrypt::Params as ScryptParams;
use sha2::block_api::{compress256, compress512};
use siphasher::sip::SipHasher24;
use siphasher::sip128::{Hasher128 as _, SipHasher24 as SipHasher12824};
use subtle::ConstantTimeEq;

const SHA256_BLOCKBYTES: usize = 64;
const SHA256_BYTES: usize = 32;
const SHA512_BLOCKBYTES: usize = 128;
const SHA512_BYTES: usize = 64;
const HMAC_SHA256_KEYBYTES: usize = 32;
const HMAC_SHA512_KEYBYTES: usize = 32;
const HMAC_SHA512256_BYTES: usize = 32;
const POLY1305_KEYBYTES: usize = 32;
const POLY1305_BYTES: usize = 16;
const BLAKE2B_BYTES_MIN: usize = 16;
const BLAKE2B_BYTES_MAX: usize = 64;
const BLAKE2B_BYTES: usize = 32;
const BLAKE2B_KEYBYTES_MIN: usize = 16;
const BLAKE2B_KEYBYTES_MAX: usize = 64;
const BLAKE2B_KEYBYTES: usize = 32;
const BLAKE2B_SALTBYTES: usize = 16;
const BLAKE2B_PERSONALBYTES: usize = 16;

const SALSA_SIGMA: [u8; 16] = *b"expand 32-byte k";
const CHACHA_SIGMA: [u8; 16] = *b"expand 32-byte k";
const STREAM_KEYBYTES: usize = 32;
const STREAM_NONCEBYTES: usize = 24;
const SECRETBOX_KEYBYTES: usize = 32;
const SECRETBOX_NONCEBYTES: usize = 24;
const SECRETBOX_MACBYTES: usize = 16;
const SECRETBOX_ZEROBYTES: usize = 32;
const SECRETBOX_BOXZEROBYTES: usize = 16;
const SECRETSTREAM_ABYTES: usize = 17;
const SECRETSTREAM_HEADERBYTES: usize = 24;
const SECRETSTREAM_KEYBYTES: usize = 32;
const SECRETSTREAM_STATEBYTES: usize = 52;
const SECRETSTREAM_TAG_MESSAGE: u8 = 0x00;
const SECRETSTREAM_TAG_PUSH: u8 = 0x01;
const SECRETSTREAM_TAG_REKEY: u8 = 0x02;
const SECRETSTREAM_TAG_FINAL: u8 = SECRETSTREAM_TAG_PUSH | SECRETSTREAM_TAG_REKEY;
const SHORT_HASH_KEYBYTES: usize = 16;
const SHORT_HASH_BYTES: usize = 8;
const SHORT_HASH_X_BYTES: usize = 16;
const AEAD_ABYTES: usize = 16;
const AEAD_CHACHA20_NONCEBYTES: usize = 8;
const AEAD_CHACHA20_IETF_NONCEBYTES: usize = 12;
const AEAD_XCHACHA20_NONCEBYTES: usize = 24;
const AES256GCM_KEYBYTES: usize = 32;
const AES256GCM_NONCEBYTES: usize = 12;
const AES256GCM_MESSAGEBYTES_MAX: usize = if usize::BITS >= 37 {
    68_719_476_704usize
} else {
    usize::MAX - AEAD_ABYTES
};
const KDF_BYTES_MIN: usize = 16;
const KDF_BYTES_MAX: usize = 64;
const KDF_CONTEXTBYTES: usize = 8;
const KDF_KEYBYTES: usize = 32;
const PWHASH_STRBYTES: usize = 128;
const PWHASH_SALTBYTES: usize = 16;
const PWHASH_BYTES_MIN: usize = 16;
const PWHASH_BYTES_MAX: usize = 0xffff_ffff;
const PWHASH_PASSWD_MAX: usize = 0xffff_ffff;
const PWHASH_MEMLIMIT_MIN: usize = 8192;
const PWHASH_MEMLIMIT_MAX: usize = if usize::BITS >= 43 {
    4_398_046_510_080usize
} else if usize::BITS >= 32 {
    2_147_483_648usize
} else {
    32_768usize
};
const PWHASH_ARGON2I_OPSLIMIT_MIN: u64 = 3;
const PWHASH_ARGON2ID_OPSLIMIT_MIN: u64 = 1;
const PWHASH_OPSLIMIT_MAX: u64 = 0xffff_ffff;
const PWHASH_ARGON2I_OPSLIMIT_INTERACTIVE: u64 = 4;
const PWHASH_ARGON2I_MEMLIMIT_INTERACTIVE: usize = 33_554_432;
const PWHASH_ARGON2I_OPSLIMIT_MODERATE: u64 = 6;
const PWHASH_ARGON2I_MEMLIMIT_MODERATE: usize = 134_217_728;
const PWHASH_ARGON2I_OPSLIMIT_SENSITIVE: u64 = 8;
const PWHASH_ARGON2I_MEMLIMIT_SENSITIVE: usize = 536_870_912;
const PWHASH_ARGON2ID_OPSLIMIT_INTERACTIVE: u64 = 2;
const PWHASH_ARGON2ID_MEMLIMIT_INTERACTIVE: usize = 67_108_864;
const PWHASH_ARGON2ID_OPSLIMIT_MODERATE: u64 = 3;
const PWHASH_ARGON2ID_MEMLIMIT_MODERATE: usize = 268_435_456;
const PWHASH_ARGON2ID_OPSLIMIT_SENSITIVE: u64 = 4;
const PWHASH_ARGON2ID_MEMLIMIT_SENSITIVE: usize = 1_073_741_824;
const PWHASH_ALG_ARGON2I13: c_int = 1;
const PWHASH_ALG_ARGON2ID13: c_int = 2;
const PWHASH_ALG_DEFAULT: c_int = PWHASH_ALG_ARGON2ID13;
const PWHASH_SCRYPT_BYTES_MIN: usize = 16;
const PWHASH_SCRYPT_BYTES_MAX: usize = if usize::BITS >= 38 {
    0x1fff_ffff_e0usize
} else {
    usize::MAX
};
const PWHASH_SCRYPT_SALTBYTES: usize = 32;
const PWHASH_SCRYPT_STRBYTES: usize = 102;
const PWHASH_SCRYPT_OPSLIMIT_MIN: u64 = 32_768;
const PWHASH_SCRYPT_OPSLIMIT_MAX: u64 = 0xffff_ffff;
const PWHASH_SCRYPT_MEMLIMIT_MIN: usize = 16_777_216;
const PWHASH_SCRYPT_MEMLIMIT_MAX: usize = if usize::BITS >= 37 {
    68_719_476_736usize
} else {
    usize::MAX
};
const PWHASH_SCRYPT_OPSLIMIT_INTERACTIVE: u64 = 524_288;
const PWHASH_SCRYPT_MEMLIMIT_INTERACTIVE: usize = 16_777_216;
const PWHASH_SCRYPT_OPSLIMIT_SENSITIVE: u64 = 33_554_432;
const PWHASH_SCRYPT_MEMLIMIT_SENSITIVE: usize = 1_073_741_824;
const PWHASH_SCRYPT_STRSETTINGBYTES: usize = 57;
const PWHASH_SCRYPT_STRSALTBYTES: usize = 32;
const PWHASH_SCRYPT_STRHASHBYTES: usize = 32;
const SCRYPT_ITOA64: &[u8; 64] =
    b"./0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

unsafe extern "C" {
    fn sodium_runtime_has_aesni() -> c_int;
    fn sodium_runtime_has_pclmul() -> c_int;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Sha256StateRepr {
    state: [u32; 8],
    count: u64,
    buf: [u8; SHA256_BLOCKBYTES],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Sha512StateRepr {
    state: [u64; 8],
    count: [u64; 2],
    buf: [u8; SHA512_BLOCKBYTES],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct HmacSha256StateRepr {
    ictx: Sha256StateRepr,
    octx: Sha256StateRepr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct HmacSha512StateRepr {
    ictx: Sha512StateRepr,
    octx: Sha512StateRepr,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Poly1305StateRepr {
    r: [u32; 5],
    h: [u32; 5],
    pad: [u32; 4],
    leftover: u64,
    buffer: [u8; POLY1305_BYTES],
    final_: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SecretstreamStateRepr {
    k: [u8; SECRETSTREAM_KEYBYTES],
    nonce: [u8; 12],
    pad: [u8; 8],
}

type Blake2bCoreBuffer = blake2::digest::block_api::Buffer<Blake2bVarCore>;

#[repr(C)]
// Use the public variable-output core so finalization can safely materialize
// the full 64-byte BLAKE2b block even when the incremental state was initialized
// for a shorter default digest, matching libsodium's ABI contract.
struct Blake2bStateRepr {
    core: Blake2bVarCore,
    buffer: Blake2bCoreBuffer,
    finalized: u8,
}

const _: () =
    assert!(size_of::<Blake2bStateRepr>() < size_of::<crypto_generichash_blake2b_state>());
const _: () =
    assert!(size_of::<Poly1305StateRepr>() <= size_of::<crypto_onetimeauth_poly1305_state>());

fn len_to_usize(len: u64) -> usize {
    usize::try_from(len).unwrap_or_else(|_| crate::foundation::core::sodium_misuse())
}

fn ct_eq(lhs: &[u8], rhs: &[u8]) -> bool {
    lhs.len() == rhs.len() && lhs.ct_eq(rhs).unwrap_u8() == 1
}

fn ranges_overlap(dst: *mut u8, src: *const u8, len: usize) -> bool {
    if len == 0 {
        return false;
    }
    let dst_start = dst as usize;
    let src_start = src as usize;
    let dst_end = dst_start.saturating_add(len);
    let src_end = src_start.saturating_add(len);
    dst_start < src_end && src_start < dst_end
}

unsafe fn copy_or_in_place(dst: *mut u8, src: *const u8, len: usize) -> &'static mut [u8] {
    let dst_slice = opt_slice_mut(dst, len);
    if len == 0 {
        return dst_slice;
    }
    if ptr::eq(dst.cast_const(), src) {
        return dst_slice;
    }
    let src_slice = opt_slice(src, len);
    if ranges_overlap(dst, src, len) {
        let tmp = src_slice.to_vec();
        dst_slice.copy_from_slice(&tmp);
    } else {
        dst_slice.copy_from_slice(src_slice);
    }
    dst_slice
}

unsafe fn write_hash_output<const N: usize>(out: *mut u8, bytes: [u8; N]) -> c_int {
    opt_slice_mut(out, N).copy_from_slice(&bytes);
    0
}

unsafe fn write_c_string(out: *mut c_char, cap: usize, s: &str) -> c_int {
    if s.len() + 1 > cap {
        return -1;
    }
    let out = opt_slice_mut(out.cast::<u8>(), cap);
    out.fill(0);
    out[..s.len()].copy_from_slice(s.as_bytes());
    0
}

unsafe fn read_c_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
}

unsafe fn c_strnlen(ptr: *const c_char, max_len: usize) -> usize {
    if ptr.is_null() {
        return 0;
    }
    let bytes = ptr.cast::<u8>();
    for i in 0..max_len {
        if *bytes.add(i) == 0 {
            return i;
        }
    }
    max_len
}

unsafe fn zero_output(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len != 0 {
        opt_slice_mut(ptr, len).fill(0);
    }
}

unsafe fn zero_char_output(ptr: *mut c_char, len: usize) {
    zero_output(ptr.cast::<u8>(), len);
}

unsafe fn sha256_state(state: *mut crypto_hash_sha256_state) -> &'static mut Sha256StateRepr {
    &mut *(state.cast::<Sha256StateRepr>())
}

unsafe fn sha512_state(state: *mut crypto_hash_sha512_state) -> &'static mut Sha512StateRepr {
    &mut *(state.cast::<Sha512StateRepr>())
}

unsafe fn hmac_sha256_state(
    state: *mut crypto_auth_hmacsha256_state,
) -> &'static mut HmacSha256StateRepr {
    &mut *(state.cast::<HmacSha256StateRepr>())
}

unsafe fn hmac_sha512_state(
    state: *mut crypto_auth_hmacsha512_state,
) -> &'static mut HmacSha512StateRepr {
    &mut *(state.cast::<HmacSha512StateRepr>())
}

unsafe fn blake2b_state(
    state: *mut crypto_generichash_blake2b_state,
) -> &'static mut Blake2bStateRepr {
    &mut *(state.cast::<Blake2bStateRepr>())
}

unsafe fn blake2b_state_finalized(state: *mut crypto_generichash_blake2b_state) -> bool {
    blake2b_state(state).finalized != 0
}

unsafe fn poly1305_state(
    state: *mut crypto_onetimeauth_poly1305_state,
) -> &'static mut Poly1305StateRepr {
    &mut *(state.cast::<Poly1305StateRepr>())
}

unsafe fn secretstream_state(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
) -> &'static mut SecretstreamStateRepr {
    &mut *(state.cast::<SecretstreamStateRepr>())
}

fn sha256_init_repr() -> Sha256StateRepr {
    Sha256StateRepr {
        state: [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
            0x5be0cd19,
        ],
        count: 0,
        buf: [0; SHA256_BLOCKBYTES],
    }
}

fn sha512_init_repr() -> Sha512StateRepr {
    Sha512StateRepr {
        state: [
            0x6a09e667f3bcc908,
            0xbb67ae8584caa73b,
            0x3c6ef372fe94f82b,
            0xa54ff53a5f1d36f1,
            0x510e527fade682d1,
            0x9b05688c2b3e6c1f,
            0x1f83d9abfb41bd6b,
            0x5be0cd19137e2179,
        ],
        count: [0, 0],
        buf: [0; SHA512_BLOCKBYTES],
    }
}

fn sha256_update_repr(state: &mut Sha256StateRepr, mut input: &[u8]) {
    let used = (state.count as usize) & (SHA256_BLOCKBYTES - 1);
    state.count = state.count.wrapping_add(input.len() as u64);

    if used != 0 {
        let take = cmp::min(SHA256_BLOCKBYTES - used, input.len());
        state.buf[used..used + take].copy_from_slice(&input[..take]);
        input = &input[take..];
        if used + take == SHA256_BLOCKBYTES {
            compress256(&mut state.state, &[state.buf]);
        } else {
            return;
        }
    }

    if input.len() >= SHA256_BLOCKBYTES {
        let blocks_len = input.len() & !(SHA256_BLOCKBYTES - 1);
        let blocks = unsafe {
            core::slice::from_raw_parts(
                input.as_ptr().cast::<[u8; SHA256_BLOCKBYTES]>(),
                blocks_len / SHA256_BLOCKBYTES,
            )
        };
        compress256(&mut state.state, blocks);
        input = &input[blocks_len..];
    }

    if !input.is_empty() {
        state.buf[..input.len()].copy_from_slice(input);
    }
}

fn sha256_final_repr(state: &mut Sha256StateRepr) -> [u8; SHA256_BYTES] {
    let mut block = [0u8; SHA256_BLOCKBYTES];
    let used = (state.count as usize) & (SHA256_BLOCKBYTES - 1);
    block[..used].copy_from_slice(&state.buf[..used]);
    block[used] = 0x80;
    if used >= 56 {
        compress256(&mut state.state, &[block]);
        block = [0; SHA256_BLOCKBYTES];
    }
    let bit_len = state.count.wrapping_mul(8);
    block[56..].copy_from_slice(&bit_len.to_be_bytes());
    compress256(&mut state.state, &[block]);

    let mut out = [0u8; SHA256_BYTES];
    for (chunk, word) in out.chunks_exact_mut(4).zip(state.state.iter()) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    out
}

fn sha512_count_bytes(state: &Sha512StateRepr) -> u128 {
    u128::from(state.count[0]) | (u128::from(state.count[1]) << 64)
}

fn sha512_set_count(state: &mut Sha512StateRepr, count: u128) {
    state.count[0] = count as u64;
    state.count[1] = (count >> 64) as u64;
}

fn sha512_update_repr(state: &mut Sha512StateRepr, mut input: &[u8]) {
    let prior = sha512_count_bytes(state);
    let used = (prior as usize) & (SHA512_BLOCKBYTES - 1);
    sha512_set_count(state, prior.wrapping_add(input.len() as u128));

    if used != 0 {
        let take = cmp::min(SHA512_BLOCKBYTES - used, input.len());
        state.buf[used..used + take].copy_from_slice(&input[..take]);
        input = &input[take..];
        if used + take == SHA512_BLOCKBYTES {
            compress512(&mut state.state, &[state.buf]);
        } else {
            return;
        }
    }

    if input.len() >= SHA512_BLOCKBYTES {
        let blocks_len = input.len() & !(SHA512_BLOCKBYTES - 1);
        let blocks = unsafe {
            core::slice::from_raw_parts(
                input.as_ptr().cast::<[u8; SHA512_BLOCKBYTES]>(),
                blocks_len / SHA512_BLOCKBYTES,
            )
        };
        compress512(&mut state.state, blocks);
        input = &input[blocks_len..];
    }

    if !input.is_empty() {
        state.buf[..input.len()].copy_from_slice(input);
    }
}

fn sha512_final_repr(state: &mut Sha512StateRepr) -> [u8; SHA512_BYTES] {
    let mut block = [0u8; SHA512_BLOCKBYTES];
    let count = sha512_count_bytes(state);
    let used = (count as usize) & (SHA512_BLOCKBYTES - 1);
    block[..used].copy_from_slice(&state.buf[..used]);
    block[used] = 0x80;
    if used >= 112 {
        compress512(&mut state.state, &[block]);
        block = [0; SHA512_BLOCKBYTES];
    }
    let bit_len = count.wrapping_mul(8);
    block[112..120].copy_from_slice(&((bit_len >> 64) as u64).to_be_bytes());
    block[120..128].copy_from_slice(&(bit_len as u64).to_be_bytes());
    compress512(&mut state.state, &[block]);

    let mut out = [0u8; SHA512_BYTES];
    for (chunk, word) in out.chunks_exact_mut(8).zip(state.state.iter()) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    out
}

fn hmac_sha256_init_repr(key: &[u8]) -> HmacSha256StateRepr {
    let mut block = [0u8; SHA256_BLOCKBYTES];
    if key.len() > SHA256_BLOCKBYTES {
        let mut tmp = sha256_init_repr();
        sha256_update_repr(&mut tmp, key);
        block[..SHA256_BYTES].copy_from_slice(&sha256_final_repr(&mut tmp));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; SHA256_BLOCKBYTES];
    let mut opad = [0x5cu8; SHA256_BLOCKBYTES];
    for i in 0..SHA256_BLOCKBYTES {
        ipad[i] ^= block[i];
        opad[i] ^= block[i];
    }
    let mut ictx = sha256_init_repr();
    let mut octx = sha256_init_repr();
    sha256_update_repr(&mut ictx, &ipad);
    sha256_update_repr(&mut octx, &opad);
    HmacSha256StateRepr { ictx, octx }
}

fn hmac_sha256_final_repr(state: &HmacSha256StateRepr) -> [u8; SHA256_BYTES] {
    let mut inner = state.ictx;
    let inner_hash = sha256_final_repr(&mut inner);
    let mut outer = state.octx;
    sha256_update_repr(&mut outer, &inner_hash);
    sha256_final_repr(&mut outer)
}

fn hmac_sha512_init_repr(key: &[u8]) -> HmacSha512StateRepr {
    let mut block = [0u8; SHA512_BLOCKBYTES];
    if key.len() > SHA512_BLOCKBYTES {
        let mut tmp = sha512_init_repr();
        sha512_update_repr(&mut tmp, key);
        block[..SHA512_BYTES].copy_from_slice(&sha512_final_repr(&mut tmp));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; SHA512_BLOCKBYTES];
    let mut opad = [0x5cu8; SHA512_BLOCKBYTES];
    for i in 0..SHA512_BLOCKBYTES {
        ipad[i] ^= block[i];
        opad[i] ^= block[i];
    }
    let mut ictx = sha512_init_repr();
    let mut octx = sha512_init_repr();
    sha512_update_repr(&mut ictx, &ipad);
    sha512_update_repr(&mut octx, &opad);
    HmacSha512StateRepr { ictx, octx }
}

fn hmac_sha512_final_repr(state: &HmacSha512StateRepr) -> [u8; SHA512_BYTES] {
    let mut inner = state.ictx;
    let inner_hash = sha512_final_repr(&mut inner);
    let mut outer = state.octx;
    sha512_update_repr(&mut outer, &inner_hash);
    sha512_final_repr(&mut outer)
}

fn load_u32_le(input: &[u8]) -> u32 {
    u32::from_le_bytes(input[..4].try_into().unwrap())
}

fn poly1305_init_repr(key: &[u8; POLY1305_KEYBYTES]) -> Poly1305StateRepr {
    Poly1305StateRepr {
        r: [
            load_u32_le(&key[0..4]) & 0x3ffffff,
            (load_u32_le(&key[3..7]) >> 2) & 0x3ffff03,
            (load_u32_le(&key[6..10]) >> 4) & 0x3ffc0ff,
            (load_u32_le(&key[9..13]) >> 6) & 0x3f03fff,
            (load_u32_le(&key[12..16]) >> 8) & 0x00fffff,
        ],
        h: [0; 5],
        pad: [
            load_u32_le(&key[16..20]),
            load_u32_le(&key[20..24]),
            load_u32_le(&key[24..28]),
            load_u32_le(&key[28..32]),
        ],
        leftover: 0,
        buffer: [0; POLY1305_BYTES],
        final_: 0,
    }
}

fn poly1305_blocks_repr(state: &mut Poly1305StateRepr, mut input: &[u8]) {
    let hibit = if state.final_ != 0 { 0u64 } else { 1u64 << 24 };
    let r0 = state.r[0] as u64;
    let r1 = state.r[1] as u64;
    let r2 = state.r[2] as u64;
    let r3 = state.r[3] as u64;
    let r4 = state.r[4] as u64;
    let s1 = r1 * 5;
    let s2 = r2 * 5;
    let s3 = r3 * 5;
    let s4 = r4 * 5;
    let mut h0 = state.h[0] as u64;
    let mut h1 = state.h[1] as u64;
    let mut h2 = state.h[2] as u64;
    let mut h3 = state.h[3] as u64;
    let mut h4 = state.h[4] as u64;

    while input.len() >= POLY1305_BYTES {
        h0 += (load_u32_le(&input[0..4]) as u64) & 0x3ffffff;
        h1 += ((load_u32_le(&input[3..7]) >> 2) as u64) & 0x3ffffff;
        h2 += ((load_u32_le(&input[6..10]) >> 4) as u64) & 0x3ffffff;
        h3 += ((load_u32_le(&input[9..13]) >> 6) as u64) & 0x3ffffff;
        h4 += ((load_u32_le(&input[12..16]) >> 8) as u64) | hibit;

        let d0 = (h0 * r0) + (h1 * s4) + (h2 * s3) + (h3 * s2) + (h4 * s1);
        let d1 = (h0 * r1) + (h1 * r0) + (h2 * s4) + (h3 * s3) + (h4 * s2);
        let d2 = (h0 * r2) + (h1 * r1) + (h2 * r0) + (h3 * s4) + (h4 * s3);
        let d3 = (h0 * r3) + (h1 * r2) + (h2 * r1) + (h3 * r0) + (h4 * s4);
        let d4 = (h0 * r4) + (h1 * r3) + (h2 * r2) + (h3 * r1) + (h4 * r0);

        let mut c = d0 >> 26;
        h0 = d0 & 0x3ffffff;
        let d1 = d1 + c;
        c = d1 >> 26;
        h1 = d1 & 0x3ffffff;
        let d2 = d2 + c;
        c = d2 >> 26;
        h2 = d2 & 0x3ffffff;
        let d3 = d3 + c;
        c = d3 >> 26;
        h3 = d3 & 0x3ffffff;
        let d4 = d4 + c;
        c = d4 >> 26;
        h4 = d4 & 0x3ffffff;
        h0 += c * 5;
        c = h0 >> 26;
        h0 &= 0x3ffffff;
        h1 += c;

        input = &input[POLY1305_BYTES..];
    }

    state.h = [h0 as u32, h1 as u32, h2 as u32, h3 as u32, h4 as u32];
}

fn poly1305_update_repr(state: &mut Poly1305StateRepr, mut input: &[u8]) {
    if state.leftover != 0 {
        let used = state.leftover as usize;
        let take = cmp::min(POLY1305_BYTES - used, input.len());
        state.buffer[used..used + take].copy_from_slice(&input[..take]);
        state.leftover += take as u64;
        input = &input[take..];
        if state.leftover as usize == POLY1305_BYTES {
            let block = state.buffer;
            poly1305_blocks_repr(state, &block);
            state.leftover = 0;
        } else {
            return;
        }
    }

    if input.len() >= POLY1305_BYTES {
        let want = input.len() & !(POLY1305_BYTES - 1);
        poly1305_blocks_repr(state, &input[..want]);
        input = &input[want..];
    }

    if !input.is_empty() {
        state.buffer[..input.len()].copy_from_slice(input);
        state.leftover = input.len() as u64;
    }
}

fn poly1305_finalize_repr(state: &mut Poly1305StateRepr) -> [u8; POLY1305_BYTES] {
    if state.leftover != 0 {
        let mut i = state.leftover as usize;
        state.buffer[i] = 1;
        i += 1;
        state.buffer[i..].fill(0);
        state.final_ = 1;
        let block = state.buffer;
        poly1305_blocks_repr(state, &block);
    }

    let mut h0 = state.h[0] as u64;
    let mut h1 = state.h[1] as u64;
    let mut h2 = state.h[2] as u64;
    let mut h3 = state.h[3] as u64;
    let mut h4 = state.h[4] as u64;

    let mut c = h1 >> 26;
    h1 &= 0x3ffffff;
    h2 += c;
    c = h2 >> 26;
    h2 &= 0x3ffffff;
    h3 += c;
    c = h3 >> 26;
    h3 &= 0x3ffffff;
    h4 += c;
    c = h4 >> 26;
    h4 &= 0x3ffffff;
    h0 += c * 5;
    c = h0 >> 26;
    h0 &= 0x3ffffff;
    h1 += c;

    let mut g0 = h0 + 5;
    c = g0 >> 26;
    g0 &= 0x3ffffff;
    let mut g1 = h1 + c;
    c = g1 >> 26;
    g1 &= 0x3ffffff;
    let mut g2 = h2 + c;
    c = g2 >> 26;
    g2 &= 0x3ffffff;
    let mut g3 = h3 + c;
    c = g3 >> 26;
    g3 &= 0x3ffffff;
    let mut g4 = h4 + c;
    g4 = g4.wrapping_sub(1 << 26);

    let mask = (g4 >> 63).wrapping_sub(1);
    g0 &= mask;
    g1 &= mask;
    g2 &= mask;
    g3 &= mask;
    g4 &= mask;
    let not_mask = !mask;
    h0 = (h0 & not_mask) | g0;
    h1 = (h1 & not_mask) | g1;
    h2 = (h2 & not_mask) | g2;
    h3 = (h3 & not_mask) | g3;
    h4 = (h4 & not_mask) | g4;

    h0 = ((h0) | (h1 << 26)) & 0xffff_ffff;
    h1 = ((h1 >> 6) | (h2 << 20)) & 0xffff_ffff;
    h2 = ((h2 >> 12) | (h3 << 14)) & 0xffff_ffff;
    h3 = ((h3 >> 18) | (h4 << 8)) & 0xffff_ffff;

    let mut f = h0 + state.pad[0] as u64;
    h0 = f & 0xffff_ffff;
    f = h1 + state.pad[1] as u64 + (f >> 32);
    h1 = f & 0xffff_ffff;
    f = h2 + state.pad[2] as u64 + (f >> 32);
    h2 = f & 0xffff_ffff;
    f = h3 + state.pad[3] as u64 + (f >> 32);
    h3 = f & 0xffff_ffff;

    let mut out = [0u8; POLY1305_BYTES];
    out[0..4].copy_from_slice(&(h0 as u32).to_le_bytes());
    out[4..8].copy_from_slice(&(h1 as u32).to_le_bytes());
    out[8..12].copy_from_slice(&(h2 as u32).to_le_bytes());
    out[12..16].copy_from_slice(&(h3 as u32).to_le_bytes());
    *state = Poly1305StateRepr::default();
    out
}

fn random_keygen(k: *mut u8, len: usize) {
    if len != 0 {
        fill_random_bytes(k, len);
    }
}

fn quarter_round_chacha(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(12);
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(7);
}

fn quarter_round_salsa(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    state[b] ^= state[a].wrapping_add(state[d]).rotate_left(7);
    state[c] ^= state[b].wrapping_add(state[a]).rotate_left(9);
    state[d] ^= state[c].wrapping_add(state[b]).rotate_left(13);
    state[a] ^= state[d].wrapping_add(state[c]).rotate_left(18);
}

fn salsa_core_rounds(
    rounds: usize,
    out: &mut [u8; 64],
    input: &[u8; 16],
    key: &[u8; 32],
    c: &[u8; 16],
) {
    let mut x = [0u32; 16];
    x[0] = u32::from_le_bytes(c[0..4].try_into().unwrap());
    x[5] = u32::from_le_bytes(c[4..8].try_into().unwrap());
    x[10] = u32::from_le_bytes(c[8..12].try_into().unwrap());
    x[15] = u32::from_le_bytes(c[12..16].try_into().unwrap());
    for (i, chunk) in key[..16].chunks_exact(4).enumerate() {
        x[1 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for (i, chunk) in input[..8].chunks_exact(4).enumerate() {
        x[6 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for (i, chunk) in input[8..].chunks_exact(4).enumerate() {
        x[8 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for (i, chunk) in key[16..].chunks_exact(4).enumerate() {
        x[11 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    let initial = x;
    for _ in 0..(rounds / 2) {
        quarter_round_salsa(&mut x, 0, 4, 8, 12);
        quarter_round_salsa(&mut x, 5, 9, 13, 1);
        quarter_round_salsa(&mut x, 10, 14, 2, 6);
        quarter_round_salsa(&mut x, 15, 3, 7, 11);
        quarter_round_salsa(&mut x, 0, 1, 2, 3);
        quarter_round_salsa(&mut x, 5, 6, 7, 4);
        quarter_round_salsa(&mut x, 10, 11, 8, 9);
        quarter_round_salsa(&mut x, 15, 12, 13, 14);
    }
    for i in 0..16 {
        x[i] = x[i].wrapping_add(initial[i]);
    }
    for (chunk, word) in out.chunks_exact_mut(4).zip(x.iter()) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
}

fn hsalsa20_core(input: &[u8; 16], key: &[u8; 32], c: &[u8; 16]) -> [u8; 32] {
    let mut x = [0u32; 16];
    x[0] = u32::from_le_bytes(c[0..4].try_into().unwrap());
    x[5] = u32::from_le_bytes(c[4..8].try_into().unwrap());
    x[10] = u32::from_le_bytes(c[8..12].try_into().unwrap());
    x[15] = u32::from_le_bytes(c[12..16].try_into().unwrap());
    for (i, chunk) in key[..16].chunks_exact(4).enumerate() {
        x[1 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for (i, chunk) in input[..].chunks_exact(4).enumerate() {
        x[6 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for (i, chunk) in key[16..].chunks_exact(4).enumerate() {
        x[11 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for _ in 0..10 {
        quarter_round_salsa(&mut x, 0, 4, 8, 12);
        quarter_round_salsa(&mut x, 5, 9, 13, 1);
        quarter_round_salsa(&mut x, 10, 14, 2, 6);
        quarter_round_salsa(&mut x, 15, 3, 7, 11);
        quarter_round_salsa(&mut x, 0, 1, 2, 3);
        quarter_round_salsa(&mut x, 5, 6, 7, 4);
        quarter_round_salsa(&mut x, 10, 11, 8, 9);
        quarter_round_salsa(&mut x, 15, 12, 13, 14);
    }
    let mut out = [0u8; 32];
    let words = [x[0], x[5], x[10], x[15], x[6], x[7], x[8], x[9]];
    for (chunk, word) in out.chunks_exact_mut(4).zip(words.iter()) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out
}

fn hchacha20_core(input: &[u8; 16], key: &[u8; 32], c: &[u8; 16]) -> [u8; 32] {
    let mut x = [0u32; 16];
    for (i, chunk) in c.chunks_exact(4).enumerate() {
        x[i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for (i, chunk) in key.chunks_exact(4).enumerate() {
        x[4 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for (i, chunk) in input.chunks_exact(4).enumerate() {
        x[12 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    for _ in 0..10 {
        quarter_round_chacha(&mut x, 0, 4, 8, 12);
        quarter_round_chacha(&mut x, 1, 5, 9, 13);
        quarter_round_chacha(&mut x, 2, 6, 10, 14);
        quarter_round_chacha(&mut x, 3, 7, 11, 15);
        quarter_round_chacha(&mut x, 0, 5, 10, 15);
        quarter_round_chacha(&mut x, 1, 6, 11, 12);
        quarter_round_chacha(&mut x, 2, 7, 8, 13);
        quarter_round_chacha(&mut x, 3, 4, 9, 14);
    }
    let words = [x[0], x[1], x[2], x[3], x[12], x[13], x[14], x[15]];
    let mut out = [0u8; 32];
    for (chunk, word) in out.chunks_exact_mut(4).zip(words.iter()) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out
}

pub unsafe fn crypto_hash(out: *mut u8, in_: *const u8, inlen: u64) -> c_int {
    crypto_hash_sha512(out, in_, inlen)
}

pub unsafe fn crypto_hash_bytes() -> usize {
    SHA512_BYTES
}

pub unsafe fn crypto_hash_primitive() -> *const c_char {
    static_cstr(b"sha512\0")
}

pub unsafe fn crypto_hash_sha256(out: *mut u8, in_: *const u8, inlen: u64) -> c_int {
    let mut state = sha256_init_repr();
    sha256_update_repr(&mut state, opt_slice(in_, len_to_usize(inlen)));
    write_hash_output(out, sha256_final_repr(&mut state))
}

pub unsafe fn crypto_hash_sha256_bytes() -> usize {
    SHA256_BYTES
}

pub unsafe fn crypto_hash_sha256_final(
    state: *mut crypto_hash_sha256_state,
    out: *mut u8,
) -> c_int {
    write_hash_output(out, sha256_final_repr(sha256_state(state)))
}

pub unsafe fn crypto_hash_sha256_init(state: *mut crypto_hash_sha256_state) -> c_int {
    ptr::write(state.cast::<Sha256StateRepr>(), sha256_init_repr());
    0
}

pub unsafe fn crypto_hash_sha256_statebytes() -> usize {
    size_of::<crypto_hash_sha256_state>()
}

pub unsafe fn crypto_hash_sha256_update(
    state: *mut crypto_hash_sha256_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    sha256_update_repr(sha256_state(state), opt_slice(in_, len_to_usize(inlen)));
    0
}

pub unsafe fn crypto_hash_sha512(out: *mut u8, in_: *const u8, inlen: u64) -> c_int {
    let mut state = sha512_init_repr();
    sha512_update_repr(&mut state, opt_slice(in_, len_to_usize(inlen)));
    write_hash_output(out, sha512_final_repr(&mut state))
}

pub unsafe fn crypto_hash_sha512_bytes() -> usize {
    SHA512_BYTES
}

pub unsafe fn crypto_hash_sha512_final(
    state: *mut crypto_hash_sha512_state,
    out: *mut u8,
) -> c_int {
    write_hash_output(out, sha512_final_repr(sha512_state(state)))
}

pub unsafe fn crypto_hash_sha512_init(state: *mut crypto_hash_sha512_state) -> c_int {
    ptr::write(state.cast::<Sha512StateRepr>(), sha512_init_repr());
    0
}

pub unsafe fn crypto_hash_sha512_statebytes() -> usize {
    size_of::<crypto_hash_sha512_state>()
}

pub unsafe fn crypto_hash_sha512_update(
    state: *mut crypto_hash_sha512_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    sha512_update_repr(sha512_state(state), opt_slice(in_, len_to_usize(inlen)));
    0
}

pub unsafe fn crypto_auth(out: *mut u8, in_: *const u8, inlen: u64, k: *const u8) -> c_int {
    crypto_auth_hmacsha512256(out, in_, inlen, k)
}

pub unsafe fn crypto_auth_bytes() -> usize {
    HMAC_SHA512256_BYTES
}

pub unsafe fn crypto_auth_keybytes() -> usize {
    HMAC_SHA512_KEYBYTES
}

pub unsafe fn crypto_auth_keygen(k: *mut u8) {
    random_keygen(k, HMAC_SHA512_KEYBYTES);
}

pub unsafe fn crypto_auth_primitive() -> *const c_char {
    static_cstr(b"hmacsha512256\0")
}

pub unsafe fn crypto_auth_verify(h: *const u8, in_: *const u8, inlen: u64, k: *const u8) -> c_int {
    crypto_auth_hmacsha512256_verify(h, in_, inlen, k)
}

pub unsafe fn crypto_auth_hmacsha256(
    out: *mut u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let key = opt_slice(k, HMAC_SHA256_KEYBYTES);
    let state = hmac_sha256_init_repr(key);
    let tag = {
        let mut st = state;
        sha256_update_repr(&mut st.ictx, opt_slice(in_, len_to_usize(inlen)));
        hmac_sha256_final_repr(&st)
    };
    write_hash_output(out, tag)
}

pub unsafe fn crypto_auth_hmacsha256_bytes() -> usize {
    SHA256_BYTES
}

pub unsafe fn crypto_auth_hmacsha256_final(
    state: *mut crypto_auth_hmacsha256_state,
    out: *mut u8,
) -> c_int {
    write_hash_output(out, hmac_sha256_final_repr(hmac_sha256_state(state)))
}

pub unsafe fn crypto_auth_hmacsha256_init(
    state: *mut crypto_auth_hmacsha256_state,
    key: *const u8,
    keylen: usize,
) -> c_int {
    ptr::write(
        state.cast::<HmacSha256StateRepr>(),
        hmac_sha256_init_repr(opt_slice(key, keylen)),
    );
    0
}

pub unsafe fn crypto_auth_hmacsha256_keybytes() -> usize {
    HMAC_SHA256_KEYBYTES
}

pub unsafe fn crypto_auth_hmacsha256_keygen(k: *mut u8) {
    random_keygen(k, HMAC_SHA256_KEYBYTES);
}

pub unsafe fn crypto_auth_hmacsha256_statebytes() -> usize {
    size_of::<crypto_auth_hmacsha256_state>()
}

pub unsafe fn crypto_auth_hmacsha256_update(
    state: *mut crypto_auth_hmacsha256_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    sha256_update_repr(
        &mut hmac_sha256_state(state).ictx,
        opt_slice(in_, len_to_usize(inlen)),
    );
    0
}

pub unsafe fn crypto_auth_hmacsha256_verify(
    h: *const u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let mut expected = [0u8; SHA256_BYTES];
    crypto_auth_hmacsha256(expected.as_mut_ptr(), in_, inlen, k);
    if ct_eq(&expected, opt_slice(h, SHA256_BYTES)) {
        0
    } else {
        -1
    }
}

pub unsafe fn crypto_auth_hmacsha512(
    out: *mut u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let key = opt_slice(k, HMAC_SHA512_KEYBYTES);
    let state = hmac_sha512_init_repr(key);
    let tag = {
        let mut st = state;
        sha512_update_repr(&mut st.ictx, opt_slice(in_, len_to_usize(inlen)));
        hmac_sha512_final_repr(&st)
    };
    write_hash_output(out, tag)
}

pub unsafe fn crypto_auth_hmacsha512_bytes() -> usize {
    SHA512_BYTES
}

pub unsafe fn crypto_auth_hmacsha512_final(
    state: *mut crypto_auth_hmacsha512_state,
    out: *mut u8,
) -> c_int {
    write_hash_output(out, hmac_sha512_final_repr(hmac_sha512_state(state)))
}

pub unsafe fn crypto_auth_hmacsha512_init(
    state: *mut crypto_auth_hmacsha512_state,
    key: *const u8,
    keylen: usize,
) -> c_int {
    ptr::write(
        state.cast::<HmacSha512StateRepr>(),
        hmac_sha512_init_repr(opt_slice(key, keylen)),
    );
    0
}

pub unsafe fn crypto_auth_hmacsha512_keybytes() -> usize {
    HMAC_SHA512_KEYBYTES
}

pub unsafe fn crypto_auth_hmacsha512_keygen(k: *mut u8) {
    random_keygen(k, HMAC_SHA512_KEYBYTES);
}

pub unsafe fn crypto_auth_hmacsha512_statebytes() -> usize {
    size_of::<crypto_auth_hmacsha512_state>()
}

pub unsafe fn crypto_auth_hmacsha512_update(
    state: *mut crypto_auth_hmacsha512_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    sha512_update_repr(
        &mut hmac_sha512_state(state).ictx,
        opt_slice(in_, len_to_usize(inlen)),
    );
    0
}

pub unsafe fn crypto_auth_hmacsha512_verify(
    h: *const u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let mut expected = [0u8; SHA512_BYTES];
    crypto_auth_hmacsha512(expected.as_mut_ptr(), in_, inlen, k);
    if ct_eq(&expected, opt_slice(h, SHA512_BYTES)) {
        0
    } else {
        -1
    }
}

pub unsafe fn crypto_auth_hmacsha512256(
    out: *mut u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let mut full = [0u8; SHA512_BYTES];
    crypto_auth_hmacsha512(full.as_mut_ptr(), in_, inlen, k);
    opt_slice_mut(out, HMAC_SHA512256_BYTES).copy_from_slice(&full[..HMAC_SHA512256_BYTES]);
    0
}

pub unsafe fn crypto_auth_hmacsha512256_bytes() -> usize {
    HMAC_SHA512256_BYTES
}

pub unsafe fn crypto_auth_hmacsha512256_final(
    state: *mut crypto_auth_hmacsha512256_state,
    out: *mut u8,
) -> c_int {
    let full = hmac_sha512_final_repr(hmac_sha512_state(state));
    opt_slice_mut(out, HMAC_SHA512256_BYTES).copy_from_slice(&full[..HMAC_SHA512256_BYTES]);
    0
}

pub unsafe fn crypto_auth_hmacsha512256_init(
    state: *mut crypto_auth_hmacsha512256_state,
    key: *const u8,
    keylen: usize,
) -> c_int {
    crypto_auth_hmacsha512_init(state.cast(), key, keylen)
}

pub unsafe fn crypto_auth_hmacsha512256_keybytes() -> usize {
    HMAC_SHA512_KEYBYTES
}

pub unsafe fn crypto_auth_hmacsha512256_keygen(k: *mut u8) {
    random_keygen(k, HMAC_SHA512_KEYBYTES);
}

pub unsafe fn crypto_auth_hmacsha512256_statebytes() -> usize {
    size_of::<crypto_auth_hmacsha512256_state>()
}

pub unsafe fn crypto_auth_hmacsha512256_update(
    state: *mut crypto_auth_hmacsha512256_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    crypto_auth_hmacsha512_update(state.cast(), in_, inlen)
}

pub unsafe fn crypto_auth_hmacsha512256_verify(
    h: *const u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let mut expected = [0u8; HMAC_SHA512256_BYTES];
    crypto_auth_hmacsha512256(expected.as_mut_ptr(), in_, inlen, k);
    if ct_eq(&expected, opt_slice(h, HMAC_SHA512256_BYTES)) {
        0
    } else {
        -1
    }
}

fn blake2b_params(
    outlen: usize,
    key: Option<&[u8]>,
    salt: Option<&[u8]>,
    personal: Option<&[u8]>,
) -> Option<Blake2bParams> {
    if outlen == 0 || outlen > BLAKE2B_BYTES_MAX {
        return None;
    }
    if let Some(key) = key {
        if key.len() > BLAKE2B_KEYBYTES_MAX {
            return None;
        }
    }
    let mut params = Blake2bParams::new();
    params.hash_length(outlen);
    if let Some(key) = key {
        params.key(key);
    }
    if let Some(salt) = salt {
        if salt.len() != BLAKE2B_SALTBYTES {
            return None;
        }
        params.salt(salt);
    }
    if let Some(personal) = personal {
        if personal.len() != BLAKE2B_PERSONALBYTES {
            return None;
        }
        params.personal(personal);
    }
    Some(params)
}

fn blake2b_state_repr(
    outlen: usize,
    key: Option<&[u8]>,
    salt: Option<&[u8]>,
    personal: Option<&[u8]>,
) -> Option<Blake2bStateRepr> {
    if outlen == 0 || outlen > BLAKE2B_BYTES_MAX {
        return None;
    }
    if let Some(key) = key {
        if key.len() > BLAKE2B_KEYBYTES_MAX {
            return None;
        }
    }
    if let Some(salt) = salt {
        if salt.len() != BLAKE2B_SALTBYTES {
            return None;
        }
    }
    if let Some(personal) = personal {
        if personal.len() != BLAKE2B_PERSONALBYTES {
            return None;
        }
    }

    let key_len = key.map_or(0, <[u8]>::len);
    let core = Blake2bVarCore::new_with_params(
        salt.unwrap_or(&[]),
        personal.unwrap_or(&[]),
        key_len,
        outlen,
    );
    let buffer = if let Some(key) = key {
        let mut padded_key = blake2::digest::block_api::Block::<Blake2bVarCore>::default();
        padded_key[..key.len()].copy_from_slice(key);
        Blake2bCoreBuffer::new(&padded_key)
    } else {
        Blake2bCoreBuffer::default()
    };

    Some(Blake2bStateRepr {
        core,
        buffer,
        finalized: 0,
    })
}

pub unsafe fn crypto_generichash(
    out: *mut u8,
    outlen: usize,
    in_: *const u8,
    inlen: u64,
    key: *const u8,
    keylen: usize,
) -> c_int {
    crypto_generichash_blake2b(out, outlen, in_, inlen, key, keylen)
}

pub unsafe fn crypto_generichash_blake2b(
    out: *mut u8,
    outlen: usize,
    in_: *const u8,
    inlen: u64,
    key: *const u8,
    keylen: usize,
) -> c_int {
    let params = match blake2b_params(
        outlen,
        if key.is_null() || keylen == 0 {
            None
        } else {
            Some(opt_slice(key, keylen))
        },
        None,
        None,
    ) {
        Some(params) => params,
        None => return -1,
    };
    let mut state = params.to_state();
    state.update(opt_slice(in_, len_to_usize(inlen)));
    let hash = state.finalize();
    opt_slice_mut(out, outlen).copy_from_slice(hash.as_bytes());
    0
}

pub unsafe fn crypto_generichash_blake2b_bytes() -> usize {
    BLAKE2B_BYTES
}

pub unsafe fn crypto_generichash_blake2b_bytes_max() -> usize {
    BLAKE2B_BYTES_MAX
}

pub unsafe fn crypto_generichash_blake2b_bytes_min() -> usize {
    BLAKE2B_BYTES_MIN
}

pub unsafe fn crypto_generichash_blake2b_final(
    state: *mut crypto_generichash_blake2b_state,
    out: *mut u8,
    outlen: usize,
) -> c_int {
    if outlen == 0 || outlen > BLAKE2B_BYTES_MAX {
        return -1;
    }
    if blake2b_state_finalized(state) {
        return -1;
    }
    let state = blake2b_state(state);
    let core = &mut state.core;
    let buffer = &mut state.buffer;
    let mut full = blake2::digest::Output::<Blake2bVarCore>::default();
    blake2::digest::block_api::VariableOutputCore::finalize_variable_core(
        core, buffer, &mut full,
    );
    opt_slice_mut(out, outlen).copy_from_slice(&full[..outlen]);
    state.finalized = 1;
    0
}

pub unsafe fn crypto_generichash_blake2b_init(
    state: *mut crypto_generichash_blake2b_state,
    key: *const u8,
    keylen: usize,
    outlen: usize,
) -> c_int {
    crypto_generichash_blake2b_init_salt_personal(
        state,
        key,
        keylen,
        outlen,
        ptr::null(),
        ptr::null(),
    )
}

pub unsafe fn crypto_generichash_blake2b_init_salt_personal(
    state: *mut crypto_generichash_blake2b_state,
    key: *const u8,
    keylen: usize,
    outlen: usize,
    salt: *const u8,
    personal: *const u8,
) -> c_int {
    let state_repr = match blake2b_state_repr(
        outlen,
        if key.is_null() || keylen == 0 {
            None
        } else {
            Some(opt_slice(key, keylen))
        },
        if salt.is_null() {
            None
        } else {
            Some(opt_slice(salt, BLAKE2B_SALTBYTES))
        },
        if personal.is_null() {
            None
        } else {
            Some(opt_slice(personal, BLAKE2B_PERSONALBYTES))
        },
    ) {
        Some(state_repr) => state_repr,
        None => return -1,
    };
    ptr::write_bytes(
        state.cast::<u8>(),
        0,
        size_of::<crypto_generichash_blake2b_state>(),
    );
    ptr::write(state.cast::<Blake2bStateRepr>(), state_repr);
    0
}

pub unsafe fn crypto_generichash_blake2b_keybytes() -> usize {
    BLAKE2B_KEYBYTES
}

pub unsafe fn crypto_generichash_blake2b_keybytes_max() -> usize {
    BLAKE2B_KEYBYTES_MAX
}

pub unsafe fn crypto_generichash_blake2b_keybytes_min() -> usize {
    BLAKE2B_KEYBYTES_MIN
}

pub unsafe fn crypto_generichash_blake2b_keygen(k: *mut u8) {
    random_keygen(k, BLAKE2B_KEYBYTES);
}

pub unsafe fn crypto_generichash_blake2b_personalbytes() -> usize {
    BLAKE2B_PERSONALBYTES
}

pub unsafe fn crypto_generichash_blake2b_salt_personal(
    out: *mut u8,
    outlen: usize,
    in_: *const u8,
    inlen: u64,
    key: *const u8,
    keylen: usize,
    salt: *const u8,
    personal: *const u8,
) -> c_int {
    let params = match blake2b_params(
        outlen,
        if key.is_null() || keylen == 0 {
            None
        } else {
            Some(opt_slice(key, keylen))
        },
        if salt.is_null() {
            None
        } else {
            Some(opt_slice(salt, BLAKE2B_SALTBYTES))
        },
        if personal.is_null() {
            None
        } else {
            Some(opt_slice(personal, BLAKE2B_PERSONALBYTES))
        },
    ) {
        Some(params) => params,
        None => return -1,
    };
    let mut state = params.to_state();
    state.update(opt_slice(in_, len_to_usize(inlen)));
    let hash = state.finalize();
    opt_slice_mut(out, outlen).copy_from_slice(hash.as_bytes());
    0
}

pub unsafe fn crypto_generichash_blake2b_saltbytes() -> usize {
    BLAKE2B_SALTBYTES
}

pub unsafe fn crypto_generichash_blake2b_statebytes() -> usize {
    size_of::<crypto_generichash_blake2b_state>()
}

pub unsafe fn crypto_generichash_blake2b_update(
    state: *mut crypto_generichash_blake2b_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    let state = blake2b_state(state);
    let core = &mut state.core;
    let buffer = &mut state.buffer;
    buffer.digest_blocks(opt_slice(in_, len_to_usize(inlen)), |blocks| {
        blake2::digest::block_api::UpdateCore::update_blocks(core, blocks);
    });
    0
}

pub unsafe fn crypto_generichash_bytes() -> usize {
    BLAKE2B_BYTES
}

pub unsafe fn crypto_generichash_bytes_max() -> usize {
    BLAKE2B_BYTES_MAX
}

pub unsafe fn crypto_generichash_bytes_min() -> usize {
    BLAKE2B_BYTES_MIN
}

pub unsafe fn crypto_generichash_final(
    state: *mut crypto_generichash_state,
    out: *mut u8,
    outlen: usize,
) -> c_int {
    crypto_generichash_blake2b_final(state.cast(), out, outlen)
}

pub unsafe fn crypto_generichash_init(
    state: *mut crypto_generichash_state,
    key: *const u8,
    keylen: usize,
    outlen: usize,
) -> c_int {
    crypto_generichash_blake2b_init(state.cast(), key, keylen, outlen)
}

pub unsafe fn crypto_generichash_keybytes() -> usize {
    BLAKE2B_KEYBYTES
}

pub unsafe fn crypto_generichash_keybytes_max() -> usize {
    BLAKE2B_KEYBYTES_MAX
}

pub unsafe fn crypto_generichash_keybytes_min() -> usize {
    BLAKE2B_KEYBYTES_MIN
}

pub unsafe fn crypto_generichash_keygen(k: *mut u8) {
    random_keygen(k, BLAKE2B_KEYBYTES);
}

pub unsafe fn crypto_generichash_primitive() -> *const c_char {
    static_cstr(b"blake2b\0")
}

pub unsafe fn crypto_generichash_statebytes() -> usize {
    size_of::<crypto_generichash_state>()
}

pub unsafe fn crypto_generichash_update(
    state: *mut crypto_generichash_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    crypto_generichash_blake2b_update(state.cast(), in_, inlen)
}

pub unsafe fn crypto_onetimeauth(out: *mut u8, in_: *const u8, inlen: u64, k: *const u8) -> c_int {
    crypto_onetimeauth_poly1305(out, in_, inlen, k)
}

pub unsafe fn crypto_onetimeauth_bytes() -> usize {
    POLY1305_BYTES
}

pub unsafe fn crypto_onetimeauth_final(
    state: *mut crypto_onetimeauth_state,
    out: *mut u8,
) -> c_int {
    crypto_onetimeauth_poly1305_final(state.cast(), out)
}

pub unsafe fn crypto_onetimeauth_init(
    state: *mut crypto_onetimeauth_state,
    key: *const u8,
) -> c_int {
    crypto_onetimeauth_poly1305_init(state.cast(), key)
}

pub unsafe fn crypto_onetimeauth_keybytes() -> usize {
    POLY1305_KEYBYTES
}

pub unsafe fn crypto_onetimeauth_keygen(k: *mut u8) {
    random_keygen(k, POLY1305_KEYBYTES);
}

pub unsafe fn crypto_onetimeauth_primitive() -> *const c_char {
    static_cstr(b"poly1305\0")
}

pub unsafe fn crypto_onetimeauth_statebytes() -> usize {
    size_of::<crypto_onetimeauth_state>()
}

pub unsafe fn crypto_onetimeauth_update(
    state: *mut crypto_onetimeauth_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    crypto_onetimeauth_poly1305_update(state.cast(), in_, inlen)
}

pub unsafe fn crypto_onetimeauth_verify(
    h: *const u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    crypto_onetimeauth_poly1305_verify(h, in_, inlen, k)
}

pub unsafe fn crypto_onetimeauth_poly1305(
    out: *mut u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let key: [u8; POLY1305_KEYBYTES] = opt_slice(k, POLY1305_KEYBYTES).try_into().unwrap();
    let mut state = poly1305_init_repr(&key);
    poly1305_update_repr(&mut state, opt_slice(in_, len_to_usize(inlen)));
    opt_slice_mut(out, POLY1305_BYTES).copy_from_slice(&poly1305_finalize_repr(&mut state));
    0
}

pub unsafe fn crypto_onetimeauth_poly1305_bytes() -> usize {
    POLY1305_BYTES
}

pub unsafe fn crypto_onetimeauth_poly1305_final(
    state: *mut crypto_onetimeauth_poly1305_state,
    out: *mut u8,
) -> c_int {
    let tag = poly1305_finalize_repr(poly1305_state(state));
    opt_slice_mut(out, POLY1305_BYTES).copy_from_slice(&tag);
    ptr::write_bytes(
        state.cast::<u8>(),
        0,
        size_of::<crypto_onetimeauth_poly1305_state>(),
    );
    0
}

pub unsafe fn crypto_onetimeauth_poly1305_init(
    state: *mut crypto_onetimeauth_poly1305_state,
    key: *const u8,
) -> c_int {
    let key: [u8; POLY1305_KEYBYTES] = opt_slice(key, POLY1305_KEYBYTES).try_into().unwrap();
    ptr::write_bytes(
        state.cast::<u8>(),
        0,
        size_of::<crypto_onetimeauth_poly1305_state>(),
    );
    ptr::write(state.cast::<Poly1305StateRepr>(), poly1305_init_repr(&key));
    0
}

pub unsafe fn crypto_onetimeauth_poly1305_keybytes() -> usize {
    POLY1305_KEYBYTES
}

pub unsafe fn crypto_onetimeauth_poly1305_keygen(k: *mut u8) {
    random_keygen(k, POLY1305_KEYBYTES);
}

pub unsafe fn crypto_onetimeauth_poly1305_statebytes() -> usize {
    size_of::<crypto_onetimeauth_poly1305_state>()
}

pub unsafe fn crypto_onetimeauth_poly1305_update(
    state: *mut crypto_onetimeauth_poly1305_state,
    in_: *const u8,
    inlen: u64,
) -> c_int {
    poly1305_update_repr(poly1305_state(state), opt_slice(in_, len_to_usize(inlen)));
    0
}

pub unsafe fn crypto_onetimeauth_poly1305_verify(
    h: *const u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let mut expected = [0u8; POLY1305_BYTES];
    crypto_onetimeauth_poly1305(expected.as_mut_ptr(), in_, inlen, k);
    if ct_eq(&expected, opt_slice(h, POLY1305_BYTES)) {
        0
    } else {
        -1
    }
}

pub unsafe fn crypto_core_hchacha20(
    out: *mut u8,
    in_: *const u8,
    k: *const u8,
    c: *const u8,
) -> c_int {
    let input: [u8; 16] = opt_slice(in_, 16).try_into().unwrap();
    let key: [u8; 32] = opt_slice(k, 32).try_into().unwrap();
    let constants: [u8; 16] = if c.is_null() {
        CHACHA_SIGMA
    } else {
        opt_slice(c, 16).try_into().unwrap()
    };
    opt_slice_mut(out, 32).copy_from_slice(&hchacha20_core(&input, &key, &constants));
    0
}

pub unsafe fn crypto_core_hchacha20_constbytes() -> usize {
    16
}

pub unsafe fn crypto_core_hchacha20_inputbytes() -> usize {
    16
}

pub unsafe fn crypto_core_hchacha20_keybytes() -> usize {
    32
}

pub unsafe fn crypto_core_hchacha20_outputbytes() -> usize {
    32
}

pub unsafe fn crypto_core_hsalsa20(
    out: *mut u8,
    in_: *const u8,
    k: *const u8,
    c: *const u8,
) -> c_int {
    let input: [u8; 16] = opt_slice(in_, 16).try_into().unwrap();
    let key: [u8; 32] = opt_slice(k, 32).try_into().unwrap();
    let constants: [u8; 16] = if c.is_null() {
        SALSA_SIGMA
    } else {
        opt_slice(c, 16).try_into().unwrap()
    };
    opt_slice_mut(out, 32).copy_from_slice(&hsalsa20_core(&input, &key, &constants));
    0
}

pub unsafe fn crypto_core_hsalsa20_constbytes() -> usize {
    16
}

pub unsafe fn crypto_core_hsalsa20_inputbytes() -> usize {
    16
}

pub unsafe fn crypto_core_hsalsa20_keybytes() -> usize {
    32
}

pub unsafe fn crypto_core_hsalsa20_outputbytes() -> usize {
    32
}

pub unsafe fn crypto_core_salsa20(
    out: *mut u8,
    in_: *const u8,
    k: *const u8,
    c: *const u8,
) -> c_int {
    let input: [u8; 16] = opt_slice(in_, 16).try_into().unwrap();
    let key: [u8; 32] = opt_slice(k, 32).try_into().unwrap();
    let constants: [u8; 16] = if c.is_null() {
        SALSA_SIGMA
    } else {
        opt_slice(c, 16).try_into().unwrap()
    };
    let mut block = [0u8; 64];
    salsa_core_rounds(20, &mut block, &input, &key, &constants);
    opt_slice_mut(out, 64).copy_from_slice(&block);
    0
}

pub unsafe fn crypto_core_salsa20_constbytes() -> usize {
    16
}

pub unsafe fn crypto_core_salsa20_inputbytes() -> usize {
    16
}

pub unsafe fn crypto_core_salsa20_keybytes() -> usize {
    32
}

pub unsafe fn crypto_core_salsa20_outputbytes() -> usize {
    64
}

pub unsafe fn crypto_core_salsa2012(
    out: *mut u8,
    in_: *const u8,
    k: *const u8,
    c: *const u8,
) -> c_int {
    let input: [u8; 16] = opt_slice(in_, 16).try_into().unwrap();
    let key: [u8; 32] = opt_slice(k, 32).try_into().unwrap();
    let constants: [u8; 16] = if c.is_null() {
        SALSA_SIGMA
    } else {
        opt_slice(c, 16).try_into().unwrap()
    };
    let mut block = [0u8; 64];
    salsa_core_rounds(12, &mut block, &input, &key, &constants);
    opt_slice_mut(out, 64).copy_from_slice(&block);
    0
}

pub unsafe fn crypto_core_salsa2012_constbytes() -> usize {
    16
}

pub unsafe fn crypto_core_salsa2012_inputbytes() -> usize {
    16
}

pub unsafe fn crypto_core_salsa2012_keybytes() -> usize {
    32
}

pub unsafe fn crypto_core_salsa2012_outputbytes() -> usize {
    64
}

pub unsafe fn crypto_core_salsa208(
    out: *mut u8,
    in_: *const u8,
    k: *const u8,
    c: *const u8,
) -> c_int {
    let input: [u8; 16] = opt_slice(in_, 16).try_into().unwrap();
    let key: [u8; 32] = opt_slice(k, 32).try_into().unwrap();
    let constants: [u8; 16] = if c.is_null() {
        SALSA_SIGMA
    } else {
        opt_slice(c, 16).try_into().unwrap()
    };
    let mut block = [0u8; 64];
    salsa_core_rounds(8, &mut block, &input, &key, &constants);
    opt_slice_mut(out, 64).copy_from_slice(&block);
    0
}

pub unsafe fn crypto_core_salsa208_constbytes() -> usize {
    16
}

pub unsafe fn crypto_core_salsa208_inputbytes() -> usize {
    16
}

pub unsafe fn crypto_core_salsa208_keybytes() -> usize {
    32
}

pub unsafe fn crypto_core_salsa208_outputbytes() -> usize {
    64
}

unsafe fn fill_stream<C: chacha20::cipher::StreamCipher>(
    out: *mut u8,
    len: u64,
    mut cipher: C,
) -> c_int {
    let out = opt_slice_mut(out, len_to_usize(len));
    out.fill(0);
    cipher.apply_keystream(out);
    0
}

unsafe fn xor_stream<C: chacha20::cipher::StreamCipher>(
    out: *mut u8,
    input: *const u8,
    len: u64,
    mut cipher: C,
) -> c_int {
    let out = copy_or_in_place(out, input, len_to_usize(len));
    cipher.apply_keystream(out);
    0
}

unsafe fn chacha20_legacy_cipher(n: *const u8, k: *const u8, ic: u64) -> ChaCha20Legacy {
    let mut cipher = ChaCha20Legacy::new_from_slices(opt_slice(k, 32), opt_slice(n, 8)).unwrap();
    cipher.seek(u128::from(ic) * 64);
    cipher
}

unsafe fn chacha20_ietf_cipher(n: *const u8, k: *const u8, ic: u32) -> ChaCha20 {
    let mut cipher = ChaCha20::new_from_slices(opt_slice(k, 32), opt_slice(n, 12)).unwrap();
    cipher.seek(u128::from(ic) * 64);
    cipher
}

pub unsafe fn crypto_stream(c: *mut u8, clen: u64, n: *const u8, k: *const u8) -> c_int {
    crypto_stream_xsalsa20(c, clen, n, k)
}

pub unsafe fn crypto_stream_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_messagebytes_max() -> usize {
    usize::MAX
}

pub unsafe fn crypto_stream_noncebytes() -> usize {
    STREAM_NONCEBYTES
}

pub unsafe fn crypto_stream_primitive() -> *const c_char {
    static_cstr(b"xsalsa20\0")
}

pub unsafe fn crypto_stream_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_stream_xsalsa20_xor(c, m, mlen, n, k)
}

pub unsafe fn crypto_stream_chacha20(c: *mut u8, clen: u64, n: *const u8, k: *const u8) -> c_int {
    fill_stream(c, clen, chacha20_legacy_cipher(n, k, 0))
}

pub unsafe fn crypto_stream_chacha20_ietf(
    c: *mut u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    fill_stream(c, clen, chacha20_ietf_cipher(n, k, 0))
}

pub unsafe fn crypto_stream_chacha20_ietf_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_chacha20_ietf_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_chacha20_ietf_messagebytes_max() -> usize {
    cmp::min(usize::MAX, ((1u128 << 32) * 64) as usize)
}

pub unsafe fn crypto_stream_chacha20_ietf_noncebytes() -> usize {
    12
}

pub unsafe fn crypto_stream_chacha20_ietf_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    xor_stream(c, m, mlen, chacha20_ietf_cipher(n, k, 0))
}

pub unsafe fn crypto_stream_chacha20_ietf_xor_ic(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    ic: u32,
    k: *const u8,
) -> c_int {
    xor_stream(c, m, mlen, chacha20_ietf_cipher(n, k, ic))
}

pub unsafe fn crypto_stream_chacha20_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_chacha20_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_chacha20_messagebytes_max() -> usize {
    usize::MAX
}

pub unsafe fn crypto_stream_chacha20_noncebytes() -> usize {
    8
}

pub unsafe fn crypto_stream_chacha20_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_stream_chacha20_xor_ic(c, m, mlen, n, 0, k)
}

pub unsafe fn crypto_stream_chacha20_xor_ic(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    ic: u64,
    k: *const u8,
) -> c_int {
    xor_stream(c, m, mlen, chacha20_legacy_cipher(n, k, ic))
}

pub unsafe fn crypto_stream_salsa20(c: *mut u8, clen: u64, n: *const u8, k: *const u8) -> c_int {
    let cipher = Salsa20::new_from_slices(opt_slice(k, 32), opt_slice(n, 8)).unwrap();
    fill_stream(c, clen, cipher)
}

pub unsafe fn crypto_stream_salsa20_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_salsa20_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_salsa20_messagebytes_max() -> usize {
    usize::MAX
}

pub unsafe fn crypto_stream_salsa20_noncebytes() -> usize {
    8
}

pub unsafe fn crypto_stream_salsa20_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_stream_salsa20_xor_ic(c, m, mlen, n, 0, k)
}

pub unsafe fn crypto_stream_salsa20_xor_ic(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    ic: u64,
    k: *const u8,
) -> c_int {
    let mut cipher = Salsa20::new_from_slices(opt_slice(k, 32), opt_slice(n, 8)).unwrap();
    cipher.seek(u128::from(ic) * 64);
    xor_stream(c, m, mlen, cipher)
}

pub unsafe fn crypto_stream_salsa2012(c: *mut u8, clen: u64, n: *const u8, k: *const u8) -> c_int {
    let cipher = Salsa12::new_from_slices(opt_slice(k, 32), opt_slice(n, 8)).unwrap();
    fill_stream(c, clen, cipher)
}

pub unsafe fn crypto_stream_salsa2012_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_salsa2012_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_salsa2012_messagebytes_max() -> usize {
    usize::MAX
}

pub unsafe fn crypto_stream_salsa2012_noncebytes() -> usize {
    8
}

pub unsafe fn crypto_stream_salsa2012_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    let cipher = Salsa12::new_from_slices(opt_slice(k, 32), opt_slice(n, 8)).unwrap();
    xor_stream(c, m, mlen, cipher)
}

pub unsafe fn crypto_stream_salsa208(c: *mut u8, clen: u64, n: *const u8, k: *const u8) -> c_int {
    let cipher = Salsa8::new_from_slices(opt_slice(k, 32), opt_slice(n, 8)).unwrap();
    fill_stream(c, clen, cipher)
}

pub unsafe fn crypto_stream_salsa208_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_salsa208_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_salsa208_messagebytes_max() -> usize {
    usize::MAX
}

pub unsafe fn crypto_stream_salsa208_noncebytes() -> usize {
    8
}

pub unsafe fn crypto_stream_salsa208_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    let cipher = Salsa8::new_from_slices(opt_slice(k, 32), opt_slice(n, 8)).unwrap();
    xor_stream(c, m, mlen, cipher)
}

pub unsafe fn crypto_stream_xsalsa20(c: *mut u8, clen: u64, n: *const u8, k: *const u8) -> c_int {
    let cipher = XSalsa20::new_from_slices(opt_slice(k, 32), opt_slice(n, 24)).unwrap();
    fill_stream(c, clen, cipher)
}

pub unsafe fn crypto_stream_xsalsa20_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_xsalsa20_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_xsalsa20_messagebytes_max() -> usize {
    usize::MAX
}

pub unsafe fn crypto_stream_xsalsa20_noncebytes() -> usize {
    24
}

pub unsafe fn crypto_stream_xsalsa20_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_stream_xsalsa20_xor_ic(c, m, mlen, n, 0, k)
}

pub unsafe fn crypto_stream_xsalsa20_xor_ic(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    ic: u64,
    k: *const u8,
) -> c_int {
    let mut cipher = XSalsa20::new_from_slices(opt_slice(k, 32), opt_slice(n, 24)).unwrap();
    cipher.seek(u128::from(ic) * 64);
    xor_stream(c, m, mlen, cipher)
}

pub unsafe fn crypto_stream_xchacha20(c: *mut u8, clen: u64, n: *const u8, k: *const u8) -> c_int {
    let subkey = hchacha20_core(
        &opt_slice(n, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let mut cipher = ChaCha20Legacy::new_from_slices(&subkey, opt_slice(n.add(16), 8)).unwrap();
    cipher.seek(0u64);
    fill_stream(c, clen, cipher)
}

pub unsafe fn crypto_stream_xchacha20_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_stream_xchacha20_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_stream_xchacha20_messagebytes_max() -> usize {
    usize::MAX
}

pub unsafe fn crypto_stream_xchacha20_noncebytes() -> usize {
    24
}

pub unsafe fn crypto_stream_xchacha20_xor(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_stream_xchacha20_xor_ic(c, m, mlen, n, 0, k)
}

pub unsafe fn crypto_stream_xchacha20_xor_ic(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    ic: u64,
    k: *const u8,
) -> c_int {
    let subkey = hchacha20_core(
        &opt_slice(n, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let mut cipher = ChaCha20Legacy::new_from_slices(&subkey, opt_slice(n.add(16), 8)).unwrap();
    cipher.seek(u128::from(ic) * 64);
    xor_stream(c, m, mlen, cipher)
}

pub unsafe fn crypto_secretbox(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_secretbox_xsalsa20poly1305(c, m, mlen, n, k)
}

pub unsafe fn crypto_secretbox_boxzerobytes() -> usize {
    SECRETBOX_BOXZEROBYTES
}

pub unsafe fn crypto_secretbox_detached(
    c: *mut u8,
    mac: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    let subkey = hsalsa20_core(
        &opt_slice(n, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &SALSA_SIGMA,
    );
    let len = len_to_usize(mlen);
    let msg = if ranges_overlap(c, m, len) && !ptr::eq(c.cast_const(), m) {
        opt_slice(m, len).to_vec()
    } else {
        Vec::new()
    };
    let m = if msg.is_empty() {
        opt_slice(m, len)
    } else {
        msg.as_slice()
    };
    let mut block0 = [0u8; 64];
    let mlen0 = cmp::min(len, 64 - SECRETBOX_ZEROBYTES);
    block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0].copy_from_slice(&m[..mlen0]);
    crypto_stream_salsa20_xor(
        block0.as_mut_ptr(),
        block0.as_ptr(),
        (mlen0 + SECRETBOX_ZEROBYTES) as u64,
        n.add(16),
        subkey.as_ptr(),
    );
    let poly_key: [u8; POLY1305_KEYBYTES] = block0[..POLY1305_KEYBYTES].try_into().unwrap();
    opt_slice_mut(c, len)[..mlen0]
        .copy_from_slice(&block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0]);
    if len > mlen0 {
        crypto_stream_salsa20_xor_ic(
            c.add(mlen0),
            m[mlen0..].as_ptr(),
            (len - mlen0) as u64,
            n.add(16),
            1,
            subkey.as_ptr(),
        );
    }
    let mut mac_state = poly1305_init_repr(&poly_key);
    poly1305_update_repr(&mut mac_state, opt_slice(c, len));
    opt_slice_mut(mac, POLY1305_BYTES).copy_from_slice(&poly1305_finalize_repr(&mut mac_state));
    0
}

pub unsafe fn crypto_secretbox_easy(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if len_to_usize(mlen) > crypto_secretbox_messagebytes_max() {
        crate::foundation::core::sodium_misuse();
    }
    crypto_secretbox_detached(c.add(SECRETBOX_MACBYTES), c, m, mlen, n, k)
}

pub unsafe fn crypto_secretbox_keybytes() -> usize {
    SECRETBOX_KEYBYTES
}

pub unsafe fn crypto_secretbox_keygen(k: *mut u8) {
    random_keygen(k, SECRETBOX_KEYBYTES);
}

pub unsafe fn crypto_secretbox_macbytes() -> usize {
    SECRETBOX_MACBYTES
}

pub unsafe fn crypto_secretbox_messagebytes_max() -> usize {
    usize::MAX - SECRETBOX_MACBYTES
}

pub unsafe fn crypto_secretbox_noncebytes() -> usize {
    SECRETBOX_NONCEBYTES
}

pub unsafe fn crypto_secretbox_open(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_secretbox_xsalsa20poly1305_open(m, c, clen, n, k)
}

pub unsafe fn crypto_secretbox_open_detached(
    m: *mut u8,
    c: *const u8,
    mac: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    let subkey = hsalsa20_core(
        &opt_slice(n, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &SALSA_SIGMA,
    );
    let len = len_to_usize(clen);
    let mut block0 = [0u8; 64];
    crypto_stream_salsa20(
        block0.as_mut_ptr(),
        POLY1305_KEYBYTES as u64,
        n.add(16),
        subkey.as_ptr(),
    );
    if crypto_onetimeauth_poly1305_verify(mac, c, clen, block0.as_ptr()) != 0 {
        return -1;
    }
    if m.is_null() {
        return 0;
    }
    let cbuf = if ranges_overlap(m, c, len) && !ptr::eq(m.cast_const(), c) {
        opt_slice(c, len).to_vec()
    } else {
        Vec::new()
    };
    let c = if cbuf.is_empty() {
        opt_slice(c, len)
    } else {
        cbuf.as_slice()
    };
    let mlen0 = cmp::min(len, 64 - SECRETBOX_ZEROBYTES);
    block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0].copy_from_slice(&c[..mlen0]);
    crypto_stream_salsa20_xor(
        block0.as_mut_ptr(),
        block0.as_ptr(),
        (SECRETBOX_ZEROBYTES + mlen0) as u64,
        n.add(16),
        subkey.as_ptr(),
    );
    opt_slice_mut(m, len)[..mlen0]
        .copy_from_slice(&block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0]);
    if len > mlen0 {
        crypto_stream_salsa20_xor_ic(
            m.add(mlen0),
            c[mlen0..].as_ptr(),
            (len - mlen0) as u64,
            n.add(16),
            1,
            subkey.as_ptr(),
        );
    }
    0
}

pub unsafe fn crypto_secretbox_open_easy(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if clen < SECRETBOX_MACBYTES as u64 {
        return -1;
    }
    crypto_secretbox_open_detached(
        m,
        c.add(SECRETBOX_MACBYTES),
        c,
        clen - SECRETBOX_MACBYTES as u64,
        n,
        k,
    )
}

pub unsafe fn crypto_secretbox_primitive() -> *const c_char {
    static_cstr(b"xsalsa20poly1305\0")
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if mlen < SECRETBOX_ZEROBYTES as u64 {
        return -1;
    }
    crypto_stream_xsalsa20_xor(c, m, mlen, n, k);
    crypto_onetimeauth_poly1305(c.add(16), c.add(32), mlen - 32, c);
    opt_slice_mut(c, 16).fill(0);
    0
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_boxzerobytes() -> usize {
    SECRETBOX_BOXZEROBYTES
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_keybytes() -> usize {
    SECRETBOX_KEYBYTES
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_keygen(k: *mut u8) {
    random_keygen(k, SECRETBOX_KEYBYTES);
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_macbytes() -> usize {
    SECRETBOX_MACBYTES
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_messagebytes_max() -> usize {
    usize::MAX - SECRETBOX_MACBYTES
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_noncebytes() -> usize {
    SECRETBOX_NONCEBYTES
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_open(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if clen < SECRETBOX_ZEROBYTES as u64 {
        return -1;
    }
    let mut subkey = [0u8; POLY1305_KEYBYTES];
    crypto_stream_xsalsa20(subkey.as_mut_ptr(), 32, n, k);
    if crypto_onetimeauth_poly1305_verify(c.add(16), c.add(32), clen - 32, subkey.as_ptr()) != 0 {
        return -1;
    }
    crypto_stream_xsalsa20_xor(m, c, clen, n, k);
    opt_slice_mut(m, 32).fill(0);
    0
}

pub unsafe fn crypto_secretbox_xsalsa20poly1305_zerobytes() -> usize {
    SECRETBOX_ZEROBYTES
}

pub unsafe fn crypto_secretbox_zerobytes() -> usize {
    SECRETBOX_ZEROBYTES
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_detached(
    c: *mut u8,
    mac: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    let subkey = hchacha20_core(
        &opt_slice(n, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let len = len_to_usize(mlen);
    let msg = if ranges_overlap(c, m, len) && !ptr::eq(c.cast_const(), m) {
        opt_slice(m, len).to_vec()
    } else {
        Vec::new()
    };
    let m = if msg.is_empty() {
        opt_slice(m, len)
    } else {
        msg.as_slice()
    };
    let mut block0 = [0u8; 64];
    let mlen0 = cmp::min(len, 64 - SECRETBOX_ZEROBYTES);
    block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0].copy_from_slice(&m[..mlen0]);
    crypto_stream_chacha20_xor(
        block0.as_mut_ptr(),
        block0.as_ptr(),
        (mlen0 + SECRETBOX_ZEROBYTES) as u64,
        n.add(16),
        subkey.as_ptr(),
    );
    let poly_key: [u8; POLY1305_KEYBYTES] = block0[..POLY1305_KEYBYTES].try_into().unwrap();
    opt_slice_mut(c, len)[..mlen0]
        .copy_from_slice(&block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0]);
    if len > mlen0 {
        crypto_stream_chacha20_xor_ic(
            c.add(mlen0),
            m[mlen0..].as_ptr(),
            (len - mlen0) as u64,
            n.add(16),
            1,
            subkey.as_ptr(),
        );
    }
    let mut mac_state = poly1305_init_repr(&poly_key);
    poly1305_update_repr(&mut mac_state, opt_slice(c, len));
    opt_slice_mut(mac, POLY1305_BYTES).copy_from_slice(&poly1305_finalize_repr(&mut mac_state));
    0
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_easy(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if len_to_usize(mlen) > crypto_secretbox_xchacha20poly1305_messagebytes_max() {
        crate::foundation::core::sodium_misuse();
    }
    crypto_secretbox_xchacha20poly1305_detached(c.add(SECRETBOX_MACBYTES), c, m, mlen, n, k)
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_keybytes() -> usize {
    SECRETBOX_KEYBYTES
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_macbytes() -> usize {
    SECRETBOX_MACBYTES
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_messagebytes_max() -> usize {
    usize::MAX - SECRETBOX_MACBYTES
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_noncebytes() -> usize {
    24
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_open_detached(
    m: *mut u8,
    c: *const u8,
    mac: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    let subkey = hchacha20_core(
        &opt_slice(n, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let len = len_to_usize(clen);
    let mut block0 = [0u8; 64];
    crypto_stream_chacha20(
        block0.as_mut_ptr(),
        POLY1305_KEYBYTES as u64,
        n.add(16),
        subkey.as_ptr(),
    );
    if crypto_onetimeauth_poly1305_verify(mac, c, clen, block0.as_ptr()) != 0 {
        return -1;
    }
    if m.is_null() {
        return 0;
    }
    let cbuf = if ranges_overlap(m, c, len) && !ptr::eq(m.cast_const(), c) {
        opt_slice(c, len).to_vec()
    } else {
        Vec::new()
    };
    let c = if cbuf.is_empty() {
        opt_slice(c, len)
    } else {
        cbuf.as_slice()
    };
    let mlen0 = cmp::min(len, 64 - SECRETBOX_ZEROBYTES);
    block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0].copy_from_slice(&c[..mlen0]);
    crypto_stream_chacha20_xor(
        block0.as_mut_ptr(),
        block0.as_ptr(),
        (SECRETBOX_ZEROBYTES + mlen0) as u64,
        n.add(16),
        subkey.as_ptr(),
    );
    opt_slice_mut(m, len)[..mlen0]
        .copy_from_slice(&block0[SECRETBOX_ZEROBYTES..SECRETBOX_ZEROBYTES + mlen0]);
    if len > mlen0 {
        crypto_stream_chacha20_xor_ic(
            m.add(mlen0),
            c[mlen0..].as_ptr(),
            (len - mlen0) as u64,
            n.add(16),
            1,
            subkey.as_ptr(),
        );
    }
    0
}

pub unsafe fn crypto_secretbox_xchacha20poly1305_open_easy(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if clen < SECRETBOX_MACBYTES as u64 {
        return -1;
    }
    crypto_secretbox_xchacha20poly1305_open_detached(
        m,
        c.add(SECRETBOX_MACBYTES),
        c,
        clen - SECRETBOX_MACBYTES as u64,
        n,
        k,
    )
}

pub unsafe fn crypto_shorthash(out: *mut u8, in_: *const u8, inlen: u64, k: *const u8) -> c_int {
    crypto_shorthash_siphash24(out, in_, inlen, k)
}

pub unsafe fn crypto_shorthash_bytes() -> usize {
    SHORT_HASH_BYTES
}

pub unsafe fn crypto_shorthash_keybytes() -> usize {
    SHORT_HASH_KEYBYTES
}

pub unsafe fn crypto_shorthash_keygen(k: *mut u8) {
    random_keygen(k, SHORT_HASH_KEYBYTES);
}

pub unsafe fn crypto_shorthash_primitive() -> *const c_char {
    static_cstr(b"siphash24\0")
}

pub unsafe fn crypto_shorthash_siphash24(
    out: *mut u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let key = opt_slice(k, SHORT_HASH_KEYBYTES);
    let mut hasher = SipHasher24::new_with_key(key.try_into().unwrap());
    hasher.write(opt_slice(in_, len_to_usize(inlen)));
    opt_slice_mut(out, SHORT_HASH_BYTES).copy_from_slice(&hasher.finish().to_le_bytes());
    0
}

pub unsafe fn crypto_shorthash_siphash24_bytes() -> usize {
    SHORT_HASH_BYTES
}

pub unsafe fn crypto_shorthash_siphash24_keybytes() -> usize {
    SHORT_HASH_KEYBYTES
}

pub unsafe fn crypto_shorthash_siphashx24(
    out: *mut u8,
    in_: *const u8,
    inlen: u64,
    k: *const u8,
) -> c_int {
    let key = opt_slice(k, SHORT_HASH_KEYBYTES);
    let mut hasher = SipHasher12824::new_with_key(key.try_into().unwrap());
    hasher.write(opt_slice(in_, len_to_usize(inlen)));
    let hash = hasher.finish128().as_u128();
    opt_slice_mut(out, SHORT_HASH_X_BYTES).copy_from_slice(&hash.to_le_bytes());
    0
}

pub unsafe fn crypto_shorthash_siphashx24_bytes() -> usize {
    SHORT_HASH_X_BYTES
}

pub unsafe fn crypto_shorthash_siphashx24_keybytes() -> usize {
    SHORT_HASH_KEYBYTES
}

fn pad16_len(len: usize) -> usize {
    (16 - (len & 0xf)) & 0xf
}

fn increment_le(counter: &mut [u8]) {
    let mut carry = 1u16;
    for byte in counter {
        carry += u16::from(*byte);
        *byte = carry as u8;
        carry >>= 8;
        if carry == 0 {
            break;
        }
    }
}

fn xor_in_place(dst: &mut [u8], src: &[u8]) {
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d ^= *s;
    }
}

fn secretstream_rekey_inner(state: &mut SecretstreamStateRepr) {
    let mut new_state = [0u8; 40];
    new_state[..32].copy_from_slice(&state.k);
    new_state[32..40].copy_from_slice(&state.nonce[4..12]);
    let mut cipher = ChaCha20::new_from_slices(&state.k, &state.nonce).unwrap();
    cipher.apply_keystream(&mut new_state);
    state.k.copy_from_slice(&new_state[..32]);
    state.nonce.fill(0);
    state.nonce[0] = 1;
    state.nonce[4..12].copy_from_slice(&new_state[32..40]);
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_abytes() -> usize {
    SECRETSTREAM_ABYTES
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_headerbytes() -> usize {
    SECRETSTREAM_HEADERBYTES
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_init_pull(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    header: *const u8,
    k: *const u8,
) -> c_int {
    let st = secretstream_state(state);
    st.k = hchacha20_core(
        &opt_slice(header, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    st.nonce = [0; 12];
    st.nonce[0] = 1;
    st.nonce[4..12].copy_from_slice(opt_slice(header.add(16), 8));
    st.pad = [0; 8];
    0
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_init_push(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    header: *mut u8,
    k: *const u8,
) -> c_int {
    fill_random_bytes(header, SECRETSTREAM_HEADERBYTES);
    crypto_secretstream_xchacha20poly1305_init_pull(state, header, k)
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_keybytes() -> usize {
    SECRETSTREAM_KEYBYTES
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_keygen(k: *mut u8) {
    random_keygen(k, SECRETSTREAM_KEYBYTES);
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_messagebytes_max() -> usize {
    cmp::min(
        usize::MAX - SECRETSTREAM_ABYTES,
        (((1u64 << 32) - 2) as usize) * 64,
    )
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_push(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    c: *mut u8,
    clen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    tag: u8,
) -> c_int {
    let st = secretstream_state(state);
    let mlen = len_to_usize(mlen);
    let ad = opt_slice(ad, len_to_usize(adlen));
    let msg = opt_slice(m, mlen);
    let out = opt_slice_mut(c, mlen + SECRETSTREAM_ABYTES);
    let mut poly = poly1305_init_repr(&{
        let mut key = [0u8; 32];
        let mut cipher = ChaCha20::new_from_slices(&st.k, &st.nonce).unwrap();
        cipher.apply_keystream(&mut key);
        key
    });
    poly1305_update_repr(&mut poly, ad);
    if pad16_len(ad.len()) != 0 {
        poly1305_update_repr(&mut poly, &[0u8; 16][..pad16_len(ad.len())]);
    }
    let mut block = [0u8; 64];
    block[0] = tag;
    let mut cipher = ChaCha20::new_from_slices(&st.k, &st.nonce).unwrap();
    cipher.seek(64u32);
    cipher.apply_keystream(&mut block);
    poly1305_update_repr(&mut poly, &block);
    out[0] = block[0];
    out[1..1 + mlen].copy_from_slice(msg);
    cipher.seek(128u32);
    cipher.apply_keystream(&mut out[1..1 + mlen]);
    poly1305_update_repr(&mut poly, &out[1..1 + mlen]);
    let mac_pad = ((16i64 - block.len() as i64 + mlen as i64) & 0xf) as usize;
    if mac_pad != 0 {
        poly1305_update_repr(&mut poly, &[0u8; 16][..mac_pad]);
    }
    let mut size_data = [0u8; 16];
    size_data[..8].copy_from_slice(&(ad.len() as u64).to_le_bytes());
    size_data[8..].copy_from_slice(&((block.len() + mlen) as u64).to_le_bytes());
    poly1305_update_repr(&mut poly, &size_data);
    let mac = poly1305_finalize_repr(&mut poly);
    out[1 + mlen..1 + mlen + 16].copy_from_slice(&mac);
    xor_in_place(&mut st.nonce[4..12], &mac[..8]);
    increment_le(&mut st.nonce[..4]);
    if tag & SECRETSTREAM_TAG_REKEY != 0 || st.nonce[..4] == [0, 0, 0, 0] {
        secretstream_rekey_inner(st);
    }
    write_opt(clen_p, (mlen + SECRETSTREAM_ABYTES) as u64);
    0
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_pull(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
    m: *mut u8,
    mlen_p: *mut u64,
    tag_p: *mut u8,
    c: *const u8,
    clen: u64,
    ad: *const u8,
    adlen: u64,
) -> c_int {
    let st = secretstream_state(state);
    let clen = len_to_usize(clen);
    if clen < SECRETSTREAM_ABYTES {
        return -1;
    }
    let ad = opt_slice(ad, len_to_usize(adlen));
    let ct = opt_slice(c, clen);
    let mlen = clen - SECRETSTREAM_ABYTES;
    let mut poly = poly1305_init_repr(&{
        let mut key = [0u8; 32];
        let mut cipher = ChaCha20::new_from_slices(&st.k, &st.nonce).unwrap();
        cipher.apply_keystream(&mut key);
        key
    });
    poly1305_update_repr(&mut poly, ad);
    if pad16_len(ad.len()) != 0 {
        poly1305_update_repr(&mut poly, &[0u8; 16][..pad16_len(ad.len())]);
    }
    let mut block = [0u8; 64];
    block[0] = ct[0];
    let mut cipher = ChaCha20::new_from_slices(&st.k, &st.nonce).unwrap();
    cipher.seek(64u32);
    cipher.apply_keystream(&mut block);
    let tag = block[0];
    block[0] = ct[0];
    poly1305_update_repr(&mut poly, &block);
    poly1305_update_repr(&mut poly, &ct[1..1 + mlen]);
    let mac_pad = ((16i64 - block.len() as i64 + mlen as i64) & 0xf) as usize;
    if mac_pad != 0 {
        poly1305_update_repr(&mut poly, &[0u8; 16][..mac_pad]);
    }
    let mut size_data = [0u8; 16];
    size_data[..8].copy_from_slice(&(ad.len() as u64).to_le_bytes());
    size_data[8..].copy_from_slice(&((block.len() + mlen) as u64).to_le_bytes());
    poly1305_update_repr(&mut poly, &size_data);
    let mac = poly1305_finalize_repr(&mut poly);
    if !ct_eq(&mac, &ct[1 + mlen..1 + mlen + 16]) {
        return -1;
    }
    if !m.is_null() {
        let out = copy_or_in_place(m, ct[1..1 + mlen].as_ptr(), mlen);
        cipher.seek(128u32);
        cipher.apply_keystream(out);
    }
    xor_in_place(&mut st.nonce[4..12], &mac[..8]);
    increment_le(&mut st.nonce[..4]);
    if tag & SECRETSTREAM_TAG_REKEY != 0 || st.nonce[..4] == [0, 0, 0, 0] {
        secretstream_rekey_inner(st);
    }
    write_opt(mlen_p, mlen as u64);
    write_opt(tag_p, tag);
    0
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_rekey(
    state: *mut crypto_secretstream_xchacha20poly1305_state,
) {
    secretstream_rekey_inner(secretstream_state(state));
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_statebytes() -> usize {
    SECRETSTREAM_STATEBYTES
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_final() -> u8 {
    SECRETSTREAM_TAG_FINAL
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_message() -> u8 {
    SECRETSTREAM_TAG_MESSAGE
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_push() -> u8 {
    SECRETSTREAM_TAG_PUSH
}

pub unsafe fn crypto_secretstream_xchacha20poly1305_tag_rekey() -> u8 {
    SECRETSTREAM_TAG_REKEY
}

unsafe fn aead_poly1305_update_lengths(poly: &mut Poly1305StateRepr, adlen: usize, clen: usize) {
    let mut size_data = [0u8; 16];
    size_data[..8].copy_from_slice(&(adlen as u64).to_le_bytes());
    size_data[8..].copy_from_slice(&(clen as u64).to_le_bytes());
    poly1305_update_repr(poly, &size_data);
}

unsafe fn aead_chacha20poly1305_checked_mlen(mlen: u64, ietf: bool) -> usize {
    let mlen = len_to_usize(mlen);
    let max = if ietf {
        crypto_aead_chacha20poly1305_ietf_messagebytes_max()
    } else {
        crypto_aead_chacha20poly1305_messagebytes_max()
    };
    if mlen > max {
        crate::foundation::core::sodium_misuse();
    }
    mlen
}

unsafe fn aead_chacha20poly1305_encrypt_detached_inner(
    c: *mut u8,
    mac: *mut u8,
    maclen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
    ietf: bool,
) -> c_int {
    let mlen = aead_chacha20poly1305_checked_mlen(mlen, ietf);
    let ad = opt_slice(ad, len_to_usize(adlen));
    let mut poly_key = [0u8; 32];
    if ietf {
        let mut cipher = chacha20_ietf_cipher(npub, k, 0);
        cipher.apply_keystream(&mut poly_key);
    } else {
        let mut cipher = chacha20_legacy_cipher(npub, k, 0);
        cipher.apply_keystream(&mut poly_key);
    }
    let out = copy_or_in_place(c, m, mlen);
    let mut size_data8 = [0u8; 8];
    if ietf {
        let mut cipher = chacha20_ietf_cipher(npub, k, 1);
        cipher.apply_keystream(out);
    } else {
        let mut cipher = chacha20_legacy_cipher(npub, k, 1);
        cipher.apply_keystream(out);
    }
    let mut poly = poly1305_init_repr(&poly_key);
    if ietf {
        poly1305_update_repr(&mut poly, ad);
        if pad16_len(ad.len()) != 0 {
            poly1305_update_repr(&mut poly, &[0u8; 16][..pad16_len(ad.len())]);
        }
        poly1305_update_repr(&mut poly, out);
        if pad16_len(out.len()) != 0 {
            poly1305_update_repr(&mut poly, &[0u8; 16][..pad16_len(out.len())]);
        }
        aead_poly1305_update_lengths(&mut poly, ad.len(), out.len());
    } else {
        poly1305_update_repr(&mut poly, ad);
        size_data8.copy_from_slice(&(ad.len() as u64).to_le_bytes());
        poly1305_update_repr(&mut poly, &size_data8);
        poly1305_update_repr(&mut poly, out);
        size_data8.copy_from_slice(&(out.len() as u64).to_le_bytes());
        poly1305_update_repr(&mut poly, &size_data8);
    }
    opt_slice_mut(mac, 16).copy_from_slice(&poly1305_finalize_repr(&mut poly));
    write_opt(maclen_p, 16);
    0
}

unsafe fn aead_chacha20poly1305_decrypt_detached_inner(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    mac: *const u8,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
    ietf: bool,
) -> c_int {
    let clen = len_to_usize(clen);
    let ad = opt_slice(ad, len_to_usize(adlen));
    let mut poly_key = [0u8; 32];
    if ietf {
        let mut cipher = chacha20_ietf_cipher(npub, k, 0);
        cipher.apply_keystream(&mut poly_key);
    } else {
        let mut cipher = chacha20_legacy_cipher(npub, k, 0);
        cipher.apply_keystream(&mut poly_key);
    }
    let ct = opt_slice(c, clen);
    let mut poly = poly1305_init_repr(&poly_key);
    let mut size_data8 = [0u8; 8];
    if ietf {
        poly1305_update_repr(&mut poly, ad);
        if pad16_len(ad.len()) != 0 {
            poly1305_update_repr(&mut poly, &[0u8; 16][..pad16_len(ad.len())]);
        }
        poly1305_update_repr(&mut poly, ct);
        if pad16_len(ct.len()) != 0 {
            poly1305_update_repr(&mut poly, &[0u8; 16][..pad16_len(ct.len())]);
        }
        aead_poly1305_update_lengths(&mut poly, ad.len(), ct.len());
    } else {
        poly1305_update_repr(&mut poly, ad);
        size_data8.copy_from_slice(&(ad.len() as u64).to_le_bytes());
        poly1305_update_repr(&mut poly, &size_data8);
        poly1305_update_repr(&mut poly, ct);
        size_data8.copy_from_slice(&(ct.len() as u64).to_le_bytes());
        poly1305_update_repr(&mut poly, &size_data8);
    }
    let expected = poly1305_finalize_repr(&mut poly);
    if !ct_eq(&expected, opt_slice(mac, 16)) {
        if !m.is_null() {
            opt_slice_mut(m, clen).fill(0);
        }
        return -1;
    }
    if !m.is_null() {
        let out = copy_or_in_place(m, c, clen);
        if ietf {
            let mut cipher = chacha20_ietf_cipher(npub, k, 1);
            cipher.apply_keystream(out);
        } else {
            let mut cipher = chacha20_legacy_cipher(npub, k, 1);
            cipher.apply_keystream(out);
        }
    }
    0
}

pub unsafe fn crypto_aead_chacha20poly1305_abytes() -> usize {
    AEAD_ABYTES
}

pub unsafe fn crypto_aead_chacha20poly1305_decrypt(
    m: *mut u8,
    mlen_p: *mut u64,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if clen < AEAD_ABYTES as u64 {
        write_opt(mlen_p, 0);
        return -1;
    }
    let ret = aead_chacha20poly1305_decrypt_detached_inner(
        m,
        c,
        clen - AEAD_ABYTES as u64,
        c.add((clen - AEAD_ABYTES as u64) as usize),
        ad,
        adlen,
        npub,
        k,
        false,
    );
    write_opt(
        mlen_p,
        if ret == 0 {
            clen - AEAD_ABYTES as u64
        } else {
            0
        },
    );
    ret
}

pub unsafe fn crypto_aead_chacha20poly1305_decrypt_detached(
    m: *mut u8,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    mac: *const u8,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    aead_chacha20poly1305_decrypt_detached_inner(m, c, clen, mac, ad, adlen, npub, k, false)
}

pub unsafe fn crypto_aead_chacha20poly1305_encrypt(
    c: *mut u8,
    clen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    let mlen_usize = aead_chacha20poly1305_checked_mlen(mlen, false);
    let ret = aead_chacha20poly1305_encrypt_detached_inner(
        c,
        c.add(mlen_usize),
        ptr::null_mut(),
        m,
        mlen,
        ad,
        adlen,
        npub,
        k,
        false,
    );
    if ret == 0 {
        write_opt(clen_p, mlen + AEAD_ABYTES as u64);
    }
    ret
}

pub unsafe fn crypto_aead_chacha20poly1305_encrypt_detached(
    c: *mut u8,
    mac: *mut u8,
    maclen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    aead_chacha20poly1305_encrypt_detached_inner(
        c, mac, maclen_p, m, mlen, ad, adlen, npub, k, false,
    )
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_abytes() -> usize {
    AEAD_ABYTES
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_decrypt(
    m: *mut u8,
    mlen_p: *mut u64,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if clen < AEAD_ABYTES as u64 {
        write_opt(mlen_p, 0);
        return -1;
    }
    let ret = aead_chacha20poly1305_decrypt_detached_inner(
        m,
        c,
        clen - AEAD_ABYTES as u64,
        c.add((clen - AEAD_ABYTES as u64) as usize),
        ad,
        adlen,
        npub,
        k,
        true,
    );
    write_opt(
        mlen_p,
        if ret == 0 {
            clen - AEAD_ABYTES as u64
        } else {
            0
        },
    );
    ret
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_decrypt_detached(
    m: *mut u8,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    mac: *const u8,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    aead_chacha20poly1305_decrypt_detached_inner(m, c, clen, mac, ad, adlen, npub, k, true)
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_encrypt(
    c: *mut u8,
    clen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    let mlen_usize = aead_chacha20poly1305_checked_mlen(mlen, true);
    let ret = aead_chacha20poly1305_encrypt_detached_inner(
        c,
        c.add(mlen_usize),
        ptr::null_mut(),
        m,
        mlen,
        ad,
        adlen,
        npub,
        k,
        true,
    );
    if ret == 0 {
        write_opt(clen_p, mlen + AEAD_ABYTES as u64);
    }
    ret
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_encrypt_detached(
    c: *mut u8,
    mac: *mut u8,
    maclen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    aead_chacha20poly1305_encrypt_detached_inner(
        c, mac, maclen_p, m, mlen, ad, adlen, npub, k, true,
    )
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_messagebytes_max() -> usize {
    cmp::min(
        usize::MAX - AEAD_ABYTES,
        (((1u128 << 32) - 1) * 64) as usize,
    )
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_npubbytes() -> usize {
    AEAD_CHACHA20_IETF_NONCEBYTES
}

pub unsafe fn crypto_aead_chacha20poly1305_ietf_nsecbytes() -> usize {
    0
}

pub unsafe fn crypto_aead_chacha20poly1305_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_aead_chacha20poly1305_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_aead_chacha20poly1305_messagebytes_max() -> usize {
    usize::MAX - AEAD_ABYTES
}

pub unsafe fn crypto_aead_chacha20poly1305_npubbytes() -> usize {
    AEAD_CHACHA20_NONCEBYTES
}

pub unsafe fn crypto_aead_chacha20poly1305_nsecbytes() -> usize {
    0
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_abytes() -> usize {
    AEAD_ABYTES
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_decrypt(
    m: *mut u8,
    mlen_p: *mut u64,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if clen < AEAD_ABYTES as u64 {
        write_opt(mlen_p, 0);
        return -1;
    }
    let subkey = hchacha20_core(
        &opt_slice(npub, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let mut nonce = [0u8; 12];
    nonce[4..].copy_from_slice(opt_slice(npub.add(16), 8));
    let ret = aead_chacha20poly1305_decrypt_detached_inner(
        m,
        c,
        clen - AEAD_ABYTES as u64,
        c.add((clen - AEAD_ABYTES as u64) as usize),
        ad,
        adlen,
        nonce.as_ptr(),
        subkey.as_ptr(),
        true,
    );
    write_opt(
        mlen_p,
        if ret == 0 {
            clen - AEAD_ABYTES as u64
        } else {
            0
        },
    );
    ret
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_decrypt_detached(
    m: *mut u8,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    mac: *const u8,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    let subkey = hchacha20_core(
        &opt_slice(npub, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let mut nonce = [0u8; 12];
    nonce[4..].copy_from_slice(opt_slice(npub.add(16), 8));
    aead_chacha20poly1305_decrypt_detached_inner(
        m,
        c,
        clen,
        mac,
        ad,
        adlen,
        nonce.as_ptr(),
        subkey.as_ptr(),
        true,
    )
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_encrypt(
    c: *mut u8,
    clen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    let mlen_usize = {
        let mlen = len_to_usize(mlen);
        if mlen > crypto_aead_xchacha20poly1305_ietf_messagebytes_max() {
            crate::foundation::core::sodium_misuse();
        }
        mlen
    };
    let subkey = hchacha20_core(
        &opt_slice(npub, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let mut nonce = [0u8; 12];
    nonce[4..].copy_from_slice(opt_slice(npub.add(16), 8));
    let ret = aead_chacha20poly1305_encrypt_detached_inner(
        c,
        c.add(mlen_usize),
        ptr::null_mut(),
        m,
        mlen,
        ad,
        adlen,
        nonce.as_ptr(),
        subkey.as_ptr(),
        true,
    );
    if ret == 0 {
        write_opt(clen_p, mlen + AEAD_ABYTES as u64);
    }
    ret
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_encrypt_detached(
    c: *mut u8,
    mac: *mut u8,
    maclen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if len_to_usize(mlen) > crypto_aead_xchacha20poly1305_ietf_messagebytes_max() {
        crate::foundation::core::sodium_misuse();
    }
    let subkey = hchacha20_core(
        &opt_slice(npub, 16).try_into().unwrap(),
        &opt_slice(k, 32).try_into().unwrap(),
        &CHACHA_SIGMA,
    );
    let mut nonce = [0u8; 12];
    nonce[4..].copy_from_slice(opt_slice(npub.add(16), 8));
    aead_chacha20poly1305_encrypt_detached_inner(
        c,
        mac,
        maclen_p,
        m,
        mlen,
        ad,
        adlen,
        nonce.as_ptr(),
        subkey.as_ptr(),
        true,
    )
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_keybytes() -> usize {
    STREAM_KEYBYTES
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_keygen(k: *mut u8) {
    random_keygen(k, STREAM_KEYBYTES);
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_messagebytes_max() -> usize {
    usize::MAX - AEAD_ABYTES
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_npubbytes() -> usize {
    AEAD_XCHACHA20_NONCEBYTES
}

pub unsafe fn crypto_aead_xchacha20poly1305_ietf_nsecbytes() -> usize {
    0
}

pub unsafe fn crypto_aead_aes256gcm_abytes() -> usize {
    AEAD_ABYTES
}

unsafe fn aes256gcm_require_available() -> c_int {
    if crypto_aead_aes256gcm_is_available() == 0 {
        set_errno(libc::ENOSYS);
        return -1;
    }
    0
}

unsafe fn aes256gcm_cipher_from_key(k: *const u8) -> aes_gcm::Aes256Gcm {
    use aes_gcm::aead::KeyInit;

    let key =
        aes_gcm::Key::<aes_gcm::Aes256Gcm>::try_from(opt_slice(k, AES256GCM_KEYBYTES))
            .expect("AES-256-GCM key length");
    aes_gcm::Aes256Gcm::new(&key)
}

unsafe fn aes256gcm_nonce_from_ptr(
    npub: *const u8,
) -> aes_gcm::aead::Nonce<aes_gcm::Aes256Gcm> {
    aes_gcm::aead::Nonce::<aes_gcm::Aes256Gcm>::try_from(opt_slice(npub, AES256GCM_NONCEBYTES))
        .expect("AES-256-GCM nonce length")
}

unsafe fn aes256gcm_tag_from_ptr(mac: *const u8) -> aes_gcm::aead::Tag<aes_gcm::Aes256Gcm> {
    aes_gcm::aead::Tag::<aes_gcm::Aes256Gcm>::try_from(opt_slice(mac, AEAD_ABYTES))
        .expect("AES-256-GCM tag length")
}

pub unsafe fn crypto_aead_aes256gcm_beforenm(
    ctx_: *mut crypto_aead_aes256gcm_state,
    k: *const u8,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    let ctx = opt_slice_mut(ctx_.cast::<u8>(), size_of::<crypto_aead_aes256gcm_state>());
    ctx.fill(0);
    ctx[..AES256GCM_KEYBYTES].copy_from_slice(opt_slice(k, AES256GCM_KEYBYTES));
    0
}

unsafe fn aes256gcm_key_from_state(
    ctx_: *const crypto_aead_aes256gcm_state,
) -> [u8; AES256GCM_KEYBYTES] {
    opt_slice(ctx_.cast::<u8>(), AES256GCM_KEYBYTES)
        .try_into()
        .unwrap()
}

pub unsafe fn crypto_aead_aes256gcm_decrypt(
    m: *mut u8,
    mlen_p: *mut u64,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    if clen < 16 {
        write_opt(mlen_p, 0);
        return -1;
    }
    let ret = crypto_aead_aes256gcm_decrypt_detached(
        m,
        ptr::null_mut(),
        c,
        clen - 16,
        c.add((clen - 16) as usize),
        ad,
        adlen,
        npub,
        k,
    );
    write_opt(mlen_p, if ret == 0 { clen - 16 } else { 0 });
    ret
}

pub unsafe fn crypto_aead_aes256gcm_decrypt_afternm(
    m: *mut u8,
    mlen_p: *mut u64,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    crypto_aead_aes256gcm_decrypt(
        m,
        mlen_p,
        ptr::null_mut(),
        c,
        clen,
        ad,
        adlen,
        npub,
        aes256gcm_key_from_state(ctx_).as_ptr(),
    )
}

pub unsafe fn crypto_aead_aes256gcm_decrypt_detached(
    m: *mut u8,
    _nsec: *mut u8,
    c: *const u8,
    clen: u64,
    mac: *const u8,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    if clen > AES256GCM_MESSAGEBYTES_MAX as u64 {
        crate::foundation::core::sodium_misuse();
    }
    use aes_gcm::aead::AeadInOut;

    let cipher = aes256gcm_cipher_from_key(k);
    let nonce = aes256gcm_nonce_from_ptr(npub);
    let tag = aes256gcm_tag_from_ptr(mac);
    let out = copy_or_in_place(m, c, len_to_usize(clen));
    cipher
        .decrypt_inout_detached(&nonce, opt_slice(ad, len_to_usize(adlen)), out.into(), &tag)
        .map(|_| 0)
        .unwrap_or(-1)
}

pub unsafe fn crypto_aead_aes256gcm_decrypt_detached_afternm(
    m: *mut u8,
    nsec: *mut u8,
    c: *const u8,
    clen: u64,
    mac: *const u8,
    ad: *const u8,
    adlen: u64,
    npub: *const u8,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    crypto_aead_aes256gcm_decrypt_detached(
        m,
        nsec,
        c,
        clen,
        mac,
        ad,
        adlen,
        npub,
        aes256gcm_key_from_state(ctx_).as_ptr(),
    )
}

pub unsafe fn crypto_aead_aes256gcm_encrypt(
    c: *mut u8,
    clen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    let ret = crypto_aead_aes256gcm_encrypt_detached(
        c,
        c.add(len_to_usize(mlen)),
        ptr::null_mut(),
        m,
        mlen,
        ad,
        adlen,
        ptr::null(),
        npub,
        k,
    );
    if ret == 0 {
        write_opt(clen_p, mlen + 16);
    }
    ret
}

pub unsafe fn crypto_aead_aes256gcm_encrypt_afternm(
    c: *mut u8,
    clen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    crypto_aead_aes256gcm_encrypt(
        c,
        clen_p,
        m,
        mlen,
        ad,
        adlen,
        ptr::null(),
        npub,
        aes256gcm_key_from_state(ctx_).as_ptr(),
    )
}

pub unsafe fn crypto_aead_aes256gcm_encrypt_detached(
    c: *mut u8,
    mac: *mut u8,
    maclen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    _nsec: *const u8,
    npub: *const u8,
    k: *const u8,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    if mlen > AES256GCM_MESSAGEBYTES_MAX as u64 {
        crate::foundation::core::sodium_misuse();
    }
    use aes_gcm::aead::AeadInOut;

    let cipher = aes256gcm_cipher_from_key(k);
    let nonce = aes256gcm_nonce_from_ptr(npub);
    let out = copy_or_in_place(c, m, len_to_usize(mlen));
    match cipher.encrypt_inout_detached(&nonce, opt_slice(ad, len_to_usize(adlen)), out.into()) {
        Ok(tag) => {
            opt_slice_mut(mac, AEAD_ABYTES).copy_from_slice(tag.as_ref());
            write_opt(maclen_p, AEAD_ABYTES as u64);
            0
        }
        Err(_) => -1,
    }
}

pub unsafe fn crypto_aead_aes256gcm_encrypt_detached_afternm(
    c: *mut u8,
    mac: *mut u8,
    maclen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    ad: *const u8,
    adlen: u64,
    nsec: *const u8,
    npub: *const u8,
    ctx_: *const crypto_aead_aes256gcm_state,
) -> c_int {
    if aes256gcm_require_available() != 0 {
        return -1;
    }
    crypto_aead_aes256gcm_encrypt_detached(
        c,
        mac,
        maclen_p,
        m,
        mlen,
        ad,
        adlen,
        nsec,
        npub,
        aes256gcm_key_from_state(ctx_).as_ptr(),
    )
}

pub unsafe fn crypto_aead_aes256gcm_is_available() -> c_int {
    sodium_runtime_has_pclmul() & sodium_runtime_has_aesni()
}

pub unsafe fn crypto_aead_aes256gcm_keybytes() -> usize {
    AES256GCM_KEYBYTES
}

pub unsafe fn crypto_aead_aes256gcm_keygen(k: *mut u8) {
    random_keygen(k, AES256GCM_KEYBYTES);
}

pub unsafe fn crypto_aead_aes256gcm_messagebytes_max() -> usize {
    AES256GCM_MESSAGEBYTES_MAX
}

pub unsafe fn crypto_aead_aes256gcm_npubbytes() -> usize {
    AES256GCM_NONCEBYTES
}

pub unsafe fn crypto_aead_aes256gcm_nsecbytes() -> usize {
    0
}

pub unsafe fn crypto_aead_aes256gcm_statebytes() -> usize {
    size_of::<crypto_aead_aes256gcm_state>()
}

pub unsafe fn crypto_kdf_blake2b_bytes_max() -> usize {
    KDF_BYTES_MAX
}

pub unsafe fn crypto_kdf_blake2b_bytes_min() -> usize {
    KDF_BYTES_MIN
}

pub unsafe fn crypto_kdf_blake2b_contextbytes() -> usize {
    KDF_CONTEXTBYTES
}

pub unsafe fn crypto_kdf_blake2b_derive_from_key(
    subkey: *mut u8,
    subkey_len: usize,
    subkey_id: u64,
    ctx: *const c_char,
    key: *const u8,
) -> c_int {
    if !(KDF_BYTES_MIN..=KDF_BYTES_MAX).contains(&subkey_len) {
        set_errno(libc::EINVAL);
        return -1;
    }
    let mut salt = [0u8; 16];
    salt[..8].copy_from_slice(&subkey_id.to_le_bytes());
    let mut personal = [0u8; 16];
    personal[..8].copy_from_slice(opt_slice(ctx.cast::<u8>(), 8));
    let mut params = Blake2bParams::new();
    params
        .hash_length(subkey_len)
        .key(opt_slice(key, KDF_KEYBYTES))
        .salt(&salt)
        .personal(&personal);
    let hash = params.hash(&[]);
    opt_slice_mut(subkey, subkey_len).copy_from_slice(&hash.as_bytes()[..subkey_len]);
    0
}

pub unsafe fn crypto_kdf_blake2b_keybytes() -> usize {
    KDF_KEYBYTES
}

pub unsafe fn crypto_kdf_bytes_max() -> usize {
    KDF_BYTES_MAX
}

pub unsafe fn crypto_kdf_bytes_min() -> usize {
    KDF_BYTES_MIN
}

pub unsafe fn crypto_kdf_contextbytes() -> usize {
    KDF_CONTEXTBYTES
}

pub unsafe fn crypto_kdf_derive_from_key(
    subkey: *mut u8,
    subkey_len: usize,
    subkey_id: u64,
    ctx: *const c_char,
    key: *const u8,
) -> c_int {
    crypto_kdf_blake2b_derive_from_key(subkey, subkey_len, subkey_id, ctx, key)
}

pub unsafe fn crypto_kdf_keybytes() -> usize {
    KDF_KEYBYTES
}

pub unsafe fn crypto_kdf_keygen(k: *mut u8) {
    random_keygen(k, KDF_KEYBYTES);
}

pub unsafe fn crypto_kdf_primitive() -> *const c_char {
    static_cstr(b"blake2b\0")
}

fn pwhash_opslimit_min(alg: c_int) -> Option<u64> {
    match alg {
        PWHASH_ALG_ARGON2I13 => Some(PWHASH_ARGON2I_OPSLIMIT_MIN),
        PWHASH_ALG_ARGON2ID13 => Some(PWHASH_ARGON2ID_OPSLIMIT_MIN),
        _ => None,
    }
}

fn pwhash_algorithm(alg: c_int) -> Option<Argon2Algorithm> {
    match alg {
        PWHASH_ALG_ARGON2I13 => Some(Argon2Algorithm::Argon2i),
        PWHASH_ALG_ARGON2ID13 => Some(Argon2Algorithm::Argon2id),
        _ => None,
    }
}

fn argon2_params(out_len: usize, opslimit: u64, memlimit: usize) -> Option<Argon2Params> {
    let mem_cost = u32::try_from(memlimit / 1024).ok()?;
    let time_cost = u32::try_from(opslimit).ok()?;
    Argon2Params::new(mem_cost, time_cost, 1, Some(out_len)).ok()
}

unsafe fn pwhash_argon2_raw_inner(
    out: *mut u8,
    outlen: u64,
    passwd: *const c_char,
    passwdlen: u64,
    salt: *const u8,
    opslimit: u64,
    memlimit: usize,
    alg: c_int,
    required_alg: Option<c_int>,
) -> c_int {
    if outlen <= usize::MAX as u64 {
        zero_output(out, outlen as usize);
    }
    if let Some(expected_alg) = required_alg {
        if alg != expected_alg {
            set_errno(libc::EINVAL);
            return -1;
        }
    }
    if outlen > PWHASH_BYTES_MAX as u64
        || passwdlen > PWHASH_PASSWD_MAX as u64
        || opslimit > PWHASH_OPSLIMIT_MAX
        || memlimit > PWHASH_MEMLIMIT_MAX
    {
        set_errno(libc::EFBIG);
        return -1;
    }
    let Some(opslimit_min) = pwhash_opslimit_min(alg) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if outlen < PWHASH_BYTES_MIN as u64 || opslimit < opslimit_min || memlimit < PWHASH_MEMLIMIT_MIN
    {
        set_errno(libc::EINVAL);
        return -1;
    }
    let outlen = outlen as usize;
    let passwdlen = passwdlen as usize;
    let Some(algorithm) = pwhash_algorithm(alg) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let Some(params) = argon2_params(outlen, opslimit, memlimit) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let argon2 = Argon2::new(algorithm, Argon2Version::V0x13, params);
    argon2
        .hash_password_into(
            opt_slice(passwd.cast::<u8>(), passwdlen),
            opt_slice(salt, PWHASH_SALTBYTES),
            opt_slice_mut(out, outlen),
        )
        .map(|_| 0)
        .unwrap_or(-1)
}

unsafe fn pwhash_argon2_str_inner(
    out: *mut c_char,
    passwd: *const c_char,
    passwdlen: u64,
    opslimit: u64,
    memlimit: usize,
    alg: c_int,
) -> c_int {
    zero_char_output(out, PWHASH_STRBYTES);
    if passwdlen > PWHASH_PASSWD_MAX as u64
        || opslimit > PWHASH_OPSLIMIT_MAX
        || memlimit > PWHASH_MEMLIMIT_MAX
    {
        set_errno(libc::EFBIG);
        return -1;
    }
    let Some(opslimit_min) = pwhash_opslimit_min(alg) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if opslimit < opslimit_min || memlimit < PWHASH_MEMLIMIT_MIN {
        set_errno(libc::EINVAL);
        return -1;
    }
    let passwd = opt_slice(passwd.cast::<u8>(), passwdlen as usize);
    let Some(algorithm) = pwhash_algorithm(alg) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let Some(params) = argon2_params(32, opslimit, memlimit) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let mut salt = [0u8; PWHASH_SALTBYTES];
    fill_random_bytes(salt.as_mut_ptr(), salt.len());
    let argon2 = Argon2::new(algorithm, Argon2Version::V0x13, params);
    match argon2.hash_password_with_salt(passwd, &salt) {
        Ok(hash) => write_c_string(out, PWHASH_STRBYTES, &hash.to_string()),
        Err(_) => -1,
    }
}

fn argon2_matches_alg(hash: &PhcPasswordHash, alg: c_int) -> bool {
    match alg {
        PWHASH_ALG_ARGON2I13 => hash.algorithm.as_str() == "argon2i",
        PWHASH_ALG_ARGON2ID13 => hash.algorithm.as_str() == "argon2id",
        _ => false,
    }
}

unsafe fn pwhash_argon2_needs_rehash_inner(
    str_: *const c_char,
    opslimit: u64,
    memlimit: usize,
    alg: c_int,
) -> c_int {
    let Some(encoded) = read_c_string(str_) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let memlimit_kib = memlimit / 1024;
    if opslimit > u32::MAX as u64
        || memlimit_kib > u32::MAX as usize
        || encoded.len() >= PWHASH_STRBYTES
    {
        set_errno(libc::EINVAL);
        return -1;
    }
    let Ok(hash) = PhcPasswordHash::new(&encoded) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if !argon2_matches_alg(&hash, alg) {
        set_errno(libc::EINVAL);
        return -1;
    }
    let Some(actual_ops): Option<u32> = hash.params.get("t").and_then(|value| value.decimal().ok())
    else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let Some(actual_mem): Option<u32> = hash.params.get("m").and_then(|value| value.decimal().ok())
    else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if u64::from(actual_ops) != opslimit || usize::try_from(actual_mem).ok() != Some(memlimit_kib) {
        return 1;
    }
    0
}

unsafe fn pwhash_argon2_verify_inner(
    str_: *const c_char,
    passwd: *const c_char,
    passwdlen: u64,
    alg: c_int,
) -> c_int {
    if passwdlen > PWHASH_PASSWD_MAX as u64 {
        set_errno(libc::EFBIG);
        return -1;
    }
    let Some(encoded) = read_c_string(str_) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let Ok(hash) = PhcPasswordHash::new(&encoded) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if !argon2_matches_alg(&hash, alg) {
        set_errno(libc::EINVAL);
        return -1;
    }
    let passwd = opt_slice(passwd.cast::<u8>(), passwdlen as usize);
    match Argon2::default().verify_password(passwd, &hash) {
        Ok(()) => 0,
        Err(_) => {
            set_errno(libc::EINVAL);
            -1
        }
    }
}

fn scrypt_pickparams(opslimit: u64, memlimit: usize) -> (u8, u32, u32) {
    let opslimit = opslimit.max(PWHASH_SCRYPT_OPSLIMIT_MIN);
    let r = 8u32;
    if opslimit < (memlimit / 32) as u64 {
        let max_n = opslimit / (u64::from(r) * 4);
        let mut log_n = 1u8;
        while log_n < 63 {
            if (1u64 << log_n) > max_n / 2 {
                break;
            }
            log_n += 1;
        }
        (log_n, r, 1)
    } else {
        let max_n = (memlimit / (r as usize * 128)) as u64;
        let mut log_n = 1u8;
        while log_n < 63 {
            if (1u64 << log_n) > max_n / 2 {
                break;
            }
            log_n += 1;
        }
        let mut max_rp = (opslimit / 4) / (1u64 << log_n);
        if max_rp > 0x3fff_ffff {
            max_rp = 0x3fff_ffff;
        }
        (log_n, r, (max_rp as u32) / r)
    }
}

fn scrypt_decode_one(byte: u8) -> Option<u32> {
    SCRYPT_ITOA64
        .iter()
        .position(|&candidate| candidate == byte)
        .map(|value| value as u32)
}

fn scrypt_decode64_uint32(src: &[u8]) -> Option<u32> {
    let mut value = 0u32;
    for (index, byte) in src.iter().copied().take(5).enumerate() {
        value |= scrypt_decode_one(byte)? << (6 * index);
    }
    Some(value)
}

fn scrypt_push_u32_encoded(dst: &mut String, mut value: u32, bits: u32) {
    let mut bit = 0u32;
    while bit < bits {
        dst.push(SCRYPT_ITOA64[(value & 0x3f) as usize] as char);
        value >>= 6;
        bit += 6;
    }
}

fn scrypt_push_bytes_encoded(dst: &mut String, src: &[u8]) {
    let mut index = 0usize;
    while index < src.len() {
        let mut value = 0u32;
        let mut bits = 0u32;
        while bits < 24 && index < src.len() {
            value |= u32::from(src[index]) << bits;
            bits += 8;
            index += 1;
        }
        scrypt_push_u32_encoded(dst, value, bits);
    }
}

fn scrypt_format_setting(log_n: u8, r: u32, p: u32, salt: &[u8]) -> Option<String> {
    if log_n > 63 || u64::from(r) * u64::from(p) >= (1u64 << 30) {
        return None;
    }
    let mut setting = String::with_capacity(PWHASH_SCRYPT_STRSETTINGBYTES);
    setting.push_str("$7$");
    setting.push(SCRYPT_ITOA64[log_n as usize] as char);
    scrypt_push_u32_encoded(&mut setting, r, 30);
    scrypt_push_u32_encoded(&mut setting, p, 30);
    scrypt_push_bytes_encoded(&mut setting, salt);
    if setting.len() == PWHASH_SCRYPT_STRSETTINGBYTES {
        Some(setting)
    } else {
        None
    }
}

fn scrypt_parse_setting(setting: &[u8]) -> Option<(u8, u32, u32)> {
    if setting.len() < 14 || &setting[..3] != b"$7$" {
        return None;
    }
    let log_n = scrypt_decode_one(setting[3])? as u8;
    let r = scrypt_decode64_uint32(&setting[4..9])?;
    let p = scrypt_decode64_uint32(&setting[9..14])?;
    Some((log_n, r, p))
}

fn scrypt_hash_inner(
    passwd: &[u8],
    salt: &[u8],
    log_n: u8,
    r: u32,
    p: u32,
    out: &mut [u8],
) -> c_int {
    let Ok(params) = ScryptParams::new(log_n, r, p) else {
        unsafe {
            set_errno(libc::EINVAL);
        }
        return -1;
    };
    scrypt::scrypt(passwd, salt, &params, out)
        .map(|_| 0)
        .unwrap_or_else(|_| {
            unsafe {
                set_errno(libc::EINVAL);
            }
            -1
        })
}

unsafe fn pwhash_scrypt_inner(
    out: *mut u8,
    outlen: u64,
    passwd: *const c_char,
    passwdlen: u64,
    salt: *const u8,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    if outlen <= usize::MAX as u64 {
        zero_output(out, outlen as usize);
    }
    if passwdlen > usize::MAX as u64 || outlen > PWHASH_SCRYPT_BYTES_MAX as u64 {
        set_errno(libc::EFBIG);
        return -1;
    }
    if outlen < PWHASH_SCRYPT_BYTES_MIN as u64 {
        set_errno(libc::EINVAL);
        return -1;
    }
    let (log_n, r, p) = scrypt_pickparams(opslimit, memlimit);
    scrypt_hash_inner(
        opt_slice(passwd.cast::<u8>(), passwdlen as usize),
        opt_slice(salt, PWHASH_SCRYPT_SALTBYTES),
        log_n,
        r,
        p,
        opt_slice_mut(out, outlen as usize),
    )
}

unsafe fn pwhash_scrypt_str_inner(
    out: *mut c_char,
    passwd: *const c_char,
    passwdlen: u64,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    zero_char_output(out, PWHASH_SCRYPT_STRBYTES);
    if passwdlen > usize::MAX as u64 {
        set_errno(libc::EFBIG);
        return -1;
    }
    let (log_n, r, p) = scrypt_pickparams(opslimit, memlimit);
    let mut raw_salt = [0u8; PWHASH_SCRYPT_STRSALTBYTES];
    fill_random_bytes(raw_salt.as_mut_ptr(), raw_salt.len());
    let Some(setting) = scrypt_format_setting(log_n, r, p, &raw_salt) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let mut hash = [0u8; PWHASH_SCRYPT_STRHASHBYTES];
    if scrypt_hash_inner(
        opt_slice(passwd.cast::<u8>(), passwdlen as usize),
        &setting.as_bytes()[14..],
        log_n,
        r,
        p,
        &mut hash,
    ) != 0
    {
        return -1;
    }
    let mut encoded_hash = String::with_capacity(43);
    scrypt_push_bytes_encoded(&mut encoded_hash, &hash);
    let value = format!("{setting}${encoded_hash}");
    write_c_string(out, PWHASH_SCRYPT_STRBYTES, &value)
}

unsafe fn pwhash_scrypt_verify_inner(
    str_: *const c_char,
    passwd: *const c_char,
    passwdlen: u64,
) -> c_int {
    if c_strnlen(str_, PWHASH_SCRYPT_STRBYTES) != PWHASH_SCRYPT_STRBYTES - 1 {
        return -1;
    }
    let Some(encoded) = read_c_string(str_) else {
        return -1;
    };
    let bytes = encoded.as_bytes();
    let Some(last_dollar) = bytes.iter().rposition(|&byte| byte == b'$') else {
        return -1;
    };
    if last_dollar + 1 + 43 != PWHASH_SCRYPT_STRBYTES - 1 {
        return -1;
    }
    let Some((log_n, r, p)) = scrypt_parse_setting(&bytes[..14]) else {
        return -1;
    };
    let mut hash = [0u8; PWHASH_SCRYPT_STRHASHBYTES];
    if scrypt_hash_inner(
        opt_slice(passwd.cast::<u8>(), passwdlen as usize),
        &bytes[14..last_dollar],
        log_n,
        r,
        p,
        &mut hash,
    ) != 0
    {
        return -1;
    }
    let mut encoded_hash = String::with_capacity(43);
    scrypt_push_bytes_encoded(&mut encoded_hash, &hash);
    let mut expected = bytes[..last_dollar].to_vec();
    expected.push(b'$');
    expected.extend_from_slice(encoded_hash.as_bytes());
    if ct_eq(&expected, bytes) {
        0
    } else {
        -1
    }
}

pub unsafe fn crypto_pwhash(
    out: *mut u8,
    outlen: u64,
    passwd: *const c_char,
    passwdlen: u64,
    salt: *const u8,
    opslimit: u64,
    memlimit: usize,
    alg: c_int,
) -> c_int {
    if pwhash_algorithm(alg).is_none() {
        set_errno(libc::EINVAL);
        return -1;
    }
    pwhash_argon2_raw_inner(
        out, outlen, passwd, passwdlen, salt, opslimit, memlimit, alg, None,
    )
}

pub unsafe fn crypto_pwhash_alg_argon2i13() -> c_int {
    PWHASH_ALG_ARGON2I13
}
pub unsafe fn crypto_pwhash_alg_argon2id13() -> c_int {
    PWHASH_ALG_ARGON2ID13
}
pub unsafe fn crypto_pwhash_alg_default() -> c_int {
    PWHASH_ALG_DEFAULT
}

pub unsafe fn crypto_pwhash_argon2i(
    out: *mut u8,
    outlen: u64,
    passwd: *const c_char,
    passwdlen: u64,
    salt: *const u8,
    opslimit: u64,
    memlimit: usize,
    alg: c_int,
) -> c_int {
    pwhash_argon2_raw_inner(
        out,
        outlen,
        passwd,
        passwdlen,
        salt,
        opslimit,
        memlimit,
        alg,
        Some(PWHASH_ALG_ARGON2I13),
    )
}

pub unsafe fn crypto_pwhash_argon2i_alg_argon2i13() -> c_int {
    PWHASH_ALG_ARGON2I13
}
pub unsafe fn crypto_pwhash_argon2i_bytes_max() -> usize {
    PWHASH_BYTES_MAX
}
pub unsafe fn crypto_pwhash_argon2i_bytes_min() -> usize {
    PWHASH_BYTES_MIN
}
pub unsafe fn crypto_pwhash_argon2i_memlimit_interactive() -> usize {
    PWHASH_ARGON2I_MEMLIMIT_INTERACTIVE
}
pub unsafe fn crypto_pwhash_argon2i_memlimit_max() -> usize {
    PWHASH_MEMLIMIT_MAX
}
pub unsafe fn crypto_pwhash_argon2i_memlimit_min() -> usize {
    PWHASH_MEMLIMIT_MIN
}
pub unsafe fn crypto_pwhash_argon2i_memlimit_moderate() -> usize {
    PWHASH_ARGON2I_MEMLIMIT_MODERATE
}
pub unsafe fn crypto_pwhash_argon2i_memlimit_sensitive() -> usize {
    PWHASH_ARGON2I_MEMLIMIT_SENSITIVE
}
pub unsafe fn crypto_pwhash_argon2i_opslimit_interactive() -> usize {
    PWHASH_ARGON2I_OPSLIMIT_INTERACTIVE as usize
}
pub unsafe fn crypto_pwhash_argon2i_opslimit_max() -> usize {
    PWHASH_OPSLIMIT_MAX as usize
}
pub unsafe fn crypto_pwhash_argon2i_opslimit_min() -> usize {
    PWHASH_ARGON2I_OPSLIMIT_MIN as usize
}
pub unsafe fn crypto_pwhash_argon2i_opslimit_moderate() -> usize {
    PWHASH_ARGON2I_OPSLIMIT_MODERATE as usize
}
pub unsafe fn crypto_pwhash_argon2i_opslimit_sensitive() -> usize {
    PWHASH_ARGON2I_OPSLIMIT_SENSITIVE as usize
}
pub unsafe fn crypto_pwhash_argon2i_passwd_max() -> usize {
    PWHASH_PASSWD_MAX
}
pub unsafe fn crypto_pwhash_argon2i_passwd_min() -> usize {
    0
}
pub unsafe fn crypto_pwhash_argon2i_saltbytes() -> usize {
    PWHASH_SALTBYTES
}

pub unsafe fn crypto_pwhash_argon2i_str(
    out: *mut c_char,
    passwd: *const c_char,
    passwdlen: u64,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    pwhash_argon2_str_inner(
        out,
        passwd,
        passwdlen,
        opslimit,
        memlimit,
        PWHASH_ALG_ARGON2I13,
    )
}

pub unsafe fn crypto_pwhash_argon2i_str_needs_rehash(
    str_: *const c_char,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    pwhash_argon2_needs_rehash_inner(str_, opslimit, memlimit, PWHASH_ALG_ARGON2I13)
}

pub unsafe fn crypto_pwhash_argon2i_str_verify(
    str_: *const c_char,
    passwd: *const c_char,
    passwdlen: u64,
) -> c_int {
    pwhash_argon2_verify_inner(str_, passwd, passwdlen, PWHASH_ALG_ARGON2I13)
}

pub unsafe fn crypto_pwhash_argon2i_strbytes() -> usize {
    PWHASH_STRBYTES
}
pub unsafe fn crypto_pwhash_argon2i_strprefix() -> *const c_char {
    static_cstr(b"$argon2i$\0")
}

pub unsafe fn crypto_pwhash_argon2id(
    out: *mut u8,
    outlen: u64,
    passwd: *const c_char,
    passwdlen: u64,
    salt: *const u8,
    opslimit: u64,
    memlimit: usize,
    alg: c_int,
) -> c_int {
    pwhash_argon2_raw_inner(
        out,
        outlen,
        passwd,
        passwdlen,
        salt,
        opslimit,
        memlimit,
        alg,
        Some(PWHASH_ALG_ARGON2ID13),
    )
}

pub unsafe fn crypto_pwhash_argon2id_alg_argon2id13() -> c_int {
    PWHASH_ALG_ARGON2ID13
}
pub unsafe fn crypto_pwhash_argon2id_bytes_max() -> usize {
    PWHASH_BYTES_MAX
}
pub unsafe fn crypto_pwhash_argon2id_bytes_min() -> usize {
    PWHASH_BYTES_MIN
}
pub unsafe fn crypto_pwhash_argon2id_memlimit_interactive() -> usize {
    PWHASH_ARGON2ID_MEMLIMIT_INTERACTIVE
}
pub unsafe fn crypto_pwhash_argon2id_memlimit_max() -> usize {
    PWHASH_MEMLIMIT_MAX
}
pub unsafe fn crypto_pwhash_argon2id_memlimit_min() -> usize {
    PWHASH_MEMLIMIT_MIN
}
pub unsafe fn crypto_pwhash_argon2id_memlimit_moderate() -> usize {
    PWHASH_ARGON2ID_MEMLIMIT_MODERATE
}
pub unsafe fn crypto_pwhash_argon2id_memlimit_sensitive() -> usize {
    PWHASH_ARGON2ID_MEMLIMIT_SENSITIVE
}
pub unsafe fn crypto_pwhash_argon2id_opslimit_interactive() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_INTERACTIVE as usize
}
pub unsafe fn crypto_pwhash_argon2id_opslimit_max() -> usize {
    PWHASH_OPSLIMIT_MAX as usize
}
pub unsafe fn crypto_pwhash_argon2id_opslimit_min() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_MIN as usize
}
pub unsafe fn crypto_pwhash_argon2id_opslimit_moderate() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_MODERATE as usize
}
pub unsafe fn crypto_pwhash_argon2id_opslimit_sensitive() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_SENSITIVE as usize
}
pub unsafe fn crypto_pwhash_argon2id_passwd_max() -> usize {
    PWHASH_PASSWD_MAX
}
pub unsafe fn crypto_pwhash_argon2id_passwd_min() -> usize {
    0
}
pub unsafe fn crypto_pwhash_argon2id_saltbytes() -> usize {
    PWHASH_SALTBYTES
}

pub unsafe fn crypto_pwhash_argon2id_str(
    out: *mut c_char,
    passwd: *const c_char,
    passwdlen: u64,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    pwhash_argon2_str_inner(
        out,
        passwd,
        passwdlen,
        opslimit,
        memlimit,
        PWHASH_ALG_ARGON2ID13,
    )
}

pub unsafe fn crypto_pwhash_argon2id_str_needs_rehash(
    str_: *const c_char,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    pwhash_argon2_needs_rehash_inner(str_, opslimit, memlimit, PWHASH_ALG_ARGON2ID13)
}

pub unsafe fn crypto_pwhash_argon2id_str_verify(
    str_: *const c_char,
    passwd: *const c_char,
    passwdlen: u64,
) -> c_int {
    pwhash_argon2_verify_inner(str_, passwd, passwdlen, PWHASH_ALG_ARGON2ID13)
}

pub unsafe fn crypto_pwhash_argon2id_strbytes() -> usize {
    PWHASH_STRBYTES
}
pub unsafe fn crypto_pwhash_argon2id_strprefix() -> *const c_char {
    static_cstr(b"$argon2id$\0")
}
pub unsafe fn crypto_pwhash_bytes_max() -> usize {
    PWHASH_BYTES_MAX
}
pub unsafe fn crypto_pwhash_bytes_min() -> usize {
    PWHASH_BYTES_MIN
}
pub unsafe fn crypto_pwhash_memlimit_interactive() -> usize {
    PWHASH_ARGON2ID_MEMLIMIT_INTERACTIVE
}
pub unsafe fn crypto_pwhash_memlimit_max() -> usize {
    PWHASH_MEMLIMIT_MAX
}
pub unsafe fn crypto_pwhash_memlimit_min() -> usize {
    PWHASH_MEMLIMIT_MIN
}
pub unsafe fn crypto_pwhash_memlimit_moderate() -> usize {
    PWHASH_ARGON2ID_MEMLIMIT_MODERATE
}
pub unsafe fn crypto_pwhash_memlimit_sensitive() -> usize {
    PWHASH_ARGON2ID_MEMLIMIT_SENSITIVE
}
pub unsafe fn crypto_pwhash_opslimit_interactive() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_INTERACTIVE as usize
}
pub unsafe fn crypto_pwhash_opslimit_max() -> usize {
    PWHASH_OPSLIMIT_MAX as usize
}
pub unsafe fn crypto_pwhash_opslimit_min() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_MIN as usize
}
pub unsafe fn crypto_pwhash_opslimit_moderate() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_MODERATE as usize
}
pub unsafe fn crypto_pwhash_opslimit_sensitive() -> usize {
    PWHASH_ARGON2ID_OPSLIMIT_SENSITIVE as usize
}
pub unsafe fn crypto_pwhash_passwd_max() -> usize {
    PWHASH_PASSWD_MAX
}
pub unsafe fn crypto_pwhash_passwd_min() -> usize {
    0
}
pub unsafe fn crypto_pwhash_primitive() -> *const c_char {
    static_cstr(b"argon2i\0")
}
pub unsafe fn crypto_pwhash_saltbytes() -> usize {
    PWHASH_SALTBYTES
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256(
    out: *mut u8,
    outlen: u64,
    passwd: *const c_char,
    passwdlen: u64,
    salt: *const u8,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    pwhash_scrypt_inner(out, outlen, passwd, passwdlen, salt, opslimit, memlimit)
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256_bytes_max() -> usize {
    PWHASH_SCRYPT_BYTES_MAX
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_bytes_min() -> usize {
    PWHASH_SCRYPT_BYTES_MIN
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256_ll(
    passwd: *const u8,
    passwdlen: usize,
    salt: *const u8,
    saltlen: usize,
    n: u64,
    r: u32,
    p: u32,
    buf: *mut u8,
    buflen: usize,
) -> c_int {
    let Some(log_n) = n.checked_ilog2().and_then(|value| u8::try_from(value).ok()) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if n == 0 || (1u64 << log_n) != n {
        set_errno(libc::EINVAL);
        return -1;
    }
    scrypt_hash_inner(
        opt_slice(passwd, passwdlen),
        opt_slice(salt, saltlen),
        log_n,
        r,
        p,
        opt_slice_mut(buf, buflen),
    )
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256_memlimit_interactive() -> usize {
    PWHASH_SCRYPT_MEMLIMIT_INTERACTIVE
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_memlimit_max() -> usize {
    PWHASH_SCRYPT_MEMLIMIT_MAX
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_memlimit_min() -> usize {
    PWHASH_SCRYPT_MEMLIMIT_MIN
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_memlimit_sensitive() -> usize {
    PWHASH_SCRYPT_MEMLIMIT_SENSITIVE
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_opslimit_interactive() -> usize {
    PWHASH_SCRYPT_OPSLIMIT_INTERACTIVE as usize
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_opslimit_max() -> usize {
    PWHASH_SCRYPT_OPSLIMIT_MAX as usize
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_opslimit_min() -> usize {
    PWHASH_SCRYPT_OPSLIMIT_MIN as usize
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_opslimit_sensitive() -> usize {
    PWHASH_SCRYPT_OPSLIMIT_SENSITIVE as usize
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_passwd_max() -> usize {
    usize::MAX
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_passwd_min() -> usize {
    0
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_saltbytes() -> usize {
    PWHASH_SCRYPT_SALTBYTES
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256_str(
    out: *mut c_char,
    passwd: *const c_char,
    passwdlen: u64,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    pwhash_scrypt_str_inner(out, passwd, passwdlen, opslimit, memlimit)
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256_str_needs_rehash(
    str_: *const c_char,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    let (log_n, r, p) = scrypt_pickparams(opslimit, memlimit);
    if c_strnlen(str_, PWHASH_SCRYPT_STRBYTES) != PWHASH_SCRYPT_STRBYTES - 1 {
        set_errno(libc::EINVAL);
        return -1;
    }
    let Some(encoded) = read_c_string(str_) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    let Some((actual_log_n, actual_r, actual_p)) = scrypt_parse_setting(encoded.as_bytes()) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if actual_log_n != log_n || actual_r != r || actual_p != p {
        1
    } else {
        0
    }
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256_str_verify(
    str_: *const c_char,
    passwd: *const c_char,
    passwdlen: u64,
) -> c_int {
    pwhash_scrypt_verify_inner(str_, passwd, passwdlen)
}

pub unsafe fn crypto_pwhash_scryptsalsa208sha256_strbytes() -> usize {
    PWHASH_SCRYPT_STRBYTES
}
pub unsafe fn crypto_pwhash_scryptsalsa208sha256_strprefix() -> *const c_char {
    static_cstr(b"$7$\0")
}

pub unsafe fn crypto_pwhash_str(
    out: *mut c_char,
    passwd: *const c_char,
    passwdlen: u64,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    crypto_pwhash_str_alg(
        out,
        passwd,
        passwdlen,
        opslimit,
        memlimit,
        PWHASH_ALG_DEFAULT,
    )
}

pub unsafe fn crypto_pwhash_str_alg(
    out: *mut c_char,
    passwd: *const c_char,
    passwdlen: u64,
    opslimit: u64,
    memlimit: usize,
    alg: c_int,
) -> c_int {
    match alg {
        PWHASH_ALG_ARGON2I13 | PWHASH_ALG_ARGON2ID13 => {
            pwhash_argon2_str_inner(out, passwd, passwdlen, opslimit, memlimit, alg)
        }
        _ => crate::foundation::core::sodium_misuse(),
    }
}

pub unsafe fn crypto_pwhash_str_needs_rehash(
    str_: *const c_char,
    opslimit: u64,
    memlimit: usize,
) -> c_int {
    let Some(encoded) = read_c_string(str_) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if encoded.starts_with("$argon2id$") {
        return pwhash_argon2_needs_rehash_inner(str_, opslimit, memlimit, PWHASH_ALG_ARGON2ID13);
    }
    if encoded.starts_with("$argon2i$") {
        return pwhash_argon2_needs_rehash_inner(str_, opslimit, memlimit, PWHASH_ALG_ARGON2I13);
    }
    set_errno(libc::EINVAL);
    -1
}

pub unsafe fn crypto_pwhash_str_verify(
    str_: *const c_char,
    passwd: *const c_char,
    passwdlen: u64,
) -> c_int {
    let Some(encoded) = read_c_string(str_) else {
        set_errno(libc::EINVAL);
        return -1;
    };
    if encoded.starts_with("$argon2id$") {
        return pwhash_argon2_verify_inner(str_, passwd, passwdlen, PWHASH_ALG_ARGON2ID13);
    }
    if encoded.starts_with("$argon2i$") {
        return pwhash_argon2_verify_inner(str_, passwd, passwdlen, PWHASH_ALG_ARGON2I13);
    }
    set_errno(libc::EINVAL);
    -1
}

pub unsafe fn crypto_pwhash_strbytes() -> usize {
    PWHASH_STRBYTES
}
pub unsafe fn crypto_pwhash_strprefix() -> *const c_char {
    static_cstr(b"$argon2id$\0")
}

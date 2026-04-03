use crate::abi::types::*;
use crate::ffi::helpers::{copy_allow_overlap, opt_slice, opt_slice_mut, static_cstr, write_opt};
use crate::foundation::randombytes::fill_random_bytes;
use blake2b_simd::Params as Blake2bParams;
use core::ffi::{c_char, c_int};
use core::mem::size_of;
use core::ptr;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::montgomery::MontgomeryPoint;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use sha2::{Digest, Sha512};

const CURVE25519_BYTES: usize = 32;
const CURVE25519_SCALARBYTES: usize = 32;
const ED25519_BYTES: usize = 32;
const ED25519_HASHBYTES: usize = 64;
const ED25519_UNIFORMBYTES: usize = 32;
const ED25519_SCALARBYTES: usize = 32;
const ED25519_NONREDUCEDSCALARBYTES: usize = 64;
const SIGN_BYTES: usize = 64;
const SIGN_SEEDBYTES: usize = 32;
const SIGN_PUBLICKEYBYTES: usize = 32;
const SIGN_SECRETKEYBYTES: usize = 64;
const RISTRETTO_BYTES: usize = 32;
const RISTRETTO_HASHBYTES: usize = 64;
const RISTRETTO_SCALARBYTES: usize = 32;
const RISTRETTO_NONREDUCEDSCALARBYTES: usize = 64;
const BOX_SEEDBYTES: usize = 32;
const BOX_PUBLICKEYBYTES: usize = 32;
const BOX_SECRETKEYBYTES: usize = 32;
const BOX_BEFORENMBYTES: usize = 32;
const BOX_NONCEBYTES: usize = 24;
const BOX_MACBYTES: usize = 16;
const BOX_BOXZEROBYTES: usize = 16;
const BOX_ZEROBYTES: usize = 32;
const BOX_SEALBYTES: usize = BOX_PUBLICKEYBYTES + BOX_MACBYTES;
const KX_PUBLICKEYBYTES: usize = 32;
const KX_SECRETKEYBYTES: usize = 32;
const KX_SEEDBYTES: usize = 32;
const KX_SESSIONKEYBYTES: usize = 32;
const ED25519_L: [u8; ED25519_SCALARBYTES] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde,
    0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x10,
];
const DOM2_PREFIX: [u8; 34] = [
    b'S', b'i', b'g', b'E', b'd', b'2', b'5', b'5', b'1', b'9', b' ', b'n', b'o', b' ', b'E',
    b'd', b'2', b'5', b'5', b'1', b'9', b' ', b'c', b'o', b'l', b'l', b'i', b's', b'i', b'o',
    b'n', b's', 1, 0,
];

fn len_to_usize(len: u64) -> usize {
    usize::try_from(len).unwrap_or_else(|_| crate::foundation::core::sodium_misuse())
}

fn is_zero(bytes: &[u8]) -> bool {
    bytes.iter().all(|&byte| byte == 0)
}

unsafe fn read_array<const N: usize>(ptr: *const u8) -> [u8; N] {
    let mut out = [0u8; N];
    out.copy_from_slice(opt_slice(ptr, N));
    out
}

unsafe fn write_array<const N: usize>(ptr: *mut u8, bytes: &[u8; N]) {
    opt_slice_mut(ptr, N).copy_from_slice(bytes);
}

fn clamp_ed25519_scalar(mut bytes: [u8; 32]) -> [u8; 32] {
    bytes[0] &= 248;
    bytes[31] &= 127;
    bytes[31] |= 64;
    bytes
}

fn scalar_from_clamped_bytes(bytes: [u8; 32]) -> Scalar {
    Scalar::from_bytes_mod_order(clamp_ed25519_scalar(bytes))
}

fn scalar_from_255bit_bytes(mut bytes: [u8; 32]) -> Scalar {
    bytes[31] &= 127;
    Scalar::from_bytes_mod_order(bytes)
}

fn decode_edwards(bytes: &[u8; 32]) -> Option<EdwardsPoint> {
    CompressedEdwardsY(*bytes).decompress()
}

fn is_canonical_edwards(bytes: &[u8; 32]) -> bool {
    decode_edwards(bytes)
        .map(|point| point.compress().to_bytes() == *bytes)
        .unwrap_or(false)
}

fn decode_edwards_prime_subgroup(
    bytes: &[u8; 32],
    require_canonical: bool,
    reject_small_order: bool,
) -> Option<EdwardsPoint> {
    let point = decode_edwards(bytes)?;
    if require_canonical && point.compress().to_bytes() != *bytes {
        return None;
    }
    if reject_small_order && point.is_small_order() {
        return None;
    }
    point.is_torsion_free().then_some(point)
}

fn decode_ristretto(bytes: &[u8; 32]) -> Option<RistrettoPoint> {
    CompressedRistretto(*bytes).decompress()
}

fn blake2b<const N: usize>(chunks: &[&[u8]]) -> [u8; N] {
    let mut state = Blake2bParams::new().hash_length(N).to_state();
    for chunk in chunks {
        state.update(chunk);
    }
    let hash = state.finalize();
    let mut out = [0u8; N];
    out.copy_from_slice(hash.as_bytes());
    out
}

fn add_le_in_place(dst: &mut [u8], src: &[u8]) {
    debug_assert_eq!(dst.len(), src.len());
    let mut carry = 0u16;
    for (dst_byte, src_byte) in dst.iter_mut().zip(src.iter()) {
        let sum = *dst_byte as u16 + *src_byte as u16 + carry;
        *dst_byte = sum as u8;
        carry = sum >> 8;
    }
}

fn sub_le_in_place(dst: &mut [u8], src: &[u8]) {
    debug_assert_eq!(dst.len(), src.len());
    let mut borrow = 0i16;
    for (dst_byte, src_byte) in dst.iter_mut().zip(src.iter()) {
        let diff = *dst_byte as i16 - *src_byte as i16 - borrow;
        if diff < 0 {
            *dst_byte = (diff + 256) as u8;
            borrow = 1;
        } else {
            *dst_byte = diff as u8;
            borrow = 0;
        }
    }
}

fn reduce_scalar64(bytes: &[u8; 64]) -> [u8; 32] {
    Scalar::from_bytes_mod_order_wide(bytes).to_bytes()
}

fn sha512(bytes: &[u8]) -> [u8; 64] {
    let digest = Sha512::digest(bytes);
    let mut out = [0u8; 64];
    out.copy_from_slice(&digest);
    out
}

fn ed25519_seed_keypair(seed: &[u8; SIGN_SEEDBYTES]) -> ([u8; SIGN_PUBLICKEYBYTES], [u8; SIGN_SECRETKEYBYTES]) {
    let hash = sha512(seed);
    let scalar = scalar_from_clamped_bytes(hash[..32].try_into().unwrap());
    let pk = EdwardsPoint::mul_base(&scalar).compress().to_bytes();
    let mut sk = [0u8; SIGN_SECRETKEYBYTES];
    sk[..SIGN_SEEDBYTES].copy_from_slice(seed);
    sk[SIGN_SEEDBYTES..].copy_from_slice(&pk);
    (pk, sk)
}

fn legacy_edwards25519sha512batch_keypair() -> ([u8; SIGN_PUBLICKEYBYTES], [u8; SIGN_SECRETKEYBYTES]) {
    let mut seed = [0u8; SIGN_SEEDBYTES];
    fill_random_bytes(seed.as_mut_ptr(), seed.len());
    let mut sk = sha512(&seed);
    let scalar = scalar_from_clamped_bytes(sk[..32].try_into().unwrap());
    let clamped = clamp_ed25519_scalar(sk[..32].try_into().unwrap());
    sk[..32].copy_from_slice(&clamped);
    let pk = EdwardsPoint::mul_base(&scalar).compress().to_bytes();
    (pk, sk)
}

fn ed25519_dom2_prefix(hasher: &mut Sha512, prehashed: bool) {
    if prehashed {
        hasher.update(DOM2_PREFIX);
    }
}

fn ed25519_sign_detached(
    msg: &[u8],
    sk: &[u8; SIGN_SECRETKEYBYTES],
    prehashed: bool,
) -> [u8; SIGN_BYTES] {
    let az = sha512(&sk[..SIGN_SEEDBYTES]);
    let a = scalar_from_clamped_bytes(az[..32].try_into().unwrap());

    let mut nonce_hasher = Sha512::new();
    ed25519_dom2_prefix(&mut nonce_hasher, prehashed);
    nonce_hasher.update(&az[32..64]);
    nonce_hasher.update(msg);
    let nonce = nonce_hasher.finalize();
    let mut nonce_bytes = [0u8; 64];
    nonce_bytes.copy_from_slice(&nonce);
    let r = Scalar::from_bytes_mod_order_wide(&nonce_bytes);
    let r_bytes = EdwardsPoint::mul_base(&r).compress().to_bytes();

    let mut hram_hasher = Sha512::new();
    ed25519_dom2_prefix(&mut hram_hasher, prehashed);
    hram_hasher.update(r_bytes);
    hram_hasher.update(&sk[SIGN_SEEDBYTES..]);
    hram_hasher.update(msg);
    let hram = hram_hasher.finalize();
    let mut hram_bytes = [0u8; 64];
    hram_bytes.copy_from_slice(&hram);
    let k = Scalar::from_bytes_mod_order_wide(&hram_bytes);
    let s = (k * a + r).to_bytes();

    let mut sig = [0u8; SIGN_BYTES];
    sig[..32].copy_from_slice(&r_bytes);
    sig[32..].copy_from_slice(&s);
    sig
}

fn ed25519_verify_detached(
    sig: &[u8; SIGN_BYTES],
    msg: &[u8],
    pk: &[u8; SIGN_PUBLICKEYBYTES],
    prehashed: bool,
) -> c_int {
    let s = match Option::<Scalar>::from(Scalar::from_canonical_bytes(sig[32..].try_into().unwrap())) {
        Some(s) => s,
        None => return -1,
    };
    if let Some(r_point) = decode_edwards(&sig[..32].try_into().unwrap()) {
        if r_point.is_small_order() {
            return -1;
        }
    }
    let a = match decode_edwards_prime_subgroup(pk, true, true) {
        Some(point) => point,
        None => return -1,
    };

    let mut hram_hasher = Sha512::new();
    ed25519_dom2_prefix(&mut hram_hasher, prehashed);
    hram_hasher.update(&sig[..32]);
    hram_hasher.update(pk);
    hram_hasher.update(msg);
    let hram = hram_hasher.finalize();
    let mut hram_bytes = [0u8; 64];
    hram_bytes.copy_from_slice(&hram);
    let k = Scalar::from_bytes_mod_order_wide(&hram_bytes);
    let rcheck =
        EdwardsPoint::vartime_double_scalar_mul_basepoint(&k, &(-a), &s).compress().to_bytes();

    if rcheck == sig[..32] {
        0
    } else {
        -1
    }
}

fn legacy_edwards25519sha512batch_sign(msg: &[u8], sk: &[u8; SIGN_SECRETKEYBYTES]) -> Vec<u8> {
    let a = scalar_from_clamped_bytes(sk[..32].try_into().unwrap());

    let mut nonce_hasher = Sha512::new();
    nonce_hasher.update(&sk[32..]);
    nonce_hasher.update(msg);
    let nonce = nonce_hasher.finalize();
    let mut nonce_bytes = [0u8; 64];
    nonce_bytes.copy_from_slice(&nonce);
    let r = Scalar::from_bytes_mod_order_wide(&nonce_bytes);
    let r_bytes = EdwardsPoint::mul_base(&r).compress().to_bytes();

    let mut h_hasher = Sha512::new();
    h_hasher.update(r_bytes);
    h_hasher.update(msg);
    let h = h_hasher.finalize();
    let mut h_bytes = [0u8; 64];
    h_bytes.copy_from_slice(&h);
    let k = Scalar::from_bytes_mod_order_wide(&h_bytes);
    let s = (k * r + a).to_bytes();

    let mut out = vec![0u8; SIGN_BYTES + msg.len()];
    out[..32].copy_from_slice(&r_bytes);
    out[32..32 + msg.len()].copy_from_slice(msg);
    out[32 + msg.len()..].copy_from_slice(&s);
    out
}

fn legacy_edwards25519sha512batch_open(sm: &[u8], pk: &[u8; SIGN_PUBLICKEYBYTES]) -> Option<Vec<u8>> {
    if sm.len() < SIGN_BYTES {
        return None;
    }
    let mlen = sm.len() - SIGN_BYTES;
    if (sm[sm.len() - 1] & 0xe0) != 0 {
        return None;
    }
    let pk_point = decode_edwards_prime_subgroup(pk, false, true)?;
    let r_bytes: [u8; 32] = sm[..32].try_into().unwrap();
    let r_point = decode_edwards_prime_subgroup(&r_bytes, false, true)?;
    let s = scalar_from_255bit_bytes(sm[32 + mlen..].try_into().unwrap());

    let mut h_hasher = Sha512::new();
    h_hasher.update(&sm[..32 + mlen]);
    let h = h_hasher.finalize();
    let mut h_bytes = [0u8; 64];
    h_bytes.copy_from_slice(&h);
    let k = Scalar::from_bytes_mod_order_wide(&h_bytes);

    let lhs = EdwardsPoint::mul_base(&s);
    let rhs = &(&k * &r_point) + &pk_point;
    if lhs.compress().to_bytes() == rhs.compress().to_bytes() {
        Some(sm[32..32 + mlen].to_vec())
    } else {
        None
    }
}

fn curve25519_seed_keypair(seed: &[u8; BOX_SEEDBYTES]) -> ([u8; BOX_PUBLICKEYBYTES], [u8; BOX_SECRETKEYBYTES]) {
    let hash = sha512(seed);
    let mut sk = [0u8; BOX_SECRETKEYBYTES];
    sk.copy_from_slice(&hash[..32]);
    let pk = MontgomeryPoint::mul_base_clamped(sk).to_bytes();
    (pk, sk)
}

fn ed25519_from_uniform_bytes(bytes: &[u8; ED25519_UNIFORMBYTES]) -> [u8; ED25519_BYTES] {
    let hash = sha512(bytes);
    let scalar = Scalar::from_bytes_mod_order_wide(&hash);
    EdwardsPoint::mul_base(&scalar).compress().to_bytes()
}

fn ed25519_from_hash_bytes(bytes: &[u8; ED25519_HASHBYTES]) -> [u8; ED25519_BYTES] {
    let scalar = Scalar::from_bytes_mod_order_wide(bytes);
    EdwardsPoint::mul_base(&scalar).compress().to_bytes()
}

fn crypto_scalarmult_curve25519_inner(n: [u8; CURVE25519_SCALARBYTES], p: [u8; CURVE25519_BYTES]) -> ([u8; CURVE25519_BYTES], c_int) {
    let out = MontgomeryPoint(p).mul_clamped(n).to_bytes();
    let ret = if is_zero(&out) { -1 } else { 0 };
    (out, ret)
}

fn crypto_scalarmult_ed25519_inner(
    n: [u8; ED25519_SCALARBYTES],
    p: [u8; ED25519_BYTES],
    clamp: bool,
) -> Result<[u8; ED25519_BYTES], c_int> {
    let point = decode_edwards_prime_subgroup(&p, true, true).ok_or(-1)?;
    let scalar = if clamp {
        scalar_from_clamped_bytes(n)
    } else {
        scalar_from_255bit_bytes(n)
    };
    Ok((&scalar * &point).compress().to_bytes())
}

fn crypto_scalarmult_ed25519_base_inner(
    n: [u8; ED25519_SCALARBYTES],
    clamp: bool,
) -> ([u8; ED25519_BYTES], c_int) {
    let scalar = if clamp {
        scalar_from_clamped_bytes(n)
    } else {
        scalar_from_255bit_bytes(n)
    };
    let out = EdwardsPoint::mul_base(&scalar).compress().to_bytes();
    let ret = if out == EdwardsPoint::identity().compress().to_bytes() || is_zero(&n) {
        -1
    } else {
        0
    };
    (out, ret)
}

fn crypto_scalarmult_ristretto255_inner(
    n: [u8; RISTRETTO_SCALARBYTES],
    p: [u8; RISTRETTO_BYTES],
) -> Result<[u8; RISTRETTO_BYTES], c_int> {
    let point = decode_ristretto(&p).ok_or(-1)?;
    let scalar = scalar_from_255bit_bytes(n);
    Ok((&scalar * &point).compress().to_bytes())
}

fn crypto_scalarmult_ristretto255_base_inner(n: [u8; RISTRETTO_SCALARBYTES]) -> ([u8; RISTRETTO_BYTES], c_int) {
    let scalar = scalar_from_255bit_bytes(n);
    let out = (&scalar * &RISTRETTO_BASEPOINT_POINT).compress().to_bytes();
    let ret = if is_zero(&out) { -1 } else { 0 };
    (out, ret)
}

fn crypto_box_curve25519xsalsa20poly1305_beforenm_inner(
    pk: [u8; BOX_PUBLICKEYBYTES],
    sk: [u8; BOX_SECRETKEYBYTES],
) -> Result<[u8; BOX_BEFORENMBYTES], c_int> {
    let (shared, ret) = crypto_scalarmult_curve25519_inner(sk, pk);
    if ret != 0 {
        return Err(-1);
    }
    let zero = [0u8; 16];
    let mut k = [0u8; BOX_BEFORENMBYTES];
    unsafe {
        crate::symmetric_impl::crypto_core_hsalsa20(
            k.as_mut_ptr(),
            zero.as_ptr(),
            shared.as_ptr(),
            ptr::null(),
        );
    }
    Ok(k)
}

fn crypto_box_curve25519xchacha20poly1305_beforenm_inner(
    pk: [u8; BOX_PUBLICKEYBYTES],
    sk: [u8; BOX_SECRETKEYBYTES],
) -> Result<[u8; BOX_BEFORENMBYTES], c_int> {
    let (shared, ret) = crypto_scalarmult_curve25519_inner(sk, pk);
    if ret != 0 {
        return Err(-1);
    }
    let zero = [0u8; 16];
    let mut k = [0u8; BOX_BEFORENMBYTES];
    unsafe {
        crate::symmetric_impl::crypto_core_hchacha20(
            k.as_mut_ptr(),
            zero.as_ptr(),
            shared.as_ptr(),
            ptr::null(),
        );
    }
    Ok(k)
}

fn box_seal_nonce<const N: usize>(pk1: &[u8; BOX_PUBLICKEYBYTES], pk2: &[u8; BOX_PUBLICKEYBYTES]) -> [u8; N] {
    blake2b(&[pk1, pk2])
}

pub unsafe fn crypto_scalarmult(q: *mut u8, n: *const u8, p: *const u8) -> c_int {
    crypto_scalarmult_curve25519(q, n, p)
}

pub unsafe fn crypto_scalarmult_base(q: *mut u8, n: *const u8) -> c_int {
    crypto_scalarmult_curve25519_base(q, n)
}

pub unsafe fn crypto_scalarmult_bytes() -> usize {
    CURVE25519_BYTES
}

pub unsafe fn crypto_scalarmult_scalarbytes() -> usize {
    CURVE25519_SCALARBYTES
}

pub unsafe fn crypto_scalarmult_primitive() -> *const c_char {
    static_cstr(b"curve25519\0")
}

pub unsafe fn crypto_scalarmult_curve25519(q: *mut u8, n: *const u8, p: *const u8) -> c_int {
    let (out, ret) = crypto_scalarmult_curve25519_inner(read_array(n), read_array(p));
    write_array(q, &out);
    ret
}

pub unsafe fn crypto_scalarmult_curve25519_base(q: *mut u8, n: *const u8) -> c_int {
    let out = MontgomeryPoint::mul_base_clamped(read_array(n)).to_bytes();
    write_array(q, &out);
    0
}

pub unsafe fn crypto_scalarmult_curve25519_bytes() -> usize {
    CURVE25519_BYTES
}

pub unsafe fn crypto_scalarmult_curve25519_scalarbytes() -> usize {
    CURVE25519_SCALARBYTES
}

pub unsafe fn crypto_scalarmult_ed25519(
    q: *mut u8,
    n: *const u8,
    p: *const u8,
) -> c_int {
    let scalar_bytes = read_array(n);
    let ret = match crypto_scalarmult_ed25519_inner(scalar_bytes, read_array(p), true) {
        Ok(out) => {
            write_array(q, &out);
            if out == EdwardsPoint::identity().compress().to_bytes() || is_zero(&scalar_bytes) {
                -1
            } else {
                0
            }
        }
        Err(ret) => ret,
    };
    ret
}

pub unsafe fn crypto_scalarmult_ed25519_noclamp(
    q: *mut u8,
    n: *const u8,
    p: *const u8,
) -> c_int {
    let scalar_bytes = read_array(n);
    match crypto_scalarmult_ed25519_inner(scalar_bytes, read_array(p), false) {
        Ok(out) => {
            write_array(q, &out);
            if out == EdwardsPoint::identity().compress().to_bytes() || is_zero(&scalar_bytes) {
                -1
            } else {
                0
            }
        }
        Err(ret) => ret,
    }
}

pub unsafe fn crypto_scalarmult_ed25519_base(q: *mut u8, n: *const u8) -> c_int {
    let (out, ret) = crypto_scalarmult_ed25519_base_inner(read_array(n), true);
    write_array(q, &out);
    ret
}

pub unsafe fn crypto_scalarmult_ed25519_base_noclamp(q: *mut u8, n: *const u8) -> c_int {
    let (out, ret) = crypto_scalarmult_ed25519_base_inner(read_array(n), false);
    write_array(q, &out);
    ret
}

pub unsafe fn crypto_scalarmult_ed25519_bytes() -> usize {
    ED25519_BYTES
}

pub unsafe fn crypto_scalarmult_ed25519_scalarbytes() -> usize {
    ED25519_SCALARBYTES
}

pub unsafe fn crypto_scalarmult_ristretto255(
    q: *mut u8,
    n: *const u8,
    p: *const u8,
) -> c_int {
    match crypto_scalarmult_ristretto255_inner(read_array(n), read_array(p)) {
        Ok(out) => {
            write_array(q, &out);
            if is_zero(&out) {
                -1
            } else {
                0
            }
        }
        Err(ret) => ret,
    }
}

pub unsafe fn crypto_scalarmult_ristretto255_base(q: *mut u8, n: *const u8) -> c_int {
    let (out, ret) = crypto_scalarmult_ristretto255_base_inner(read_array(n));
    write_array(q, &out);
    ret
}

pub unsafe fn crypto_scalarmult_ristretto255_bytes() -> usize {
    RISTRETTO_BYTES
}

pub unsafe fn crypto_scalarmult_ristretto255_scalarbytes() -> usize {
    RISTRETTO_SCALARBYTES
}

pub unsafe fn crypto_core_ed25519_is_valid_point(p: *const u8) -> c_int {
    let p = read_array(p);
    c_int::from(
        is_canonical_edwards(&p)
            && decode_edwards_prime_subgroup(&p, false, true).is_some()
            && !decode_edwards(&p).unwrap().is_small_order(),
    )
}

pub unsafe fn crypto_core_ed25519_add(r: *mut u8, p: *const u8, q: *const u8) -> c_int {
    let p = match decode_edwards(&read_array(p)) {
        Some(point) => point,
        None => return -1,
    };
    let q = match decode_edwards(&read_array(q)) {
        Some(point) => point,
        None => return -1,
    };
    write_array(r, &(p + q).compress().to_bytes());
    0
}

pub unsafe fn crypto_core_ed25519_sub(r: *mut u8, p: *const u8, q: *const u8) -> c_int {
    let p = match decode_edwards(&read_array(p)) {
        Some(point) => point,
        None => return -1,
    };
    let q = match decode_edwards(&read_array(q)) {
        Some(point) => point,
        None => return -1,
    };
    write_array(r, &(p - q).compress().to_bytes());
    0
}

pub unsafe fn crypto_core_ed25519_from_uniform(p: *mut u8, r: *const u8) -> c_int {
    let point = ed25519_from_uniform_bytes(&read_array(r));
    write_array(p, &point);
    0
}

pub unsafe fn crypto_core_ed25519_from_hash(p: *mut u8, h: *const u8) -> c_int {
    let point = ed25519_from_hash_bytes(&read_array(h));
    write_array(p, &point);
    0
}

pub unsafe fn crypto_core_ed25519_random(p: *mut u8) {
    let mut hash = [0u8; ED25519_HASHBYTES];
    fill_random_bytes(hash.as_mut_ptr(), hash.len());
    let point = ed25519_from_hash_bytes(&hash);
    write_array(p, &point);
}

pub unsafe fn crypto_core_ed25519_scalar_random(r: *mut u8) {
    let out = opt_slice_mut(r, ED25519_SCALARBYTES);
    loop {
        fill_random_bytes(out.as_mut_ptr(), out.len());
        out[ED25519_SCALARBYTES - 1] &= 0x1f;
        if Option::<Scalar>::from(Scalar::from_canonical_bytes(out.try_into().unwrap())).is_some()
            && !is_zero(out)
        {
            break;
        }
    }
}

pub unsafe fn crypto_core_ed25519_scalar_invert(recip: *mut u8, s: *const u8) -> c_int {
    let s_bytes = read_array(s);
    let inv = Scalar::from_bytes_mod_order(s_bytes).invert().to_bytes();
    write_array(recip, &inv);
    if is_zero(&s_bytes) {
        -1
    } else {
        0
    }
}

pub unsafe fn crypto_core_ed25519_scalar_negate(neg: *mut u8, s: *const u8) {
    let mut t = [0u8; ED25519_NONREDUCEDSCALARBYTES];
    let mut s64 = [0u8; ED25519_NONREDUCEDSCALARBYTES];
    t[ED25519_SCALARBYTES..].copy_from_slice(&ED25519_L);
    s64[..ED25519_SCALARBYTES].copy_from_slice(&read_array::<ED25519_SCALARBYTES>(s));
    sub_le_in_place(&mut t, &s64);
    write_array(neg, &reduce_scalar64(&t));
}

pub unsafe fn crypto_core_ed25519_scalar_complement(comp: *mut u8, s: *const u8) {
    let mut t = [0u8; ED25519_NONREDUCEDSCALARBYTES];
    let mut s64 = [0u8; ED25519_NONREDUCEDSCALARBYTES];
    t[0] = 1;
    t[ED25519_SCALARBYTES..].copy_from_slice(&ED25519_L);
    s64[..ED25519_SCALARBYTES].copy_from_slice(&read_array::<ED25519_SCALARBYTES>(s));
    sub_le_in_place(&mut t, &s64);
    write_array(comp, &reduce_scalar64(&t));
}

pub unsafe fn crypto_core_ed25519_scalar_add(z: *mut u8, x: *const u8, y: *const u8) {
    let mut wide = [0u8; ED25519_NONREDUCEDSCALARBYTES];
    wide[..ED25519_SCALARBYTES].copy_from_slice(&read_array::<ED25519_SCALARBYTES>(x));
    add_le_in_place(
        &mut wide[..ED25519_SCALARBYTES],
        opt_slice(y, ED25519_SCALARBYTES),
    );
    write_array(z, &reduce_scalar64(&wide));
}

pub unsafe fn crypto_core_ed25519_scalar_sub(z: *mut u8, x: *const u8, y: *const u8) {
    let mut yn = [0u8; ED25519_SCALARBYTES];
    crypto_core_ed25519_scalar_negate(yn.as_mut_ptr(), y);
    crypto_core_ed25519_scalar_add(z, x, yn.as_ptr());
}

pub unsafe fn crypto_core_ed25519_scalar_mul(z: *mut u8, x: *const u8, y: *const u8) {
    let x = Scalar::from_bytes_mod_order(read_array(x));
    let y = Scalar::from_bytes_mod_order(read_array(y));
    write_array(z, &(x * y).to_bytes());
}

pub unsafe fn crypto_core_ed25519_scalar_reduce(r: *mut u8, s: *const u8) {
    write_array(r, &Scalar::from_bytes_mod_order_wide(&read_array(s)).to_bytes());
}

pub unsafe fn crypto_core_ed25519_bytes() -> usize {
    ED25519_BYTES
}

pub unsafe fn crypto_core_ed25519_uniformbytes() -> usize {
    ED25519_UNIFORMBYTES
}

pub unsafe fn crypto_core_ed25519_hashbytes() -> usize {
    ED25519_HASHBYTES
}

pub unsafe fn crypto_core_ed25519_scalarbytes() -> usize {
    ED25519_SCALARBYTES
}

pub unsafe fn crypto_core_ed25519_nonreducedscalarbytes() -> usize {
    ED25519_NONREDUCEDSCALARBYTES
}

pub unsafe fn crypto_core_ristretto255_is_valid_point(p: *const u8) -> c_int {
    c_int::from(decode_ristretto(&read_array(p)).is_some())
}

pub unsafe fn crypto_core_ristretto255_add(r: *mut u8, p: *const u8, q: *const u8) -> c_int {
    let p = match decode_ristretto(&read_array(p)) {
        Some(point) => point,
        None => return -1,
    };
    let q = match decode_ristretto(&read_array(q)) {
        Some(point) => point,
        None => return -1,
    };
    write_array(r, &(p + q).compress().to_bytes());
    0
}

pub unsafe fn crypto_core_ristretto255_sub(r: *mut u8, p: *const u8, q: *const u8) -> c_int {
    let p = match decode_ristretto(&read_array(p)) {
        Some(point) => point,
        None => return -1,
    };
    let q = match decode_ristretto(&read_array(q)) {
        Some(point) => point,
        None => return -1,
    };
    write_array(r, &(p - q).compress().to_bytes());
    0
}

pub unsafe fn crypto_core_ristretto255_from_hash(p: *mut u8, r: *const u8) -> c_int {
    write_array(p, &RistrettoPoint::from_uniform_bytes(&read_array(r)).compress().to_bytes());
    0
}

pub unsafe fn crypto_core_ristretto255_random(p: *mut u8) {
    let mut hash = [0u8; RISTRETTO_HASHBYTES];
    fill_random_bytes(hash.as_mut_ptr(), hash.len());
    write_array(p, &RistrettoPoint::from_uniform_bytes(&hash).compress().to_bytes());
}

pub unsafe fn crypto_core_ristretto255_scalar_random(r: *mut u8) {
    crypto_core_ed25519_scalar_random(r)
}

pub unsafe fn crypto_core_ristretto255_scalar_invert(recip: *mut u8, s: *const u8) -> c_int {
    crypto_core_ed25519_scalar_invert(recip, s)
}

pub unsafe fn crypto_core_ristretto255_scalar_negate(neg: *mut u8, s: *const u8) {
    crypto_core_ed25519_scalar_negate(neg, s)
}

pub unsafe fn crypto_core_ristretto255_scalar_complement(comp: *mut u8, s: *const u8) {
    crypto_core_ed25519_scalar_complement(comp, s)
}

pub unsafe fn crypto_core_ristretto255_scalar_add(z: *mut u8, x: *const u8, y: *const u8) {
    crypto_core_ed25519_scalar_add(z, x, y)
}

pub unsafe fn crypto_core_ristretto255_scalar_sub(z: *mut u8, x: *const u8, y: *const u8) {
    crypto_core_ed25519_scalar_sub(z, x, y)
}

pub unsafe fn crypto_core_ristretto255_scalar_mul(z: *mut u8, x: *const u8, y: *const u8) {
    crypto_core_ed25519_scalar_mul(z, x, y)
}

pub unsafe fn crypto_core_ristretto255_scalar_reduce(r: *mut u8, s: *const u8) {
    crypto_core_ed25519_scalar_reduce(r, s)
}

pub unsafe fn crypto_core_ristretto255_bytes() -> usize {
    RISTRETTO_BYTES
}

pub unsafe fn crypto_core_ristretto255_hashbytes() -> usize {
    RISTRETTO_HASHBYTES
}

pub unsafe fn crypto_core_ristretto255_scalarbytes() -> usize {
    RISTRETTO_SCALARBYTES
}

pub unsafe fn crypto_core_ristretto255_nonreducedscalarbytes() -> usize {
    RISTRETTO_NONREDUCEDSCALARBYTES
}

pub unsafe fn crypto_sign_ed25519ph_statebytes() -> usize {
    size_of::<crypto_sign_ed25519ph_state>()
}

pub unsafe fn crypto_sign_ed25519_bytes() -> usize {
    SIGN_BYTES
}

pub unsafe fn crypto_sign_ed25519_seedbytes() -> usize {
    SIGN_SEEDBYTES
}

pub unsafe fn crypto_sign_ed25519_publickeybytes() -> usize {
    SIGN_PUBLICKEYBYTES
}

pub unsafe fn crypto_sign_ed25519_secretkeybytes() -> usize {
    SIGN_SECRETKEYBYTES
}

pub unsafe fn crypto_sign_ed25519_messagebytes_max() -> usize {
    usize::MAX - SIGN_BYTES
}

pub unsafe fn crypto_sign_ed25519_seed_keypair(
    pk: *mut u8,
    sk: *mut u8,
    seed: *const u8,
) -> c_int {
    let (pk_bytes, sk_bytes) = ed25519_seed_keypair(&read_array(seed));
    write_array(pk, &pk_bytes);
    write_array(sk, &sk_bytes);
    0
}

pub unsafe fn crypto_sign_ed25519_keypair(pk: *mut u8, sk: *mut u8) -> c_int {
    let mut seed = [0u8; SIGN_SEEDBYTES];
    fill_random_bytes(seed.as_mut_ptr(), seed.len());
    crypto_sign_ed25519_seed_keypair(pk, sk, seed.as_ptr())
}

pub unsafe fn crypto_sign_ed25519_pk_to_curve25519(
    curve25519_pk: *mut u8,
    ed25519_pk: *const u8,
) -> c_int {
    let point = match decode_edwards_prime_subgroup(&read_array(ed25519_pk), false, true) {
        Some(point) => point,
        None => return -1,
    };
    write_array(curve25519_pk, &point.to_montgomery().to_bytes());
    0
}

pub unsafe fn crypto_sign_ed25519_sk_to_curve25519(
    curve25519_sk: *mut u8,
    ed25519_sk: *const u8,
) -> c_int {
    let hash = sha512(opt_slice(ed25519_sk, SIGN_SEEDBYTES));
    let mut out = [0u8; CURVE25519_BYTES];
    out.copy_from_slice(&clamp_ed25519_scalar(hash[..32].try_into().unwrap()));
    write_array(curve25519_sk, &out);
    0
}

pub unsafe fn crypto_sign_ed25519_sk_to_seed(seed: *mut u8, sk: *const u8) -> c_int {
    copy_allow_overlap(seed, sk, SIGN_SEEDBYTES);
    0
}

pub unsafe fn crypto_sign_ed25519_sk_to_pk(pk: *mut u8, sk: *const u8) -> c_int {
    copy_allow_overlap(pk, sk.add(SIGN_SEEDBYTES), SIGN_PUBLICKEYBYTES);
    0
}

pub unsafe fn crypto_sign_ed25519_detached(
    sig: *mut u8,
    siglen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    sk: *const u8,
) -> c_int {
    let sig_bytes = ed25519_sign_detached(opt_slice(m, len_to_usize(mlen)), &read_array(sk), false);
    write_array(sig, &sig_bytes);
    write_opt(siglen_p, SIGN_BYTES as u64);
    0
}

pub unsafe fn crypto_sign_ed25519(
    sm: *mut u8,
    smlen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    sk: *const u8,
) -> c_int {
    let mlen = len_to_usize(mlen);
    copy_allow_overlap(sm.add(SIGN_BYTES), m, mlen);
    let ret = crypto_sign_ed25519_detached(sm, ptr::null_mut(), sm.add(SIGN_BYTES), mlen as u64, sk);
    if ret != 0 {
        if !smlen_p.is_null() {
            *smlen_p = 0;
        }
        opt_slice_mut(sm, mlen + SIGN_BYTES).fill(0);
        return -1;
    }
    write_opt(smlen_p, (mlen + SIGN_BYTES) as u64);
    0
}

pub unsafe fn crypto_sign_ed25519_verify_detached(
    sig: *const u8,
    m: *const u8,
    mlen: u64,
    pk: *const u8,
) -> c_int {
    ed25519_verify_detached(&read_array(sig), opt_slice(m, len_to_usize(mlen)), &read_array(pk), false)
}

pub unsafe fn crypto_sign_ed25519_open(
    m: *mut u8,
    mlen_p: *mut u64,
    sm: *const u8,
    smlen: u64,
    pk: *const u8,
) -> c_int {
    let smlen = len_to_usize(smlen);
    if smlen < SIGN_BYTES || smlen - SIGN_BYTES > crypto_sign_ed25519_messagebytes_max() {
        write_opt(mlen_p, 0);
        return -1;
    }
    let mlen = smlen - SIGN_BYTES;
    if crypto_sign_ed25519_verify_detached(sm, sm.add(SIGN_BYTES), mlen as u64, pk) != 0 {
        if !m.is_null() {
            opt_slice_mut(m, mlen).fill(0);
        }
        write_opt(mlen_p, 0);
        return -1;
    }
    write_opt(mlen_p, mlen as u64);
    if !m.is_null() {
        copy_allow_overlap(m, sm.add(SIGN_BYTES), mlen);
    }
    0
}

pub unsafe fn crypto_sign_ed25519ph_init(state: *mut crypto_sign_ed25519ph_state) -> c_int {
    crate::symmetric_impl::crypto_hash_sha512_init(state.cast())
}

pub unsafe fn crypto_sign_ed25519ph_update(
    state: *mut crypto_sign_ed25519ph_state,
    m: *const u8,
    mlen: u64,
) -> c_int {
    crate::symmetric_impl::crypto_hash_sha512_update(state.cast(), m, mlen)
}

pub unsafe fn crypto_sign_ed25519ph_final_create(
    state: *mut crypto_sign_ed25519ph_state,
    sig: *mut u8,
    siglen_p: *mut u64,
    sk: *const u8,
) -> c_int {
    let mut ph = [0u8; ED25519_HASHBYTES];
    crate::symmetric_impl::crypto_hash_sha512_final(state.cast(), ph.as_mut_ptr());
    let sig_bytes = ed25519_sign_detached(&ph, &read_array(sk), true);
    write_array(sig, &sig_bytes);
    write_opt(siglen_p, SIGN_BYTES as u64);
    0
}

pub unsafe fn crypto_sign_ed25519ph_final_verify(
    state: *mut crypto_sign_ed25519ph_state,
    sig: *const u8,
    pk: *const u8,
) -> c_int {
    let mut ph = [0u8; ED25519_HASHBYTES];
    crate::symmetric_impl::crypto_hash_sha512_final(state.cast(), ph.as_mut_ptr());
    ed25519_verify_detached(&read_array(sig), &ph, &read_array(pk), true)
}

pub unsafe fn crypto_sign_statebytes() -> usize {
    size_of::<crypto_sign_state>()
}

pub unsafe fn crypto_sign_bytes() -> usize {
    SIGN_BYTES
}

pub unsafe fn crypto_sign_seedbytes() -> usize {
    SIGN_SEEDBYTES
}

pub unsafe fn crypto_sign_publickeybytes() -> usize {
    SIGN_PUBLICKEYBYTES
}

pub unsafe fn crypto_sign_secretkeybytes() -> usize {
    SIGN_SECRETKEYBYTES
}

pub unsafe fn crypto_sign_messagebytes_max() -> usize {
    usize::MAX - SIGN_BYTES
}

pub unsafe fn crypto_sign_primitive() -> *const c_char {
    static_cstr(b"ed25519\0")
}

pub unsafe fn crypto_sign_seed_keypair(pk: *mut u8, sk: *mut u8, seed: *const u8) -> c_int {
    crypto_sign_ed25519_seed_keypair(pk, sk, seed)
}

pub unsafe fn crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> c_int {
    crypto_sign_ed25519_keypair(pk, sk)
}

pub unsafe fn crypto_sign(
    sm: *mut u8,
    smlen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    sk: *const u8,
) -> c_int {
    crypto_sign_ed25519(sm, smlen_p, m, mlen, sk)
}

pub unsafe fn crypto_sign_open(
    m: *mut u8,
    mlen_p: *mut u64,
    sm: *const u8,
    smlen: u64,
    pk: *const u8,
) -> c_int {
    crypto_sign_ed25519_open(m, mlen_p, sm, smlen, pk)
}

pub unsafe fn crypto_sign_detached(
    sig: *mut u8,
    siglen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    sk: *const u8,
) -> c_int {
    crypto_sign_ed25519_detached(sig, siglen_p, m, mlen, sk)
}

pub unsafe fn crypto_sign_verify_detached(
    sig: *const u8,
    m: *const u8,
    mlen: u64,
    pk: *const u8,
) -> c_int {
    crypto_sign_ed25519_verify_detached(sig, m, mlen, pk)
}

pub unsafe fn crypto_sign_init(state: *mut crypto_sign_state) -> c_int {
    crypto_sign_ed25519ph_init(state.cast())
}

pub unsafe fn crypto_sign_update(
    state: *mut crypto_sign_state,
    m: *const u8,
    mlen: u64,
) -> c_int {
    crypto_sign_ed25519ph_update(state.cast(), m, mlen)
}

pub unsafe fn crypto_sign_final_create(
    state: *mut crypto_sign_state,
    sig: *mut u8,
    siglen_p: *mut u64,
    sk: *const u8,
) -> c_int {
    crypto_sign_ed25519ph_final_create(state.cast(), sig, siglen_p, sk)
}

pub unsafe fn crypto_sign_final_verify(
    state: *mut crypto_sign_state,
    sig: *const u8,
    pk: *const u8,
) -> c_int {
    crypto_sign_ed25519ph_final_verify(state.cast(), sig, pk)
}

pub unsafe fn crypto_sign_edwards25519sha512batch_keypair(pk: *mut u8, sk: *mut u8) -> c_int {
    let (pk_bytes, sk_bytes) = legacy_edwards25519sha512batch_keypair();
    write_array(pk, &pk_bytes);
    write_array(sk, &sk_bytes);
    0
}

pub unsafe fn crypto_sign_edwards25519sha512batch(
    sm: *mut u8,
    smlen_p: *mut u64,
    m: *const u8,
    mlen: u64,
    sk: *const u8,
) -> c_int {
    let signed = legacy_edwards25519sha512batch_sign(opt_slice(m, len_to_usize(mlen)), &read_array(sk));
    opt_slice_mut(sm, signed.len()).copy_from_slice(&signed);
    write_opt(smlen_p, signed.len() as u64);
    0
}

pub unsafe fn crypto_sign_edwards25519sha512batch_open(
    m: *mut u8,
    mlen_p: *mut u64,
    sm: *const u8,
    smlen: u64,
    pk: *const u8,
) -> c_int {
    write_opt(mlen_p, 0);
    let smlen = len_to_usize(smlen);
    let opened = match legacy_edwards25519sha512batch_open(opt_slice(sm, smlen), &read_array(pk)) {
        Some(msg) => msg,
        None => return -1,
    };
    if !m.is_null() {
        copy_allow_overlap(m, sm.add(32), opened.len());
    }
    write_opt(mlen_p, opened.len() as u64);
    0
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_seed_keypair(
    pk: *mut u8,
    sk: *mut u8,
    seed: *const u8,
) -> c_int {
    let (pk_bytes, sk_bytes) = curve25519_seed_keypair(&read_array(seed));
    write_array(pk, &pk_bytes);
    write_array(sk, &sk_bytes);
    0
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_keypair(pk: *mut u8, sk: *mut u8) -> c_int {
    let sk_out = opt_slice_mut(sk, BOX_SECRETKEYBYTES);
    fill_random_bytes(sk_out.as_mut_ptr(), sk_out.len());
    crypto_scalarmult_curve25519_base(pk, sk)
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_beforenm(
    k: *mut u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    match crypto_box_curve25519xsalsa20poly1305_beforenm_inner(read_array(pk), read_array(sk)) {
        Ok(key) => {
            write_array(k, &key);
            0
        }
        Err(ret) => ret,
    }
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_afternm(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crate::symmetric_impl::crypto_secretbox_xsalsa20poly1305(c, m, mlen, n, k)
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_open_afternm(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crate::symmetric_impl::crypto_secretbox_xsalsa20poly1305_open(m, c, clen, n, k)
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    let mut k = [0u8; BOX_BEFORENMBYTES];
    let ret = crypto_box_curve25519xsalsa20poly1305_beforenm(k.as_mut_ptr(), pk, sk);
    if ret != 0 {
        return -1;
    }
    let ret = crypto_box_curve25519xsalsa20poly1305_afternm(c, m, mlen, n, k.as_ptr());
    k.fill(0);
    ret
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_open(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    let mut k = [0u8; BOX_BEFORENMBYTES];
    let ret = crypto_box_curve25519xsalsa20poly1305_beforenm(k.as_mut_ptr(), pk, sk);
    if ret != 0 {
        return -1;
    }
    let ret = crypto_box_curve25519xsalsa20poly1305_open_afternm(m, c, clen, n, k.as_ptr());
    k.fill(0);
    ret
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_seedbytes() -> usize {
    BOX_SEEDBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_publickeybytes() -> usize {
    BOX_PUBLICKEYBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_secretkeybytes() -> usize {
    BOX_SECRETKEYBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_beforenmbytes() -> usize {
    BOX_BEFORENMBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_noncebytes() -> usize {
    BOX_NONCEBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_zerobytes() -> usize {
    BOX_ZEROBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_boxzerobytes() -> usize {
    BOX_BOXZEROBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_macbytes() -> usize {
    BOX_MACBYTES
}

pub unsafe fn crypto_box_curve25519xsalsa20poly1305_messagebytes_max() -> usize {
    crate::symmetric_impl::crypto_secretbox_xsalsa20poly1305_messagebytes_max()
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_seed_keypair(
    pk: *mut u8,
    sk: *mut u8,
    seed: *const u8,
) -> c_int {
    let (pk_bytes, sk_bytes) = curve25519_seed_keypair(&read_array(seed));
    write_array(pk, &pk_bytes);
    write_array(sk, &sk_bytes);
    0
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_keypair(pk: *mut u8, sk: *mut u8) -> c_int {
    let sk_out = opt_slice_mut(sk, BOX_SECRETKEYBYTES);
    fill_random_bytes(sk_out.as_mut_ptr(), sk_out.len());
    crypto_scalarmult_curve25519_base(pk, sk)
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_beforenm(
    k: *mut u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    match crypto_box_curve25519xchacha20poly1305_beforenm_inner(read_array(pk), read_array(sk)) {
        Ok(key) => {
            write_array(k, &key);
            0
        }
        Err(ret) => ret,
    }
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_detached_afternm(
    c: *mut u8,
    mac: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crate::symmetric_impl::crypto_secretbox_xchacha20poly1305_detached(c, mac, m, mlen, n, k)
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_detached(
    c: *mut u8,
    mac: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    let mut k = [0u8; BOX_BEFORENMBYTES];
    if crypto_box_curve25519xchacha20poly1305_beforenm(k.as_mut_ptr(), pk, sk) != 0 {
        return -1;
    }
    let ret =
        crypto_box_curve25519xchacha20poly1305_detached_afternm(c, mac, m, mlen, n, k.as_ptr());
    k.fill(0);
    ret
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_easy_afternm(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if len_to_usize(mlen) > crypto_box_curve25519xchacha20poly1305_messagebytes_max() {
        crate::foundation::core::sodium_misuse();
    }
    crypto_box_curve25519xchacha20poly1305_detached_afternm(
        c.add(BOX_MACBYTES),
        c,
        m,
        mlen,
        n,
        k,
    )
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_easy(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    if len_to_usize(mlen) > crypto_box_curve25519xchacha20poly1305_messagebytes_max() {
        crate::foundation::core::sodium_misuse();
    }
    crypto_box_curve25519xchacha20poly1305_detached(c.add(BOX_MACBYTES), c, m, mlen, n, pk, sk)
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_open_detached_afternm(
    m: *mut u8,
    c: *const u8,
    mac: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crate::symmetric_impl::crypto_secretbox_xchacha20poly1305_open_detached(m, c, mac, clen, n, k)
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_open_detached(
    m: *mut u8,
    c: *const u8,
    mac: *const u8,
    clen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    let mut k = [0u8; BOX_BEFORENMBYTES];
    if crypto_box_curve25519xchacha20poly1305_beforenm(k.as_mut_ptr(), pk, sk) != 0 {
        return -1;
    }
    let ret =
        crypto_box_curve25519xchacha20poly1305_open_detached_afternm(m, c, mac, clen, n, k.as_ptr());
    k.fill(0);
    ret
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_open_easy_afternm(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if len_to_usize(clen) < BOX_MACBYTES {
        return -1;
    }
    crypto_box_curve25519xchacha20poly1305_open_detached_afternm(
        m,
        c.add(BOX_MACBYTES),
        c,
        clen - BOX_MACBYTES as u64,
        n,
        k,
    )
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_open_easy(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    if len_to_usize(clen) < BOX_MACBYTES {
        return -1;
    }
    crypto_box_curve25519xchacha20poly1305_open_detached(
        m,
        c.add(BOX_MACBYTES),
        c,
        clen - BOX_MACBYTES as u64,
        n,
        pk,
        sk,
    )
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_seal(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    pk: *const u8,
) -> c_int {
    let mut epk = [0u8; BOX_PUBLICKEYBYTES];
    let mut esk = [0u8; BOX_SECRETKEYBYTES];
    if crypto_box_curve25519xchacha20poly1305_keypair(epk.as_mut_ptr(), esk.as_mut_ptr()) != 0 {
        return -1;
    }
    write_array(c, &epk);
    let nonce = box_seal_nonce::<BOX_NONCEBYTES>(&epk, &read_array(pk));
    let ret = crypto_box_curve25519xchacha20poly1305_easy(
        c.add(BOX_PUBLICKEYBYTES),
        m,
        mlen,
        nonce.as_ptr(),
        pk,
        esk.as_ptr(),
    );
    esk.fill(0);
    epk.fill(0);
    ret
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_seal_open(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    if len_to_usize(clen) < BOX_SEALBYTES {
        return -1;
    }
    let nonce = box_seal_nonce::<BOX_NONCEBYTES>(&read_array(c), &read_array(pk));
    crypto_box_curve25519xchacha20poly1305_open_easy(
        m,
        c.add(BOX_PUBLICKEYBYTES),
        clen - BOX_PUBLICKEYBYTES as u64,
        nonce.as_ptr(),
        c,
        sk,
    )
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_seedbytes() -> usize {
    BOX_SEEDBYTES
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_publickeybytes() -> usize {
    BOX_PUBLICKEYBYTES
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_secretkeybytes() -> usize {
    BOX_SECRETKEYBYTES
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_beforenmbytes() -> usize {
    BOX_BEFORENMBYTES
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_noncebytes() -> usize {
    BOX_NONCEBYTES
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_macbytes() -> usize {
    BOX_MACBYTES
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_messagebytes_max() -> usize {
    crate::symmetric_impl::crypto_secretbox_xchacha20poly1305_messagebytes_max()
}

pub unsafe fn crypto_box_curve25519xchacha20poly1305_sealbytes() -> usize {
    BOX_SEALBYTES
}

pub unsafe fn crypto_box_seedbytes() -> usize {
    BOX_SEEDBYTES
}

pub unsafe fn crypto_box_publickeybytes() -> usize {
    BOX_PUBLICKEYBYTES
}

pub unsafe fn crypto_box_secretkeybytes() -> usize {
    BOX_SECRETKEYBYTES
}

pub unsafe fn crypto_box_beforenmbytes() -> usize {
    BOX_BEFORENMBYTES
}

pub unsafe fn crypto_box_noncebytes() -> usize {
    BOX_NONCEBYTES
}

pub unsafe fn crypto_box_zerobytes() -> usize {
    BOX_ZEROBYTES
}

pub unsafe fn crypto_box_boxzerobytes() -> usize {
    BOX_BOXZEROBYTES
}

pub unsafe fn crypto_box_macbytes() -> usize {
    BOX_MACBYTES
}

pub unsafe fn crypto_box_messagebytes_max() -> usize {
    crate::symmetric_impl::crypto_secretbox_messagebytes_max()
}

pub unsafe fn crypto_box_primitive() -> *const c_char {
    static_cstr(b"curve25519xsalsa20poly1305\0")
}

pub unsafe fn crypto_box_seed_keypair(pk: *mut u8, sk: *mut u8, seed: *const u8) -> c_int {
    crypto_box_curve25519xsalsa20poly1305_seed_keypair(pk, sk, seed)
}

pub unsafe fn crypto_box_keypair(pk: *mut u8, sk: *mut u8) -> c_int {
    crypto_box_curve25519xsalsa20poly1305_keypair(pk, sk)
}

pub unsafe fn crypto_box_beforenm(k: *mut u8, pk: *const u8, sk: *const u8) -> c_int {
    crypto_box_curve25519xsalsa20poly1305_beforenm(k, pk, sk)
}

pub unsafe fn crypto_box_afternm(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_box_curve25519xsalsa20poly1305_afternm(c, m, mlen, n, k)
}

pub unsafe fn crypto_box_open_afternm(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crypto_box_curve25519xsalsa20poly1305_open_afternm(m, c, clen, n, k)
}

pub unsafe fn crypto_box(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    crypto_box_curve25519xsalsa20poly1305(c, m, mlen, n, pk, sk)
}

pub unsafe fn crypto_box_open(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    crypto_box_curve25519xsalsa20poly1305_open(m, c, clen, n, pk, sk)
}

pub unsafe fn crypto_box_detached_afternm(
    c: *mut u8,
    mac: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crate::symmetric_impl::crypto_secretbox_detached(c, mac, m, mlen, n, k)
}

pub unsafe fn crypto_box_detached(
    c: *mut u8,
    mac: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    let mut k = [0u8; BOX_BEFORENMBYTES];
    if crypto_box_beforenm(k.as_mut_ptr(), pk, sk) != 0 {
        return -1;
    }
    let ret = crypto_box_detached_afternm(c, mac, m, mlen, n, k.as_ptr());
    k.fill(0);
    ret
}

pub unsafe fn crypto_box_easy_afternm(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if len_to_usize(mlen) > crypto_box_messagebytes_max() {
        crate::foundation::core::sodium_misuse();
    }
    crypto_box_detached_afternm(c.add(BOX_MACBYTES), c, m, mlen, n, k)
}

pub unsafe fn crypto_box_easy(
    c: *mut u8,
    m: *const u8,
    mlen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    if len_to_usize(mlen) > crypto_box_messagebytes_max() {
        crate::foundation::core::sodium_misuse();
    }
    crypto_box_detached(c.add(BOX_MACBYTES), c, m, mlen, n, pk, sk)
}

pub unsafe fn crypto_box_open_detached_afternm(
    m: *mut u8,
    c: *const u8,
    mac: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    crate::symmetric_impl::crypto_secretbox_open_detached(m, c, mac, clen, n, k)
}

pub unsafe fn crypto_box_open_detached(
    m: *mut u8,
    c: *const u8,
    mac: *const u8,
    clen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    let mut k = [0u8; BOX_BEFORENMBYTES];
    if crypto_box_beforenm(k.as_mut_ptr(), pk, sk) != 0 {
        return -1;
    }
    let ret = crypto_box_open_detached_afternm(m, c, mac, clen, n, k.as_ptr());
    k.fill(0);
    ret
}

pub unsafe fn crypto_box_open_easy_afternm(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    k: *const u8,
) -> c_int {
    if len_to_usize(clen) < BOX_MACBYTES {
        return -1;
    }
    crypto_box_open_detached_afternm(m, c.add(BOX_MACBYTES), c, clen - BOX_MACBYTES as u64, n, k)
}

pub unsafe fn crypto_box_open_easy(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    n: *const u8,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    if len_to_usize(clen) < BOX_MACBYTES {
        return -1;
    }
    crypto_box_open_detached(m, c.add(BOX_MACBYTES), c, clen - BOX_MACBYTES as u64, n, pk, sk)
}

pub unsafe fn crypto_box_sealbytes() -> usize {
    BOX_SEALBYTES
}

pub unsafe fn crypto_box_seal(c: *mut u8, m: *const u8, mlen: u64, pk: *const u8) -> c_int {
    let mut epk = [0u8; BOX_PUBLICKEYBYTES];
    let mut esk = [0u8; BOX_SECRETKEYBYTES];
    if crypto_box_keypair(epk.as_mut_ptr(), esk.as_mut_ptr()) != 0 {
        return -1;
    }
    write_array(c, &epk);
    let nonce = box_seal_nonce::<BOX_NONCEBYTES>(&epk, &read_array(pk));
    let ret = crypto_box_easy(c.add(BOX_PUBLICKEYBYTES), m, mlen, nonce.as_ptr(), pk, esk.as_ptr());
    esk.fill(0);
    epk.fill(0);
    ret
}

pub unsafe fn crypto_box_seal_open(
    m: *mut u8,
    c: *const u8,
    clen: u64,
    pk: *const u8,
    sk: *const u8,
) -> c_int {
    if len_to_usize(clen) < BOX_SEALBYTES {
        return -1;
    }
    let nonce = box_seal_nonce::<BOX_NONCEBYTES>(&read_array(c), &read_array(pk));
    crypto_box_open_easy(m, c.add(BOX_PUBLICKEYBYTES), clen - BOX_PUBLICKEYBYTES as u64, nonce.as_ptr(), c, sk)
}

pub unsafe fn crypto_kx_seed_keypair(pk: *mut u8, sk: *mut u8, seed: *const u8) -> c_int {
    let sk_bytes = blake2b::<KX_SECRETKEYBYTES>(&[opt_slice(seed, KX_SEEDBYTES)]);
    write_array(sk, &sk_bytes);
    crypto_scalarmult_base(pk, sk)
}

pub unsafe fn crypto_kx_keypair(pk: *mut u8, sk: *mut u8) -> c_int {
    let sk_slice = opt_slice_mut(sk, KX_SECRETKEYBYTES);
    fill_random_bytes(sk_slice.as_mut_ptr(), sk_slice.len());
    crypto_scalarmult_base(pk, sk)
}

unsafe fn kx_session_keys(
    rx: *mut u8,
    tx: *mut u8,
    local_pk: *const u8,
    local_sk: *const u8,
    remote_pk: *const u8,
    client_role: bool,
) -> c_int {
    let mut rx = rx;
    let mut tx = tx;
    if rx.is_null() {
        rx = tx;
    }
    if tx.is_null() {
        tx = rx;
    }
    if rx.is_null() {
        crate::foundation::core::sodium_misuse();
    }

    let (client_pk, server_pk) = if client_role {
        (local_pk, remote_pk)
    } else {
        (remote_pk, local_pk)
    };

    let mut q = [0u8; CURVE25519_BYTES];
    if crypto_scalarmult(q.as_mut_ptr(), local_sk, remote_pk) != 0 {
        return -1;
    }
    let keys = blake2b::<{ 2 * KX_SESSIONKEYBYTES }>(&[
        &q,
        opt_slice(client_pk, KX_PUBLICKEYBYTES),
        opt_slice(server_pk, KX_PUBLICKEYBYTES),
    ]);
    q.fill(0);
    for i in 0..KX_SESSIONKEYBYTES {
        if client_role {
            *rx.add(i) = keys[i];
            *tx.add(i) = keys[i + KX_SESSIONKEYBYTES];
        } else {
            *tx.add(i) = keys[i];
            *rx.add(i) = keys[i + KX_SESSIONKEYBYTES];
        }
    }
    0
}

pub unsafe fn crypto_kx_client_session_keys(
    rx: *mut u8,
    tx: *mut u8,
    client_pk: *const u8,
    client_sk: *const u8,
    server_pk: *const u8,
) -> c_int {
    kx_session_keys(rx, tx, client_pk, client_sk, server_pk, true)
}

pub unsafe fn crypto_kx_server_session_keys(
    rx: *mut u8,
    tx: *mut u8,
    server_pk: *const u8,
    server_sk: *const u8,
    client_pk: *const u8,
) -> c_int {
    kx_session_keys(rx, tx, server_pk, server_sk, client_pk, false)
}

pub unsafe fn crypto_kx_publickeybytes() -> usize {
    KX_PUBLICKEYBYTES
}

pub unsafe fn crypto_kx_secretkeybytes() -> usize {
    KX_SECRETKEYBYTES
}

pub unsafe fn crypto_kx_seedbytes() -> usize {
    KX_SEEDBYTES
}

pub unsafe fn crypto_kx_sessionkeybytes() -> usize {
    KX_SESSIONKEYBYTES
}

pub unsafe fn crypto_kx_primitive() -> *const c_char {
    static_cstr(b"x25519blake2b\0")
}

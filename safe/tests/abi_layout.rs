use sodium::abi::types::*;
use std::mem::{align_of, size_of};

fn assert_layout(
    name: &str,
    actual_size: usize,
    actual_align: usize,
    expected_size: usize,
    expected_align: usize,
) {
    assert_eq!(actual_size, expected_size, "size mismatch for {name}");
    assert_eq!(
        actual_align, expected_align,
        "alignment mismatch for {name}"
    );
}

#[test]
fn public_state_layouts_match_headers() {
    assert_layout(
        "crypto_aead_aes256gcm_state",
        size_of::<crypto_aead_aes256gcm_state>(),
        align_of::<crypto_aead_aes256gcm_state>(),
        CRYPTO_AEAD_AES256GCM_STATE_SIZE,
        CRYPTO_AEAD_AES256GCM_STATE_ALIGN,
    );
    assert_layout(
        "crypto_auth_hmacsha256_state",
        size_of::<crypto_auth_hmacsha256_state>(),
        align_of::<crypto_auth_hmacsha256_state>(),
        CRYPTO_AUTH_HMACSHA256_STATE_SIZE,
        CRYPTO_AUTH_HMACSHA256_STATE_ALIGN,
    );
    assert_layout(
        "crypto_auth_hmacsha512_state",
        size_of::<crypto_auth_hmacsha512_state>(),
        align_of::<crypto_auth_hmacsha512_state>(),
        CRYPTO_AUTH_HMACSHA512_STATE_SIZE,
        CRYPTO_AUTH_HMACSHA512_STATE_ALIGN,
    );
    assert_layout(
        "crypto_auth_hmacsha512256_state",
        size_of::<crypto_auth_hmacsha512256_state>(),
        align_of::<crypto_auth_hmacsha512256_state>(),
        CRYPTO_AUTH_HMACSHA512256_STATE_SIZE,
        CRYPTO_AUTH_HMACSHA512256_STATE_ALIGN,
    );
    assert_layout(
        "crypto_generichash_blake2b_state",
        size_of::<crypto_generichash_blake2b_state>(),
        align_of::<crypto_generichash_blake2b_state>(),
        CRYPTO_GENERICHASH_BLAKE2B_STATE_SIZE,
        CRYPTO_GENERICHASH_BLAKE2B_STATE_ALIGN,
    );
    assert_layout(
        "crypto_generichash_state",
        size_of::<crypto_generichash_state>(),
        align_of::<crypto_generichash_state>(),
        CRYPTO_GENERICHASH_STATE_SIZE,
        CRYPTO_GENERICHASH_STATE_ALIGN,
    );
    assert_layout(
        "crypto_onetimeauth_poly1305_state",
        size_of::<crypto_onetimeauth_poly1305_state>(),
        align_of::<crypto_onetimeauth_poly1305_state>(),
        CRYPTO_ONETIMEAUTH_POLY1305_STATE_SIZE,
        CRYPTO_ONETIMEAUTH_POLY1305_STATE_ALIGN,
    );
    assert_layout(
        "crypto_onetimeauth_state",
        size_of::<crypto_onetimeauth_state>(),
        align_of::<crypto_onetimeauth_state>(),
        CRYPTO_ONETIMEAUTH_STATE_SIZE,
        CRYPTO_ONETIMEAUTH_STATE_ALIGN,
    );
    assert_layout(
        "crypto_hash_sha256_state",
        size_of::<crypto_hash_sha256_state>(),
        align_of::<crypto_hash_sha256_state>(),
        CRYPTO_HASH_SHA256_STATE_SIZE,
        CRYPTO_HASH_SHA256_STATE_ALIGN,
    );
    assert_layout(
        "crypto_hash_sha512_state",
        size_of::<crypto_hash_sha512_state>(),
        align_of::<crypto_hash_sha512_state>(),
        CRYPTO_HASH_SHA512_STATE_SIZE,
        CRYPTO_HASH_SHA512_STATE_ALIGN,
    );
    assert_layout(
        "crypto_secretstream_xchacha20poly1305_state",
        size_of::<crypto_secretstream_xchacha20poly1305_state>(),
        align_of::<crypto_secretstream_xchacha20poly1305_state>(),
        CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_STATE_SIZE,
        CRYPTO_SECRETSTREAM_XCHACHA20POLY1305_STATE_ALIGN,
    );
    assert_layout(
        "crypto_sign_ed25519ph_state",
        size_of::<crypto_sign_ed25519ph_state>(),
        align_of::<crypto_sign_ed25519ph_state>(),
        CRYPTO_SIGN_ED25519PH_STATE_SIZE,
        CRYPTO_SIGN_ED25519PH_STATE_ALIGN,
    );
    assert_layout(
        "crypto_sign_state",
        size_of::<crypto_sign_state>(),
        align_of::<crypto_sign_state>(),
        CRYPTO_SIGN_STATE_SIZE,
        CRYPTO_SIGN_STATE_ALIGN,
    );
    assert_layout(
        "randombytes_implementation",
        size_of::<randombytes_implementation>(),
        align_of::<randombytes_implementation>(),
        RANDOMBYTES_IMPLEMENTATION_SIZE,
        RANDOMBYTES_IMPLEMENTATION_ALIGN,
    );
}

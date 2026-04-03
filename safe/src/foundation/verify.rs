use crate::ffi::helpers::abort_on_panic;
use core::ffi::c_int;

const CRYPTO_VERIFY_16_BYTES: usize = 16;
const CRYPTO_VERIFY_32_BYTES: usize = 32;
const CRYPTO_VERIFY_64_BYTES: usize = 64;

fn crypto_verify_n(x: *const u8, y: *const u8, n: usize) -> c_int {
    let mut diff = 0u16;
    for index in 0..n {
        unsafe {
            diff |= (*x.add(index) ^ *y.add(index)) as u16;
        }
    }
    (((diff.wrapping_sub(1) >> 8) & 1) as c_int) - 1
}

#[no_mangle]
pub extern "C" fn crypto_verify_16_bytes() -> usize {
    CRYPTO_VERIFY_16_BYTES
}

#[no_mangle]
pub extern "C" fn crypto_verify_32_bytes() -> usize {
    CRYPTO_VERIFY_32_BYTES
}

#[no_mangle]
pub extern "C" fn crypto_verify_64_bytes() -> usize {
    CRYPTO_VERIFY_64_BYTES
}

#[no_mangle]
pub extern "C" fn crypto_verify_16(x: *const u8, y: *const u8) -> c_int {
    abort_on_panic(|| crypto_verify_n(x, y, CRYPTO_VERIFY_16_BYTES))
}

#[no_mangle]
pub extern "C" fn crypto_verify_32(x: *const u8, y: *const u8) -> c_int {
    abort_on_panic(|| crypto_verify_n(x, y, CRYPTO_VERIFY_32_BYTES))
}

#[no_mangle]
pub extern "C" fn crypto_verify_64(x: *const u8, y: *const u8) -> c_int {
    abort_on_panic(|| crypto_verify_n(x, y, CRYPTO_VERIFY_64_BYTES))
}

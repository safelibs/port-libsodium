use crate::abi::types::randombytes_implementation;
use crate::ffi::helpers::{abort_on_panic, static_cstr};
use core::ffi::{c_char, c_int, c_void};
use core::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Mutex, MutexGuard};

const RANDOMBYTES_SEEDBYTES: usize = 32;
const RANDOMBYTES_DETERMINISTIC_MAX: usize = 0x4000_0000_00;
const CHACHA20_KEYBYTES: usize = 32;
const CHACHA20_NONCEBYTES: usize = 12;
const CHACHA20_BLOCKBYTES: usize = 64;
const DETERMINISTIC_NONCE: [u8; CHACHA20_NONCEBYTES] = *b"LibsodiumDRG";

static IMPLEMENTATION: AtomicPtr<randombytes_implementation> = AtomicPtr::new(ptr::null_mut());
static SYSRANDOM_STATE: Mutex<SysRandomState> = Mutex::new(SysRandomState {
    fd: -1,
    use_getrandom: true,
});

struct SysRandomState {
    fd: c_int,
    use_getrandom: bool,
}

fn sysrandom_guard() -> MutexGuard<'static, SysRandomState> {
    match SYSRANDOM_STATE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn default_implementation() -> *mut randombytes_implementation {
    ptr::addr_of!(randombytes_sysrandom_implementation) as *mut randombytes_implementation
}

fn load_implementation() -> *mut randombytes_implementation {
    IMPLEMENTATION.load(Ordering::SeqCst)
}

fn randombytes_init_if_needed() {
    if load_implementation().is_null() {
        IMPLEMENTATION.store(default_implementation(), Ordering::SeqCst);
        randombytes_stir();
    }
}

fn implementation_name_for(ptr: *mut randombytes_implementation) -> *const c_char {
    unsafe {
        (*ptr)
            .implementation_name
            .expect("randombytes implementation_name callback is required")()
    }
}

fn random_u32_for(ptr: *mut randombytes_implementation) -> u32 {
    unsafe {
        (*ptr)
            .random
            .expect("randombytes random callback is required")()
    }
}

fn buf_for(ptr: *mut randombytes_implementation, buf: *mut c_void, size: usize) {
    unsafe {
        (*ptr).buf.expect("randombytes buf callback is required")(buf, size);
    }
}

pub(crate) fn fill_random_bytes(buf: *mut u8, size: usize) {
    if size == 0 {
        return;
    }

    let mut state = sysrandom_guard();
    if try_getrandom(&mut state, buf, size) {
        return;
    }

    let fd = open_urandom(&mut state);
    if fd < 0 {
        crate::foundation::core::sodium_misuse();
    }
    unsafe { read_all(fd, buf, size) };
}

fn try_getrandom(state: &mut SysRandomState, mut buf: *mut u8, mut size: usize) -> bool {
    if !state.use_getrandom {
        return false;
    }

    while size != 0 {
        let chunk = size.min(256);
        let readnb = unsafe { libc::getrandom(buf.cast(), chunk, 0) };
        if readnb == chunk as isize {
            unsafe {
                buf = buf.add(chunk);
            }
            size -= chunk;
            continue;
        }
        if readnb >= 0 {
            let readnb = readnb as usize;
            unsafe {
                buf = buf.add(readnb);
            }
            size -= readnb;
            continue;
        }

        let errno = errno();
        if errno == libc::EINTR || errno == libc::EAGAIN {
            continue;
        }
        if errno == libc::ENOSYS {
            state.use_getrandom = false;
        }
        return false;
    }

    true
}

fn open_urandom(state: &mut SysRandomState) -> c_int {
    if state.fd >= 0 {
        return state.fd;
    }

    let path = b"/dev/urandom\0";
    let fd = unsafe { libc::open(path.as_ptr().cast(), libc::O_RDONLY | libc::O_CLOEXEC) };
    if fd >= 0 {
        state.fd = fd;
    }
    fd
}

unsafe fn read_all(fd: c_int, mut buf: *mut u8, mut size: usize) {
    while size != 0 {
        let readnb = libc::read(fd, buf.cast(), size);
        if readnb > 0 {
            let readnb = readnb as usize;
            buf = buf.add(readnb);
            size -= readnb;
            continue;
        }
        if readnb < 0 {
            let err = errno();
            if err == libc::EINTR || err == libc::EAGAIN {
                continue;
            }
        }
        crate::foundation::core::sodium_misuse();
    }
}

fn close_urandom() -> c_int {
    let mut state = sysrandom_guard();
    if state.fd < 0 {
        return 0;
    }

    let fd = state.fd;
    state.fd = -1;
    unsafe { libc::close(fd) }
}

fn errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

unsafe extern "C" fn sysrandom_implementation_name() -> *const c_char {
    static_cstr(b"sysrandom\0")
}

unsafe extern "C" fn sysrandom_random() -> u32 {
    let mut bytes = [0u8; 4];
    fill_random_bytes(bytes.as_mut_ptr(), bytes.len());
    u32::from_ne_bytes(bytes)
}

unsafe extern "C" fn sysrandom_stir() {}

unsafe extern "C" fn sysrandom_buf(buf: *mut c_void, size: usize) {
    fill_random_bytes(buf.cast(), size);
}

unsafe extern "C" fn sysrandom_close() -> c_int {
    close_urandom()
}

unsafe extern "C" fn internal_implementation_name() -> *const c_char {
    static_cstr(b"internal\0")
}

unsafe extern "C" fn internal_random() -> u32 {
    sysrandom_random()
}

unsafe extern "C" fn internal_stir() {}

unsafe extern "C" fn internal_buf(buf: *mut c_void, size: usize) {
    sysrandom_buf(buf, size);
}

unsafe extern "C" fn internal_close() -> c_int {
    close_urandom()
}

#[no_mangle]
pub static randombytes_sysrandom_implementation: randombytes_implementation =
    randombytes_implementation {
        implementation_name: Some(sysrandom_implementation_name),
        random: Some(sysrandom_random),
        stir: Some(sysrandom_stir),
        uniform: None,
        buf: Some(sysrandom_buf),
        close: Some(sysrandom_close),
    };

#[no_mangle]
pub static randombytes_internal_implementation: randombytes_implementation =
    randombytes_implementation {
        implementation_name: Some(internal_implementation_name),
        random: Some(internal_random),
        stir: Some(internal_stir),
        uniform: None,
        buf: Some(internal_buf),
        close: Some(internal_close),
    };

#[no_mangle]
pub extern "C" fn randombytes_set_implementation(
    implementation: *mut randombytes_implementation,
) -> c_int {
    abort_on_panic(|| {
        IMPLEMENTATION.store(implementation, Ordering::SeqCst);
        0
    })
}

#[no_mangle]
pub extern "C" fn randombytes_implementation_name() -> *const c_char {
    abort_on_panic(|| {
        randombytes_init_if_needed();
        implementation_name_for(load_implementation())
    })
}

#[no_mangle]
pub extern "C" fn randombytes_random() -> u32 {
    abort_on_panic(|| {
        randombytes_init_if_needed();
        random_u32_for(load_implementation())
    })
}

#[no_mangle]
pub extern "C" fn randombytes_uniform(upper_bound: u32) -> u32 {
    abort_on_panic(|| {
        randombytes_init_if_needed();
        let implementation = load_implementation();

        unsafe {
            if let Some(uniform) = (*implementation).uniform {
                return uniform(upper_bound);
            }
        }
        if upper_bound < 2 {
            return 0;
        }

        let min = (1u32.wrapping_add(!upper_bound)).wrapping_rem(upper_bound);
        loop {
            let value = randombytes_random();
            if value >= min {
                return value % upper_bound;
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn randombytes_stir() {
    abort_on_panic(|| {
        randombytes_init_if_needed();
        let implementation = load_implementation();
        unsafe {
            if let Some(stir) = (*implementation).stir {
                stir();
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn randombytes_buf(buf: *mut c_void, size: usize) {
    abort_on_panic(|| {
        randombytes_init_if_needed();
        if size != 0 {
            buf_for(load_implementation(), buf, size);
        }
    })
}

#[no_mangle]
pub extern "C" fn randombytes_buf_deterministic(buf: *mut c_void, size: usize, seed: *const u8) {
    abort_on_panic(|| unsafe {
        if usize::BITS > 38 && size > RANDOMBYTES_DETERMINISTIC_MAX {
            crate::foundation::core::sodium_misuse();
        }

        let mut key = [0u8; CHACHA20_KEYBYTES];
        key.copy_from_slice(core::slice::from_raw_parts(seed, RANDOMBYTES_SEEDBYTES));
        chacha20_ietf_keystream(buf.cast(), size, &DETERMINISTIC_NONCE, &key);
    })
}

#[no_mangle]
pub extern "C" fn randombytes_seedbytes() -> usize {
    RANDOMBYTES_SEEDBYTES
}

#[no_mangle]
pub extern "C" fn randombytes_close() -> c_int {
    abort_on_panic(|| {
        let implementation = load_implementation();
        if implementation.is_null() {
            return 0;
        }
        unsafe {
            if let Some(close) = (*implementation).close {
                return close();
            }
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn randombytes(buf: *mut u8, buf_len: u64) {
    abort_on_panic(|| {
        assert!(buf_len <= usize::MAX as u64);
        randombytes_buf(buf.cast(), buf_len as usize);
    })
}

pub(crate) fn chacha20_ietf_keystream(
    mut out: *mut u8,
    mut len: usize,
    nonce: &[u8; CHACHA20_NONCEBYTES],
    key: &[u8; CHACHA20_KEYBYTES],
) {
    let mut counter = 0u32;

    while len != 0 {
        let block = chacha20_ietf_block(key, counter, nonce);
        let take = len.min(CHACHA20_BLOCKBYTES);
        unsafe {
            ptr::copy_nonoverlapping(block.as_ptr(), out, take);
            out = out.add(take);
        }
        len -= take;
        counter = counter.wrapping_add(1);
    }
}

fn chacha20_ietf_block(
    key: &[u8; CHACHA20_KEYBYTES],
    counter: u32,
    nonce: &[u8; CHACHA20_NONCEBYTES],
) -> [u8; CHACHA20_BLOCKBYTES] {
    let mut state = [0u32; 16];
    state[0] = 0x6170_7865;
    state[1] = 0x3320_646e;
    state[2] = 0x7962_2d32;
    state[3] = 0x6b20_6574;

    for (index, chunk) in key.chunks_exact(4).enumerate() {
        state[4 + index] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    state[12] = counter;
    for (index, chunk) in nonce.chunks_exact(4).enumerate() {
        state[13 + index] = u32::from_le_bytes(chunk.try_into().unwrap());
    }

    let mut working = state;
    for _ in 0..10 {
        quarter_round(&mut working, 0, 4, 8, 12);
        quarter_round(&mut working, 1, 5, 9, 13);
        quarter_round(&mut working, 2, 6, 10, 14);
        quarter_round(&mut working, 3, 7, 11, 15);
        quarter_round(&mut working, 0, 5, 10, 15);
        quarter_round(&mut working, 1, 6, 11, 12);
        quarter_round(&mut working, 2, 7, 8, 13);
        quarter_round(&mut working, 3, 4, 9, 14);
    }

    for (lhs, rhs) in working.iter_mut().zip(state.iter()) {
        *lhs = lhs.wrapping_add(*rhs);
    }

    let mut out = [0u8; CHACHA20_BLOCKBYTES];
    for (index, word) in working.iter().enumerate() {
        out[index * 4..(index + 1) * 4].copy_from_slice(&word.to_le_bytes());
    }
    out
}

fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
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

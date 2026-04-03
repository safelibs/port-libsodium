use crate::ffi::helpers::{abort_on_panic, checked_mul, set_errno, write_opt};
use crate::foundation::randombytes::fill_random_bytes;
use core::ffi::{c_int, c_void};
use core::mem::size_of;
use core::ptr;
use std::sync::OnceLock;

const CANARY_SIZE: usize = 16;
const GARBAGE_VALUE: u8 = 0xdb;

struct AllocConfig {
    page_size: usize,
    canary: [u8; CANARY_SIZE],
}

static ALLOC_CONFIG: OnceLock<AllocConfig> = OnceLock::new();

fn alloc_config() -> &'static AllocConfig {
    ALLOC_CONFIG.get_or_init(|| {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        if page_size <= 0 {
            crate::foundation::core::sodium_misuse();
        }
        let page_size = page_size as usize;
        if page_size < CANARY_SIZE || page_size < size_of::<usize>() {
            crate::foundation::core::sodium_misuse();
        }

        let mut canary = [0u8; CANARY_SIZE];
        fill_random_bytes(canary.as_mut_ptr(), canary.len());

        AllocConfig { page_size, canary }
    })
}

#[inline]
fn page_round(size: usize, page_size: usize) -> usize {
    let page_mask = page_size - 1;
    (size + page_mask) & !page_mask
}

unsafe fn sodium_memzero_inner(pnt: *mut u8, len: usize) {
    for index in 0..len {
        ptr::write_volatile(pnt.add(index), 0);
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

fn sodium_memcmp_inner(b1: *const u8, b2: *const u8, len: usize) -> c_int {
    let mut diff = 0u16;
    for index in 0..len {
        unsafe {
            diff |= (*b1.add(index) ^ *b2.add(index)) as u16;
        }
    }
    (((diff.wrapping_sub(1) >> 8) & 1) as c_int) - 1
}

fn sodium_compare_inner(b1: *const u8, b2: *const u8, len: usize) -> c_int {
    let mut gt = 0u16;
    let mut eq = 1u16;

    let mut index = len;
    while index != 0 {
        index -= 1;
        let x1 = unsafe { *b1.add(index) as u16 };
        let x2 = unsafe { *b2.add(index) as u16 };
        gt |= ((x2.wrapping_sub(x1)) >> 8) & eq;
        eq &= ((x2 ^ x1).wrapping_sub(1)) >> 8;
    }

    (gt + gt + eq) as c_int - 1
}

fn sodium_is_zero_inner(n: *const u8, len: usize) -> c_int {
    let mut diff = 0u16;
    for index in 0..len {
        unsafe {
            diff |= *n.add(index) as u16;
        }
    }
    ((diff.wrapping_sub(1) >> 8) & 1) as c_int
}

unsafe fn sodium_increment_inner(n: *mut u8, len: usize) {
    let mut carry: u16 = 1;
    for index in 0..len {
        carry = carry.wrapping_add(*n.add(index) as u16);
        *n.add(index) = carry as u8;
        carry >>= 8;
    }
}

unsafe fn sodium_add_inner(a: *mut u8, b: *const u8, len: usize) {
    let mut carry: u16 = 0;
    for index in 0..len {
        carry = carry
            .wrapping_add(*a.add(index) as u16)
            .wrapping_add(*b.add(index) as u16);
        *a.add(index) = carry as u8;
        carry >>= 8;
    }
}

unsafe fn sodium_sub_inner(a: *mut u8, b: *const u8, len: usize) {
    let mut borrow: u16 = 0;
    for index in 0..len {
        borrow = (*a.add(index) as u16)
            .wrapping_sub(*b.add(index) as u16)
            .wrapping_sub(borrow);
        *a.add(index) = borrow as u8;
        borrow = (borrow >> 8) & 1;
    }
}

unsafe fn mprotect_noaccess(ptr: *mut c_void, size: usize) -> c_int {
    libc::mprotect(ptr, size, libc::PROT_NONE)
}

unsafe fn mprotect_readonly(ptr: *mut c_void, size: usize) -> c_int {
    libc::mprotect(ptr, size, libc::PROT_READ)
}

unsafe fn mprotect_readwrite(ptr: *mut c_void, size: usize) -> c_int {
    libc::mprotect(ptr, size, libc::PROT_READ | libc::PROT_WRITE)
}

#[inline]
unsafe fn out_of_bounds() -> ! {
    libc::raise(libc::SIGSEGV);
    libc::abort();
}

unsafe fn alloc_aligned(size: usize) -> *mut u8 {
    let ptr = libc::mmap(
        ptr::null_mut(),
        size,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
        -1,
        0,
    );
    if ptr == libc::MAP_FAILED {
        ptr::null_mut()
    } else {
        ptr.cast()
    }
}

unsafe fn free_aligned(ptr: *mut u8, size: usize) {
    libc::munmap(ptr.cast(), size);
}

unsafe fn unprotected_ptr_from_user_ptr(ptr: *mut c_void) -> *mut u8 {
    let config = alloc_config();
    let canary_ptr = (ptr as *mut u8).sub(CANARY_SIZE);
    let page_mask = !(config.page_size - 1);
    let unprotected = ((canary_ptr as usize) & page_mask) as *mut u8;
    if (unprotected as usize) <= config.page_size * 2 {
        crate::foundation::core::sodium_misuse();
    }
    unprotected
}

unsafe fn sodium_malloc_inner(size: usize) -> *mut c_void {
    let config = alloc_config();

    if size >= usize::MAX - config.page_size * 4 {
        set_errno(libc::ENOMEM);
        return ptr::null_mut();
    }

    let size_with_canary = CANARY_SIZE + size;
    let unprotected_size = page_round(size_with_canary, config.page_size);
    let total_size = config.page_size * 3 + unprotected_size;
    let base_ptr = alloc_aligned(total_size);
    if base_ptr.is_null() {
        return ptr::null_mut();
    }

    let unprotected_ptr = base_ptr.add(config.page_size * 2);
    let _ = mprotect_noaccess(base_ptr.add(config.page_size).cast(), config.page_size);
    let _ = mprotect_noaccess(
        unprotected_ptr.add(unprotected_size).cast(),
        config.page_size,
    );
    let _ = sodium_mlock(unprotected_ptr.cast(), unprotected_size);

    let canary_ptr = unprotected_ptr.add(unprotected_size - size_with_canary);
    let user_ptr = canary_ptr.add(CANARY_SIZE);

    ptr::copy_nonoverlapping(config.canary.as_ptr(), canary_ptr, CANARY_SIZE);
    ptr::write(base_ptr.cast::<usize>(), unprotected_size);
    let _ = mprotect_readonly(base_ptr.cast(), config.page_size);

    debug_assert_eq!(
        unprotected_ptr_from_user_ptr(user_ptr.cast()),
        unprotected_ptr
    );

    user_ptr.cast()
}

unsafe fn sodium_mprotect_inner(
    ptr: *mut c_void,
    callback: unsafe fn(*mut c_void, usize) -> c_int,
) -> c_int {
    let config = alloc_config();
    let unprotected_ptr = unprotected_ptr_from_user_ptr(ptr);
    let base_ptr = unprotected_ptr.sub(config.page_size * 2);
    let unprotected_size = ptr::read(base_ptr.cast::<usize>());

    callback(unprotected_ptr.cast(), unprotected_size)
}

pub(crate) fn alloc_init() -> c_int {
    abort_on_panic(|| {
        let _ = alloc_config();
        0
    })
}

#[no_mangle]
pub extern "C" fn sodium_memzero(pnt: *mut c_void, len: usize) {
    abort_on_panic(|| unsafe {
        sodium_memzero_inner(pnt.cast(), len);
    })
}

#[no_mangle]
pub extern "C" fn sodium_stackzero(len: usize) {
    abort_on_panic(|| {
        let mut scratch = vec![0u8; len];
        unsafe {
            sodium_memzero_inner(scratch.as_mut_ptr(), scratch.len());
        }
    })
}

#[no_mangle]
pub extern "C" fn sodium_memcmp(b1: *const c_void, b2: *const c_void, len: usize) -> c_int {
    abort_on_panic(|| sodium_memcmp_inner(b1.cast(), b2.cast(), len))
}

#[no_mangle]
pub extern "C" fn sodium_compare(b1: *const u8, b2: *const u8, len: usize) -> c_int {
    abort_on_panic(|| sodium_compare_inner(b1, b2, len))
}

#[no_mangle]
pub extern "C" fn sodium_is_zero(n: *const u8, nlen: usize) -> c_int {
    abort_on_panic(|| sodium_is_zero_inner(n, nlen))
}

#[no_mangle]
pub extern "C" fn sodium_increment(n: *mut u8, nlen: usize) {
    abort_on_panic(|| unsafe {
        sodium_increment_inner(n, nlen);
    })
}

#[no_mangle]
pub extern "C" fn sodium_add(a: *mut u8, b: *const u8, len: usize) {
    abort_on_panic(|| unsafe {
        sodium_add_inner(a, b, len);
    })
}

#[no_mangle]
pub extern "C" fn sodium_sub(a: *mut u8, b: *const u8, len: usize) {
    abort_on_panic(|| unsafe {
        sodium_sub_inner(a, b, len);
    })
}

#[no_mangle]
pub extern "C" fn sodium_mlock(addr: *mut c_void, len: usize) -> c_int {
    abort_on_panic(|| unsafe { libc::mlock(addr, len) })
}

#[no_mangle]
pub extern "C" fn sodium_munlock(addr: *mut c_void, len: usize) -> c_int {
    abort_on_panic(|| unsafe {
        sodium_memzero_inner(addr.cast(), len);
        libc::munlock(addr, len)
    })
}

#[no_mangle]
pub extern "C" fn sodium_malloc(size: usize) -> *mut c_void {
    abort_on_panic(|| unsafe {
        let ptr = sodium_malloc_inner(size);
        if !ptr.is_null() && size != 0 {
            ptr::write_bytes(ptr.cast::<u8>(), GARBAGE_VALUE, size);
        }
        ptr
    })
}

#[no_mangle]
pub extern "C" fn sodium_allocarray(count: usize, size: usize) -> *mut c_void {
    abort_on_panic(|| {
        if count > 0 && size >= usize::MAX / count {
            unsafe {
                set_errno(libc::ENOMEM);
            }
            return ptr::null_mut();
        }
        match checked_mul(count, size) {
            Some(total) => sodium_malloc(total),
            None => {
                unsafe {
                    set_errno(libc::ENOMEM);
                }
                ptr::null_mut()
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn sodium_free(ptr: *mut c_void) {
    abort_on_panic(|| unsafe {
        if ptr.is_null() {
            return;
        }

        let config = alloc_config();
        let canary_ptr = (ptr as *mut u8).sub(CANARY_SIZE);
        let unprotected_ptr = unprotected_ptr_from_user_ptr(ptr);
        let base_ptr = unprotected_ptr.sub(config.page_size * 2);
        let unprotected_size = ptr::read(base_ptr.cast::<usize>());
        let total_size = config.page_size * 3 + unprotected_size;

        let _ = mprotect_readwrite(base_ptr.cast(), total_size);
        if sodium_memcmp_inner(canary_ptr, config.canary.as_ptr(), CANARY_SIZE) != 0 {
            out_of_bounds();
        }
        let _ = sodium_munlock(unprotected_ptr.cast(), unprotected_size);
        free_aligned(base_ptr, total_size);
    })
}

#[no_mangle]
pub extern "C" fn sodium_mprotect_noaccess(ptr: *mut c_void) -> c_int {
    abort_on_panic(|| unsafe { sodium_mprotect_inner(ptr, mprotect_noaccess) })
}

#[no_mangle]
pub extern "C" fn sodium_mprotect_readonly(ptr: *mut c_void) -> c_int {
    abort_on_panic(|| unsafe { sodium_mprotect_inner(ptr, mprotect_readonly) })
}

#[no_mangle]
pub extern "C" fn sodium_mprotect_readwrite(ptr: *mut c_void) -> c_int {
    abort_on_panic(|| unsafe { sodium_mprotect_inner(ptr, mprotect_readwrite) })
}

#[no_mangle]
pub extern "C" fn sodium_pad(
    padded_buflen_p: *mut usize,
    buf: *mut u8,
    unpadded_buflen: usize,
    blocksize: usize,
    max_buflen: usize,
) -> c_int {
    abort_on_panic(|| unsafe {
        if blocksize == 0 {
            return -1;
        }

        let mut xpadlen = blocksize - 1;
        if (blocksize & (blocksize - 1)) == 0 {
            xpadlen -= unpadded_buflen & (blocksize - 1);
        } else {
            xpadlen -= unpadded_buflen % blocksize;
        }

        if usize::MAX - unpadded_buflen <= xpadlen {
            crate::foundation::core::sodium_misuse();
        }

        let xpadded_len = unpadded_buflen + xpadlen;
        if xpadded_len >= max_buflen {
            return -1;
        }

        write_opt(padded_buflen_p, xpadded_len + 1);
        let tail = buf.add(xpadded_len);
        let shift = (size_of::<usize>() - 1) * 8;
        let mut mask = 0u8;
        for i in 0..blocksize {
            let barrier_mask = (((i ^ xpadlen).wrapping_sub(1)) >> shift) as u8;
            let target = tail.sub(i);
            *target = (*target & mask) | (0x80 & barrier_mask);
            mask |= barrier_mask;
        }
        0
    })
}

#[no_mangle]
pub extern "C" fn sodium_unpad(
    unpadded_buflen_p: *mut usize,
    buf: *const u8,
    padded_buflen: usize,
    blocksize: usize,
) -> c_int {
    abort_on_panic(|| unsafe {
        if padded_buflen < blocksize || blocksize == 0 {
            return -1;
        }

        let tail = buf.add(padded_buflen - 1);
        let mut acc = 0usize;
        let mut valid = 0u8;
        let mut pad_len = 0usize;

        for i in 0..blocksize {
            let c = *tail.sub(i) as usize;
            let is_barrier =
                (((acc.wrapping_sub(1)) & pad_len.wrapping_sub(1) & (c ^ 0x80).wrapping_sub(1))
                    >> 8)
                    & 1;
            acc |= c;
            pad_len |= i & (1usize.wrapping_add(!is_barrier));
            valid |= is_barrier as u8;
        }

        write_opt(unpadded_buflen_p, padded_buflen - 1 - pad_len);
        c_int::from(valid) - 1
    })
}

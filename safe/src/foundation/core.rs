use crate::ffi::helpers::{abort_on_panic, cast_handler};
use core::ffi::{c_int, c_void};
use core::ptr;
use std::sync::atomic::{AtomicI32, AtomicPtr, Ordering};
use std::sync::{Mutex, MutexGuard};

static INIT_LOCK: Mutex<()> = Mutex::new(());
static INITIALIZED: AtomicI32 = AtomicI32::new(0);
static MISUSE_HANDLER: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());

type InitHook = fn();

const INIT_HOOKS: [InitHook; 6] = [
    pick_best_argon2_implementation,
    pick_best_generichash_blake2b_implementation,
    pick_best_onetimeauth_poly1305_implementation,
    pick_best_scalarmult_curve25519_implementation,
    pick_best_stream_chacha20_implementation,
    pick_best_stream_salsa20_implementation,
];

#[inline]
fn init_guard() -> MutexGuard<'static, ()> {
    match INIT_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn pick_best_argon2_implementation() {}
fn pick_best_generichash_blake2b_implementation() {}
fn pick_best_onetimeauth_poly1305_implementation() {}
fn pick_best_scalarmult_curve25519_implementation() {}
fn pick_best_stream_chacha20_implementation() {}
fn pick_best_stream_salsa20_implementation() {}

#[no_mangle]
pub extern "C" fn sodium_init() -> c_int {
    abort_on_panic(|| {
        let _guard = init_guard();
        if INITIALIZED.load(Ordering::Acquire) != 0 {
            return 1;
        }

        let _ = crate::foundation::runtime::initialize_cpu_features();
        crate::foundation::randombytes::randombytes_stir();
        let _ = crate::foundation::utils::alloc_init();
        for hook in INIT_HOOKS {
            hook();
        }
        INITIALIZED.store(1, Ordering::Release);

        0
    })
}

#[no_mangle]
pub extern "C" fn sodium_set_misuse_handler(handler: Option<unsafe extern "C" fn()>) -> c_int {
    abort_on_panic(|| {
        let handler_ptr = handler
            .map(|f| f as *const () as *mut c_void)
            .unwrap_or(ptr::null_mut());
        MISUSE_HANDLER.store(handler_ptr, Ordering::SeqCst);
        0
    })
}

#[no_mangle]
pub extern "C" fn sodium_misuse() -> ! {
    unsafe {
        if let Some(handler) = cast_handler(MISUSE_HANDLER.load(Ordering::SeqCst)) {
            handler();
        }
        libc::abort();
    }
}

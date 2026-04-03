use core::ffi::c_int;

unsafe extern "C" {
    fn _sodium_runtime_get_cpu_features() -> c_int;
}

pub(crate) fn initialize_cpu_features() -> c_int {
    unsafe { _sodium_runtime_get_cpu_features() }
}

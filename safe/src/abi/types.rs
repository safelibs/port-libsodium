#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct randombytes_implementation {
    pub implementation_name: Option<unsafe extern "C" fn() -> *const c_char>,
    pub random: Option<unsafe extern "C" fn() -> u32>,
    pub stir: Option<unsafe extern "C" fn()>,
    pub uniform: Option<unsafe extern "C" fn(upper_bound: u32) -> u32>,
    pub buf: Option<unsafe extern "C" fn(buf: *mut c_void, size: usize)>,
    pub close: Option<unsafe extern "C" fn() -> c_int>,
}

include!(concat!(env!("OUT_DIR"), "/abi_layout_generated.rs"));

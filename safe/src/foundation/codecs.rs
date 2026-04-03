use crate::ffi::helpers::{abort_on_panic, set_errno, write_opt};
use core::ffi::{c_char, c_int};

fn eq(x: u32, y: u32) -> u32 {
    ((0u32.wrapping_sub(x ^ y) >> 8) & 0xff) ^ 0xff
}

fn gt(x: u32, y: u32) -> u32 {
    (y.wrapping_sub(x) >> 8) & 0xff
}

fn ge(x: u32, y: u32) -> u32 {
    gt(y, x) ^ 0xff
}

fn lt(x: u32, y: u32) -> u32 {
    gt(y, x)
}

fn le(x: u32, y: u32) -> u32 {
    ge(y, x)
}

fn b64_byte_to_char(x: u32) -> u8 {
    ((lt(x, 26) & x.wrapping_add(u32::from(b'A')))
        | (ge(x, 26) & lt(x, 52) & x.wrapping_add(u32::from(b'a') - 26))
        | (ge(x, 52) & lt(x, 62) & x.wrapping_add(u32::from(b'0').wrapping_sub(52)))
        | (eq(x, 62) & u32::from(b'+'))
        | (eq(x, 63) & u32::from(b'/'))) as u8
}

fn b64_char_to_byte(c: u8) -> u32 {
    let c = u32::from(c);
    let x = (ge(c, u32::from(b'A')) & le(c, u32::from(b'Z')) & c.wrapping_sub(u32::from(b'A')))
        | (ge(c, u32::from(b'a')) & le(c, u32::from(b'z')) & c.wrapping_sub(u32::from(b'a') - 26))
        | (ge(c, u32::from(b'0')) & le(c, u32::from(b'9')) & c.wrapping_sub(u32::from(b'0') - 52))
        | (eq(c, u32::from(b'+')) & 62)
        | (eq(c, u32::from(b'/')) & 63);

    x | (eq(x, 0) & (eq(c, u32::from(b'A')) ^ 0xff))
}

fn b64_byte_to_urlsafe_char(x: u32) -> u8 {
    ((lt(x, 26) & x.wrapping_add(u32::from(b'A')))
        | (ge(x, 26) & lt(x, 52) & x.wrapping_add(u32::from(b'a') - 26))
        | (ge(x, 52) & lt(x, 62) & x.wrapping_add(u32::from(b'0').wrapping_sub(52)))
        | (eq(x, 62) & u32::from(b'-'))
        | (eq(x, 63) & u32::from(b'_'))) as u8
}

fn b64_urlsafe_char_to_byte(c: u8) -> u32 {
    let c = u32::from(c);
    let x = (ge(c, u32::from(b'A')) & le(c, u32::from(b'Z')) & c.wrapping_sub(u32::from(b'A')))
        | (ge(c, u32::from(b'a')) & le(c, u32::from(b'z')) & c.wrapping_sub(u32::from(b'a') - 26))
        | (ge(c, u32::from(b'0')) & le(c, u32::from(b'9')) & c.wrapping_sub(u32::from(b'0') - 52))
        | (eq(c, u32::from(b'-')) & 62)
        | (eq(c, u32::from(b'_')) & 63);

    x | (eq(x, 0) & (eq(c, u32::from(b'A')) ^ 0xff))
}

fn variant_is_valid(variant: c_int) -> bool {
    ((variant as u32) & !0x6) == 0x1
}

fn encoded_len(bin_len: usize, variant: c_int) -> usize {
    let variant = variant as usize;
    ((bin_len / 3) * 4)
        + ((((bin_len - (bin_len / 3) * 3) | ((bin_len - (bin_len / 3) * 3) >> 1)) & 1)
            * (4 - (!(((variant & 2) >> 1).wrapping_sub(1)) & (3 - (bin_len - (bin_len / 3) * 3))))
            + 1)
}

unsafe fn contains_char(ignore: *const c_char, c: u8) -> bool {
    if ignore.is_null() {
        return false;
    }

    let mut ptr = ignore.cast::<u8>();
    while *ptr != 0 {
        if *ptr == c {
            return true;
        }
        ptr = ptr.add(1);
    }
    false
}

unsafe fn skip_padding(
    b64: *const u8,
    b64_len: usize,
    b64_pos: &mut usize,
    ignore: *const c_char,
    mut padding_len: usize,
) -> c_int {
    while padding_len > 0 {
        if *b64_pos >= b64_len {
            set_errno(libc::ERANGE);
            return -1;
        }
        let c = *b64.add(*b64_pos);
        if c == b'=' {
            padding_len -= 1;
        } else if !contains_char(ignore, c) {
            set_errno(libc::EINVAL);
            return -1;
        }
        *b64_pos += 1;
    }
    0
}

#[no_mangle]
pub extern "C" fn sodium_bin2hex(
    hex: *mut c_char,
    hex_maxlen: usize,
    bin: *const u8,
    bin_len: usize,
) -> *mut c_char {
    abort_on_panic(|| unsafe {
        if bin_len >= usize::MAX / 2 || hex_maxlen <= bin_len * 2 {
            crate::foundation::core::sodium_misuse();
        }

        for index in 0..bin_len {
            let byte = *bin.add(index);
            let high = byte >> 4;
            let low = byte & 0x0f;
            *hex.add(index * 2) = (if high < 10 {
                b'0' + high
            } else {
                b'a' + (high - 10)
            }) as c_char;
            *hex.add(index * 2 + 1) = (if low < 10 {
                b'0' + low
            } else {
                b'a' + (low - 10)
            }) as c_char;
        }
        *hex.add(bin_len * 2) = 0;
        hex
    })
}

#[no_mangle]
pub extern "C" fn sodium_hex2bin(
    bin: *mut u8,
    bin_maxlen: usize,
    hex: *const c_char,
    hex_len: usize,
    ignore: *const c_char,
    bin_len: *mut usize,
    hex_end: *mut *const c_char,
) -> c_int {
    abort_on_panic(|| unsafe {
        let mut bin_pos = 0usize;
        let mut hex_pos = 0usize;
        let mut ret = 0;
        let mut c_acc = 0u8;
        let mut state = 0u8;
        let hex = hex.cast::<u8>();

        while hex_pos < hex_len {
            let c = *hex.add(hex_pos);
            let c_num = c ^ 48;
            let c_num0 = ((c_num as u16).wrapping_sub(10) >> 8) as u8;
            let c_alpha = (c & !32).wrapping_sub(55);
            let c_alpha0 = (((c_alpha as u16).wrapping_sub(10) ^ (c_alpha as u16).wrapping_sub(16))
                >> 8) as u8;
            if (c_num0 | c_alpha0) == 0 {
                if state == 0 && contains_char(ignore, c) {
                    hex_pos += 1;
                    continue;
                }
                break;
            }
            let c_val = (c_num0 & c_num) | (c_alpha0 & c_alpha);
            if bin_pos >= bin_maxlen {
                ret = -1;
                set_errno(libc::ERANGE);
                break;
            }
            if state == 0 {
                c_acc = c_val << 4;
            } else {
                *bin.add(bin_pos) = c_acc | c_val;
                bin_pos += 1;
            }
            state = !state;
            hex_pos += 1;
        }

        if state != 0 {
            hex_pos = hex_pos.saturating_sub(1);
            set_errno(libc::EINVAL);
            ret = -1;
        }
        if ret != 0 {
            bin_pos = 0;
        }

        if !hex_end.is_null() {
            *hex_end = hex.add(hex_pos).cast();
        } else if hex_pos != hex_len {
            set_errno(libc::EINVAL);
            ret = -1;
        }

        write_opt(bin_len, bin_pos);
        ret
    })
}

#[no_mangle]
pub extern "C" fn sodium_base64_encoded_len(bin_len: usize, variant: c_int) -> usize {
    abort_on_panic(|| {
        if !variant_is_valid(variant) {
            crate::foundation::core::sodium_misuse();
        }
        encoded_len(bin_len, variant)
    })
}

#[no_mangle]
pub extern "C" fn sodium_bin2base64(
    b64: *mut c_char,
    b64_maxlen: usize,
    bin: *const u8,
    bin_len: usize,
    variant: c_int,
) -> *mut c_char {
    abort_on_panic(|| unsafe {
        if !variant_is_valid(variant) {
            crate::foundation::core::sodium_misuse();
        }

        let mut acc_len = 0usize;
        let mut b64_len = (bin_len / 3) * 4;
        let remainder = bin_len - (bin_len / 3) * 3;
        if remainder != 0 {
            if ((variant as u32) & 0x2) == 0 {
                b64_len += 4;
            } else {
                b64_len += 2 + (remainder >> 1);
            }
        }
        if b64_maxlen <= b64_len {
            crate::foundation::core::sodium_misuse();
        }

        let urlsafe = ((variant as u32) & 0x4) != 0;
        let mut acc = 0u32;
        let mut bin_pos = 0usize;
        let mut b64_pos = 0usize;
        let b64 = b64.cast::<u8>();

        while bin_pos < bin_len {
            acc = (acc << 8) + *bin.add(bin_pos) as u32;
            bin_pos += 1;
            acc_len += 8;
            while acc_len >= 6 {
                acc_len -= 6;
                let ch = if urlsafe {
                    b64_byte_to_urlsafe_char((acc >> acc_len) & 0x3f)
                } else {
                    b64_byte_to_char((acc >> acc_len) & 0x3f)
                };
                *b64.add(b64_pos) = ch;
                b64_pos += 1;
            }
        }

        if acc_len > 0 {
            let ch = if urlsafe {
                b64_byte_to_urlsafe_char((acc << (6 - acc_len)) & 0x3f)
            } else {
                b64_byte_to_char((acc << (6 - acc_len)) & 0x3f)
            };
            *b64.add(b64_pos) = ch;
            b64_pos += 1;
        }

        while b64_pos < b64_len {
            *b64.add(b64_pos) = b'=';
            b64_pos += 1;
        }
        while b64_pos < b64_maxlen {
            *b64.add(b64_pos) = 0;
            b64_pos += 1;
        }

        b64.cast()
    })
}

#[no_mangle]
pub extern "C" fn sodium_base642bin(
    bin: *mut u8,
    bin_maxlen: usize,
    b64: *const c_char,
    b64_len: usize,
    ignore: *const c_char,
    bin_len: *mut usize,
    b64_end: *mut *const c_char,
    variant: c_int,
) -> c_int {
    abort_on_panic(|| unsafe {
        if !variant_is_valid(variant) {
            crate::foundation::core::sodium_misuse();
        }

        let mut acc_len = 0usize;
        let mut b64_pos = 0usize;
        let mut bin_pos = 0usize;
        let mut ret = 0;
        let mut acc = 0u32;
        let urlsafe = ((variant as u32) & 0x4) != 0;
        let no_padding = ((variant as u32) & 0x2) != 0;
        let b64 = b64.cast::<u8>();

        while b64_pos < b64_len {
            let c = *b64.add(b64_pos);
            let d = if urlsafe {
                b64_urlsafe_char_to_byte(c)
            } else {
                b64_char_to_byte(c)
            };
            if d == 0xff {
                if contains_char(ignore, c) {
                    b64_pos += 1;
                    continue;
                }
                break;
            }
            acc = (acc << 6) + d;
            acc_len += 6;
            if acc_len >= 8 {
                acc_len -= 8;
                if bin_pos >= bin_maxlen {
                    set_errno(libc::ERANGE);
                    ret = -1;
                    break;
                }
                *bin.add(bin_pos) = ((acc >> acc_len) & 0xff) as u8;
                bin_pos += 1;
            }
            b64_pos += 1;
        }

        if acc_len > 4 || (acc & ((1u32 << acc_len) - 1)) != 0 {
            ret = -1;
        } else if ret == 0 && !no_padding {
            ret = skip_padding(b64, b64_len, &mut b64_pos, ignore, acc_len / 2);
        }
        if ret != 0 {
            bin_pos = 0;
        } else if !ignore.is_null() {
            while b64_pos < b64_len && contains_char(ignore, *b64.add(b64_pos)) {
                b64_pos += 1;
            }
        }

        if !b64_end.is_null() {
            *b64_end = b64.add(b64_pos).cast();
        } else if b64_pos != b64_len {
            set_errno(libc::EINVAL);
            ret = -1;
        }

        write_opt(bin_len, bin_pos);
        ret
    })
}

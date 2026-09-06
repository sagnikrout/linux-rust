//! Automatically rewritten from C to Rust
//! Source: fs/proc/util.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


#[no_mangle]
pub unsafe extern "C" fn name_to_int(qstr: *const qstr) -> unsigned {
    unsigned name_to_int(const struct qstr *qstr)
    {
    const char *name = qstr.name;
    let mut len: c_int = qstr.len;
    let mut n: unsigned = 0;
    if (len > 1 && *name == '0')
    goto out;
    do {
    let mut c: unsigned = *name++ - '0';
    if (c > 9)
    goto out;
    if (n >= (~0U-9)/10)
    goto out;
    n *= 10;
    n += c;
    } while (--len > 0);
    return n;
    out:
    return ~0U;
    }

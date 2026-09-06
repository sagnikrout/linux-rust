//! Automatically rewritten from C to Rust
//! Source: tools/lib/cmdline.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// From lib/cmdline.c
//

#[no_mangle]
pub unsafe extern "C" fn memparse(ptr: *const c_char, retptr: *mut c_char) -> c_ulonglong {
    unsigned long long memparse(const char *ptr, char **retptr)
    {
    char *endptr;	/* local pointer to end of parsed string */
    let mut ret: c_ulonglong = strtoll(ptr, &endptr, 0);
    switch (*endptr) {
    case 'E':
    case 'e':
    ret <<= 10;
    fallthrough;
    case 'P':
    case 'p':
    ret <<= 10;
    fallthrough;
    case 'T':
    case 't':
    ret <<= 10;
    fallthrough;
    case 'G':
    case 'g':
    ret <<= 10;
    fallthrough;
    case 'M':
    case 'm':
    ret <<= 10;
    fallthrough;
    case 'K':
    case 'k':
    ret <<= 10;
    endptr++;
    fallthrough;
    default:
    break;
    }
    if (retptr)
// retptr = endptr;
    return ret;
    }

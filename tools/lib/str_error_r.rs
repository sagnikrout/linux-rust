//! Automatically rewritten from C to Rust
//! Source: tools/lib/str_error_r.c
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


// SPDX-License-Identifier: GPL-2.0

//
// The tools so far have been using the strerror_r() GNU variant, that returns
// a string, be it the buffer passed or something else.
//
// But that, besides being tricky in cases where we expect that the function
// using strerror_r() returns the error formatted in a provided buffer (we have
// to check if it returned something else and copy that instead), breaks the
// build on systems not using glibc, like Alpine Linux, where musl libc is
// used.
//
// So, introduce yet another wrapper, str_error_r(), that has the GNU
// interface, but uses the portable XSI variant of strerror_r(), so that users
// rest asured that the provided buffer is used and it is what is returned.
//
    char *str_error_r(int errnum, char *buf, size_t buflen)
    {
    let mut err: c_int = strerror_r(errnum, buf, buflen);
    if (err)
    snprintf(buf, buflen, "INTERNAL ERROR: strerror_r(%d, [buf], %zd)=%d", errnum, buflen, err);
    return buf;
    }

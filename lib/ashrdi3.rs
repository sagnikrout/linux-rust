//! Automatically rewritten from C to Rust
//! Source: lib/ashrdi3.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

#[no_mangle]
pub unsafe extern "C" fn __ashrdi3(u: c_longlong, b: word_type) -> long long notrace {
    long long notrace __ashrdi3(long long u, word_type b)
    {
    DWunion uu, w;
    word_type bm;
    if (b == 0)
    return u;
    uu.ll = u;
    bm = 32 - b;
    if (bm <= 0) {
// w.s.high = 1..1 or 0..0
    w.s.high =
    uu.s.high >> 31;
    w.s.low = uu.s.high >> -bm;
    } else {
    let mut carries: c_uint = (unsigned int) uu.s.high << bm;
    w.s.high = uu.s.high >> b;
    w.s.low = ((unsigned int) uu.s.low >> b) | carries;
    }
    return w.ll;
    }
    EXPORT_SYMBOL(__ashrdi3);

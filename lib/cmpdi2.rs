//! Automatically rewritten from C to Rust
//! Source: lib/cmpdi2.c
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
pub unsafe extern "C" fn __cmpdi2(a: c_longlong, b: c_longlong) -> word_type notrace {
    word_type notrace __cmpdi2(long long a, long long b)
    {
    const DWunion au = {
    .ll = a
    };
    const DWunion bu = {
    .ll = b
    };
    if (au.s.high < bu.s.high)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(bu.s.high: au.s.high >) -> else {
    else if (au.s.high > bu.s.high)
    return 2;
    if ((unsigned int) au.s.low < (unsigned int) bu.s.low)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(bu.s.low: (unsigned int) au.s.low > (unsigned int)) -> else {
    else if ((unsigned int) au.s.low > (unsigned int) bu.s.low)
    return 2;
    return 1;
    }
    EXPORT_SYMBOL(__cmpdi2);

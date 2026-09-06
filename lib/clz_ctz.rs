//! Automatically rewritten from C to Rust
//! Source: lib/clz_ctz.c
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
// lib/clz_ctz.c
//
// Copyright (C) 2013 Chanho Min <chanho.min@lge.com>
//
// The functions in this file aren't called directly, but are required by
// GCC builtins such as __builtin_ctz, and therefore they can't be removed
// despite appearing unreferenced in kernel source.
//
// __c[lt]z[sd]i2 can be overridden by linking arch-specific versions.
//

    int __weak __ctzsi2(int val);
#[no_mangle]
pub unsafe extern "C" fn __ctzsi2(val: c_int) -> int __weak __attribute_const__ {
    int __weak __attribute_const__ __ctzsi2(int val)
    {
    return __ffs(val);
    }
    EXPORT_SYMBOL(__ctzsi2);
    int __weak __clzsi2(int val);
#[no_mangle]
pub unsafe extern "C" fn __clzsi2(val: c_int) -> int __weak __attribute_const__ {
    int __weak __attribute_const__ __clzsi2(int val)
    {
    return 32 - fls(val);
    }
    EXPORT_SYMBOL(__clzsi2);
    int __weak __clzdi2(u64 val);
#[no_mangle]
pub unsafe extern "C" fn __clzdi2(val: u64) -> int __weak __attribute_const__ {
    int __weak __attribute_const__ __clzdi2(u64 val)
    {
    return 64 - fls64(val);
    }
    EXPORT_SYMBOL(__clzdi2);
    int __weak __ctzdi2(u64 val);
#[no_mangle]
pub unsafe extern "C" fn __ctzdi2(val: u64) -> int __weak __attribute_const__ {
    int __weak __attribute_const__ __ctzdi2(u64 val)
    {
    return __ffs64(val);
    }
    EXPORT_SYMBOL(__ctzdi2);

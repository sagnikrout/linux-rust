//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/vdso/vdso64/vgetrandom.c
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
// Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[no_mangle]
pub unsafe extern "C" fn __vdso_getrandom(buffer: *mut c_void, len: usize, flags: c_uint, opaque_state: *mut c_void, opaque_len: usize) -> isize {
    ssize_t __vdso_getrandom(void *buffer, size_t len, unsigned int flags, void *opaque_state, size_t opaque_len)
    {
    return __cvdso_getrandom(buffer, len, flags, opaque_state, opaque_len);
    }
#[no_mangle]
pub unsafe extern "C" fn getrandom(: *mut c_void, _arg: usize, int: unsigned, : *mut c_void, _arg: usize) -> isize {
    ssize_t getrandom(void *, size_t, unsigned int, void *, size_t)
    __attribute__((weak, alias("__vdso_getrandom")));

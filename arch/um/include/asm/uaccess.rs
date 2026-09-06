//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/asm/uaccess.h
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
// Copyright (C) 2002 Jeff Dike (jdike@karaya.com)
// Copyright (C) 2015 Richard Weinberger (richard@nod.at)
//

extern "C" {
    pub fn raw_copy_from_user(to: *mut c_void, from: *const void __user, n: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn raw_copy_to_user(to: *mut void __user, from: *const c_void, n: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn __clear_user(mem: *mut void __user, len: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn __access_ok(ptr: *const void __user, size: c_ulong) -> c_int;
}
// Teach asm-generic/uaccess.h that we have C functions for these.

// Macro flag: #define INLINE_COPY_USER

extern "C" {
    pub fn __addr_range_nowrap(_arg: addr, __under_task_size(addr: size) &&, _arg: size) -> return;
}

// ((type *)dst) = (type) 0;				\
// ((type *)dst) = get_unaligned((type *)(src));			\


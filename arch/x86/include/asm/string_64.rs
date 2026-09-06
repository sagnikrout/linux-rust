//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/string_64.h
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

// Written 2002 by Andi Kleen
// Even with __builtin_ the compiler may decide to use the out of line

pub const __HAVE_ARCH_MEMCPY: c_int = 1;
//
// KMSAN needs to instrument as much code as possible. Use C versions of
// memsetXX() from lib/string.c under KMSAN.
//

extern "C" {
    pub fn memcmp(cs: *const c_void, ct: *const c_void, count: usize) -> c_int;
}
extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
}
extern "C" {
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
}

pub const __HAVE_ARCH_MEMCPY_FLUSHCACHE: c_int = 1;
extern "C" {
    pub fn __memcpy_flushcache(dst: *mut c_void, src: *const c_void, cnt: usize);
}


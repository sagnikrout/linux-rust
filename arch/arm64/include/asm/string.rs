//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/string.h
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
// Copyright (C) 2013 ARM Ltd.
//

extern "C" {
    pub fn strcmp(: *const c_char, : *const c_char) -> c_int;
}
extern "C" {
    pub fn strncmp(: *const c_char, : *const c_char, _arg: __kernel_size_t) -> c_int;
}
extern "C" {
    pub fn strlen(: *const c_char) -> __kernel_size_t;
}
extern "C" {
    pub fn strnlen(: *const c_char, _arg: __kernel_size_t) -> __kernel_size_t;
}
extern "C" {
    pub fn memcmp(: *const c_void, : *const c_void, _arg: usize) -> c_int;
}

extern "C" {
    pub fn memcpy_flushcache(dst: *mut c_void, src: *const c_void, cnt: usize);
}

//
// For files that are not instrumented (e.g. mm/slub.c) we
// should use not instrumented version of mem* functions.
//


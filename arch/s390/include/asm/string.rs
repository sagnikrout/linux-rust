//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/string.h
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
// S390 version
// Copyright IBM Corp. 1999
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
//

// Prototypes for non-inlined arch strings functions.
extern "C" {
    pub fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}
extern "C" {
    pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

extern "C" {
    pub fn __memset16(_arg: s, _arg: v, sizeof(v): *mut *mut count) -> return;
}

extern "C" {
    pub fn __memset32(_arg: s, _arg: v, sizeof(v): *mut *mut count) -> return;
}

extern "C" {
    pub fn __memset64(_arg: s, _arg: v, sizeof(v): *mut *mut count) -> return;
}

extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
}
extern "C" {
    pub fn strnlen(s: *const *const c_char, n: usize) -> usize;
}


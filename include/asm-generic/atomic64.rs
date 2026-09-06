//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/atomic64.h
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
// Generic implementation of 64-bit atomics using spinlocks,
// useful on processors that don't have 64-bit atomic instructions.
//
// Copyright © 2009 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
//

extern "C" {
    pub fn generic_atomic64_read(v: *const core::sync::atomic::AtomicI64) -> i64;
}
extern "C" {
    pub fn generic_atomic64_set(v: *mut core::sync::atomic::AtomicI64, i: i64);
}

extern "C" {
    pub fn generic_atomic64_dec_if_positive(v: *mut core::sync::atomic::AtomicI64) -> i64;
}
extern "C" {
    pub fn generic_atomic64_cmpxchg(v: *mut core::sync::atomic::AtomicI64, o: i64, n: i64) -> i64;
}
extern "C" {
    pub fn generic_atomic64_xchg(v: *mut core::sync::atomic::AtomicI64, new: i64) -> i64;
}
extern "C" {
    pub fn generic_atomic64_fetch_add_unless(v: *mut core::sync::atomic::AtomicI64, a: i64, u: i64) -> i64;
}


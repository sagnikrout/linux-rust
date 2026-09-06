//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/cmpxchg.h
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
// Generic UP xchg and cmpxchg using interrupt disablement.  Does not
// support SMP.
//

//
// This function doesn't exist, so you'll get a linker error if
// something tries to do an invalidly-sized xchg().
//
extern "C" {
    pub fn __generic_xchg_called_with_bad_pointer();
}

extern "C" {
    pub fn __xchg_u8(_arg: x, _arg: ptr) -> return;
}

// (volatile u8 *)ptr = (x & 0xffu);

extern "C" {
    pub fn __xchg_u16(_arg: x, _arg: ptr) -> return;
}

// (volatile u16 *)ptr = (x & 0xffffu);

extern "C" {
    pub fn __xchg_u32(_arg: x, _arg: ptr) -> return;
}

// (volatile u32 *)ptr = (x & 0xffffffffu);

extern "C" {
    pub fn __xchg_u64(_arg: x, _arg: ptr) -> return;
}

// (volatile u64 *)ptr = x;

//
// Atomic compare and exchange.
//


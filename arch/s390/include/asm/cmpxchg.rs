//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/cmpxchg.h
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
// Copyright IBM Corp. 1999, 2011
//
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
//

extern "C" {
    pub fn __cmpxchg_called_with_bad_pointer();
}

// __oldp = __prev;				\

// __oldp = __prev;					\

extern "C" {
    pub fn __xchg_called_with_bad_pointer();
}
extern "C" {
    pub fn __arch_xchg1(_arg: ptr, 0xff: x &) -> return;
}
extern "C" {
    pub fn __arch_xchg2(_arg: ptr, 0xffff: x &) -> return;
}

pub const system_has_cmpxchg128(): c_int = 1;

extern "C" {
    pub fn likely(0: cc ==) -> return;
}


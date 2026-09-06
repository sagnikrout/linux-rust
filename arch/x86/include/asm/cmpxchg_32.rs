//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cmpxchg_32.h
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
// Note: if you use __cmpxchg64(), or their variants,
// you need to test for the feature in boot_cpu_data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union __u64_halves {
    pub full: u64,
    pub high: u32 low,,
}

extern "C" {
    pub fn __arch_cmpxchg64(_arg: ptr, _arg: old, _arg: new, _arg: LOCK_PREFIX) -> return;
}
extern "C" {
    pub fn __arch_cmpxchg64(_arg: ptr, _arg: old, _arg: new, _arg: ) -> return;
}

// (_oldp) = o.full;					\
extern "C" {
    pub fn __arch_try_cmpxchg64(_arg: ptr, _arg: oldp, _arg: new, _arg: LOCK_PREFIX) -> return;
}
extern "C" {
    pub fn __arch_try_cmpxchg64(_arg: ptr, _arg: oldp, _arg: new, _arg: ) -> return;
}

//
// Building a kernel capable running on 80386 and 80486. It may be necessary
// to simulate the cmpxchg8b on the 80386 and 80486 CPU.
//

extern "C" {
    pub fn __arch_cmpxchg64_emu(_arg: ptr, _arg: old, _arg: new, _arg: , ": "lock) -> return;
}

extern "C" {
    pub fn __arch_cmpxchg64_emu(_arg: ptr, _arg: old, _arg: new, _arg: , _arg: ) -> return;
}

// (_oldp) = o.full;					\
extern "C" {
    pub fn __arch_try_cmpxchg64_emu(_arg: ptr, _arg: oldp, _arg: new, _arg: , ": "lock) -> return;
}

extern "C" {
    pub fn __arch_try_cmpxchg64_emu(_arg: ptr, _arg: oldp, _arg: new, _arg: , _arg: ) -> return;
}


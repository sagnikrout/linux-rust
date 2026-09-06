//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cmpxchg.h
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
// Atomic exchange
//
// Changes the memory location '*p' to be val and returns
// the previous value stored there.
//

extern "C" {
    pub fn __xchg_u8_local(_arg: ptr, _arg: x) -> return;
}
extern "C" {
    pub fn __xchg_u16_local(_arg: ptr, _arg: x) -> return;
}
extern "C" {
    pub fn __xchg_u32_local(_arg: ptr, _arg: x) -> return;
}

extern "C" {
    pub fn __xchg_u64_local(_arg: ptr, _arg: x) -> return;
}

extern "C" {
    pub fn __xchg_u8_relaxed(_arg: ptr, _arg: x) -> return;
}
extern "C" {
    pub fn __xchg_u16_relaxed(_arg: ptr, _arg: x) -> return;
}
extern "C" {
    pub fn __xchg_u32_relaxed(_arg: ptr, _arg: x) -> return;
}

extern "C" {
    pub fn __xchg_u64_relaxed(_arg: ptr, _arg: x) -> return;
}

//
// Compare and exchange - if *p == old, set it to new,
// and return the old value of *p.
//

//
// cmpxchg family don't have order guarantee if cmp part fails, therefore we
// can avoid superfluous barriers if we use assembly code to implement
// cmpxchg() and cmpxchg_acquire(), however we don't do the similar for
// cmpxchg_release() because that will result in putting a barrier in the
// middle of a ll/sc loop, which is probably a bad idea. For example, this
// might cause the conditional store more likely to fail.
//

extern "C" {
    pub fn __cmpxchg_u8(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u16(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u32(_arg: ptr, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __cmpxchg_u64(_arg: ptr, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __cmpxchg_u8_local(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u16_local(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u32_local(_arg: ptr, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __cmpxchg_u64_local(_arg: ptr, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __cmpxchg_u8_relaxed(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u16_relaxed(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u32_relaxed(_arg: ptr, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __cmpxchg_u64_relaxed(_arg: ptr, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __cmpxchg_u8_acquire(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u16_acquire(_arg: ptr, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn __cmpxchg_u32_acquire(_arg: ptr, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn __cmpxchg_u64_acquire(_arg: ptr, _arg: old, _arg: new) -> return;
}


//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/atomic64_64.h
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

// The 64-bit atomic type

extern "C" {
    pub fn __READ_ONCE(_arg: (v)->counter) -> return;
}
extern "C" {
    pub fn GEN_BINARY_RMWcc("subq": LOCK_PREFIX, _arg: v->counter, _arg: e, _arg: "er", _arg: i) -> return;
}

extern "C" {
    pub fn GEN_UNARY_RMWcc("decq": LOCK_PREFIX, _arg: v->counter, _arg: e) -> return;
}

extern "C" {
    pub fn GEN_UNARY_RMWcc("incq": LOCK_PREFIX, _arg: v->counter, _arg: e) -> return;
}

extern "C" {
    pub fn GEN_BINARY_RMWcc("addq": LOCK_PREFIX, _arg: v->counter, _arg: s, _arg: "er", _arg: i) -> return;
}

extern "C" {
    pub fn xadd(_arg: &v->counter, _arg: i) -> return;
}

extern "C" {
    pub fn arch_cmpxchg(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn arch_try_cmpxchg(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn arch_xchg(_arg: &v->counter, _arg: new) -> return;
}


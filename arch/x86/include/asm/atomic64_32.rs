//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/atomic64_32.h
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

// #include <asm/cmpxchg.h>
// An 64bit atomic type

//
// Read an atomic64_t non-atomically.
//
// This is intended to be used in cases where a subsequent atomic operation
// will handle the torn value, and can be used to prime the first iteration
// of unconditional try_cmpxchg() loops, e.g.:
//
// s64 val = arch_atomic64_read_nonatomic(v);
// do { } while (!arch_atomic64_try_cmpxchg(v, &val, val OP i);
//
// This is NOT safe to use where the value is not always checked by a
// subsequent atomic operation, such as in conditional try_cmpxchg() loops
// that can break before the atomic operation, e.g.:
//
// s64 val = arch_atomic64_read_nonatomic(v);
// do {
// if (condition(val))
// break;
// } while (!arch_atomic64_try_cmpxchg(v, &val, val OP i);
//
// See comment in arch_atomic_read().
extern "C" {
    pub fn __READ_ONCE(_arg: v->counter) -> return;
}

extern "C" {
    pub fn arch_cmpxchg64(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

extern "C" {
    pub fn arch_try_cmpxchg64(_arg: &v->counter, _arg: old, _arg: new) -> return;
}

// no output */,
// no input */,

// no input */,

// no input */,
// no output */,

// no output */,


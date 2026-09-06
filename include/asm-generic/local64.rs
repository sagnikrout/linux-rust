//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/local64.h
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
// A signed long type for operations which are atomic for a single CPU.
// Usually used in combination with per-cpu variables.
//
// This is the default implementation, which uses atomic64_t.  Which is
// rather pointless.  The whole point behind local64_t is that some processors
// can perform atomic adds and subtracts in a manner which is atomic wrt IRQs
// running on this CPU.  local64_t allows exploitation of such capabilities.
//
// Implement in terms of atomics.

extern "C" {
    pub fn local_cmpxchg(_arg: &l->a, _arg: old, _arg: new) -> return;
}
extern "C" {
    pub fn local_try_cmpxchg(_arg: &l->a, )old: *mut (long, _arg: new) -> return;
}

// Non-atomic variants, ie. preemption disabled and won't be touched
// in interrupt, etc.  Some archs can optimize this case well.

// Don't use typedef: don't want them to be mixed with atomic_t's.

// Non-atomic variants, ie. preemption disabled and won't be touched
// in interrupt, etc.  Some archs can optimize this case well.


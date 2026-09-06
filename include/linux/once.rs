//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/once.h
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

// Helpers used from arbitrary contexts.
// Hard irqs are blocked, be cautious.
//
extern "C" {
    pub fn __do_once_start(done: *mut bool, flags: *mut c_ulong) -> bool;
}
// Variant for process contexts only.
extern "C" {
    pub fn __do_once_sleepable_start(done: *mut bool) -> bool;
}
// Call a function exactly once. The idea of DO_ONCE() is to perform
// a function call such as initialization of random seeds, etc, only
// once, where DO_ONCE() can live in the fast-path. After @func has
// been called with the passed arguments, the static key will patch
// out the condition into a nop. DO_ONCE() guarantees type safety of
// arguments!
//
// Note that the following is not equivalent ...
//
// DO_ONCE(func, arg);
//
// ... to this version:
//
// void foo(void)
// {
// DO_ONCE(func, arg);
// }
//
// foo();
//
// In case the one-time invocation could be triggered from multiple
// places, then a common helper function must be defined, so that only
// a single static key will be placed there!
//

// Variant of DO_ONCE() for process/sleepable contexts.


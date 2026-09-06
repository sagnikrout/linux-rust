//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/local.h
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

extern "C" {
    pub fn READ_ONCE(_arg: l->v) -> return;
}

//
// local_inc_and_test - increment and test
// @l: pointer of type local_t
//
// Atomically increments @l by 1
// and returns true if the result is zero, or false for all
// other cases.
//

// po = r;
extern "C" {
    pub fn likely(o: r ==) -> return;
}
//
// local_add_unless - add unless the number is already a given value
// @l: pointer of type local_t
// @a: the amount to add to v...
// @u: ...unless v is equal to u.
//
// Atomically adds @a to @l, if @v was not already @u.
// Returns true if the addition was done.
//

// Use these for per-cpu local_t variables: on some archs they are
// much more efficient than these naive implementations.  Note they take
// a variable, not an address.
//


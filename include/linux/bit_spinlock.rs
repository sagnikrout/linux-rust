//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bit_spinlock.h
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
// For static context analysis, we need a unique token for each possible bit
// that can be used as a bit_spinlock. The easiest way to do that is to create a
// fake context that we can cast to with the __bitlock(bitnum, addr) macro
// below, which will give us unique instances for each (bit, addr) pair that the
// static analysis can use.
//

//
// bit-based spin_lock()
//
// Don't use this unless you really need to: spin_lock() and spin_unlock()
// are significantly faster.
//
// Assuming the lock is uncontended, this never enters
// the body of the outer loop. If it is contended, then
// within the inner loop a non-atomic test is used to
// busywait with less bus contention for a good time to
// attempt to acquire the lock bit.
//

//
// Return true if it was acquired
//

//
// bit-based spin_unlock()
//

//
// bit-based spin_unlock()
// non-atomic version, which can be used eg. if the bit lock itself is
// protecting the rest of the flags in the word.
//

//
// Return true if the lock is held.
//

extern "C" {
    pub fn test_bit(_arg: bitnum, _arg: addr) -> return;
}

extern "C" {
    pub fn preempt_count() -> return;
}


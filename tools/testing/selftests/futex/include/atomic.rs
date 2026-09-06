//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/futex/include/atomic.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright © International Business Machines  Corp., 2009
//
// DESCRIPTION
// GCC atomic builtin wrappers
// http://gcc.gnu.org/onlinedocs/gcc-4.1.0/gcc/Atomic-Builtins.html
//
// AUTHOR
// Darren Hart <dvhart@linux.intel.com>
//
// HISTORY
// 2009-Nov-17: Initial version by Darren Hart <dvhart@linux.intel.com>
//

//
// atomic_cmpxchg() - Atomic compare and exchange
// @uaddr:	The address of the futex to be modified
// @oldval:	The expected value of the futex
// @newval:	The new value to try and assign the futex
//
// Return the old value of addr->val.
//
extern "C" {
    pub fn __sync_val_compare_and_swap(_arg: &addr->val, _arg: oldval, _arg: newval) -> return;
}
//
// atomic_inc() - Atomic incrememnt
// @addr:	Address of the variable to increment
//
// Return the new value of addr->val.
//
extern "C" {
    pub fn __sync_add_and_fetch(_arg: &addr->val, _arg: 1) -> return;
}
//
// atomic_dec() - Atomic decrement
// @addr:	Address of the variable to decrement
//
// Return the new value of addr-val.
//
extern "C" {
    pub fn __sync_sub_and_fetch(_arg: &addr->val, _arg: 1) -> return;
}
//
// atomic_set() - Atomic set
// @addr:	Address of the variable to set
// @newval:	New value for the atomic_t
//
// Return the new value of addr->val.
//

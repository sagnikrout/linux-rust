//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/futex.h
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
// The following implementation only for uniprocessor machines.
// It relies on preempt_disable() ensuring mutual exclusion.
//

//
// futex_atomic_op_inuser_local() - Atomic arithmetic operation with constant
// argument and comparison of the previous
// futex value with another constant.
//
// @op:		operation to execute
// @oparg:	argument of the operation
// @oval:	previous value at @uaddr on successful return
// @uaddr:	pointer to user space address
//
// Return:
// 0 - On success
// -EFAULT - User access resulted in a page fault
// -EAGAIN - Atomic operation was unable to complete due to contention
// -ENOSYS - Operation not supported
//
// oval = oldval;
//
// futex_atomic_cmpxchg_inatomic_local() - Compare and exchange the content of the
// uaddr with newval if the current value is
// oldval.
// @uval:	pointer to store content of @uaddr
// @uaddr:	pointer to user space address
// @oldval:	old value
// @newval:	new value to store to @uaddr
//
// Return:
// 0 - On success
// -EFAULT - User access resulted in a page fault
// -EAGAIN - Atomic operation was unable to complete due to contention
//
// uval = val;

//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/futex/include/futextest.h
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
// Glibc independent futex library for testing kernel functionality.
//
// AUTHOR
// Darren Hart <dvhart@linux.intel.com>
//
// HISTORY
// 2009-Nov-6: Initial version by Darren Hart <dvhart@linux.intel.com>
//

pub type futex_t = volatile u_int32_t;
pub const FUTEX_INITIALIZER: c_int = 0;
// Define the newer op codes if the system header file is not up to date.

pub const FUTEX_WAIT_BITSET: c_int = 9;

pub const FUTEX_WAKE_BITSET: c_int = 10;

pub const FUTEX_WAIT_REQUEUE_PI: c_int = 11;

pub const FUTEX_CMP_REQUEUE_PI: c_int = 12;

pub const FUTEX_ROBUST_UNLOCK: c_int = 512;

pub const FUTEX_ROBUST_LIST32: c_int = 1024;

//
// SYS_futex is expected from system C library, in glibc some 32-bit
// architectures (e.g. RV32) are using 64-bit time_t, therefore it doesn't have
// SYS_futex defined but just SYS_futex_time64. Define SYS_futex as
// SYS_futex_time64 in this situation to ensure the compilation and the
// compatibility.
//

//
// On 32bit systems if we use "-D_FILE_OFFSET_BITS=64 -D_TIME_BITS=64" or if
// we are using a newer compiler then the size of the timestamps will be 64bit,
// however, the SYS_futex will still point to the 32bit futex system call.
//

//
// futex() - SYS_futex syscall wrapper
// @uaddr:	address of first futex
// @op:		futex op code
// @val:	typically expected value of uaddr, but varies by op
// @timeout:	typically an absolute struct timespec (except where noted
// otherwise). Overloaded by some ops
// @uaddr2:	address of second futex for some ops\
// @val3:	varies by op
// @opflags:	flags to be bitwise OR'd with op, such as FUTEX_PRIVATE_FLAG
//
// futex() is used by all the following futex op wrappers. It can also be
// used for misuse and abuse testing. Generally, the specific op wrappers
// should be used instead. It is a macro instead of an static inline function as
// some of the types over overloaded (timeout is used for nr_requeue for
// example).
//
// These argument descriptions are the defaults for all
// like-named arguments in the following wrappers except where noted below.
//

//
// futex_wait() - block on uaddr with optional timeout
// @timeout:	relative timeout
//
extern "C" {
    pub fn futex(_arg: uaddr, _arg: FUTEX_WAIT, _arg: val, _arg: timeout, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_wake() - wake one or more tasks blocked on uaddr
// @nr_wake:	wake up to this many tasks
//
extern "C" {
    pub fn futex(_arg: uaddr, _arg: FUTEX_WAKE, _arg: nr_wake, _arg: NULL, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_wait_bitset() - block on uaddr with bitset
// @bitset:	bitset to be used with futex_wake_bitset
//
// futex_wake_bitset() - wake one or more tasks blocked on uaddr with bitset
// @bitset:	bitset to compare with that used in futex_wait_bitset
//
// futex_lock_pi() - block on uaddr as a PI mutex
// @detect:	whether (1) or not (0) to perform deadlock detection
//
extern "C" {
    pub fn futex(_arg: uaddr, _arg: FUTEX_LOCK_PI, _arg: detect, _arg: timeout, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_unlock_pi() - release uaddr as a PI mutex, waking the top waiter
//
extern "C" {
    pub fn futex(_arg: uaddr, _arg: FUTEX_UNLOCK_PI, _arg: 0, _arg: NULL, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_wake_op() - FIXME: COME UP WITH A GOOD ONE LINE DESCRIPTION
//
// futex_requeue() - requeue without expected value comparison, deprecated
// @nr_wake:	wake up to this many tasks
// @nr_requeue:	requeue up to this many tasks
//
// Due to its inherently racy implementation, futex_requeue() is deprecated in
// favor of futex_cmp_requeue().
//
// futex_cmp_requeue() - requeue tasks from uaddr to uaddr2
// @nr_wake:	wake up to this many tasks
// @nr_requeue:	requeue up to this many tasks
//
// futex_wait_requeue_pi() - block on uaddr and prepare to requeue to uaddr2
// @uaddr:	non-PI futex source
// @uaddr2:	PI futex target
//
// This is the first half of the requeue_pi mechanism. It shall always be
// paired with futex_cmp_requeue_pi().
//
// futex_cmp_requeue_pi() - requeue tasks from uaddr to uaddr2 (PI aware)
// @uaddr:	non-PI futex source
// @uaddr2:	PI futex target
// @nr_wake:	wake up to this many tasks
// @nr_requeue:	requeue up to this many tasks
//
// futex_cmpxchg() - atomic compare and exchange
// @uaddr:	The address of the futex to be modified
// @oldval:	The expected value of the futex
// @newval:	The new value to try and assign the futex
//
// Implement cmpxchg using gcc atomic builtins.
// http://gcc.gnu.org/onlinedocs/gcc-4.1.0/gcc/Atomic-Builtins.html
//
// Return the old futex value.
//
extern "C" {
    pub fn __sync_val_compare_and_swap(_arg: uaddr, _arg: oldval, _arg: newval) -> return;
}
//
// futex_dec() - atomic decrement of the futex value
// @uaddr:	The address of the futex to be modified
//
// Return the new futex value.
//
extern "C" {
    pub fn __sync_sub_and_fetch(_arg: uaddr, _arg: 1) -> return;
}
//
// futex_inc() - atomic increment of the futex value
// @uaddr:	the address of the futex to be modified
//
// Return the new futex value.
//
extern "C" {
    pub fn __sync_add_and_fetch(_arg: uaddr, _arg: 1) -> return;
}
//
// futex_set() - atomic decrement of the futex value
// @uaddr:	the address of the futex to be modified
// @newval:	New value for the atomic_t
//
// Return the new futex value.
//
// uaddr = newval;

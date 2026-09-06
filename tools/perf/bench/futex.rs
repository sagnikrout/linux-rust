//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/bench/futex.h
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
// Glibc independent futex library for testing kernel functionality.
// Shamelessly stolen from Darren Hart <dvhltc@us.ibm.com>
// http://git.kernel.org/cgit/linux/kernel/git/dvhart/futextest.git
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bench_futex_parameters {
    pub silent: bool,
    pub fshared: bool,
    pub mlockall: bool,
    pub /: *mut *mut bool multi; / lock-pi,
    pub /: *mut *mut bool pi; / requeue-pi,
    pub /: *mut *mut bool broadcast; / requeue,
    pub seconds*/: *mut *mut unsigned int runtime; /,
    pub nthreads: c_uint,
    pub nfutexes: c_uint,
    pub nwakes: c_uint,
    pub nrequeue: c_uint,
    pub nbuckets: c_int,
}

//
// futex_syscall() - SYS_futex syscall wrapper
// @uaddr:	address of first futex
// @op:		futex op code
// @val:	typically expected value of uaddr, but varies by op
// @timeout:	typically an absolute struct timespec (except where noted
// otherwise). Overloaded by some ops
// @uaddr2:	address of second futex for some ops
// @val3:	varies by op
// @opflags:	flags to be bitwise OR'd with op, such as FUTEX_PRIVATE_FLAG
//
// futex_syscall() is used by all the following futex op wrappers. It can also be
// used for misuse and abuse testing. Generally, the specific op wrappers
// should be used instead.
//
// These argument descriptions are the defaults for all
// like-named arguments in the following wrappers except where noted below.
//
extern "C" {
    pub fn syscall(_arg: SYS_futex, _arg: uaddr, opflags: op |, _arg: val, _arg: timeout, _arg: uaddr2, _arg: val3) -> return;
}
extern "C" {
    pub fn syscall(_arg: SYS_futex, _arg: uaddr, opflags: op |, _arg: val, _arg: nr_requeue, _arg: uaddr2, _arg: val3) -> return;
}
//
// futex_wait() - block on uaddr with optional timeout
// @timeout:	relative timeout
//
extern "C" {
    pub fn futex_syscall(_arg: uaddr, _arg: FUTEX_WAIT, _arg: val, _arg: timeout, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_wake() - wake one or more tasks blocked on uaddr
// @nr_wake:	wake up to this many tasks
//
extern "C" {
    pub fn futex_syscall(_arg: uaddr, _arg: FUTEX_WAKE, _arg: nr_wake, _arg: NULL, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_lock_pi() - block on uaddr as a PI mutex
//
extern "C" {
    pub fn futex_syscall(_arg: uaddr, _arg: FUTEX_LOCK_PI, _arg: 0, _arg: timeout, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_unlock_pi() - release uaddr as a PI mutex, waking the top waiter
//
extern "C" {
    pub fn futex_syscall(_arg: uaddr, _arg: FUTEX_UNLOCK_PI, _arg: 0, _arg: NULL, _arg: NULL, _arg: 0, _arg: opflags) -> return;
}
//
// futex_cmp_requeue() - requeue tasks from uaddr to uaddr2
// @nr_wake:        wake up to this many tasks
// @nr_requeue:     requeue up to this many tasks
//
// futex_wait_requeue_pi() - block on uaddr and prepare to requeue to uaddr2
// @uaddr:	non-PI futex source
// @uaddr2:	PI futex target
//
// This is the first half of the requeue_pi mechanism. It shall always be
// paired with futex_cmp_requeue_pi().
//
// futex_cmp_requeue_pi() - requeue tasks from uaddr to uaddr2
// @uaddr:	non-PI futex source
// @uaddr2:	PI futex target
// @nr_requeue:	requeue up to this many tasks
//
// This is the second half of the requeue_pi mechanism. It shall always be
// paired with futex_wait_requeue_pi(). The first waker is always awoken.
//
extern "C" {
    pub fn futex_set_nbuckets_param(params: *mut bench_futex_parameters);
}
extern "C" {
    pub fn futex_print_nbuckets(params: *mut bench_futex_parameters);
}

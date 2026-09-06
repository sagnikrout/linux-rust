//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/posix-timers_types.h
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

// Macro flag: #define _linux_POSIX_TIMERS_TYPES_H

//
// Bit fields within a clockid:
//
// The most significant 29 bits hold either a pid or a file descriptor.
//
// Bit 2 indicates whether a cpu clock refers to a thread or a process.
//
// Bits 1 and 0 give the type: PROF=0, VIRT=1, SCHED=2, or FD=3.
//
// A clockid is invalid if bits 2, 1, and 0 are all set.
//

pub const CPUCLOCK_PERTHREAD_MASK: c_int = 4;

pub const CPUCLOCK_CLOCK_MASK: c_int = 3;
pub const CPUCLOCK_PROF: c_int = 0;
pub const CPUCLOCK_VIRT: c_int = 1;
pub const CPUCLOCK_SCHED: c_int = 2;
pub const CPUCLOCK_MAX: c_int = 3;

//
// struct posix_cputimer_base - Container per posix CPU clock
// @nextevt:		Earliest-expiration cache
// @tqhead:		timerqueue head for cpu_timers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_cputimer_base {
    pub nextevt: u64,
    pub tqhead: timerqueue_head,
}

//
// struct posix_cputimers - Container for posix CPU timer related data
// @bases:		Base container for posix CPU clocks
// @timers_active:	Timers are queued.
// @expiry_active:	Timer expiry is active. Used for
// process wide timers to avoid multiple
// task trying to handle expiry concurrently
//
// Used in task_struct and signal_struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_cputimers {
    pub bases: [posix_cputimer_base; CPUCLOCK_MAX],
    pub timers_active: c_uint,
    pub expiry_active: c_uint,
}

//
// struct posix_cputimers_work - Container for task work based posix CPU timer expiry
// @work:	The task work to be scheduled
// @mutex:	Mutex held around expiry in context of this task work
// @scheduled:  @work has been scheduled already, no further processing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_cputimers_work {
    pub work: callback_head,
    pub mutex: mutex,
    pub scheduled: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_cputimers {


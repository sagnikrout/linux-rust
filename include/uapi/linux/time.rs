//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/time.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub /: *mut *mut __kernel_old_time_t tv_sec; / seconds,
    pub /: *mut *mut long tv_nsec; / nanoseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub /: *mut *mut __kernel_old_time_t tv_sec; / seconds,
    pub /: *mut *mut __kernel_suseconds_t tv_usec; / microseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct itimerspec {
    pub /: *mut *mut timespec it_interval;/ timer period,
    pub /: *mut *mut timespec it_value; / timer expiration,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct itimerval {
    pub /: *mut *mut timeval it_interval;/ timer interval,
    pub /: *mut *mut timeval it_value; / current value,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
    pub /: *mut *mut int tz_minuteswest; / minutes west of Greenwich,
    pub /: *mut *mut int tz_dsttime; / type of dst correction,
}

//
// Names of the interval timers, and structure
// defining a timer setting:
//
pub const ITIMER_REAL: c_int = 0;
pub const ITIMER_VIRTUAL: c_int = 1;
pub const ITIMER_PROF: c_int = 2;
//
// The IDs of the various system clocks (for POSIX.1b interval timers):
//
pub const CLOCK_REALTIME: c_int = 0;
pub const CLOCK_MONOTONIC: c_int = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: c_int = 2;
pub const CLOCK_THREAD_CPUTIME_ID: c_int = 3;
pub const CLOCK_MONOTONIC_RAW: c_int = 4;
pub const CLOCK_REALTIME_COARSE: c_int = 5;
pub const CLOCK_MONOTONIC_COARSE: c_int = 6;
pub const CLOCK_BOOTTIME: c_int = 7;
pub const CLOCK_REALTIME_ALARM: c_int = 8;
pub const CLOCK_BOOTTIME_ALARM: c_int = 9;
//
// The driver implementing this got removed. The clock ID is kept as a
// place holder. Do not reuse!
//
pub const CLOCK_SGI_CYCLE: c_int = 10;
pub const CLOCK_TAI: c_int = 11;
pub const MAX_CLOCKS: c_int = 16;
//
// AUX clock support. AUXiliary clocks are dynamically configured by
// enabling a clock ID. These clock can be steered independently of the
// core timekeeper. The kernel can support up to 8 auxiliary clocks, but
// the actual limit depends on eventual architecture constraints vs. VDSO.
//

pub const MAX_AUX_CLOCKS: c_int = 8;

//
// The various flags for setting POSIX.1b interval timers:
//
pub const TIMER_ABSTIME: c_uint = 0x01;

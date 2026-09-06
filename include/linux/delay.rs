//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/delay.h
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
// Copyright (C) 1993 Linus Torvalds
//
// Delay routines, using a pre-computed "loops_per_jiffy" value.
// Sleep routines using timer list timers or hrtimers.
//

extern "C" {
    pub fn delay_read_timer(t: *mut c_ulong) -> bool;
}
//
// Using udelay() for intervals greater than a few milliseconds can
// risk overflow for high loops_per_jiffy (high bogomips) machines. The
// mdelay() provides a wrapper to prevent this.  For delays greater
// than MAX_UDELAY_MS milliseconds, the wrapper is used.  Architecture
// specific values can be defined in asm-???/delay.h as an override.
// The 2nd mdelay() definition ensures GCC will optimize away the
// while loop for the common cases where n <= MAX_UDELAY_MS  --  Paul G.
//

pub const MAX_UDELAY_MS: c_int = 5;

//
// mdelay - Inserting a delay based on milliseconds with busy waiting
// @n:	requested delay in milliseconds
//
// See udelay() for basic information about mdelay() and it's variants.
//
// Please double check, whether mdelay() is the right way to go or whether a
// refactoring of the code is the better variant to be able to use msleep()
// instead.
//

extern "C" {
    pub fn calibrate_delay();
}
extern "C" {
    pub fn calibrate_delay_is_known() -> c_ulong;
}
extern "C" {
    pub fn __attribute__(calibration_delay_done(void: (weak)));
}
extern "C" {
    pub fn msleep(msecs: c_uint);
}
extern "C" {
    pub fn msleep_interruptible(msecs: c_uint) -> c_ulong;
}
//
// usleep_range - Sleep for an approximate time
// @min:	Minimum time in microseconds to sleep
// @max:	Maximum time in microseconds to sleep
//
// For basic information please refer to usleep_range_state().
//
// The task will be in the state TASK_UNINTERRUPTIBLE during the sleep.
//
// usleep_range_idle - Sleep for an approximate time with idle time accounting
// @min:	Minimum time in microseconds to sleep
// @max:	Maximum time in microseconds to sleep
//
// For basic information please refer to usleep_range_state().
//
// The sleeping task has the state TASK_IDLE during the sleep to prevent
// contribution to the load average.
//
// ssleep - wrapper for seconds around msleep
// @seconds:	Requested sleep duration in seconds
//
// Please refer to msleep() for detailed information.
//

//
// fsleep - flexible sleep which autoselects the best mechanism
// @usecs:	requested sleep duration in microseconds
//
// fsleep() selects the best mechanism that will provide maximum 25% slack
// to the requested sleep duration. Therefore it uses:
//
// * udelay() loop for sleep durations <= 10 microseconds to avoid hrtimer
// overhead for really short sleep durations.
// * usleep_range() for sleep durations which would lead with the usage of
// msleep() to a slack larger than 25%. This depends on the granularity of
// jiffies.
// * msleep() for all other sleep durations.
//
// Note: When %CONFIG_HIGH_RES_TIMERS is not set, all sleeps are processed with
// the granularity of jiffies and the slack might exceed 25% especially for
// short sleep durations.
//

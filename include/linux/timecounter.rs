//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/timecounter.h
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
// linux/include/linux/timecounter.h
//
// based on code that migrated away from
// linux/include/linux/clocksource.h
//

// simplify initialization of mask field

//
// struct cyclecounter - hardware abstraction for a free running counter
// Provides completely state-free accessors to the underlying hardware.
// Depending on which hardware it reads, the cycle counter may wrap
// around quickly. Locking rules (if necessary) have to be defined
// by the implementor and user of specific instances of this API.
//
// @read:		returns the current cycle value
// @mask:		bitmask for two's complement
// subtraction of non-64-bit counters,
// see CYCLECOUNTER_MASK() helper macro
// @mult:		cycle to nanosecond multiplier
// @shift:		cycle to nanosecond divisor (power of two)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyclecounter {
    pub cc): *mut *mut u64 (read)(struct cyclecounter,
    pub mask: u64,
    pub mult: u32,
    pub shift: u32,
}

//
// struct timecounter - layer above a &struct cyclecounter which counts nanoseconds
// Contains the state needed by timecounter_read() to detect
// cycle counter wrap around. Initialize with
// timecounter_init(). Also used to convert cycle counts into the
// corresponding nanosecond counts with timecounter_cyc2time(). Users
// of this code are responsible for initializing the underlying
// cycle counter hardware, locking issues and reading the time
// more often than the cycle counter wraps around. The nanosecond
// counter will only wrap around after ~585 years.
//
// @cc:			the cycle counter used by this instance
// @cycle_last:		most recent cycle counter value seen by
// timecounter_read()
// @nsec:		continuously increasing count
// @mask:		bit mask for maintaining the 'frac' field
// @frac:		accumulated fractional nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timecounter {
    pub cc: *mut cyclecounter,
    pub cycle_last: u64,
    pub nsec: u64,
    pub mask: u64,
    pub frac: u64,
}

//
// cyclecounter_cyc2ns - converts cycle counter cycles to nanoseconds
// @cc:		Pointer to cycle counter.
// @cycles:	Cycles
// @mask:	bit mask for maintaining the 'frac' field
// @frac:	pointer to storage for the fractional nanoseconds.
//
// Returns: cycle counter cycles converted to nanoseconds
//
// frac = ns & mask;
//
// timecounter_adjtime - Shifts the time of the clock.
// @tc:		The &struct timecounter to adjust
// @delta:	Desired change in nanoseconds.
//
// timecounter_init - initialize a time counter
// @tc:			Pointer to time counter which is to be initialized/reset
// @cc:			A cycle counter, ready to be used.
// @start_tstamp:	Arbitrary initial time stamp.
//
// After this call the current cycle register (roughly) corresponds to
// the initial time stamp. Every call to timecounter_read() increments
// the time stamp counter by the number of elapsed nanoseconds.
//
// timecounter_read - return nanoseconds elapsed since timecounter_init()
// plus the initial time stamp
// @tc:          Pointer to time counter.
//
// In other words, keeps track of time since the same epoch as
// the function which generated the initial time stamp.
//
// Returns: nanoseconds since the initial time stamp
//
extern "C" {
    pub fn timecounter_read(tc: *mut timecounter) -> u64;
}
//
// This is like cyclecounter_cyc2ns(), but it is used for computing a
// time previous to the time stored in the cycle counter.
//
// timecounter_cyc2time - convert a cycle counter to same
// time base as values returned by
// timecounter_read()
// @tc:		Pointer to time counter.
// @cycle_tstamp:	a value returned by tc->cc->read()
//
// Cycle counts that are converted correctly as long as they
// fall into the interval [-1/2 max cycle count, +1/2 max cycle count],
// with "max cycle count" == cs->mask+1.
//
// This allows conversion of cycle counter values which were generated
// in the past.
//
// Returns: cycle counter converted to nanoseconds since the initial time stamp
//
// Instead of always treating cycle_tstamp as more recent than
// tc->cycle_last, detect when it is too far in the future and
// treat it as old time stamp instead.
//

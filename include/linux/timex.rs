//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/timex.h
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


//
// Copyright (c) David L. Mills 1993
//
// Permission to use, copy, modify, and distribute this software and its
// documentation for any purpose and without fee is hereby granted, provided
// that the above copyright notice appears in all copies and that both the
// copyright notice and this permission notice appear in supporting
// documentation, and that the name University of Delaware not be used in
// advertising or publicity pertaining to distribution of the software
// without specific, written prior permission.  The University of Delaware
// makes no representations about the suitability this software for any
// purpose.  It is provided "as is" without express or implied warranty.
//
// Modification history timex.h
//
// 29 Dec 97	Russell King
// Moved CLOCK_TICK_RATE, CLOCK_TICK_FACTOR and FINETUNE to asm/timex.h
// for ARM machines
//
// 9 Jan 97    Adrian Sun
// Shifted LATCH define to allow access to alpha machines.
//
// 26 Sep 94	David L. Mills
// Added defines for hybrid phase/frequency-lock loop.
//
// 19 Mar 94	David L. Mills
// Moved defines from kernel routines to header file and added new
// defines for PPS phase-lock loop.
//
// 20 Feb 94	David L. Mills
// Revised status codes and structures for external clock and PPS
// signal discipline.
//
// 28 Nov 93	David L. Mills
// Adjusted parameters to improve stability and increase poll
// interval.
//
// 17 Sep 93    David L. Mills
// Created file $NTP/include/sys/timex.h
// 07 Oct 93    Torsten Duwe
// Derived linux/timex.h
// 1995-08-13    Torsten Duwe
// kernel PLL updated to 1994-12-13 specs (rfc-1589)
// 1997-08-30    Ulrich Windl
// Added new constant NTP_PHASE_LIMIT
// 2004-08-12    Christoph Lameter
// Reworked time interpolation logic
//

pub const ADJ_ADJTIME: c_uint = 0x8000	/* switch between adjtime/adjtimex modes */;
pub const ADJ_OFFSET_SINGLESHOT: c_uint = 0x0001	/* old-fashioned adjtime */;
pub const ADJ_OFFSET_READONLY: c_uint = 0x2000	/* read-only adjtime */;

extern "C" {
    pub fn random_get_entropy_fallback() -> c_ulong;
}

//
// The random_get_entropy() function is used by the /dev/random driver
// in order to extract entropy via the relative unpredictability of
// when an interrupt takes places versus a high speed, fine-grained
// timing source or cycle counter.  Since it will be occurred on every
// single interrupt, it must have a very low cost/overhead.
//
// By default we use get_cycles() for this purpose, but individual
// architectures may override this in their asm/timex.h header file.
// If a given arch does not have get_cycles(), then we fallback to
// using random_get_entropy_fallback().
//

//
// SHIFT_PLL is used as a dampening factor to define how much we
// adjust the frequency correction for a given offset in PLL mode.
// It also used in dampening the offset correction, to define how
// much of the current value in time_offset we correct for each
// second. Changing this value changes the stiffness of the ntp
// adjustment code. A lower value makes it more flexible, reducing
// NTP convergence time. A higher value makes it stiffer, increasing
// convergence time, but making the clock more stable.
//
// In David Mills' nanokernel reference implementation SHIFT_PLL is 4.
// However this seems to increase convergence time much too long.
//
// https://lists.ntp.org/pipermail/hackers/2008-January/003487.html
//
// In the above mailing list discussion, it seems the value of 4
// was appropriate for other Unix systems with HZ=100, and that
// SHIFT_PLL should be decreased as HZ increases. However, Linux's
// clock steering implementation is HZ independent.
//
// Through experimentation, a SHIFT_PLL value of 2 was found to allow
// for fast convergence (very similar to the NTPv3 code used prior to
// v2.6.19), with good clock stability.
//
// SHIFT_FLL is used as a dampening factor to define how much we
// adjust the frequency correction for a given offset in FLL mode.
// In David Mills' nanokernel reference implementation SHIFT_FLL is 2.
//
// MAXTC establishes the maximum time constant of the PLL.
//

//
// SHIFT_USEC defines the scaling (shift) of the time_freq and
// time_tolerance variables, which represent the current frequency
// offset and maximum frequency tolerance.
//

pub const PPM_SCALE_INV_SHIFT: c_int = 19;

// Required to safely shift negative values

pub const NTP_SCALE_SHIFT: c_int = 32;

extern "C" {
    pub fn do_adjtimex(: *mut __kernel_timex) -> c_int;
}
extern "C" {
    pub fn do_clock_adjtime(which_clock: clockid_t, ktx: *mut *mut __kernel_timex) -> c_int;
}
extern "C" {
    pub fn hardpps(: *const timespec64, : *const timespec64);
}
// The clock frequency of the i8253/i8254 PIT


//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/timex.h
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

//
// syscall interface - used (mainly by NTP daemon)
// to discipline kernel clock oscillator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timex {
    pub /: *mut *mut unsigned int modes; / mode selector,
    pub /: *mut *mut __kernel_long_t offset; / time offset (usec),
    pub /: *mut *mut __kernel_long_t freq; / frequency offset (scaled ppm),
    pub /: *mut *mut __kernel_long_t maxerror;/ maximum error (usec),
    pub /: *mut *mut __kernel_long_t esterror;/ estimated error (usec),
    pub /: *mut *mut int status; / clock command/status,
    pub /: *mut *mut __kernel_long_t constant;/ pll time constant,
    pub /: *mut *mut __kernel_long_t precision;/ clock precision (usec) (read only),
    pub (ppm): *mut *mut __kernel_long_t tolerance;/ clock frequency tolerance,
// (read only)
//
    pub /: *mut *mut timeval time; / (read only, except for ADJ_SETOFFSET),
    pub /: *mut *mut __kernel_long_t tick; / (modified) usecs between clock ticks,
    pub /: *mut *mut __kernel_long_t ppsfreq;/ pps frequency (scaled ppm) (ro),
    pub /: *mut *mut __kernel_long_t jitter; / pps jitter (us) (ro),
    pub /: *mut *mut int shift; / interval duration (s) (shift) (ro),
    pub /: *mut *mut __kernel_long_t stabil; / pps stability (scaled ppm) (ro),
    pub /: *mut *mut __kernel_long_t jitcnt; / jitter limit exceeded (ro),
    pub /: *mut *mut __kernel_long_t calcnt; / calibration intervals (ro),
    pub /: *mut *mut __kernel_long_t errcnt; / calibration errors (ro),
    pub /: *mut *mut __kernel_long_t stbcnt; / stability limit exceeded (ro),
    pub /: *mut *mut int tai; / TAI offset (ro),
    pub :32: int :32; int :32; int :32; int,
    pub :32: int :32; int :32; int :32; int,
    pub :32: int :32; int :32; int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_timex_timeval {
    pub tv_sec: __kernel_time64_t,
    pub tv_usec: c_longlong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_timex {
    pub /: *mut *mut unsigned int modes; / mode selector,
    pub /: *mut *mut int :32; / pad,
    pub /: *mut *mut long long offset; / time offset (usec),
    pub /: *mut *mut long long freq; / frequency offset (scaled ppm),
    pub /: *mut *mut long long maxerror;/ maximum error (usec),
    pub /: *mut *mut long long esterror;/ estimated error (usec),
    pub /: *mut *mut int status; / clock command/status,
    pub /: *mut *mut int :32; / pad,
    pub /: *mut *mut long long constant;/ pll time constant,
    pub /: *mut *mut long long precision;/ clock precision (usec) (read only),
    pub (ppm): *mut *mut long long tolerance;/ clock frequency tolerance,
// (read only)
//
    pub /: *mut *mut __kernel_timex_timeval time; / (read only, except for ADJ_SETOFFSET),
    pub /: *mut *mut long long tick; / (modified) usecs between clock ticks,
    pub /: *mut *mut long long ppsfreq;/ pps frequency (scaled ppm) (ro),
    pub /: *mut *mut long long jitter; / pps jitter (us) (ro),
    pub /: *mut *mut int shift; / interval duration (s) (shift) (ro),
    pub /: *mut *mut int :32; / pad,
    pub /: *mut *mut long long stabil; / pps stability (scaled ppm) (ro),
    pub /: *mut *mut long long jitcnt; / jitter limit exceeded (ro),
    pub /: *mut *mut long long calcnt; / calibration intervals (ro),
    pub /: *mut *mut long long errcnt; / calibration errors (ro),
    pub /: *mut *mut long long stbcnt; / stability limit exceeded (ro),
    pub /: *mut *mut int tai; / TAI offset (ro),
    pub :32: int :32; int :32; int :32; int,
    pub :32: int :32; int :32; int :32; int,
    pub :32: int :32; int :32; int,
}

//
// Mode codes (timex.mode)
//
pub const ADJ_OFFSET: c_uint = 0x0001	/* time offset */;
pub const ADJ_FREQUENCY: c_uint = 0x0002	/* frequency offset */;
pub const ADJ_MAXERROR: c_uint = 0x0004	/* maximum time error */;
pub const ADJ_ESTERROR: c_uint = 0x0008	/* estimated time error */;
pub const ADJ_STATUS: c_uint = 0x0010	/* clock status */;
pub const ADJ_TIMECONST: c_uint = 0x0020	/* pll time constant */;
pub const ADJ_TAI: c_uint = 0x0080	/* set TAI offset */;
pub const ADJ_SETOFFSET: c_uint = 0x0100  /* add 'time' to current time */;
pub const ADJ_MICRO: c_uint = 0x1000	/* select microsecond resolution */;
pub const ADJ_NANO: c_uint = 0x2000	/* select nanosecond resolution */;
pub const ADJ_TICK: c_uint = 0x4000	/* tick value */;
pub const ADJ_OFFSET_SINGLESHOT: c_uint = 0x8001	/* old-fashioned adjtime */;
pub const ADJ_OFFSET_SS_READ: c_uint = 0xa001	/* read-only adjtime */;

// NTP userland likes the MOD_ prefix better

//
// Status codes (timex.status)
//
pub const STA_PLL: c_uint = 0x0001	/* enable PLL updates (rw) */;
pub const STA_PPSFREQ: c_uint = 0x0002	/* enable PPS freq discipline (rw) */;
pub const STA_PPSTIME: c_uint = 0x0004	/* enable PPS time discipline (rw) */;
pub const STA_FLL: c_uint = 0x0008	/* select frequency-lock mode (rw) */;
pub const STA_INS: c_uint = 0x0010	/* insert leap (rw) */;
pub const STA_DEL: c_uint = 0x0020	/* delete leap (rw) */;
pub const STA_UNSYNC: c_uint = 0x0040	/* clock unsynchronized (rw) */;
pub const STA_FREQHOLD: c_uint = 0x0080	/* hold frequency (rw) */;
pub const STA_PPSSIGNAL: c_uint = 0x0100	/* PPS signal present (ro) */;
pub const STA_PPSJITTER: c_uint = 0x0200	/* PPS signal jitter exceeded (ro) */;
pub const STA_PPSWANDER: c_uint = 0x0400	/* PPS signal wander exceeded (ro) */;
pub const STA_PPSERROR: c_uint = 0x0800	/* PPS signal calibration error (ro) */;
pub const STA_CLOCKERR: c_uint = 0x1000	/* clock hardware fault (ro) */;
pub const STA_NANO: c_uint = 0x2000	/* resolution (0 = us, 1 = ns) (ro) */;
pub const STA_MODE: c_uint = 0x4000	/* mode (0 = PLL, 1 = FLL) (ro) */;
pub const STA_CLK: c_uint = 0x8000	/* clock source (0 = A, 1 = B) (ro) */;
// read-only bits

//
// Clock states (time_state)
//


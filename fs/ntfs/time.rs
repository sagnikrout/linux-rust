//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/time.h
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
// NTFS time conversion functions.
//
// Copyright (c) 2001-2005 Anton Altaparmakov
//

//
// utc2ntfs - convert Linux UTC time to NTFS time
// @ts:		Linux UTC time to convert to NTFS time
//
// Convert the Linux UTC time @ts to its corresponding NTFS time and return
// that in little endian format.
//
// Linux stores time in a struct timespec64 consisting of a time64_t tv_sec
// and a long tv_nsec where tv_sec is the number of 1-second intervals since
// 1st January 1970, 00:00:00 UTC and tv_nsec is the number of 1-nano-second
// intervals since the value of tv_sec.
//
// NTFS uses Microsoft's standard time format which is stored in a s64 and is
// measured as the number of 100-nano-second intervals since 1st January 1601,
// 00:00:00 UTC.
//
// Convert the seconds to 100ns intervals, add the nano-seconds
// converted to 100ns intervals, and then add the NTFS time offset.
//
// get_current_ntfs_time - get the current time in little endian NTFS format
//
// Get the current time from the Linux kernel, convert it to its corresponding
// NTFS time and return that in little endian format.
//
extern "C" {
    pub fn utc2ntfs(_arg: ts) -> return;
}
//
// ntfs2utc - convert NTFS time to Linux time
// @time:	NTFS time (little endian) to convert to Linux UTC
//
// Convert the little endian NTFS time @time to its corresponding Linux UTC
// time and return that in cpu format.
//
// Linux stores time in a struct timespec64 consisting of a time64_t tv_sec
// and a long tv_nsec where tv_sec is the number of 1-second intervals since
// 1st January 1970, 00:00:00 UTC and tv_nsec is the number of 1-nano-second
// intervals since the value of tv_sec.
//
// NTFS uses Microsoft's standard time format which is stored in a s64 and is
// measured as the number of 100 nano-second intervals since 1st January 1601,
// 00:00:00 UTC.
//
// Subtract the NTFS time offset.
//
// Convert the time to 1-second intervals and the remainder to
// 1-nano-second intervals.
//

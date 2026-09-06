//! Automatically rewritten from C to Rust
//! Source: fs/udf/udftime.c
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


// SPDX-License-Identifier: LGPL-2.0+
// Copyright (C) 1993, 1994, 1995, 1996, 1997 Free Software Foundation, Inc.
    This file is part of the GNU C Library.
    Contributed by Paul Eggert (eggert@twinsun.com). */
//
// dgb 10/02/98: ripped this from glibc source to help convert timestamps
// to unix time
// 10/04/98: added new table-based lookup after seeing how ugly
// the gnu code is
// blf 09/27/99: ripped out all the old code and inserted new table from
// John Brockmeyer (without leap second corrections)
// rewrote udf_stamp_to_time and fixed timezone accounting in
// udf_time_to_stamp.
//
// We don't take into account leap seconds. This may be correct or incorrect.
// For more NIST information (especially dealing with leap seconds), see:
// http://www.boulder.nist.gov/timefreq/pubs/bulletin/leapsecond.htm
//

    void
    udf_disk_stamp_to_time(struct timespec64 *dest, struct timestamp src)
    {
    let mut typeAndTimezone: u16 = le16_to_cpu(src.typeAndTimezone);
    let mut year: u16 = le16_to_cpu(src.year);
    let mut type: u8 = typeAndTimezone >> 12;
    int16_t offset;
    if (type == 1) {
    offset = typeAndTimezone << 4;
// sign extent offset
    offset = (offset >> 4);
    if (offset == -2047) /* unspecified offset */
    offset = 0;
    } else
    offset = 0;
    dest.tv_sec = mktime64(year, src.month, src.day, src.hour, src.minute,
    src.second);
    dest.tv_sec -= offset * 60;
//
// Sanitize nanosecond field since reportedly some filesystems are
// recorded with bogus sub-second values.
//
    if (src.centiseconds < 100 && src.hundredsOfMicroseconds < 100 &&
    src.microseconds < 100) {
    dest.tv_nsec = 1000 * (src.centiseconds * 10000 +
    src.hundredsOfMicroseconds * 100 + src.microseconds);
    } else {
    dest.tv_nsec = 0;
    }
    }
    void
    udf_time_to_disk_stamp(struct timestamp *dest, struct timespec64 ts)
    {
    time64_t seconds;
    int16_t offset;
    struct tm tm;
    offset = -sys_tz.tz_minuteswest;
    dest.typeAndTimezone = cpu_to_le16(0x1000 | (offset & 0x0FFF));
    seconds = ts.tv_sec + offset * 60;
    time64_to_tm(seconds, 0, &tm);
    dest.year = cpu_to_le16(tm.tm_year + 1900);
    dest.month = tm.tm_mon + 1;
    dest.day = tm.tm_mday;
    dest.hour = tm.tm_hour;
    dest.minute = tm.tm_min;
    dest.second = tm.tm_sec;
    dest.centiseconds = ts.tv_nsec / 10000000;
    dest.hundredsOfMicroseconds = (ts.tv_nsec / 1000 -
    dest.centiseconds * 10000) / 100;
    dest.microseconds = (ts.tv_nsec / 1000 - dest.centiseconds * 10000 -
    dest.hundredsOfMicroseconds * 100);
    }
// EOF

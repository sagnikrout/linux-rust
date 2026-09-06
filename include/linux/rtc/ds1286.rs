//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtc/ds1286.h
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
// Copyright (C) 1998, 1999, 2003 Ralf Baechle
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// register summary
//
pub const RTC_HUNDREDTH_SECOND: c_int = 0;
pub const RTC_SECONDS: c_int = 1;
pub const RTC_MINUTES: c_int = 2;
pub const RTC_MINUTES_ALARM: c_int = 3;
pub const RTC_HOURS: c_int = 4;
pub const RTC_HOURS_ALARM: c_int = 5;
pub const RTC_DAY: c_int = 6;
pub const RTC_DAY_ALARM: c_int = 7;
pub const RTC_DATE: c_int = 8;
pub const RTC_MONTH: c_int = 9;
pub const RTC_YEAR: c_int = 10;
pub const RTC_CMD: c_int = 11;
pub const RTC_WHSEC: c_int = 12;
pub const RTC_WSEC: c_int = 13;
pub const RTC_UNUSED: c_int = 14;
// RTC_*_alarm is always true if 2 MSBs are set

//
// Bits in the month register
//
pub const RTC_EOSC: c_uint = 0x80;
pub const RTC_ESQW: c_uint = 0x40;
//
// Bits in the Command register
//
pub const RTC_TDF: c_uint = 0x01;
pub const RTC_WAF: c_uint = 0x02;
pub const RTC_TDM: c_uint = 0x04;
pub const RTC_WAM: c_uint = 0x08;
pub const RTC_PU_LVL: c_uint = 0x10;
pub const RTC_IBH_LO: c_uint = 0x20;
pub const RTC_IPSW: c_uint = 0x40;
pub const RTC_TE: c_uint = 0x80;

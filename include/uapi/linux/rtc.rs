//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/rtc.h
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
//
// Generic RTC interface.
// This version contains the part of the user interface to the Real Time Clock
// service. It is used with both the legacy mc146818 and also  EFI
// Struct rtc_time and first 12 ioctl by Paul Gortmaker, 1996 - separated out
// from <linux/mc146818rtc.h> to this file for 2.4 kernels.
//
// Copyright (C) 1999 Hewlett-Packard Co.
// Copyright (C) 1999 Stephane Eranian <eranian@hpl.hp.com>
//

//
// The struct used to pass data via the following ioctl. Similar to the
// struct tm in <time.h>, but it needs to be here so that the kernel
// source is self contained, allowing cross-compiles, etc. etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_time {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}

//
// This data structure is inspired by the EFI (v0.92) wakeup
// alarm API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_wkalrm {
    pub /: *mut *mut unsigned char enabled; / 0 = alarm disabled, 1 = alarm enabled,
    pub /: *mut *mut unsigned char pending; / 0 = alarm not pending, 1 = alarm pending,
    pub /: *mut *mut rtc_time time; / time the alarm is set to,
}

//
// Data structure to control PLL correction some better RTC feature
// pll_value is used to get or set current value of correction,
// the rest of the struct is used to query HW capabilities.
// This is modeled after the RTC used in Q40/Q60 computers but
// should be sufficiently flexible for other devices
//
// +ve pll_value means clock will run faster by
// pll_value*pll_posmult/pll_clock
// -ve pll_value means clock will run slower by
// pll_value*pll_negmult/pll_clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_pll_info {
    pub /: *mut *mut int pll_ctrl; / placeholder for fancier control,
    pub /: *mut *mut int pll_value; / get/set correction value,
    pub /: *mut *mut int pll_max; / max +ve (faster) adjustment value,
    pub /: *mut *mut int pll_min; / max -ve (slower) adjustment value,
    pub /: *mut *mut int pll_posmult; / factor for +ve correction,
    pub /: *mut *mut int pll_negmult; / factor for -ve correction,
    pub /: *mut *mut long pll_clock; / base PLL frequency,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_param {
    pub param: __u64,
    pub uvalue: __u64,
    pub svalue: __s64,
    pub ptr: __u64,
}

//
// ioctl calls that are permitted to the /dev/rtc interface, if
// any of the RTC drivers are enabled.
//

// interrupt flags
pub const RTC_IRQF: c_uint = 0x80	/* Any of the following is active */;
pub const RTC_PF: c_uint = 0x40	/* Periodic interrupt */;
pub const RTC_AF: c_uint = 0x20	/* Alarm interrupt */;
pub const RTC_UF: c_uint = 0x10	/* Update interrupt for 1Hz RTC */;
// feature list
pub const RTC_FEATURE_ALARM: c_int = 0;
pub const RTC_FEATURE_ALARM_RES_MINUTE: c_int = 1;
pub const RTC_FEATURE_NEED_WEEK_DAY: c_int = 2;
pub const RTC_FEATURE_ALARM_RES_2S: c_int = 3;
pub const RTC_FEATURE_UPDATE_INTERRUPT: c_int = 4;
pub const RTC_FEATURE_CORRECTION: c_int = 5;
pub const RTC_FEATURE_BACKUP_SWITCH_MODE: c_int = 6;
pub const RTC_FEATURE_ALARM_WAKEUP_ONLY: c_int = 7;
pub const RTC_FEATURE_CNT: c_int = 8;
// parameter list
pub const RTC_PARAM_FEATURES: c_int = 0;
pub const RTC_PARAM_CORRECTION: c_int = 1;
pub const RTC_PARAM_BACKUP_SWITCH_MODE: c_int = 2;
pub const RTC_BSM_DISABLED: c_int = 0;
pub const RTC_BSM_DIRECT: c_int = 1;
pub const RTC_BSM_LEVEL: c_int = 2;
pub const RTC_BSM_STANDBY: c_int = 3;
pub const RTC_MAX_FREQ: c_int = 8192;

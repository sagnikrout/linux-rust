//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8350/rtc.h
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
// rtc.h  --  RTC driver for Wolfson WM8350 PMIC
//
// Copyright 2007 Wolfson Microelectronics PLC
//

//
// Register values.
//
pub const WM8350_RTC_SECONDS_MINUTES: c_uint = 0x10;
pub const WM8350_RTC_HOURS_DAY: c_uint = 0x11;
pub const WM8350_RTC_DATE_MONTH: c_uint = 0x12;
pub const WM8350_RTC_YEAR: c_uint = 0x13;
pub const WM8350_ALARM_SECONDS_MINUTES: c_uint = 0x14;
pub const WM8350_ALARM_HOURS_DAY: c_uint = 0x15;
pub const WM8350_ALARM_DATE_MONTH: c_uint = 0x16;
pub const WM8350_RTC_TIME_CONTROL: c_uint = 0x17;
//
// R16 (0x10) - RTC Seconds/Minutes
//
pub const WM8350_RTC_MINS_MASK: c_uint = 0x7F00;
pub const WM8350_RTC_MINS_SHIFT: c_int = 8;
pub const WM8350_RTC_SECS_MASK: c_uint = 0x007F;
pub const WM8350_RTC_SECS_SHIFT: c_int = 0;
//
// R17 (0x11) - RTC Hours/Day
//
pub const WM8350_RTC_DAY_MASK: c_uint = 0x0700;
pub const WM8350_RTC_DAY_SHIFT: c_int = 8;
pub const WM8350_RTC_HPM_MASK: c_uint = 0x0020;
pub const WM8350_RTC_HPM_SHIFT: c_int = 5;
pub const WM8350_RTC_HRS_MASK: c_uint = 0x001F;
pub const WM8350_RTC_HRS_SHIFT: c_int = 0;
// Bit values for R21 (0x15)
pub const WM8350_RTC_DAY_SUN: c_int = 1;
pub const WM8350_RTC_DAY_MON: c_int = 2;
pub const WM8350_RTC_DAY_TUE: c_int = 3;
pub const WM8350_RTC_DAY_WED: c_int = 4;
pub const WM8350_RTC_DAY_THU: c_int = 5;
pub const WM8350_RTC_DAY_FRI: c_int = 6;
pub const WM8350_RTC_DAY_SAT: c_int = 7;
pub const WM8350_RTC_HPM_AM: c_int = 0;
pub const WM8350_RTC_HPM_PM: c_int = 1;
//
// R18 (0x12) - RTC Date/Month
//
pub const WM8350_RTC_MTH_MASK: c_uint = 0x1F00;
pub const WM8350_RTC_MTH_SHIFT: c_int = 8;
pub const WM8350_RTC_DATE_MASK: c_uint = 0x003F;
pub const WM8350_RTC_DATE_SHIFT: c_int = 0;
// Bit values for R22 (0x16)
pub const WM8350_RTC_MTH_JAN: c_int = 1;
pub const WM8350_RTC_MTH_FEB: c_int = 2;
pub const WM8350_RTC_MTH_MAR: c_int = 3;
pub const WM8350_RTC_MTH_APR: c_int = 4;
pub const WM8350_RTC_MTH_MAY: c_int = 5;
pub const WM8350_RTC_MTH_JUN: c_int = 6;
pub const WM8350_RTC_MTH_JUL: c_int = 7;
pub const WM8350_RTC_MTH_AUG: c_int = 8;
pub const WM8350_RTC_MTH_SEP: c_int = 9;
pub const WM8350_RTC_MTH_OCT: c_int = 10;
pub const WM8350_RTC_MTH_NOV: c_int = 11;
pub const WM8350_RTC_MTH_DEC: c_int = 12;
pub const WM8350_RTC_MTH_JAN_BCD: c_uint = 0x01;
pub const WM8350_RTC_MTH_FEB_BCD: c_uint = 0x02;
pub const WM8350_RTC_MTH_MAR_BCD: c_uint = 0x03;
pub const WM8350_RTC_MTH_APR_BCD: c_uint = 0x04;
pub const WM8350_RTC_MTH_MAY_BCD: c_uint = 0x05;
pub const WM8350_RTC_MTH_JUN_BCD: c_uint = 0x06;
pub const WM8350_RTC_MTH_JUL_BCD: c_uint = 0x07;
pub const WM8350_RTC_MTH_AUG_BCD: c_uint = 0x08;
pub const WM8350_RTC_MTH_SEP_BCD: c_uint = 0x09;
pub const WM8350_RTC_MTH_OCT_BCD: c_uint = 0x10;
pub const WM8350_RTC_MTH_NOV_BCD: c_uint = 0x11;
pub const WM8350_RTC_MTH_DEC_BCD: c_uint = 0x12;
//
// R19 (0x13) - RTC Year
//
pub const WM8350_RTC_YHUNDREDS_MASK: c_uint = 0x3F00;
pub const WM8350_RTC_YHUNDREDS_SHIFT: c_int = 8;
pub const WM8350_RTC_YUNITS_MASK: c_uint = 0x00FF;
pub const WM8350_RTC_YUNITS_SHIFT: c_int = 0;
//
// R20 (0x14) - Alarm Seconds/Minutes
//
pub const WM8350_RTC_ALMMINS_MASK: c_uint = 0x7F00;
pub const WM8350_RTC_ALMMINS_SHIFT: c_int = 8;
pub const WM8350_RTC_ALMSECS_MASK: c_uint = 0x007F;
pub const WM8350_RTC_ALMSECS_SHIFT: c_int = 0;
// Bit values for R20 (0x14)

//
// R21 (0x15) - Alarm Hours/Day
//
pub const WM8350_RTC_ALMDAY_MASK: c_uint = 0x0F00;
pub const WM8350_RTC_ALMDAY_SHIFT: c_int = 8;
pub const WM8350_RTC_ALMHPM_MASK: c_uint = 0x0020;
pub const WM8350_RTC_ALMHPM_SHIFT: c_int = 5;
pub const WM8350_RTC_ALMHRS_MASK: c_uint = 0x001F;
pub const WM8350_RTC_ALMHRS_SHIFT: c_int = 0;
// Bit values for R21 (0x15)

pub const WM8350_RTC_ALMDAY_SUN: c_int = 1;
pub const WM8350_RTC_ALMDAY_MON: c_int = 2;
pub const WM8350_RTC_ALMDAY_TUE: c_int = 3;
pub const WM8350_RTC_ALMDAY_WED: c_int = 4;
pub const WM8350_RTC_ALMDAY_THU: c_int = 5;
pub const WM8350_RTC_ALMDAY_FRI: c_int = 6;
pub const WM8350_RTC_ALMDAY_SAT: c_int = 7;
pub const WM8350_RTC_ALMHPM_AM: c_int = 0;
pub const WM8350_RTC_ALMHPM_PM: c_int = 1;

//
// R22 (0x16) - Alarm Date/Month
//
pub const WM8350_RTC_ALMMTH_MASK: c_uint = 0x1F00;
pub const WM8350_RTC_ALMMTH_SHIFT: c_int = 8;
pub const WM8350_RTC_ALMDATE_MASK: c_uint = 0x003F;
pub const WM8350_RTC_ALMDATE_SHIFT: c_int = 0;
// Bit values for R22 (0x16)

pub const WM8350_RTC_ALMMTH_JAN: c_int = 1;
pub const WM8350_RTC_ALMMTH_FEB: c_int = 2;
pub const WM8350_RTC_ALMMTH_MAR: c_int = 3;
pub const WM8350_RTC_ALMMTH_APR: c_int = 4;
pub const WM8350_RTC_ALMMTH_MAY: c_int = 5;
pub const WM8350_RTC_ALMMTH_JUN: c_int = 6;
pub const WM8350_RTC_ALMMTH_JUL: c_int = 7;
pub const WM8350_RTC_ALMMTH_AUG: c_int = 8;
pub const WM8350_RTC_ALMMTH_SEP: c_int = 9;
pub const WM8350_RTC_ALMMTH_OCT: c_int = 10;
pub const WM8350_RTC_ALMMTH_NOV: c_int = 11;
pub const WM8350_RTC_ALMMTH_DEC: c_int = 12;
pub const WM8350_RTC_ALMMTH_JAN_BCD: c_uint = 0x01;
pub const WM8350_RTC_ALMMTH_FEB_BCD: c_uint = 0x02;
pub const WM8350_RTC_ALMMTH_MAR_BCD: c_uint = 0x03;
pub const WM8350_RTC_ALMMTH_APR_BCD: c_uint = 0x04;
pub const WM8350_RTC_ALMMTH_MAY_BCD: c_uint = 0x05;
pub const WM8350_RTC_ALMMTH_JUN_BCD: c_uint = 0x06;
pub const WM8350_RTC_ALMMTH_JUL_BCD: c_uint = 0x07;
pub const WM8350_RTC_ALMMTH_AUG_BCD: c_uint = 0x08;
pub const WM8350_RTC_ALMMTH_SEP_BCD: c_uint = 0x09;
pub const WM8350_RTC_ALMMTH_OCT_BCD: c_uint = 0x10;
pub const WM8350_RTC_ALMMTH_NOV_BCD: c_uint = 0x11;
pub const WM8350_RTC_ALMMTH_DEC_BCD: c_uint = 0x12;
//
// R23 (0x17) - RTC Time Control
//
pub const WM8350_RTC_BCD: c_uint = 0x8000;
pub const WM8350_RTC_BCD_MASK: c_uint = 0x8000;
pub const WM8350_RTC_BCD_SHIFT: c_int = 15;
pub const WM8350_RTC_12HR: c_uint = 0x4000;
pub const WM8350_RTC_12HR_MASK: c_uint = 0x4000;
pub const WM8350_RTC_12HR_SHIFT: c_int = 14;
pub const WM8350_RTC_DST: c_uint = 0x2000;
pub const WM8350_RTC_DST_MASK: c_uint = 0x2000;
pub const WM8350_RTC_DST_SHIFT: c_int = 13;
pub const WM8350_RTC_SET: c_uint = 0x0800;
pub const WM8350_RTC_SET_MASK: c_uint = 0x0800;
pub const WM8350_RTC_SET_SHIFT: c_int = 11;
pub const WM8350_RTC_STS: c_uint = 0x0400;
pub const WM8350_RTC_STS_MASK: c_uint = 0x0400;
pub const WM8350_RTC_STS_SHIFT: c_int = 10;
pub const WM8350_RTC_ALMSET: c_uint = 0x0200;
pub const WM8350_RTC_ALMSET_MASK: c_uint = 0x0200;
pub const WM8350_RTC_ALMSET_SHIFT: c_int = 9;
pub const WM8350_RTC_ALMSTS: c_uint = 0x0100;
pub const WM8350_RTC_ALMSTS_MASK: c_uint = 0x0100;
pub const WM8350_RTC_ALMSTS_SHIFT: c_int = 8;
pub const WM8350_RTC_PINT: c_uint = 0x0070;
pub const WM8350_RTC_PINT_MASK: c_uint = 0x0070;
pub const WM8350_RTC_PINT_SHIFT: c_int = 4;
pub const WM8350_RTC_DSW: c_uint = 0x000F;
pub const WM8350_RTC_DSW_MASK: c_uint = 0x000F;
pub const WM8350_RTC_DSW_SHIFT: c_int = 0;
// Bit values for R23 (0x17)
pub const WM8350_RTC_BCD_BINARY: c_int = 0;
pub const WM8350_RTC_BCD_BCD: c_int = 1;
pub const WM8350_RTC_12HR_24HR: c_int = 0;
pub const WM8350_RTC_12HR_12HR: c_int = 1;
pub const WM8350_RTC_DST_DISABLED: c_int = 0;
pub const WM8350_RTC_DST_ENABLED: c_int = 1;
pub const WM8350_RTC_SET_RUN: c_int = 0;
pub const WM8350_RTC_SET_SET: c_int = 1;
pub const WM8350_RTC_STS_RUNNING: c_int = 0;
pub const WM8350_RTC_STS_STOPPED: c_int = 1;
pub const WM8350_RTC_ALMSET_RUN: c_int = 0;
pub const WM8350_RTC_ALMSET_SET: c_int = 1;
pub const WM8350_RTC_ALMSTS_RUNNING: c_int = 0;
pub const WM8350_RTC_ALMSTS_STOPPED: c_int = 1;
pub const WM8350_RTC_PINT_DISABLED: c_int = 0;
pub const WM8350_RTC_PINT_SECS: c_int = 1;
pub const WM8350_RTC_PINT_MINS: c_int = 2;
pub const WM8350_RTC_PINT_HRS: c_int = 3;
pub const WM8350_RTC_PINT_DAYS: c_int = 4;
pub const WM8350_RTC_PINT_MTHS: c_int = 5;
pub const WM8350_RTC_DSW_DISABLED: c_int = 0;
pub const WM8350_RTC_DSW_1HZ: c_int = 1;
pub const WM8350_RTC_DSW_2HZ: c_int = 2;
pub const WM8350_RTC_DSW_4HZ: c_int = 3;
pub const WM8350_RTC_DSW_8HZ: c_int = 4;
pub const WM8350_RTC_DSW_16HZ: c_int = 5;
pub const WM8350_RTC_DSW_32HZ: c_int = 6;
pub const WM8350_RTC_DSW_64HZ: c_int = 7;
pub const WM8350_RTC_DSW_128HZ: c_int = 8;
pub const WM8350_RTC_DSW_256HZ: c_int = 9;
pub const WM8350_RTC_DSW_512HZ: c_int = 10;
pub const WM8350_RTC_DSW_1024HZ: c_int = 11;
//
// R218 (0xDA) - RTC Tick Control
//
pub const WM8350_RTC_TICKSTS: c_uint = 0x4000;
pub const WM8350_RTC_CLKSRC: c_uint = 0x2000;
pub const WM8350_RTC_TRIM_MASK: c_uint = 0x03FF;
//
// RTC Interrupts.
//
pub const WM8350_IRQ_RTC_PER: c_int = 7;
pub const WM8350_IRQ_RTC_SEC: c_int = 8;
pub const WM8350_IRQ_RTC_ALM: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_rtc {
    pub pdev: *mut platform_device,
    pub rtc: *mut rtc_device,
    pub /: *mut *mut int alarm_enabled; / used over suspend/resume,
    pub update_enabled: c_int,
}

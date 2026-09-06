//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mc146818rtc.h
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


// mc146818rtc.h - register definitions for the Real-Time-Clock / CMOS RAM
// Copyright Torsten Duwe <duwe@informatik.uni-erlangen.de> 1993
// derived from Data Sheet, Copyright Motorola 1984 (!).
// It was written to be part of the Linux operating system.
//
// permission is hereby granted to copy, modify and redistribute this code
// in terms of the GNU Library General Public License, Version 2 or later,
// at your option.
//

// Some RTCs extend the mc146818 register set to support alarms of more
// than 24 hours in the future; or dates that include a century code.
// This platform_data structure can pass this information to the driver.
//
// Also, some platforms need suspend()/resume() hooks to kick in special
// handling of wake alarms, e.g. activating ACPI BIOS hooks or setting up
// a separate wakeup alarm used by some almost-clone chips.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmos_rtc_board_info {
    pub dev): *mut *mut void (wake_on)(struct device,
    pub dev): *mut *mut void (wake_off)(struct device,
    pub flags: u32,

    pub address_space: c_int,
    pub /: *mut *mut u8 rtc_day_alarm; / zero, or register index,
    pub /: *mut *mut u8 rtc_mon_alarm; / zero, or register index,
    pub /: *mut *mut u8 rtc_century; / zero, or register index,
}

//
// register summary
//
pub const RTC_SECONDS: c_int = 0;
pub const RTC_SECONDS_ALARM: c_int = 1;
pub const RTC_MINUTES: c_int = 2;
pub const RTC_MINUTES_ALARM: c_int = 3;
pub const RTC_HOURS: c_int = 4;
pub const RTC_HOURS_ALARM: c_int = 5;
// RTC_*_alarm is always true if 2 MSBs are set

pub const RTC_DAY_OF_WEEK: c_int = 6;
pub const RTC_DAY_OF_MONTH: c_int = 7;
pub const RTC_MONTH: c_int = 8;
pub const RTC_YEAR: c_int = 9;
// control registers - Moto names
//
pub const RTC_REG_A: c_int = 10;
pub const RTC_REG_B: c_int = 11;
pub const RTC_REG_C: c_int = 12;
pub const RTC_REG_D: c_int = 13;
//
// register details
//

// update-in-progress  - set to "1" 244 microsecs before RTC goes off the bus,
// reset after update (may take 1.984ms @ 32768Hz RefClock) is complete,
// totalling to a max high interval of 2.228 ms.
//

// divider control: refclock values 4.194 / 1.049 MHz / 32.768 kHz

// 2 values for divider stage reset, others for "testing purposes only"

// In AMD BKDG bit 5 and 6 are reserved, bit 4 is for select dv0 bank

// Periodic intr. / Square wave rate select. 0=none, 1=32.8kHz,... 15=2Hz

//

//

// caution - cleared by read

//

//

pub const RTC_IO_EXTENT: c_uint = 0x8;
pub const RTC_IO_EXTENT_USED: c_uint = 0x2;

extern "C" {
    pub fn mc146818_does_rtc_work() -> bool;
}
extern "C" {
    pub fn mc146818_get_time(time: *mut rtc_time, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn mc146818_set_time(time: *mut rtc_time) -> c_int;
}

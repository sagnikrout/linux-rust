//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rtc/m48t59.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// include/linux/rtc/m48t59.h
//
// Definitions for the platform data of m48t59 RTC chip driver.
//
// Copyright (c) 2007 Wind River Systems, Inc.
//
// Mark Zhan <rongkai.zhan@windriver.com>
//
// M48T59 Register Offset
//
pub const M48T59_YEAR: c_uint = 0xf;
pub const M48T59_MONTH: c_uint = 0xe;
pub const M48T59_MDAY: c_uint = 0xd	/* Day of Month */;
pub const M48T59_WDAY: c_uint = 0xc	/* Day of Week */;
pub const M48T59_WDAY_CB: c_uint = 0x20	/* Century Bit */;
pub const M48T59_WDAY_CEB: c_uint = 0x10	/* Century Enable Bit */;
pub const M48T59_HOUR: c_uint = 0xb;
pub const M48T59_MIN: c_uint = 0xa;
pub const M48T59_SEC: c_uint = 0x9;
pub const M48T59_CNTL: c_uint = 0x8;
pub const M48T59_CNTL_READ: c_uint = 0x40;
pub const M48T59_CNTL_WRITE: c_uint = 0x80;
pub const M48T59_WATCHDOG: c_uint = 0x7;
pub const M48T59_INTR: c_uint = 0x6;
pub const M48T59_INTR_AFE: c_uint = 0x80	/* Alarm Interrupt Enable */;
pub const M48T59_INTR_ABE: c_uint = 0x20;
pub const M48T59_ALARM_DATE: c_uint = 0x5;
pub const M48T59_ALARM_HOUR: c_uint = 0x4;
pub const M48T59_ALARM_MIN: c_uint = 0x3;
pub const M48T59_ALARM_SEC: c_uint = 0x2;
pub const M48T59_UNUSED: c_uint = 0x1;
pub const M48T59_FLAGS: c_uint = 0x0;
pub const M48T59_FLAGS_WDT: c_uint = 0x80	/* watchdog timer expired */;
pub const M48T59_FLAGS_AF: c_uint = 0x40	/* alarm */;
pub const M48T59_FLAGS_BF: c_uint = 0x10	/* low battery */;

pub const M48T59RTC_TYPE_M48T02: c_int = 1;
pub const M48T59RTC_TYPE_M48T08: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m48t59_plat_data {
// The method to access M48T59 registers
    pub val): *mut *mut *mut void (write_byte)(struct device dev, u32 ofs, u8,
    pub ofs): *mut *mut *mut unsigned char (read_byte)(struct device dev, u32,
    pub /: *mut *mut int type; / RTC model,
// ioaddr mapped externally
    pub ioaddr: *mut void __iomem,
// offset to RTC registers, automatically set according to the type
    pub offset: c_uint,
// YY digits (in RTC) are offset, i.e. year is 1900 + yy_offset + YY
    pub yy_offset: c_int,
}

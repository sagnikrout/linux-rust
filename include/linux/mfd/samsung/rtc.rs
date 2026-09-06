//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/samsung/rtc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2011-2014 Samsung Electronics Co., Ltd
// http://www.samsung.com
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s5m_rtc_reg {
    S5M_RTC_SEC,
    S5M_RTC_MIN,
    S5M_RTC_HOUR,
    S5M_RTC_WEEKDAY,
    S5M_RTC_DATE,
    S5M_RTC_MONTH,
    S5M_RTC_YEAR1,
    S5M_RTC_YEAR2,
    S5M_ALARM0_SEC,
    S5M_ALARM0_MIN,
    S5M_ALARM0_HOUR,
    S5M_ALARM0_WEEKDAY,
    S5M_ALARM0_DATE,
    S5M_ALARM0_MONTH,
    S5M_ALARM0_YEAR1,
    S5M_ALARM0_YEAR2,
    S5M_ALARM1_SEC,
    S5M_ALARM1_MIN,
    S5M_ALARM1_HOUR,
    S5M_ALARM1_WEEKDAY,
    S5M_ALARM1_DATE,
    S5M_ALARM1_MONTH,
    S5M_ALARM1_YEAR1,
    S5M_ALARM1_YEAR2,
    S5M_ALARM0_CONF,
    S5M_ALARM1_CONF,
    S5M_RTC_STATUS,
    S5M_WTSR_SMPL_CNTL,
    S5M_RTC_UDR_CON,

    S5M_RTC_REG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s2mps_rtc_reg {
    S2MPS_RTC_CTRL,
    S2MPS_WTSR_SMPL_CNTL,
    S2MPS_RTC_UDR_CON,
    S2MPS_RSVD,
    S2MPS_RTC_SEC,
    S2MPS_RTC_MIN,
    S2MPS_RTC_HOUR,
    S2MPS_RTC_WEEKDAY,
    S2MPS_RTC_DATE,
    S2MPS_RTC_MONTH,
    S2MPS_RTC_YEAR,
    S2MPS_ALARM0_SEC,
    S2MPS_ALARM0_MIN,
    S2MPS_ALARM0_HOUR,
    S2MPS_ALARM0_WEEKDAY,
    S2MPS_ALARM0_DATE,
    S2MPS_ALARM0_MONTH,
    S2MPS_ALARM0_YEAR,
    S2MPS_ALARM1_SEC,
    S2MPS_ALARM1_MIN,
    S2MPS_ALARM1_HOUR,
    S2MPS_ALARM1_WEEKDAY,
    S2MPS_ALARM1_DATE,
    S2MPS_ALARM1_MONTH,
    S2MPS_ALARM1_YEAR,
    S2MPS_OFFSRC,

    S2MPS_RTC_REG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum s2mpg10_rtc_reg {
    S2MPG10_RTC_CTRL,
    S2MPG10_RTC_UPDATE,
    S2MPG10_RTC_SMPL,
    S2MPG10_RTC_WTSR,
    S2MPG10_RTC_CAP_SEL,
    S2MPG10_RTC_MSEC,
    S2MPG10_RTC_SEC,
    S2MPG10_RTC_MIN,
    S2MPG10_RTC_HOUR,
    S2MPG10_RTC_WEEK,
    S2MPG10_RTC_DAY,
    S2MPG10_RTC_MON,
    S2MPG10_RTC_YEAR,
    S2MPG10_RTC_A0SEC,
    S2MPG10_RTC_A0MIN,
    S2MPG10_RTC_A0HOUR,
    S2MPG10_RTC_A0WEEK,
    S2MPG10_RTC_A0DAY,
    S2MPG10_RTC_A0MON,
    S2MPG10_RTC_A0YEAR,
    S2MPG10_RTC_A1SEC,
    S2MPG10_RTC_A1MIN,
    S2MPG10_RTC_A1HOUR,
    S2MPG10_RTC_A1WEEK,
    S2MPG10_RTC_A1DAY,
    S2MPG10_RTC_A1MON,
    S2MPG10_RTC_A1YEAR,
    S2MPG10_RTC_OSC_CTRL,
}

// RTC Control Register
pub const BCD_EN_SHIFT: c_int = 0;

pub const MODEL24_SHIFT: c_int = 1;

// RTC Update Register1
pub const S5M_RTC_UDR_SHIFT: c_int = 0;

pub const S2MPS_RTC_WUDR_SHIFT: c_int = 4;

pub const S2MPS15_RTC_AUDR_SHIFT: c_int = 4;

pub const S2MPS13_RTC_AUDR_SHIFT: c_int = 1;

pub const S2MPS15_RTC_WUDR_SHIFT: c_int = 1;

pub const S2MPS_RTC_RUDR_SHIFT: c_int = 0;

pub const RTC_TCON_SHIFT: c_int = 1;

pub const S5M_RTC_TIME_EN_SHIFT: c_int = 3;

//
// UDR_T field in S5M_RTC_UDR_CON register determines the time needed
// for updating alarm and time registers. Default is 7.32 ms.
//
pub const S5M_RTC_UDR_T_SHIFT: c_int = 6;

// RTC Hour register
pub const HOUR_PM_SHIFT: c_int = 6;

// RTC Alarm Enable
pub const ALARM_ENABLE_SHIFT: c_int = 7;

// WTSR & SMPL registers
pub const SMPL_ENABLE_SHIFT: c_int = 7;

pub const WTSR_ENABLE_SHIFT: c_int = 6;


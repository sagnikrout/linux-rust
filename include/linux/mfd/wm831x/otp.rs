//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/otp.h
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
// include/linux/mfd/wm831x/otp.h -- OTP interface for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
extern "C" {
    pub fn wm831x_otp_init(wm831x: *mut wm831x) -> c_int;
}
extern "C" {
    pub fn wm831x_otp_exit(wm831x: *mut wm831x);
}
//
// R30720 (0x7800) - Unique ID 1
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30721 (0x7801) - Unique ID 2
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30722 (0x7802) - Unique ID 3
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30723 (0x7803) - Unique ID 4
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30724 (0x7804) - Unique ID 5
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30725 (0x7805) - Unique ID 6
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30726 (0x7806) - Unique ID 7
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30727 (0x7807) - Unique ID 8
//
pub const WM831X_UNIQUE_ID_MASK: c_uint = 0xFFFF  /* UNIQUE_ID - [15:0] */;

//
// R30728 (0x7808) - Factory OTP ID
//
pub const WM831X_OTP_FACT_ID_MASK: c_uint = 0xFFFE  /* OTP_FACT_ID - [15:1] */;

pub const WM831X_OTP_FACT_FINAL: c_uint = 0x0001  /* OTP_FACT_FINAL */;
pub const WM831X_OTP_FACT_FINAL_MASK: c_uint = 0x0001  /* OTP_FACT_FINAL */;

//
// R30729 (0x7809) - Factory OTP 1
//
pub const WM831X_DC3_TRIM_MASK: c_uint = 0xF000  /* DC3_TRIM - [15:12] */;

pub const WM831X_DC2_TRIM_MASK: c_uint = 0x0FC0  /* DC2_TRIM - [11:6] */;

pub const WM831X_DC1_TRIM_MASK: c_uint = 0x003F  /* DC1_TRIM - [5:0] */;

//
// R30730 (0x780A) - Factory OTP 2
//
pub const WM831X_CHIP_ID_MASK: c_uint = 0xFFFF  /* CHIP_ID - [15:0] */;

//
// R30731 (0x780B) - Factory OTP 3
//
pub const WM831X_OSC_TRIM_MASK: c_uint = 0x0780  /* OSC_TRIM - [10:7] */;

pub const WM831X_BG_TRIM_MASK: c_uint = 0x0078  /* BG_TRIM - [6:3] */;

pub const WM831X_LPBG_TRIM_MASK: c_uint = 0x0007  /* LPBG_TRIM - [2:0] */;

//
// R30732 (0x780C) - Factory OTP 4
//
pub const WM831X_CHILD_I2C_ADDR_MASK: c_uint = 0x00FE  /* CHILD_I2C_ADDR - [7:1] */;

pub const WM831X_CH_AW: c_uint = 0x0001  /* CH_AW */;
pub const WM831X_CH_AW_MASK: c_uint = 0x0001  /* CH_AW */;

//
// R30733 (0x780D) - Factory OTP 5
//
pub const WM831X_CHARGE_TRIM_MASK: c_uint = 0x003F  /* CHARGE_TRIM - [5:0] */;

//
// R30736 (0x7810) - Customer OTP ID
//
pub const WM831X_OTP_AUTO_PROG: c_uint = 0x8000  /* OTP_AUTO_PROG */;
pub const WM831X_OTP_AUTO_PROG_MASK: c_uint = 0x8000  /* OTP_AUTO_PROG */;

pub const WM831X_OTP_CUST_ID_MASK: c_uint = 0x7FFE  /* OTP_CUST_ID - [14:1] */;

pub const WM831X_OTP_CUST_FINAL: c_uint = 0x0001  /* OTP_CUST_FINAL */;
pub const WM831X_OTP_CUST_FINAL_MASK: c_uint = 0x0001  /* OTP_CUST_FINAL */;

//
// R30759 (0x7827) - DBE CHECK DATA
//
pub const WM831X_DBE_VALID_DATA_MASK: c_uint = 0xFFFF  /* DBE_VALID_DATA - [15:0] */;


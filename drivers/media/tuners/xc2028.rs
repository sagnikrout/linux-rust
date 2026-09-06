//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/xc2028.h
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


// SPDX-License-Identifier: GPL-2.0
//
// xc2028
//
// Copyright (c) 2007-2008 Mauro Carvalho Chehab <mchehab@kernel.org>
//

// Dmoduler		IF (kHz)

pub const XC3028_FE_LG60: c_int = 6000;
pub const XC3028_FE_ATI638: c_int = 6380;
pub const XC3028_FE_OREN538: c_int = 5380;
pub const XC3028_FE_OREN36: c_int = 3600;
pub const XC3028_FE_TOYOTA388: c_int = 3880;
pub const XC3028_FE_TOYOTA794: c_int = 7940;
pub const XC3028_FE_DIBCOM52: c_int = 5200;
pub const XC3028_FE_ZARLINK456: c_int = 4560;
pub const XC3028_FE_CHINA: c_int = 5200;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum firmware_type {
    XC2028_AUTO = 0,        /* By default, auto-detects */
    XC2028_D2633,
    XC2028_D2620,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xc2028_ctrl {
    pub fname: *mut c_char,
    pub max_len: c_int,
    pub msleep: c_int,
    pub scode_table: c_uint,
    pub :1: unsigned int mts,
    pub input1:1: c_uint,
    pub vhfbw7:1: c_uint,
    pub uhfbw8:1: c_uint,
    pub disable_power_mgmt:1: c_uint,
    pub read_not_reliable:1: c_uint,
    pub demod: c_uint,
    pub type:2: firmware_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xc2028_config {
    pub i2c_adap: *mut i2c_adapter,
    pub i2c_addr: u8,
    pub ctrl: *mut xc2028_ctrl,
}

// xc2028 commands for callback
pub const XC2028_TUNER_RESET: c_int = 0;
pub const XC2028_RESET_CLK: c_int = 1;
pub const XC2028_I2C_FLUSH: c_int = 2;


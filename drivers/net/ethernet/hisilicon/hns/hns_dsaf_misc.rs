//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hns_dsaf_misc.h
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
// Copyright (c) 2014-2015 Hisilicon Limited.
//

pub const CPLD_ADDR_PORT_OFFSET: c_uint = 0x4;
pub const HS_LED_ON: c_uint = 0xE;
pub const HS_LED_OFF: c_uint = 0xF;
pub const CPLD_LED_ON_VALUE: c_int = 1;
pub const CPLD_LED_DEFAULT_VALUE: c_int = 0;
pub const MAC_SFP_PORT_OFFSET: c_uint = 0x2;
pub const DSAF_LED_SPEED_S: c_int = 0;

pub const DSAF_LED_LINK_B: c_int = 2;
pub const DSAF_LED_DATA_B: c_int = 4;
pub const DSAF_LED_ANCHOR_B: c_int = 5;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/magnetometer/hmc5843.h
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
// Header file for hmc5843 driver
//
// Split from hmc5843.c
// Copyright (C) Josef Gajdusek <atx@atx.name>
//

pub const HMC5843_CONFIG_REG_A: c_uint = 0x00;
pub const HMC5843_CONFIG_REG_B: c_uint = 0x01;
pub const HMC5843_MODE_REG: c_uint = 0x02;
pub const HMC5843_DATA_OUT_MSB_REGS: c_uint = 0x03;
pub const HMC5843_STATUS_REG: c_uint = 0x09;
pub const HMC5843_ID_REG: c_uint = 0x0a;
pub const HMC5843_ID_END: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hmc5843_ids {
    HMC5843_ID,
    HMC5883_ID,
    HMC5883L_ID,
    HMC5983_ID,
}

//
// struct hmc5843_data	- device specific data
// @dev:		actual device
// @lock:		update and read regmap data
// @regmap:		hardware access register maps
// @variant:		describe chip variants
// @scan:		buffer to pack data for passing to
// iio_push_to_buffers_with_ts()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmc5843_data {
    pub dev: *mut device,
    pub lock: mutex,
    pub regmap: *mut regmap,
    pub variant: *const hmc5843_chip_info,
    pub orientation: iio_mount_matrix,
    pub chans: [__be16; 3],
    pub timestamp: aligned_s64,
    pub scan: },
}

extern "C" {
    pub fn hmc5843_common_remove(dev: *mut device);
}

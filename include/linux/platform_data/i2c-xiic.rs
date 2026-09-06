//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/i2c-xiic.h
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
// i2c-xiic.h
// Copyright (c) 2009 Intel Corporation
//
// Supports:
// Xilinx IIC
//
// struct xiic_i2c_platform_data - Platform data of the Xilinx I2C driver
// @num_devices:	Number of devices that shall be added when the driver
// is probed.
// @devices:		The actuall devices to add.
//
// This purpose of this platform data struct is to be able to provide a number
// of devices that should be added to the I2C bus. The reason is that sometimes
// the I2C board info is not enough, a new PCI board can for instance be
// plugged into a standard PC, and the bus number might be unknown at
// early init time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xiic_i2c_platform_data {
    pub num_devices: u8,
    pub devices: *const i2c_board_info,
}

//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/s3c_camif.h
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
// s3c24xx/s3c64xx SoC series Camera Interface (CAMIF) driver
//
// Copyright (C) 2012 Sylwester Nawrocki <sylvester.nawrocki@gmail.com>
//

// Macro flag: #define MEDIA_S3C_CAMIF_

//
// struct s3c_camif_sensor_info - an image sensor description
// @i2c_board_info: pointer to an I2C sensor subdevice board info
// @clock_frequency: frequency of the clock the host provides to a sensor
// @mbus_type: media bus type
// @i2c_bus_num: i2c control bus id the sensor is attached to
// @flags: the parallel bus flags defining signals polarity (V4L2_MBUS_*)
// @use_field: 1 if parallel bus FIELD signal is used (only s3c64xx)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_camif_sensor_info {
    pub i2c_board_info: i2c_board_info,
    pub clock_frequency: c_ulong,
    pub mbus_type: v4l2_mbus_type,
    pub i2c_bus_num: u16,
    pub flags: u16,
    pub use_field: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c_camif_plat_data {
    pub sensor: s3c_camif_sensor_info,
}

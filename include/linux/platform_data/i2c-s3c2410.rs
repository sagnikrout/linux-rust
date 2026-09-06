//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/i2c-s3c2410.h
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
// Copyright 2004-2009 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// S3C - I2C Controller platform_device info
//

//
// struct s3c2410_platform_i2c - Platform data for s3c I2C.
// @bus_num: The bus number to use (if possible).
// @flags: Any flags for the I2C bus (E.g. S3C_IICFLK_FILTER).
// @slave_addr: The I2C address for the slave device (if enabled).
// @frequency: The desired frequency in Hz of the bus.  This is
// guaranteed to not be exceeded.  If the caller does
// not care, use zero and the driver will select a
// useful default.
// @sda_delay: The delay (in ns) applied to SDA edges.
// @cfg_gpio: A callback to configure the pins for I2C operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c2410_platform_i2c {
    pub bus_num: c_int,
    pub flags: c_uint,
    pub slave_addr: c_uint,
    pub frequency: c_ulong,
    pub sda_delay: c_uint,
    pub dev): *mut *mut void (cfg_gpio)(struct platform_device,
}

//
// s3c_i2c0_set_platdata - set platform data for i2c0 device
// @i2c: The platform data to set, or NULL for default data.
//
// Register the given platform data for use with the i2c0 device. This
// call copies the platform data, so the caller can use __initdata for
// their copy.
//
// This call will set cfg_gpio if is null to the default platform
// implementation.
//
// Any user of s3c_device_i2c0 should call this, even if it is with
// NULL to ensure that the device is given the default platform data
// as the driver will no longer carry defaults.
//
extern "C" {
    pub fn s3c_i2c0_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s3c_i2c1_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s3c_i2c2_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s3c_i2c3_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s3c_i2c4_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s3c_i2c5_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s3c_i2c6_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s3c_i2c7_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
extern "C" {
    pub fn s5p_i2c_hdmiphy_set_platdata(i2c: *mut s3c2410_platform_i2c);
}
// defined by architecture to configure gpio
extern "C" {
    pub fn s3c_i2c0_cfg_gpio(dev: *mut platform_device);
}
extern "C" {
    pub fn s3c_i2c1_cfg_gpio(dev: *mut platform_device);
}
extern "C" {
    pub fn s3c_i2c2_cfg_gpio(dev: *mut platform_device);
}
extern "C" {
    pub fn s3c_i2c3_cfg_gpio(dev: *mut platform_device);
}
extern "C" {
    pub fn s3c_i2c4_cfg_gpio(dev: *mut platform_device);
}
extern "C" {
    pub fn s3c_i2c5_cfg_gpio(dev: *mut platform_device);
}
extern "C" {
    pub fn s3c_i2c6_cfg_gpio(dev: *mut platform_device);
}
extern "C" {
    pub fn s3c_i2c7_cfg_gpio(dev: *mut platform_device);
}

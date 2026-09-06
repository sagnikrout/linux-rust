//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunxi-rsb.h
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


//
// Allwinner Reduced Serial Bus Driver
//
// Copyright (c) 2015 Chen-Yu Tsai
//
// Author: Chen-Yu Tsai <wens@csie.org>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// struct sunxi_rsb_device - Basic representation of an RSB device
// @dev:	Driver model representation of the device.
// @ctrl:	RSB controller managing the bus hosting this device.
// @rtaddr:	This device's runtime address
// @hwaddr:	This device's hardware address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_rsb_device {
    pub dev: device,
    pub rsb: *mut sunxi_rsb,
    pub irq: c_int,
    pub rtaddr: u8,
    pub hwaddr: u16,
}

extern "C" {
    pub fn container_of(_arg: d, sunxi_rsb_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &rdev->dev) -> return;
}
//
// struct sunxi_rsb_driver - RSB slave device driver
// @driver:	RSB device drivers should initialize name and owner field of
// this structure.
// @probe:	binds this driver to a RSB device.
// @remove:	unbinds this driver from the RSB device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_rsb_driver {
    pub driver: device_driver,
    pub rdev): *mut *mut int (probe)(struct sunxi_rsb_device,
    pub rdev): *mut *mut void (remove)(struct sunxi_rsb_device,
}

extern "C" {
    pub fn container_of(_arg: d, sunxi_rsb_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn sunxi_rsb_driver_register(rdrv: *mut sunxi_rsb_driver) -> c_int;
}
//
// sunxi_rsb_driver_unregister() - unregister an RSB client driver
// @rdrv:	the driver to unregister
//

//
// devm_regmap_init_sunxi_rsb(): Initialise managed register map
//
// @rdev: Device that will be interacted with
// @config: Configuration for register map
//
// The return value will be an ERR_PTR() on error or a valid pointer
// to a struct regmap.  The regmap will be automatically freed by the
// device management code.
//


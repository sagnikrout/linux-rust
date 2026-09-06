//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/misc/tsi.h
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
// AMD SBTSI shared data structure and auxiliary bus definitions.
//
// Copyright (C) 2026 Advanced Micro Devices, Inc.
//

//
// struct sbtsi_data - driver private data for an AMD SB-TSI device
// @client:	underlying I2C client
// @i3cdev:	underlying I3C device (when using I3C bus)
// @sbtsi_misc_dev: miscdevice exposing ioctl interface at /dev/sbtsi-<addr>
// @lock:           mutex protecting concurrent access to the device
// @kref:      reference count; keeps @sbtsi_data alive while misc fds are open
// @dev_addr:	I2C/I3C device address, used as the auxiliary device instance id
// and name the misc device node
// @ext_range_mode:	sensor uses extended temperature range
// @read_order:	if set, decimal part must be read before integer part
// @is_i3c:	true when the device is accessed over I3C
// @detached:  set on driver unbind; open/ioctl return -ENODEV afterward
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbtsi_data {
    pub client: *mut i2c_client,
    pub i3cdev: *mut i3c_device,
}

//
// Name of the auxiliary device published on the auxiliary bus by the core
// driver.  The full device name is "amd-sbtsi.temp-sensor.<id>". where
// <id> is the auxiliary device instance id.
//

//
// sbtsi_xfer - Perform a register read or write transfer on an AMD SB-TSI device.
//
// @data:    Pointer to the sbtsi_data structure containing the device context
// @reg:     Register address to access.
// @val:     Pointer to the value to read into or write from.
// @is_read: If true, performs a read transfer and stores the result in @val.
// If false, performs a write transfer using the value in @val.
//
// Returns 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn sbtsi_xfer(data: *mut sbtsi_data, reg: u8, val: *mut u8, is_read: bool) -> c_int;
}

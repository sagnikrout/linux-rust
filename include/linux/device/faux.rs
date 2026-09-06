//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device/faux.h
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
// Copyright (c) 2025 Greg Kroah-Hartman <gregkh@linuxfoundation.org>
// Copyright (c) 2025 The Linux Foundation
//
// A "simple" faux bus that allows devices to be created and added
// automatically to it.  This is to be used whenever you need to create a
// device that is not associated with any "real" system resources, and do
// not want to have to deal with a bus/driver binding logic.  It is
// intended to be very simple, with only a create and a destroy function
// available.
//

//
// struct faux_device - a "faux" device
// @dev:	internal struct device of the object
//
// A simple faux device that can be created/destroyed.  To be used when a
// driver only needs to have a device to "hang" something off.  This can be
// used for downloading firmware or other basic tasks.  Use this instead of
// a struct platform_device if the device has no resources assigned to
// it at all.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct faux_device {
    pub dev: device,
}

//
// struct faux_device_ops - a set of callbacks for a struct faux_device
// @probe:	called when a faux device is probed by the driver core
// before the device is fully bound to the internal faux bus
// code.  If probe succeeds, return 0, otherwise return a
// negative error number to stop the probe sequence from
// succeeding.
// @remove:	called when a faux device is removed from the system
//
// Both @probe and @remove are optional, if not needed, set to NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct faux_device_ops {
    pub faux_dev): *mut *mut int (probe)(struct faux_device,
    pub faux_dev): *mut *mut void (remove)(struct faux_device,
}

extern "C" {
    pub fn faux_device_destroy(faux_dev: *mut faux_device);
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &faux_dev->dev) -> return;
}

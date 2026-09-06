//! Automatically rewritten from C Header to Rust Module
//! Source: tools/usb/usbip/libsrc/usbip_host_common.h
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
// Copyright (C) 2015-2016 Samsung Electronics
// Igor Kotrasinski <i.kotrasinsk@samsung.com>
// Krzysztof Opasiak <k.opasiak@samsung.com>
//
// Refactored from usbip_host_driver.c, which is:
// Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
// 2005-2007 Takahiro Hirofuchi
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_host_driver_ops {
    pub hdriver): *mut *mut int (open)(struct usbip_host_driver,
    pub hdriver): *mut *mut void (close)(struct usbip_host_driver,
    pub hdriver): *mut *mut int (refresh_device_list)(struct usbip_host_driver,
    pub num): *mut *mut usbip_host_driver hdriver, int,
    pub dev): *mut usbip_usb_device,
    pub uinf): *mut usbip_usb_interface,
    pub udev): *mut *mut int (is_my_device)(struct udev_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_host_driver {
    pub ndevs: c_int,
// list of exported device
    pub edev_list: list_head,
    pub udev_subsystem: *const c_char,
    pub ops: usbip_host_driver_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_exported_device {
    pub sudev: *mut udev_device,
    pub status: i32,
    pub udev: usbip_usb_device,
    pub node: list_head,
    pub uinf: [usbip_usb_interface; ],
}

// External API to access the driver
// Helper functions for implementing driver backend
extern "C" {
    pub fn usbip_generic_driver_open(hdriver: *mut usbip_host_driver) -> c_int;
}
extern "C" {
    pub fn usbip_generic_driver_close(hdriver: *mut usbip_host_driver);
}
extern "C" {
    pub fn usbip_generic_refresh_device_list(hdriver: *mut usbip_host_driver) -> c_int;
}
extern "C" {
    pub fn usbip_export_device(edev: *mut usbip_exported_device, sockfd: c_int) -> c_int;
}

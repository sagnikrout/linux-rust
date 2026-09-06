//! Automatically rewritten from C Header to Rust Module
//! Source: tools/usb/usbip/libsrc/vhci_driver.h
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
// Copyright (C) 2005-2007 Takahiro Hirofuchi
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hub_speed {
    HUB_SPEED_HIGH = 0,
    HUB_SPEED_SUPER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_imported_device {
    pub hub: hub_speed,
    pub port: u8,
    pub status: u32,
    pub devid: u32,
    pub busnum: u8,
    pub devnum: u8,
// usbip_class_device list
    pub udev: usbip_usb_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_vhci_driver {
// /sys/devices/platform/vhci_hcd
    pub hc_device: *mut udev_device,
    pub ncontrollers: c_int,
    pub nports: c_int,
    pub idev: [usbip_imported_device; ],
}

extern "C" {
    pub fn usbip_vhci_driver_open() -> c_int;
}
extern "C" {
    pub fn usbip_vhci_driver_close();
}
extern "C" {
    pub fn usbip_vhci_refresh_device_list() -> c_int;
}
extern "C" {
    pub fn usbip_vhci_get_free_port(speed: u32) -> c_int;
}
// will be removed
extern "C" {
    pub fn usbip_vhci_detach_device(port: u8) -> c_int;
}
extern "C" {
    pub fn usbip_vhci_imported_device_dump(idev: *mut usbip_imported_device) -> c_int;
}

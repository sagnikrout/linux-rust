//! Automatically rewritten from C Header to Rust Module
//! Source: tools/usb/usbip/libsrc/usbip_common.h
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

// kernel module names

// sysfs constants

pub const SYSFS_PATH_MAX: c_int = 256;
pub const SYSFS_BUS_ID_SIZE: c_int = 32;
// Defines for op_code status in server/client op_common PDUs
pub const ST_OK: c_uint = 0x00;
pub const ST_NA: c_uint = 0x01;
// Device requested for import is not available
pub const ST_DEV_BUSY: c_uint = 0x02;
// Device requested for import is in error state
pub const ST_DEV_ERR: c_uint = 0x03;
pub const ST_NODEV: c_uint = 0x04;
pub const ST_ERROR: c_uint = 0x05;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_usb_interface {
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub /: *mut *mut uint8_t padding; / alignment,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbip_usb_device {
    pub path: [c_char; SYSFS_PATH_MAX],
    pub busid: [c_char; SYSFS_BUS_ID_SIZE],
    pub busnum: u32,
    pub devnum: u32,
    pub speed: u32,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bConfigurationValue: u8,
    pub bNumConfigurations: u8,
    pub bNumInterfaces: u8,
    pub __attribute__((packed)): },

    pub ): *mut void dump_usb_interface(struct usbip_usb_interface,
    pub ): *mut void dump_usb_device(struct usbip_usb_device,
    pub udev): *mut *mut int read_usb_device(struct udev_device sdev, struct usbip_usb_device,
    pub format): *const c_char,
    pub uinf): *mut usbip_usb_interface,
    pub num): *const *const char usbip_speed_string(int,
    pub status): *const *const char usbip_status_string(int32_t,
    pub status): *const *const char usbip_op_common_status_string(int,
    pub ): *mut int usbip_names_init(char,
    pub usbip_names_free(void): c_void,
    pub product): u16,
    pub protocol): uint8_t subclass, uint8_t,

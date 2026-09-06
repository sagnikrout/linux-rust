//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/xsens_mt.c
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
// Xsens MT USB driver
//
// Copyright (C) 2013 Xsens <info@xsens.com>
//

pub const XSENS_VID: c_uint = 0x2639;
pub const MTi_10_IMU_PID: c_uint = 0x0001;
pub const MTi_20_VRU_PID: c_uint = 0x0002;
pub const MTi_30_AHRS_PID: c_uint = 0x0003;
pub const MTi_100_IMU_PID: c_uint = 0x0011;
pub const MTi_200_VRU_PID: c_uint = 0x0012;
pub const MTi_300_AHRS_PID: c_uint = 0x0013;
pub const MTi_G_700_GPS_INS_PID: c_uint = 0x0017;
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(XSENS_VID, MTi_10_IMU_PID) },
    { USB_DEVICE(XSENS_VID, MTi_20_VRU_PID) },
    { USB_DEVICE(XSENS_VID, MTi_30_AHRS_PID) },
    { USB_DEVICE(XSENS_VID, MTi_100_IMU_PID) },
    { USB_DEVICE(XSENS_VID, MTi_200_VRU_PID) },
    { USB_DEVICE(XSENS_VID, MTi_300_AHRS_PID) },
    { USB_DEVICE(XSENS_VID, MTi_G_700_GPS_INS_PID) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
    static int xsens_mt_probe(struct usb_serial *serial,
    const struct usb_device_id *id)
    {
    if (serial.interface.cur_altsetting.desc.bInterfaceNumber == 1)
    return 0;
    return -ENODEV;
    }
    static struct usb_serial_driver xsens_mt_device = {
    .driver = {
    .name = "xsens_mt",
    },
    .id_table = id_table,
    .num_ports = 1,
    .probe = xsens_mt_probe,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &xsens_mt_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_AUTHOR("Frans Klaver <frans.klaver@xsens.com>");
    MODULE_DESCRIPTION("USB-serial driver for Xsens motion trackers");
    MODULE_LICENSE("GPL v2");

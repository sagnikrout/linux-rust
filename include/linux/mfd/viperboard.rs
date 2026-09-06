//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/viperboard.h
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
// include/linux/mfd/viperboard.h
//
// Nano River Technologies viperboard definitions
//
// (C) 2012 by Lemonage GmbH
// Author: Lars Poeschel <poeschel@lemonage.de>
// All rights reserved.
//

pub const VPRBRD_EP_OUT: c_uint = 0x02;
pub const VPRBRD_EP_IN: c_uint = 0x86;

pub const VPRBRD_I2C_CMD_WRITE: c_uint = 0x00;
pub const VPRBRD_I2C_CMD_READ: c_uint = 0x01;
pub const VPRBRD_I2C_CMD_ADDR: c_uint = 0x02;
pub const VPRBRD_USB_TYPE_OUT: c_uint = 0x40;
pub const VPRBRD_USB_TYPE_IN: c_uint = 0xc0;
pub const VPRBRD_USB_TIMEOUT_MS: c_int = 100;
pub const VPRBRD_USB_REQUEST_I2C_FREQ: c_uint = 0xe6;
pub const VPRBRD_USB_REQUEST_I2C: c_uint = 0xe9;
pub const VPRBRD_USB_REQUEST_MAJOR: c_uint = 0xea;
pub const VPRBRD_USB_REQUEST_MINOR: c_uint = 0xeb;
pub const VPRBRD_USB_REQUEST_ADC: c_uint = 0xec;
pub const VPRBRD_USB_REQUEST_GPIOA: c_uint = 0xed;
pub const VPRBRD_USB_REQUEST_GPIOB: c_uint = 0xdd;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_i2c_write_hdr {
    pub cmd: u8,
    pub addr: __le16,
    pub len1: u8,
    pub len2: u8,
    pub last: u8,
    pub chan: u8,
    pub spi: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_i2c_read_hdr {
    pub cmd: u8,
    pub addr: __le16,
    pub len0: u8,
    pub len1: u8,
    pub len2: u8,
    pub len3: u8,
    pub len4: u8,
    pub len5: u8,
    pub /: *mut *mut __le16 tf1; / transfer 1 length,
    pub /: *mut *mut __le16 tf2; / transfer 2 length,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_i2c_status {
    pub unknown: [u8; 11],
    pub status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_i2c_write_msg {
    pub header: vprbrd_i2c_write_hdr,
    pub vprbrd_i2c_write_hdr)]: - sizeof(struct,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_i2c_read_msg {
    pub header: vprbrd_i2c_read_hdr,
    pub vprbrd_i2c_read_hdr)]: - sizeof(struct,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd_i2c_addr_msg {
    pub cmd: u8,
    pub addr: u8,
    pub unknown1: u8,
    pub len: __le16,
    pub unknown2: u8,
    pub unknown3: u8,
    pub __packed: },
// Structure to hold all device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vprbrd {
    pub /: *mut *mut *mut usb_device usb_dev; / the usb device for this device,
    pub lock: mutex,
    pub vprbrd_i2c_write_msg)]: u8 buf[sizeof(struct,
    pub pdev: platform_device,
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/ec168.h
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
// E3C EC168 DVB USB driver
//
// Copyright (C) 2009 Antti Palosaari <crope@iki.fi>
//

pub const EC168_USB_TIMEOUT: c_int = 1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec168_req {
    pub /: *mut *mut u8 cmd; / [1],
    pub /: *mut *mut u16 value; / [2|3],
    pub /: *mut *mut u16 index; / [4|5],
    pub /: *mut *mut u16 size; / [6|7],
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ec168_cmd {
    DOWNLOAD_FIRMWARE    = 0x00,
    CONFIG               = 0x01,
    DEMOD_RW             = 0x03,
    GPIO                 = 0x04,
    STREAMING_CTRL       = 0x10,
    READ_I2C             = 0x20,
    WRITE_I2C            = 0x21,
    HID_DOWNLOAD         = 0x30,
    GET_CONFIG,
    SET_CONFIG,
    READ_DEMOD,
    WRITE_DEMOD,
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/gp8psk.h
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
// DVB USB compliant Linux driver for the
// - GENPIX 8pks/qpsk/DCII USB2.0 DVB-S module
//
// Copyright (C) 2006 Alan Nisota (alannisota@gmail.com)
// Copyright (C) 2006,2007 Alan Nisota (alannisota@gmail.com)
//
// Thanks to GENPIX for the sample code used to implement this module.
//
// This module is based off the vp7045 and vp702x modules
//
// see Documentation/driver-api/media/drivers/dvb-usb.rst for more information
//

pub const GET_USB_SPEED: c_uint = 0x07;
pub const RESET_FX2: c_uint = 0x13;
pub const FW_VERSION_READ: c_uint = 0x0B;
pub const VENDOR_STRING_READ: c_uint = 0x0C;
pub const PRODUCT_STRING_READ: c_uint = 0x0D;
pub const FW_BCD_VERSION_READ: c_uint = 0x14;

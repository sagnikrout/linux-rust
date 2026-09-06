//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/usb/smsc75xx.h
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
// Copyright (C) 2007-2010 SMSC
//
// Tx command words

// Rx command words

// SCSRs

// MAC CSRs

// Vendor-specific PHY Definitions
// Mode Control/Status Register

// USB Vendor Requests
pub const USB_VENDOR_REQUEST_WRITE_REGISTER: c_uint = 0xA0;
pub const USB_VENDOR_REQUEST_READ_REGISTER: c_uint = 0xA1;
pub const USB_VENDOR_REQUEST_GET_STATS: c_uint = 0xA2;
// Interrupt Endpoint status word bitfields


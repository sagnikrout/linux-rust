//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/r8152.h
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
// Copyright (c) 2020 Realtek Semiconductor Corp. All rights reserved.
//
pub const RTL8152_REQT_READ: c_uint = 0xc0;
pub const RTL8152_REQT_WRITE: c_uint = 0x40;
pub const RTL8152_REQ_GET_REGS: c_uint = 0x05;
pub const RTL8152_REQ_SET_REGS: c_uint = 0x05;
pub const BYTE_EN_DWORD: c_uint = 0xff;
pub const BYTE_EN_WORD: c_uint = 0x33;
pub const BYTE_EN_BYTE: c_uint = 0x11;
pub const BYTE_EN_SIX_BYTES: c_uint = 0x3f;
pub const BYTE_EN_START_MASK: c_uint = 0x0f;
pub const BYTE_EN_END_MASK: c_uint = 0xf0;
pub const MCU_TYPE_PLA: c_uint = 0x0100;
pub const MCU_TYPE_USB: c_uint = 0x0000;
// Define these values to match your device
pub const VENDOR_ID_REALTEK: c_uint = 0x0bda;
pub const VENDOR_ID_MICROSOFT: c_uint = 0x045e;
pub const VENDOR_ID_SAMSUNG: c_uint = 0x04e8;
pub const VENDOR_ID_LENOVO: c_uint = 0x17ef;
pub const VENDOR_ID_LINKSYS: c_uint = 0x13b1;
pub const VENDOR_ID_NVIDIA: c_uint = 0x0955;
pub const VENDOR_ID_TPLINK: c_uint = 0x2357;
pub const VENDOR_ID_DLINK: c_uint = 0x2001;
pub const VENDOR_ID_DELL: c_uint = 0x413c;
pub const VENDOR_ID_ASUS: c_uint = 0x0b05;
pub const VENDOR_ID_TRENDNET: c_uint = 0x20f4;

extern "C" {
    pub fn rtl8152_get_version(intf: *mut usb_interface) -> u8;
}


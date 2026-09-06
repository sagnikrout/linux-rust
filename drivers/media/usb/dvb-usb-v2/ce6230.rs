//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/ce6230.h
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
// Intel CE6230 DVB USB driver
//
// Copyright (C) 2009 Antti Palosaari <crope@iki.fi>
//

pub const CE6230_USB_TIMEOUT: c_int = 1000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_req {
    pub /: *mut *mut u8 cmd; / [1],
    pub /: *mut *mut u16 value; / [2|3],
    pub /: *mut *mut u16 index; / [4|5],
    pub /: *mut *mut u16 data_len; / [6|7],
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ce6230_cmd {
    CONFIG_READ          = 0xd0, /* rd 0 (unclear) */
    UNKNOWN_WRITE        = 0xc7, /* wr 7 (unclear) */
    I2C_READ             = 0xd9, /* rd 9 (unclear) */
    I2C_WRITE            = 0xca, /* wr a */
    DEMOD_READ           = 0xdb, /* rd b */
    DEMOD_WRITE          = 0xcc, /* wr c */
    REG_READ             = 0xde, /* rd e */
    REG_WRITE            = 0xcf, /* wr f */
}

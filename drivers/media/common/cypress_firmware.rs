//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/common/cypress_firmware.h
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
// Copyright (C) 2004-6 Patrick Boettcher (patrick.boettcher@posteo.de)
// see dvb-usb-init.c for copyright information.
//
// This file contains functions for downloading the firmware to Cypress FX 1
// and 2 based devices.
//
pub const CYPRESS_AN2135: c_int = 0;
pub const CYPRESS_AN2235: c_int = 1;
pub const CYPRESS_FX2: c_int = 2;
// commonly used firmware download types and function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hexline {
    pub len: u8,
    pub addr: u32,
    pub type: u8,
    pub data: [u8; 255],
    pub chk: u8,
}

extern "C" {
    pub fn cypress_load_firmware(: *mut usb_device, : *const firmware, _arg: c_int) -> c_int;
}

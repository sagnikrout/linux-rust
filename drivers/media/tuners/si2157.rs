//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/si2157.h
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
// Silicon Labs Si2146/2147/2148/2157/2158 silicon tuner driver
//
// Copyright (C) 2014 Antti Palosaari <crope@iki.fi>
//

//
// struct si2157_config - configuration parameters for si2157
//
// @fe:
// frontend returned by driver
// @mdev:
// media device returned by driver
// @inversion:
// spectral inversion
// @dont_load_firmware:
// Instead of uploading a new firmware, use the existing one
// @if_port:
// Port selection
// Select the RF interface to use (pins 9+11 or 12+13)
//
// Note:
// The I2C address of this demod is 0x60.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si2157_config {
    pub fe: *mut dvb_frontend,

    pub mdev: *mut media_device,

    pub inversion:1: c_uint,
    pub dont_load_firmware:1: c_uint,
    pub if_port: u8,
}

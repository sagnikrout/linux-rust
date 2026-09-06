//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/it913x.h
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
// ITE Tech IT9137 silicon tuner driver
//
// Copyright (C) 2011 Malcolm Priestley (tvboxspy@gmail.com)
// IT9137 Copyright (C) ITE Tech Inc.
//

//
// struct it913x_platform_data - Platform data for the it913x driver
// @regmap: af9033 demod driver regmap.
// @fe: af9033 demod driver DVB frontend.
// @role: Chip role, single or dual configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct it913x_platform_data {
    pub regmap: *mut regmap,
    pub fe: *mut dvb_frontend,
pub const IT913X_ROLE_SINGLE: c_int = 0;
pub const IT913X_ROLE_DUAL_MASTER: c_int = 1;
pub const IT913X_ROLE_DUAL_SLAVE: c_int = 2;
    pub role:2: c_uint,
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/touchscreen/goodix_berlin.h
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
// Goodix Touchscreen Driver
// Copyright (C) 2020 - 2021 Goodix, Inc.
// Copyright (C) 2023 Linaro Ltd.
//
// Based on goodix_berlin_berlin driver.
//

pub const GOODIX_BERLIN_FW_VERSION_INFO_ADDR_A: c_uint = 0x1000C;
pub const GOODIX_BERLIN_FW_VERSION_INFO_ADDR_D: c_uint = 0x10014;
pub const GOODIX_BERLIN_IC_INFO_ADDR_A: c_uint = 0x10068;
pub const GOODIX_BERLIN_IC_INFO_ADDR_D: c_uint = 0x10070;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_berlin_ic_data {
    pub fw_version_info_addr: c_int,
    pub ic_info_addr: c_int,
    pub read_dummy_len: isize,
    pub read_prefix_len: isize,
}

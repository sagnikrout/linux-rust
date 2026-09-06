//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/mt6660.h
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
// Copyright (c) 2019 MediaTek Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6660_platform_data {
    pub init_setting_num: u8,
    pub init_setting_addr: *mut u32,
    pub init_setting_mask: *mut u32,
    pub init_setting_val: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6660_chip {
    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub param_dev: *mut platform_device,
    pub plat_data: mt6660_platform_data,
    pub io_lock: mutex,
    pub regmap: *mut regmap,
    pub chip_rev: u16,
}


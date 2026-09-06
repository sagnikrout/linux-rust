//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bma150.h
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
// Copyright (c) 2011 Bosch Sensortec GmbH
// Copyright (c) 2011 Unixphere
//

pub const BMA150_RANGE_2G: c_int = 0;
pub const BMA150_RANGE_4G: c_int = 1;
pub const BMA150_RANGE_8G: c_int = 2;
pub const BMA150_BW_25HZ: c_int = 0;
pub const BMA150_BW_50HZ: c_int = 1;
pub const BMA150_BW_100HZ: c_int = 2;
pub const BMA150_BW_190HZ: c_int = 3;
pub const BMA150_BW_375HZ: c_int = 4;
pub const BMA150_BW_750HZ: c_int = 5;
pub const BMA150_BW_1500HZ: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bma150_cfg {
    pub /: *mut *mut bool any_motion_int; / Set to enable any-motion interrupt,
    pub /: *mut *mut bool hg_int; / Set to enable high-G interrupt,
    pub /: *mut *mut bool lg_int; / Set to enable low-G interrupt,
    pub /: *mut *mut unsigned char any_motion_dur; / Any-motion duration,
    pub /: *mut *mut unsigned char any_motion_thres; / Any-motion threshold,
    pub /: *mut *mut unsigned char hg_hyst; / High-G hysterisis,
    pub /: *mut *mut unsigned char hg_dur; / High-G duration,
    pub /: *mut *mut unsigned char hg_thres; / High-G threshold,
    pub /: *mut *mut unsigned char lg_hyst; / Low-G hysterisis,
    pub /: *mut *mut unsigned char lg_dur; / Low-G duration,
    pub /: *mut *mut unsigned char lg_thres; / Low-G threshold,
    pub /: *mut *mut unsigned char range; / one of BMA150_RANGE_xxx,
    pub /: *mut *mut unsigned char bandwidth; / one of BMA150_BW_xxx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bma150_platform_data {
    pub cfg: bma150_cfg,
    pub (*irq_gpio_cfg)(void): *mut c_int,
}

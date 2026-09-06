//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/aw88395/aw88395.h
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
// aw88395.h --  ALSA SoC AW88395 codec support
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

pub const FADE_TIME_MAX: c_int = 100000;
pub const FADE_TIME_MIN: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw88395 {
    pub aw_pa: *mut aw_device,
    pub lock: mutex,
    pub reset_gpio: *mut gpio_desc,
    pub start_work: delayed_work,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
}

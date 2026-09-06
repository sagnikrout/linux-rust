//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/rt5660.h
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
// linux/sound/rt5660.h -- Platform data for RT5660
//
// Copyright 2016 Realtek Semiconductor Corp.
// Author: Oder Chiou <oder_chiou@realtek.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt5660_dmic1_data_pin {
    RT5660_DMIC1_NULL,
    RT5660_DMIC1_DATA_GPIO2,
    RT5660_DMIC1_DATA_IN1P,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5660_platform_data {
// IN1 & IN3 can optionally be differential
    pub in1_diff: bool,
    pub in3_diff: bool,
    pub use_ldo2: bool,
    pub poweroff_codec_in_suspend: bool,
    pub dmic1_data_pin: rt5660_dmic1_data_pin,
}

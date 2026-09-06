//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/meson/axg-tdm-formatter.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 Baylibre SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_tdm_formatter_hw {
    pub skew_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_tdm_formatter_ops {
    pub w): *mut *mut *mut axg_tdm_stream (get_stream)(snd_soc_dapm_widget,
    pub map): *mut *mut void (enable)(struct regmap,
    pub map): *mut *mut void (disable)(struct regmap,
    pub ts): *mut axg_tdm_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axg_tdm_formatter_driver {
    pub component_drv: *const snd_soc_component_driver,
    pub regmap_cfg: *const regmap_config,
    pub ops: *const axg_tdm_formatter_ops,
    pub quirks: *const axg_tdm_formatter_hw,
}

extern "C" {
    pub fn axg_tdm_formatter_probe(pdev: *mut platform_device) -> c_int;
}

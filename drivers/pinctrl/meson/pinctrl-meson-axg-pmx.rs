//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/meson/pinctrl-meson-axg-pmx.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Copyright (c) 2017 Baylibre SAS.
// Author:  Jerome Brunet  <jbrunet@baylibre.com>
//
// Copyright (c) 2017 Amlogic, Inc. All rights reserved.
// Author: Xingyu Chen <xingyu.chen@amlogic.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pmx_bank {
    pub name: *const c_char,
    pub first: c_uint,
    pub last: c_uint,
    pub reg: c_uint,
    pub offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_axg_pmx_data {
    pub pmx_banks: *const meson_pmx_bank,
    pub num_pmx_banks: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_pmx_axg_data {
    pub func: c_uint,
}


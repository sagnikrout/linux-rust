//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_cmm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// R-Car Display Unit Color Management Module
//
// Copyright (C) 2019 Jacopo Mondi <jacopo+renesas@jmondi.org>
//
pub const CM2_LUT_SIZE: c_int = 256;
//
// struct rcar_cmm_config - CMM configuration
//
// @lut:	1D-LUT configuration
// @lut.table:	1D-LUT table entries. Disable LUT operations when NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_cmm_config {
    pub table: *mut drm_color_lut,
    pub lut: },
}

extern "C" {
    pub fn rcar_cmm_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn rcar_cmm_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn rcar_cmm_disable(dev: *mut device);
}


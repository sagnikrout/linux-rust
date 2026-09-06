//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/cma3000.h
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
// VTI CMA3000_Dxx Accelerometer driver
//
// Copyright (C) 2010 Texas Instruments
// Author: Hemanth V <hemanthv@ti.com>
//
pub const CMAMODE_DEFAULT: c_int = 0;
pub const CMAMODE_MEAS100: c_int = 1;
pub const CMAMODE_MEAS400: c_int = 2;
pub const CMAMODE_MEAS40: c_int = 3;
pub const CMAMODE_MOTDET: c_int = 4;
pub const CMAMODE_FF100: c_int = 5;
pub const CMAMODE_FF400: c_int = 6;
pub const CMAMODE_POFF: c_int = 7;
pub const CMARANGE_2G: c_int = 2000;
pub const CMARANGE_8G: c_int = 8000;
//
// struct cma3000_i2c_platform_data - CMA3000 Platform data
// @fuzz_x: Noise on X Axis
// @fuzz_y: Noise on Y Axis
// @fuzz_z: Noise on Z Axis
// @g_range: G range in milli g i.e 2000 or 8000
// @mode: Operating mode
// @mdthr: Motion detect threshold value
// @mdfftmr: Motion detect and free fall time value
// @ffthr: Free fall threshold value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cma3000_platform_data {
    pub fuzz_x: c_int,
    pub fuzz_y: c_int,
    pub fuzz_z: c_int,
    pub g_range: c_int,
    pub mode: u8,
    pub mdthr: u8,
    pub mdfftmr: u8,
    pub ffthr: u8,
    pub irqflags: c_ulong,
}

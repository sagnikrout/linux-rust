//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/tvp514x.h
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
// drivers/media/video/tvp514x.h
//
// Copyright (C) 2008 Texas Instruments Inc
// Author: Vaibhav Hiremath <hvaibhav@ti.com>
//
// Contributors:
// Sivaraj R <sivaraj@ti.com>
// Brijesh R Jadav <brijesh.j@ti.com>
// Hardik Shah <hardik.shah@ti.com>
// Manjunath Hadli <mrh@ti.com>
// Karicheri Muralidharan <m-karicheri2@ti.com>
//
// Other macros
//

// Number of pixels and number of lines per frame for different standards

// enum for different decoder input pin configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tvp514x_input {
//
// CVBS input selection
//
    INPUT_CVBS_VI1A = 0x0,
    INPUT_CVBS_VI1B,
    INPUT_CVBS_VI1C,
    INPUT_CVBS_VI2A = 0x04,
    INPUT_CVBS_VI2B,
    INPUT_CVBS_VI2C,
    INPUT_CVBS_VI3A = 0x08,
    INPUT_CVBS_VI3B,
    INPUT_CVBS_VI3C,
    INPUT_CVBS_VI4A = 0x0C,
//
// S-Video input selection
//
    INPUT_SVIDEO_VI2A_VI1A = 0x44,
    INPUT_SVIDEO_VI2B_VI1B,
    INPUT_SVIDEO_VI2C_VI1C,
    INPUT_SVIDEO_VI2A_VI3A = 0x54,
    INPUT_SVIDEO_VI2B_VI3B,
    INPUT_SVIDEO_VI2C_VI3C,
    INPUT_SVIDEO_VI4A_VI1A = 0x4C,
    INPUT_SVIDEO_VI4A_VI1B,
    INPUT_SVIDEO_VI4A_VI1C,
    INPUT_SVIDEO_VI4A_VI3A = 0x5C,
    INPUT_SVIDEO_VI4A_VI3B,
    INPUT_SVIDEO_VI4A_VI3C,

// Need to add entries for
// RGB, YPbPr and SCART.
//
    INPUT_INVALID
}

// enum for output format supported.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tvp514x_output {
    OUTPUT_10BIT_422_EMBEDDED_SYNC = 0,
    OUTPUT_20BIT_422_SEPERATE_SYNC,
    OUTPUT_10BIT_422_SEPERATE_SYNC = 3,
    OUTPUT_INVALID
}

//
// struct tvp514x_platform_data - Platform data values and access functions.
// @clk_polarity: Clock polarity of the current interface.
// @hs_polarity: HSYNC Polarity configuration for current interface.
// @vs_polarity: VSYNC Polarity configuration for current interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tvp514x_platform_data {
// Interface control params
    pub clk_polarity: bool,
    pub hs_polarity: bool,
    pub vs_polarity: bool,
}

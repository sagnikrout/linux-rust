//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/tvp7002.h
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
// Texas Instruments Triple 8-/10-BIT 165-/110-MSPS Video and Graphics
// Digitizer with Horizontal PLL registers
//
// Copyright (C) 2009 Texas Instruments Inc
// Author: Santiago Nunez-Corrales <santiago.nunez@ridgerun.com>
//
// This code is partially based upon the TVP5150 driver
// written by Mauro Carvalho Chehab <mchehab@kernel.org>,
// the TVP514x driver written by Vaibhav Hiremath <hvaibhav@ti.com>
// and the TVP7002 driver in the TI LSP 2.10.00.14
//

//
// struct tvp7002_config - Platform dependent data
// @clk_polarity: Clock polarity
// 0 - Data clocked out on rising edge of DATACLK signal
// 1 - Data clocked out on falling edge of DATACLK signal
// @hs_polarity:  HSYNC polarity
// 0 - Active low HSYNC output, 1 - Active high HSYNC output
// @vs_polarity: VSYNC Polarity
// 0 - Active low VSYNC output, 1 - Active high VSYNC output
// @fid_polarity: Active-high Field ID polarity.
// 0 - The field ID output is set to logic 1 for an odd field
// (field 1) and set to logic 0 for an even field (field 0).
// 1 - Operation with polarity inverted.
// @sog_polarity: Active high Sync on Green output polarity.
// 0 - Normal operation, 1 - Operation with polarity inverted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tvp7002_config {
    pub clk_polarity: bool,
    pub hs_polarity: bool,
    pub vs_polarity: bool,
    pub fid_polarity: bool,
    pub sog_polarity: bool,
}

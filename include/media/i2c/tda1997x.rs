//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/tda1997x.h
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
// tda1997x - NXP HDMI receiver
//
// Copyright 2017 Tim Harvey <tharvey@gateworks.com>
//
// Platform Data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda1997x_platform_data {
    pub vidout_bus_type: v4l2_mbus_type,
    pub vidout_bus_width: u32,
    pub vidout_port_cfg: [u8; 9],
// pin polarity (1=invert)
    pub vidout_inv_de: bool,
    pub vidout_inv_hs: bool,
    pub vidout_inv_vs: bool,
    pub vidout_inv_pclk: bool,
// clock delays (0=-8, 1=-7 ... 15=+7 pixels)
    pub vidout_delay_hs: u8,
    pub vidout_delay_vs: u8,
    pub vidout_delay_de: u8,
    pub vidout_delay_pclk: u8,
// sync selections (controls how sync pins are derived)
    pub vidout_sel_hs: u8,
    pub vidout_sel_vs: u8,
    pub vidout_sel_de: u8,
// Audio Port Output
    pub audout_format: c_int,
    pub /: *mut *mut u32 audout_mclk_fs; / clock multiplier,
    pub /: *mut *mut u32 audout_width; / 13 or 32 bit,
    pub /: *mut *mut u32 audout_layout; / layout0=AP0 layout1=AP0,AP1,AP2,AP3,
    pub /: *mut *mut bool audout_layoutauto; / audio layout dictated by pkt header,
    pub /: *mut *mut bool audout_invert_clk; / data valid on rising edge of BCLK,
    pub /: *mut *mut bool audio_auto_mute; / enable hardware audio auto-mute,
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/wm8505fb_regs.h
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
// GOVR registers list for WM8505 chips
//
// Copyright (C) 2010 Ed Spiridonov <edo.rus@gmail.com>
// Based on VIA/WonderMedia wm8510-govrh-reg.h
// http://github.com/projectgus/kernel_wm8505/blob/wm8505_2.6.29
// drivers/video/wmt/register/wm8510/wm8510-govrh-reg.h
//
// Color space select register, default value 0x1c
// BIT0 GOVRH_DVO_YUV2RGB_ENABLE
// BIT1 GOVRH_VGA_YUV2RGB_ENABLE
// BIT2 GOVRH_RGB_MODE
// BIT3 GOVRH_DAC_CLKINV
// BIT4 GOVRH_BLANK_ZERO
//
pub const WMT_GOVR_COLORSPACE: c_uint = 0x1e4;
//
// Another colorspace select register, default value 1
// BIT0 GOVRH_DVO_RGB
// BIT1 GOVRH_DVO_YUV422
//
pub const WMT_GOVR_COLORSPACE1: c_uint = 0x30;
pub const WMT_GOVR_CONTRAST: c_uint = 0x1b8;
pub const WMT_GOVR_BRGHTNESS: c_uint = 0x1bc /* incompatible with RGB? */;
// Framubeffer address
pub const WMT_GOVR_FBADDR: c_uint = 0x90;
pub const WMT_GOVR_FBADDR1: c_uint = 0x94 /* UV offset in YUV mode */;
// Offset of visible window
pub const WMT_GOVR_XPAN: c_uint = 0xa4;
pub const WMT_GOVR_YPAN: c_uint = 0xa0;
pub const WMT_GOVR_XRES: c_uint = 0x98;
pub const WMT_GOVR_XRES_VIRTUAL: c_uint = 0x9c;
pub const WMT_GOVR_MIF_ENABLE: c_uint = 0x80;
pub const WMT_GOVR_FHI: c_uint = 0xa8;
pub const WMT_GOVR_REG_UPDATE: c_uint = 0xe4;
//
// BIT0 GOVRH_DVO_OUTWIDTH
// BIT1 GOVRH_DVO_SYNC_POLAR
// BIT2 GOVRH_DVO_ENABLE
//
pub const WMT_GOVR_DVO_SET: c_uint = 0x148;
// Timing generator?
pub const WMT_GOVR_TG: c_uint = 0x100;
// Timings
pub const WMT_GOVR_TIMING_H_ALL: c_uint = 0x108;
pub const WMT_GOVR_TIMING_V_ALL: c_uint = 0x10c;
pub const WMT_GOVR_TIMING_V_START: c_uint = 0x110;
pub const WMT_GOVR_TIMING_V_END: c_uint = 0x114;
pub const WMT_GOVR_TIMING_H_START: c_uint = 0x118;
pub const WMT_GOVR_TIMING_H_END: c_uint = 0x11c;
pub const WMT_GOVR_TIMING_V_SYNC: c_uint = 0x128;
pub const WMT_GOVR_TIMING_H_SYNC: c_uint = 0x12c;

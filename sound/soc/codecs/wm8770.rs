//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8770.h
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
// wm8770.h  --  WM8770 ASoC driver
//
// Copyright 2010 Wolfson Microelectronics plc
//
// Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
//
// Registers
pub const WM8770_VOUT1LVOL: c_int = 0;
pub const WM8770_VOUT1RVOL: c_uint = 0x1;
pub const WM8770_VOUT2LVOL: c_uint = 0x2;
pub const WM8770_VOUT2RVOL: c_uint = 0x3;
pub const WM8770_VOUT3LVOL: c_uint = 0x4;
pub const WM8770_VOUT3RVOL: c_uint = 0x5;
pub const WM8770_VOUT4LVOL: c_uint = 0x6;
pub const WM8770_VOUT4RVOL: c_uint = 0x7;
pub const WM8770_MSALGVOL: c_uint = 0x8;
pub const WM8770_DAC1LVOL: c_uint = 0x9;
pub const WM8770_DAC1RVOL: c_uint = 0xa;
pub const WM8770_DAC2LVOL: c_uint = 0xb;
pub const WM8770_DAC2RVOL: c_uint = 0xc;
pub const WM8770_DAC3LVOL: c_uint = 0xd;
pub const WM8770_DAC3RVOL: c_uint = 0xe;
pub const WM8770_DAC4LVOL: c_uint = 0xf;
pub const WM8770_DAC4RVOL: c_uint = 0x10;
pub const WM8770_MSDIGVOL: c_uint = 0x11;
pub const WM8770_DACPHASE: c_uint = 0x12;
pub const WM8770_DACCTRL1: c_uint = 0x13;
pub const WM8770_DACMUTE: c_uint = 0x14;
pub const WM8770_DACCTRL2: c_uint = 0x15;
pub const WM8770_IFACECTRL: c_uint = 0x16;
pub const WM8770_MSTRCTRL: c_uint = 0x17;
pub const WM8770_PWDNCTRL: c_uint = 0x18;
pub const WM8770_ADCLCTRL: c_uint = 0x19;
pub const WM8770_ADCRCTRL: c_uint = 0x1a;
pub const WM8770_ADCMUX: c_uint = 0x1b;
pub const WM8770_OUTMUX1: c_uint = 0x1c;
pub const WM8770_OUTMUX2: c_uint = 0x1d;
pub const WM8770_RESET: c_uint = 0x31;
pub const WM8770_CACHEREGNUM: c_uint = 0x20;

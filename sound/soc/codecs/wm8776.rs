//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8776.h
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
// wm8776.h  --  WM8776 ASoC driver
//
// Copyright 2009 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// Registers
pub const WM8776_HPLVOL: c_uint = 0x00;
pub const WM8776_HPRVOL: c_uint = 0x01;
pub const WM8776_HPMASTER: c_uint = 0x02;
pub const WM8776_DACLVOL: c_uint = 0x03;
pub const WM8776_DACRVOL: c_uint = 0x04;
pub const WM8776_DACMASTER: c_uint = 0x05;
pub const WM8776_PHASESWAP: c_uint = 0x06;
pub const WM8776_DACCTRL1: c_uint = 0x07;
pub const WM8776_DACMUTE: c_uint = 0x08;
pub const WM8776_DACCTRL2: c_uint = 0x09;
pub const WM8776_DACIFCTRL: c_uint = 0x0a;
pub const WM8776_ADCIFCTRL: c_uint = 0x0b;
pub const WM8776_MSTRCTRL: c_uint = 0x0c;
pub const WM8776_PWRDOWN: c_uint = 0x0d;
pub const WM8776_ADCLVOL: c_uint = 0x0e;
pub const WM8776_ADCRVOL: c_uint = 0x0f;
pub const WM8776_ALCCTRL1: c_uint = 0x10;
pub const WM8776_ALCCTRL2: c_uint = 0x11;
pub const WM8776_ALCCTRL3: c_uint = 0x12;
pub const WM8776_NOISEGATE: c_uint = 0x13;
pub const WM8776_LIMITER: c_uint = 0x14;
pub const WM8776_ADCMUX: c_uint = 0x15;
pub const WM8776_OUTMUX: c_uint = 0x16;
pub const WM8776_RESET: c_uint = 0x17;
pub const WM8776_CACHEREGNUM: c_uint = 0x17;
pub const WM8776_DAI_DAC: c_int = 0;
pub const WM8776_DAI_ADC: c_int = 1;

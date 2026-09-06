//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max9867.h
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
// max9867.h -- MAX9867 ALSA SoC Audio driver
//
// Copyright 2013-2015 Maxim Integrated Products
//
// MAX9867 register space
pub const MAX9867_STATUS: c_uint = 0x00;
pub const MAX9867_JACKSTATUS: c_uint = 0x01;
pub const MAX9867_AUXHIGH: c_uint = 0x02;
pub const MAX9867_AUXLOW: c_uint = 0x03;
pub const MAX9867_INTEN: c_uint = 0x04;
pub const MAX9867_SYSCLK: c_uint = 0x05;
pub const MAX9867_FREQ_MASK: c_uint = 0xF;
pub const MAX9867_PSCLK_SHIFT: c_uint = 0x4;
pub const MAX9867_PSCLK_WIDTH: c_uint = 0x2;

pub const MAX9867_PSCLK_10_20: c_uint = 0x1;
pub const MAX9867_PSCLK_20_40: c_uint = 0x2;
pub const MAX9867_PSCLK_40_60: c_uint = 0x3;
pub const MAX9867_AUDIOCLKHIGH: c_uint = 0x06;
pub const MAX9867_NI_HIGH_MASK: c_uint = 0x7F;
pub const MAX9867_NI_LOW_MASK: c_uint = 0xFE;

pub const MAX9867_AUDIOCLKLOW: c_uint = 0x07;
pub const MAX9867_RAPID_LOCK: c_uint = 0x01;
pub const MAX9867_IFC1A: c_uint = 0x08;

pub const MAX9867_IFC1B: c_uint = 0x09;
pub const MAX9867_IFC1B_BCLK_MASK: c_int = 7;
pub const MAX9867_IFC1B_64X: c_uint = 0x01;
pub const MAX9867_IFC1B_48X: c_uint = 0x02;
pub const MAX9867_IFC1B_PCLK_2: c_uint = 0x04;
pub const MAX9867_IFC1B_PCLK_4: c_uint = 0x05;
pub const MAX9867_IFC1B_PCLK_8: c_uint = 0x06;
pub const MAX9867_IFC1B_PCLK_16: c_uint = 0x07;
pub const MAX9867_CODECFLTR: c_uint = 0x0a;

pub const MAX9867_SIDETONE: c_uint = 0x0b;
pub const MAX9867_DACLEVEL: c_uint = 0x0c;
pub const MAX9867_ADCLEVEL: c_uint = 0x0d;
pub const MAX9867_LEFTLINELVL: c_uint = 0x0e;
pub const MAX9867_RIGHTLINELVL: c_uint = 0x0f;
pub const MAX9867_LEFTVOL: c_uint = 0x10;
pub const MAX9867_RIGHTVOL: c_uint = 0x11;
pub const MAX9867_LEFTMICGAIN: c_uint = 0x12;
pub const MAX9867_RIGHTMICGAIN: c_uint = 0x13;
pub const MAX9867_INPUTCONFIG: c_uint = 0x14;
pub const MAX9867_MICCONFIG: c_uint = 0x15;
pub const MAX9867_MODECONFIG: c_uint = 0x16;
pub const MAX9867_PWRMAN: c_uint = 0x17;

pub const MAX9867_REVISION: c_uint = 0xff;
pub const MAX9867_CACHEREGNUM: c_int = 10;

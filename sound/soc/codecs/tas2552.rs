//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tas2552.h
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
// tas2552.h - ALSA SoC Texas Instruments TAS2552 Mono Audio Amplifier
//
// Copyright (C) 2014 Texas Instruments Incorporated -  https://www.ti.com
//
// Author: Dan Murphy <dmurphy@ti.com>
//
// Register Address Map
pub const TAS2552_DEVICE_STATUS: c_uint = 0x00;
pub const TAS2552_CFG_1: c_uint = 0x01;
pub const TAS2552_CFG_2: c_uint = 0x02;
pub const TAS2552_CFG_3: c_uint = 0x03;
pub const TAS2552_DOUT: c_uint = 0x04;
pub const TAS2552_SER_CTRL_1: c_uint = 0x05;
pub const TAS2552_SER_CTRL_2: c_uint = 0x06;
pub const TAS2552_OUTPUT_DATA: c_uint = 0x07;
pub const TAS2552_PLL_CTRL_1: c_uint = 0x08;
pub const TAS2552_PLL_CTRL_2: c_uint = 0x09;
pub const TAS2552_PLL_CTRL_3: c_uint = 0x0a;
pub const TAS2552_BTIP: c_uint = 0x0b;
pub const TAS2552_BTS_CTRL: c_uint = 0x0c;
pub const TAS2552_RESERVED_0D: c_uint = 0x0d;
pub const TAS2552_LIMIT_RATE_HYS: c_uint = 0x0e;
pub const TAS2552_LIMIT_RELEASE: c_uint = 0x0f;
pub const TAS2552_LIMIT_INT_COUNT: c_uint = 0x10;
pub const TAS2552_PDM_CFG: c_uint = 0x11;
pub const TAS2552_PGA_GAIN: c_uint = 0x12;
pub const TAS2552_EDGE_RATE_CTRL: c_uint = 0x13;
pub const TAS2552_BOOST_APT_CTRL: c_uint = 0x14;
pub const TAS2552_VER_NUM: c_uint = 0x16;
pub const TAS2552_VBAT_DATA: c_uint = 0x19;

// CFG1 Register Masks

// CFG2 Register Masks

// CFG3 Register Masks

// DOUT Register Masks

// Serial Interface Control Register Masks

// OUTPUT_DATA register

// PDM CFG Register

// Boost Auto-pass through register

// PLL Control Register
pub const TAS2552_PLL_J_MASK: c_uint = 0x7f;


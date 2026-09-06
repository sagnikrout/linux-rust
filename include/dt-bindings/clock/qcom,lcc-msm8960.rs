//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,lcc-msm8960.h
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
// Copyright (c) 2014, The Linux Foundation. All rights reserved.
//
pub const PLL4: c_int = 0;
pub const MI2S_OSR_SRC: c_int = 1;
pub const MI2S_OSR_CLK: c_int = 2;
pub const MI2S_DIV_CLK: c_int = 3;
pub const MI2S_BIT_DIV_CLK: c_int = 4;
pub const MI2S_BIT_CLK: c_int = 5;
pub const PCM_SRC: c_int = 6;
pub const PCM_CLK_OUT: c_int = 7;
pub const PCM_CLK: c_int = 8;
pub const SLIMBUS_SRC: c_int = 9;
pub const AUDIO_SLIMBUS_CLK: c_int = 10;
pub const SPS_SLIMBUS_CLK: c_int = 11;
pub const CODEC_I2S_MIC_OSR_SRC: c_int = 12;
pub const CODEC_I2S_MIC_OSR_CLK: c_int = 13;
pub const CODEC_I2S_MIC_DIV_CLK: c_int = 14;
pub const CODEC_I2S_MIC_BIT_DIV_CLK: c_int = 15;
pub const CODEC_I2S_MIC_BIT_CLK: c_int = 16;
pub const SPARE_I2S_MIC_OSR_SRC: c_int = 17;
pub const SPARE_I2S_MIC_OSR_CLK: c_int = 18;
pub const SPARE_I2S_MIC_DIV_CLK: c_int = 19;
pub const SPARE_I2S_MIC_BIT_DIV_CLK: c_int = 20;
pub const SPARE_I2S_MIC_BIT_CLK: c_int = 21;
pub const CODEC_I2S_SPKR_OSR_SRC: c_int = 22;
pub const CODEC_I2S_SPKR_OSR_CLK: c_int = 23;
pub const CODEC_I2S_SPKR_DIV_CLK: c_int = 24;
pub const CODEC_I2S_SPKR_BIT_DIV_CLK: c_int = 25;
pub const CODEC_I2S_SPKR_BIT_CLK: c_int = 26;
pub const SPARE_I2S_SPKR_OSR_SRC: c_int = 27;
pub const SPARE_I2S_SPKR_OSR_CLK: c_int = 28;
pub const SPARE_I2S_SPKR_DIV_CLK: c_int = 29;
pub const SPARE_I2S_SPKR_BIT_DIV_CLK: c_int = 30;
pub const SPARE_I2S_SPKR_BIT_CLK: c_int = 31;

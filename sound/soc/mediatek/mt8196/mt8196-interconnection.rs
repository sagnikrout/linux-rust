//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8196/mt8196-interconnection.h
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
// Mediatek MT8196 audio driver interconnection definition
//
// Copyright (c) 2025 MediaTek Inc.
// Author: Darren Ye <darren.ye@mediatek.com>
//
// in port define
pub const I_CONNSYS_I2S_CH1: c_int = 0;
pub const I_CONNSYS_I2S_CH2: c_int = 1;
pub const I_GAIN0_OUT_CH1: c_int = 6;
pub const I_GAIN0_OUT_CH2: c_int = 7;
pub const I_GAIN1_OUT_CH1: c_int = 8;
pub const I_GAIN1_OUT_CH2: c_int = 9;
pub const I_GAIN2_OUT_CH1: c_int = 10;
pub const I_GAIN2_OUT_CH2: c_int = 11;
pub const I_GAIN3_OUT_CH1: c_int = 12;
pub const I_GAIN3_OUT_CH2: c_int = 13;
pub const I_STF_CH1: c_int = 14;
pub const I_ADDA_UL_CH1: c_int = 16;
pub const I_ADDA_UL_CH2: c_int = 17;
pub const I_ADDA_UL_CH3: c_int = 18;
pub const I_ADDA_UL_CH4: c_int = 19;
pub const I_UL_PROX_CH1: c_int = 20;
pub const I_UL_PROX_CH2: c_int = 21;
pub const I_ADDA_UL_CH5: c_int = 24;
pub const I_ADDA_UL_CH6: c_int = 25;
pub const I_DMIC0_CH1: c_int = 28;
pub const I_DMIC0_CH2: c_int = 29;
pub const I_DMIC1_CH1: c_int = 30;
pub const I_DMIC1_CH2: c_int = 31;
// in port define >= 32
pub const I_32_OFFSET: c_int = 32;

// in port define >= 64
pub const I_64_OFFSET: c_int = 64;

// in port define >= 128
pub const I_128_OFFSET: c_int = 128;

// in port define >= 160
pub const I_160_OFFSET: c_int = 160;

// in port define >= 192
pub const I_192_OFFSET: c_int = 192;


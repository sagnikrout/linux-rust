//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/mt8192/mt8192-interconnection.h
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
// Mediatek MT8192 audio driver interconnection definition
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//
// in port define
pub const I_I2S0_CH1: c_int = 0;
pub const I_I2S0_CH2: c_int = 1;
pub const I_ADDA_UL_CH1: c_int = 3;
pub const I_ADDA_UL_CH2: c_int = 4;
pub const I_DL1_CH1: c_int = 5;
pub const I_DL1_CH2: c_int = 6;
pub const I_DL2_CH1: c_int = 7;
pub const I_DL2_CH2: c_int = 8;
pub const I_PCM_1_CAP_CH1: c_int = 9;
pub const I_GAIN1_OUT_CH1: c_int = 10;
pub const I_GAIN1_OUT_CH2: c_int = 11;
pub const I_GAIN2_OUT_CH1: c_int = 12;
pub const I_GAIN2_OUT_CH2: c_int = 13;
pub const I_PCM_2_CAP_CH1: c_int = 14;
pub const I_ADDA_UL_CH3: c_int = 17;
pub const I_ADDA_UL_CH4: c_int = 18;
pub const I_DL12_CH1: c_int = 19;
pub const I_DL12_CH2: c_int = 20;
pub const I_PCM_2_CAP_CH2: c_int = 21;
pub const I_PCM_1_CAP_CH2: c_int = 22;
pub const I_DL3_CH1: c_int = 23;
pub const I_DL3_CH2: c_int = 24;
pub const I_I2S2_CH1: c_int = 25;
pub const I_I2S2_CH2: c_int = 26;
pub const I_I2S2_CH3: c_int = 27;
pub const I_I2S2_CH4: c_int = 28;
// in port define >= 32
pub const I_32_OFFSET: c_int = 32;


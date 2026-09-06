//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/88pm860x-codec.h
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
// 88pm860x-codec.h -- 88PM860x ALSA SoC Audio Driver
//
// Copyright 2010 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//
pub const PM860X_PCM_IFACE_1: c_uint = 0xb0;
pub const PM860X_PCM_IFACE_2: c_uint = 0xb1;
pub const PM860X_PCM_IFACE_3: c_uint = 0xb2;
pub const PM860X_PCM_RATE: c_uint = 0xb3;
pub const PM860X_EC_PATH: c_uint = 0xb4;
pub const PM860X_SIDETONE_L_GAIN: c_uint = 0xb5;
pub const PM860X_SIDETONE_R_GAIN: c_uint = 0xb6;
pub const PM860X_SIDETONE_SHIFT: c_uint = 0xb7;
pub const PM860X_ADC_OFFSET_1: c_uint = 0xb8;
pub const PM860X_ADC_OFFSET_2: c_uint = 0xb9;
pub const PM860X_DMIC_DELAY: c_uint = 0xba;
pub const PM860X_I2S_IFACE_1: c_uint = 0xbb;
pub const PM860X_I2S_IFACE_2: c_uint = 0xbc;
pub const PM860X_I2S_IFACE_3: c_uint = 0xbd;
pub const PM860X_I2S_IFACE_4: c_uint = 0xbe;
pub const PM860X_EQUALIZER_N0_1: c_uint = 0xbf;
pub const PM860X_EQUALIZER_N0_2: c_uint = 0xc0;
pub const PM860X_EQUALIZER_N1_1: c_uint = 0xc1;
pub const PM860X_EQUALIZER_N1_2: c_uint = 0xc2;
pub const PM860X_EQUALIZER_D1_1: c_uint = 0xc3;
pub const PM860X_EQUALIZER_D1_2: c_uint = 0xc4;
pub const PM860X_LOFI_GAIN_LEFT: c_uint = 0xc5;
pub const PM860X_LOFI_GAIN_RIGHT: c_uint = 0xc6;
pub const PM860X_HIFIL_GAIN_LEFT: c_uint = 0xc7;
pub const PM860X_HIFIL_GAIN_RIGHT: c_uint = 0xc8;
pub const PM860X_HIFIR_GAIN_LEFT: c_uint = 0xc9;
pub const PM860X_HIFIR_GAIN_RIGHT: c_uint = 0xca;
pub const PM860X_DAC_OFFSET: c_uint = 0xcb;
pub const PM860X_OFFSET_LEFT_1: c_uint = 0xcc;
pub const PM860X_OFFSET_LEFT_2: c_uint = 0xcd;
pub const PM860X_OFFSET_RIGHT_1: c_uint = 0xce;
pub const PM860X_OFFSET_RIGHT_2: c_uint = 0xcf;
pub const PM860X_ADC_ANA_1: c_uint = 0xd0;
pub const PM860X_ADC_ANA_2: c_uint = 0xd1;
pub const PM860X_ADC_ANA_3: c_uint = 0xd2;
pub const PM860X_ADC_ANA_4: c_uint = 0xd3;
pub const PM860X_ANA_TO_ANA: c_uint = 0xd4;
pub const PM860X_HS1_CTRL: c_uint = 0xd5;
pub const PM860X_HS2_CTRL: c_uint = 0xd6;
pub const PM860X_LO1_CTRL: c_uint = 0xd7;
pub const PM860X_LO2_CTRL: c_uint = 0xd8;
pub const PM860X_EAR_CTRL_1: c_uint = 0xd9;
pub const PM860X_EAR_CTRL_2: c_uint = 0xda;
pub const PM860X_AUDIO_SUPPLIES_1: c_uint = 0xdb;
pub const PM860X_AUDIO_SUPPLIES_2: c_uint = 0xdc;
pub const PM860X_ADC_EN_1: c_uint = 0xdd;
pub const PM860X_ADC_EN_2: c_uint = 0xde;
pub const PM860X_DAC_EN_1: c_uint = 0xdf;
pub const PM860X_DAC_EN_2: c_uint = 0xe1;
pub const PM860X_AUDIO_CAL_1: c_uint = 0xe2;
pub const PM860X_AUDIO_CAL_2: c_uint = 0xe3;
pub const PM860X_AUDIO_CAL_3: c_uint = 0xe4;
pub const PM860X_AUDIO_CAL_4: c_uint = 0xe5;
pub const PM860X_AUDIO_CAL_5: c_uint = 0xe6;
pub const PM860X_ANA_INPUT_SEL_1: c_uint = 0xe7;
pub const PM860X_ANA_INPUT_SEL_2: c_uint = 0xe8;
pub const PM860X_PCM_IFACE_4: c_uint = 0xe9;
pub const PM860X_I2S_IFACE_5: c_uint = 0xea;
pub const PM860X_SHORTS: c_uint = 0x3b;
pub const PM860X_PLL_ADJ_1: c_uint = 0x3c;
pub const PM860X_PLL_ADJ_2: c_uint = 0x3d;
// bits definition
pub const PM860X_CLK_DIR_IN: c_int = 0;
pub const PM860X_CLK_DIR_OUT: c_int = 1;

pub const PM860X_DET_MASK: c_uint = 0x1F;

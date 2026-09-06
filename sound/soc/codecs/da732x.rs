//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/da732x.h
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
// da732x.h -- Dialog DA732X ALSA SoC Audio Driver Header File
//
// Copyright (C) 2012 Dialog Semiconductor GmbH
//
// Author: Michal Hajduk <Michal.Hajduk@diasemi.com>
//

// General
pub const DA732X_U8_MASK: c_uint = 0xFF;
pub const DA732X_4BYTES: c_int = 4;
pub const DA732X_3BYTES: c_int = 3;
pub const DA732X_2BYTES: c_int = 2;
pub const DA732X_1BYTE: c_int = 1;
pub const DA732X_1BYTE_SHIFT: c_int = 8;
pub const DA732X_2BYTES_SHIFT: c_int = 16;
pub const DA732X_3BYTES_SHIFT: c_int = 24;
pub const DA732X_4BYTES_SHIFT: c_int = 32;
pub const DA732X_DACS_DIS: c_uint = 0x0;
pub const DA732X_HP_DIS: c_uint = 0x0;
pub const DA732X_CLEAR_REG: c_uint = 0x0;
// Calibration
pub const DA732X_DAC_OFFSET_STEP: c_uint = 0x20;
pub const DA732X_OUTPUT_OFFSET_STEP: c_uint = 0x80;
pub const DA732X_HP_OUT_TRIM_VAL: c_uint = 0x0;
pub const DA732X_WAIT_FOR_STABILIZATION: c_int = 1;
pub const DA732X_HPL_DAC: c_int = 0;
pub const DA732X_HPR_DAC: c_int = 1;
pub const DA732X_HP_DACS: c_int = 2;
pub const DA732X_HPL_AMP: c_int = 0;
pub const DA732X_HPR_AMP: c_int = 1;
pub const DA732X_HP_AMPS: c_int = 2;
// Clock settings
pub const DA732X_STARTUP_DELAY: c_int = 100;
pub const DA732X_PLL_OUT_196608: c_int = 196608000;
pub const DA732X_PLL_OUT_180634: c_int = 180633600;
pub const DA732X_PLL_OUT_SRM: c_int = 188620800;
pub const DA732X_MCLK_10MHZ: c_int = 10000000;
pub const DA732X_MCLK_20MHZ: c_int = 20000000;
pub const DA732X_MCLK_40MHZ: c_int = 40000000;
pub const DA732X_MCLK_54MHZ: c_int = 54000000;
pub const DA732X_MCLK_VAL_0_10MHZ: c_int = 0;
pub const DA732X_MCLK_VAL_10_20MHZ: c_int = 1;
pub const DA732X_MCLK_VAL_20_40MHZ: c_int = 2;
pub const DA732X_MCLK_VAL_40_54MHZ: c_int = 3;
pub const DA732X_DAI_ID1: c_int = 0;
pub const DA732X_DAI_ID2: c_int = 1;
pub const DA732X_SRCCLK_PLL: c_int = 0;
pub const DA732X_SRCCLK_MCLK: c_int = 1;
pub const DA732X_LIN_LP_VOL: c_uint = 0x4F;
pub const DA732X_LP_VOL: c_uint = 0x40;
// Kcontrols
pub const DA732X_DAC_EN_MAX: c_int = 2;
pub const DA732X_ADCL_MUX_MAX: c_int = 2;
pub const DA732X_ADCR_MUX_MAX: c_int = 3;
pub const DA732X_HPF_MODE_MAX: c_int = 3;
pub const DA732X_HPF_MODE_SHIFT: c_int = 4;
pub const DA732X_HPF_MUSIC_SHIFT: c_int = 0;
pub const DA732X_HPF_MUSIC_MAX: c_int = 4;
pub const DA732X_HPF_VOICE_SHIFT: c_int = 4;
pub const DA732X_HPF_VOICE_MAX: c_int = 8;
pub const DA732X_EQ_EN_MAX: c_int = 1;
pub const DA732X_HPF_VOICE: c_int = 1;
pub const DA732X_HPF_MUSIC: c_int = 2;
pub const DA732X_HPF_DISABLED: c_int = 0;
pub const DA732X_NO_INVERT: c_int = 0;
pub const DA732X_INVERT: c_int = 1;
pub const DA732X_SWITCH_MAX: c_int = 1;
pub const DA732X_ENABLE_CP: c_int = 1;
pub const DA732X_DISABLE_CP: c_int = 0;
pub const DA732X_DISABLE_ALL_CLKS: c_int = 0;
pub const DA732X_RESET_ADCS: c_int = 0;
// dB values
pub const DA732X_MIC_VOL_DB_MIN: c_int = 0;
pub const DA732X_MIC_VOL_DB_INC: c_int = 50;
pub const DA732X_MIC_PRE_VOL_DB_MIN: c_int = 0;
pub const DA732X_MIC_PRE_VOL_DB_INC: c_int = 600;

pub const DA732X_AUX_VOL_DB_INC: c_int = 150;

pub const DA732X_HP_VOL_DB_INC: c_int = 150;

pub const DA732X_LIN2_VOL_DB_INC: c_int = 150;

pub const DA732X_LIN3_VOL_DB_INC: c_int = 150;

pub const DA732X_LIN4_VOL_DB_INC: c_int = 150;

pub const DA732X_EQ_BAND_VOL_DB_INC: c_int = 150;

pub const DA732X_DAC_VOL_DB_INC: c_int = 75;
pub const DA732X_ADC_VOL_DB_MIN: c_int = 0;

pub const DA732X_EQ_OVERALL_VOL_DB_INC: c_int = 600;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da732x_sysctl {
    DA732X_SR_8KHZ		= 0x1,
    DA732X_SR_11_025KHZ	= 0x2,
    DA732X_SR_12KHZ		= 0x3,
    DA732X_SR_16KHZ		= 0x5,
    DA732X_SR_22_05KHZ	= 0x6,
    DA732X_SR_24KHZ		= 0x7,
    DA732X_SR_32KHZ		= 0x9,
    DA732X_SR_44_1KHZ	= 0xA,
    DA732X_SR_48KHZ		= 0xB,
    DA732X_SR_88_1KHZ	= 0xE,
    DA732X_SR_96KHZ		= 0xF,
}

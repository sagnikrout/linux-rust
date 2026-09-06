//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98520.h
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
// Copyright (c) 2021, Maxim Integrated.
//
pub const MAX98520_R2000_SW_RESET: c_uint = 0x2000;
pub const MAX98520_R2001_STATUS_1: c_uint = 0x2001;
pub const MAX98520_R2002_STATUS_2: c_uint = 0x2002;
pub const MAX98520_R2020_THERM_WARN_THRESH: c_uint = 0x2020;
pub const MAX98520_R2021_THERM_SHDN_THRESH: c_uint = 0x2021;
pub const MAX98520_R2022_THERM_HYSTERESIS: c_uint = 0x2022;
pub const MAX98520_R2023_THERM_FOLDBACK_SET: c_uint = 0x2023;
pub const MAX98520_R2027_THERM_FOLDBACK_EN: c_uint = 0x2027;
pub const MAX98520_R2030_CLK_MON_CTRL: c_uint = 0x2030;
pub const MAX98520_R2037_ERR_MON_CTRL: c_uint = 0x2037;
pub const MAX98520_R2040_PCM_MODE_CFG: c_uint = 0x2040;
pub const MAX98520_R2041_PCM_CLK_SETUP: c_uint = 0x2041;
pub const MAX98520_R2042_PCM_SR_SETUP: c_uint = 0x2042;
pub const MAX98520_R2043_PCM_RX_SRC1: c_uint = 0x2043;
pub const MAX98520_R2044_PCM_RX_SRC2: c_uint = 0x2044;
pub const MAX98520_R204F_PCM_RX_EN: c_uint = 0x204F;
pub const MAX98520_R2090_AMP_VOL_CTRL: c_uint = 0x2090;
pub const MAX98520_R2091_AMP_PATH_GAIN: c_uint = 0x2091;
pub const MAX98520_R2092_AMP_DSP_CFG: c_uint = 0x2092;
pub const MAX98520_R2094_SSM_CFG: c_uint = 0x2094;
pub const MAX98520_R2095_AMP_CFG: c_uint = 0x2095;
pub const MAX98520_R209F_AMP_EN: c_uint = 0x209F;
pub const MAX98520_R20B0_ADC_SR: c_uint = 0x20B0;
pub const MAX98520_R20B1_ADC_RESOLUTION: c_uint = 0x20B1;
pub const MAX98520_R20B2_ADC_PVDD0_CFG: c_uint = 0x20B2;
pub const MAX98520_R20B3_ADC_THERMAL_CFG: c_uint = 0x20B3;
pub const MAX98520_R20B4_ADC_READBACK_CTRL: c_uint = 0x20B4;
pub const MAX98520_R20B5_ADC_READBACK_UPDATE: c_uint = 0x20B5;
pub const MAX98520_R20B6_ADC_PVDD_READBACK_MSB: c_uint = 0x20B6;
pub const MAX98520_R20B7_ADC_PVDD_READBACK_LSB: c_uint = 0x20B7;
pub const MAX98520_R20B8_ADC_TEMP_READBACK_MSB: c_uint = 0x20B8;
pub const MAX98520_R20B9_ADC_TEMP_READBACK_LSB: c_uint = 0x20B9;
pub const MAX98520_R20BA_ADC_LOW_PVDD_READBACK_MSB: c_uint = 0x20BA;
pub const MAX98520_R20BB_ADC_LOW_READBACK_LSB: c_uint = 0x20BB;
pub const MAX98520_R20BC_ADC_HIGH_TEMP_READBACK_MSB: c_uint = 0x20BC;
pub const MAX98520_R20BD_ADC_HIGH_TEMP_READBACK_LSB: c_uint = 0x20BD;
pub const MAX98520_R20CF_MEAS_ADC_CFG: c_uint = 0x20CF;
pub const MAX98520_R20D0_DHT_CFG1: c_uint = 0x20D0;
pub const MAX98520_R20D1_LIMITER_CFG1: c_uint = 0x20D1;
pub const MAX98520_R20D2_LIMITER_CFG2: c_uint = 0x20D2;
pub const MAX98520_R20D3_DHT_CFG2: c_uint = 0x20D3;
pub const MAX98520_R20D4_DHT_CFG3: c_uint = 0x20D4;
pub const MAX98520_R20D5_DHT_CFG4: c_uint = 0x20D5;
pub const MAX98520_R20D6_DHT_HYSTERESIS_CFG: c_uint = 0x20D6;
pub const MAX98520_R20D8_DHT_EN: c_uint = 0x20D8;
pub const MAX98520_R210E_AUTO_RESTART_BEHAVIOR: c_uint = 0x210E;
pub const MAX98520_R210F_GLOBAL_EN: c_uint = 0x210F;
pub const MAX98520_R2161_BOOST_TM1: c_uint = 0x2161;
pub const MAX98520_R2162_BOOST_TM2: c_uint = 0x2162;
pub const MAX98520_R2163_BOOST_TM3: c_uint = 0x2163;
pub const MAX98520_R21FF_REVISION_ID: c_uint = 0x21FF;
// MAX98520_R2030_CLK_MON_CTRL

// MAX98520_R2037_ERR_MON_CTRL

// MAX98520_R2040_PCM_MODE_CFG

// MAX98520_R2041_PCM_CLK_SETUP

// MAX98520_R2042_PCM_SR_SETUP

// MAX98520_R2044_PCM_RX_SRC2

// MAX98520_R204F_PCM_RX_EN

// MAX98520_R2092_AMP_DSP_CFG

// MAX98520_R2094_SSM_CFG

// MAX98520_R2095_AMP_CFG

// MAX98520_R20D0_DHT_CFG1

// MAX98520_R20D1_LIMITER_CFG1

// MAX98520_R20D2_DHT_CFG2

// MAX98520_R20D3_DHT_CFG2

// MAX98520_R20D6_DHT_HYSTERESIS_CFG

// MAX98520_R20B2_ADC_PVDD0_CFG, MAX98520_R20B3_ADC_THERMAL_CFG

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98520_priv {
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub ch_size: c_uint,
    pub tdm_mode: bool,
}

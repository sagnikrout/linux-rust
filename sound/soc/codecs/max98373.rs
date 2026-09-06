//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98373.h
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
// Copyright (c) 2017 Maxim Integrated
pub const MAX98373_R2000_SW_RESET: c_uint = 0x2000;
pub const MAX98373_R2001_INT_RAW1: c_uint = 0x2001;
pub const MAX98373_R2002_INT_RAW2: c_uint = 0x2002;
pub const MAX98373_R2003_INT_RAW3: c_uint = 0x2003;
pub const MAX98373_R2004_INT_STATE1: c_uint = 0x2004;
pub const MAX98373_R2005_INT_STATE2: c_uint = 0x2005;
pub const MAX98373_R2006_INT_STATE3: c_uint = 0x2006;
pub const MAX98373_R2007_INT_FLAG1: c_uint = 0x2007;
pub const MAX98373_R2008_INT_FLAG2: c_uint = 0x2008;
pub const MAX98373_R2009_INT_FLAG3: c_uint = 0x2009;
pub const MAX98373_R200A_INT_EN1: c_uint = 0x200A;
pub const MAX98373_R200B_INT_EN2: c_uint = 0x200B;
pub const MAX98373_R200C_INT_EN3: c_uint = 0x200C;
pub const MAX98373_R200D_INT_FLAG_CLR1: c_uint = 0x200D;
pub const MAX98373_R200E_INT_FLAG_CLR2: c_uint = 0x200E;
pub const MAX98373_R200F_INT_FLAG_CLR3: c_uint = 0x200F;
pub const MAX98373_R2010_IRQ_CTRL: c_uint = 0x2010;
pub const MAX98373_R2014_THERM_WARN_THRESH: c_uint = 0x2014;
pub const MAX98373_R2015_THERM_SHDN_THRESH: c_uint = 0x2015;
pub const MAX98373_R2016_THERM_HYSTERESIS: c_uint = 0x2016;
pub const MAX98373_R2017_THERM_FOLDBACK_SET: c_uint = 0x2017;
pub const MAX98373_R2018_THERM_FOLDBACK_EN: c_uint = 0x2018;
pub const MAX98373_R201E_PIN_DRIVE_STRENGTH: c_uint = 0x201E;
pub const MAX98373_R2020_PCM_TX_HIZ_EN_1: c_uint = 0x2020;
pub const MAX98373_R2021_PCM_TX_HIZ_EN_2: c_uint = 0x2021;
pub const MAX98373_R2022_PCM_TX_SRC_1: c_uint = 0x2022;
pub const MAX98373_R2023_PCM_TX_SRC_2: c_uint = 0x2023;
pub const MAX98373_R2024_PCM_DATA_FMT_CFG: c_uint = 0x2024;
pub const MAX98373_R2025_AUDIO_IF_MODE: c_uint = 0x2025;
pub const MAX98373_R2026_PCM_CLOCK_RATIO: c_uint = 0x2026;
pub const MAX98373_R2027_PCM_SR_SETUP_1: c_uint = 0x2027;
pub const MAX98373_R2028_PCM_SR_SETUP_2: c_uint = 0x2028;
pub const MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1: c_uint = 0x2029;
pub const MAX98373_R202A_PCM_TO_SPK_MONO_MIX_2: c_uint = 0x202A;
pub const MAX98373_R202B_PCM_RX_EN: c_uint = 0x202B;
pub const MAX98373_R202C_PCM_TX_EN: c_uint = 0x202C;
pub const MAX98373_R202E_ICC_RX_CH_EN_1: c_uint = 0x202E;
pub const MAX98373_R202F_ICC_RX_CH_EN_2: c_uint = 0x202F;
pub const MAX98373_R2030_ICC_TX_HIZ_EN_1: c_uint = 0x2030;
pub const MAX98373_R2031_ICC_TX_HIZ_EN_2: c_uint = 0x2031;
pub const MAX98373_R2032_ICC_LINK_EN_CFG: c_uint = 0x2032;
pub const MAX98373_R2034_ICC_TX_CNTL: c_uint = 0x2034;
pub const MAX98373_R2035_ICC_TX_EN: c_uint = 0x2035;
pub const MAX98373_R2036_SOUNDWIRE_CTRL: c_uint = 0x2036;
pub const MAX98373_R203D_AMP_DIG_VOL_CTRL: c_uint = 0x203D;
pub const MAX98373_R203E_AMP_PATH_GAIN: c_uint = 0x203E;
pub const MAX98373_R203F_AMP_DSP_CFG: c_uint = 0x203F;
pub const MAX98373_R2040_TONE_GEN_CFG: c_uint = 0x2040;
pub const MAX98373_R2041_AMP_CFG: c_uint = 0x2041;
pub const MAX98373_R2042_AMP_EDGE_RATE_CFG: c_uint = 0x2042;
pub const MAX98373_R2043_AMP_EN: c_uint = 0x2043;
pub const MAX98373_R2046_IV_SENSE_ADC_DSP_CFG: c_uint = 0x2046;
pub const MAX98373_R2047_IV_SENSE_ADC_EN: c_uint = 0x2047;
pub const MAX98373_R2051_MEAS_ADC_SAMPLING_RATE: c_uint = 0x2051;
pub const MAX98373_R2052_MEAS_ADC_PVDD_FLT_CFG: c_uint = 0x2052;
pub const MAX98373_R2053_MEAS_ADC_THERM_FLT_CFG: c_uint = 0x2053;
pub const MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK: c_uint = 0x2054;
pub const MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK: c_uint = 0x2055;
pub const MAX98373_R2056_MEAS_ADC_PVDD_CH_EN: c_uint = 0x2056;
pub const MAX98373_R2090_BDE_LVL_HOLD: c_uint = 0x2090;
pub const MAX98373_R2091_BDE_GAIN_ATK_REL_RATE: c_uint = 0x2091;
pub const MAX98373_R2092_BDE_CLIPPER_MODE: c_uint = 0x2092;
pub const MAX98373_R2097_BDE_L1_THRESH: c_uint = 0x2097;
pub const MAX98373_R2098_BDE_L2_THRESH: c_uint = 0x2098;
pub const MAX98373_R2099_BDE_L3_THRESH: c_uint = 0x2099;
pub const MAX98373_R209A_BDE_L4_THRESH: c_uint = 0x209A;
pub const MAX98373_R209B_BDE_THRESH_HYST: c_uint = 0x209B;
pub const MAX98373_R20A8_BDE_L1_CFG_1: c_uint = 0x20A8;
pub const MAX98373_R20A9_BDE_L1_CFG_2: c_uint = 0x20A9;
pub const MAX98373_R20AA_BDE_L1_CFG_3: c_uint = 0x20AA;
pub const MAX98373_R20AB_BDE_L2_CFG_1: c_uint = 0x20AB;
pub const MAX98373_R20AC_BDE_L2_CFG_2: c_uint = 0x20AC;
pub const MAX98373_R20AD_BDE_L2_CFG_3: c_uint = 0x20AD;
pub const MAX98373_R20AE_BDE_L3_CFG_1: c_uint = 0x20AE;
pub const MAX98373_R20AF_BDE_L3_CFG_2: c_uint = 0x20AF;
pub const MAX98373_R20B0_BDE_L3_CFG_3: c_uint = 0x20B0;
pub const MAX98373_R20B1_BDE_L4_CFG_1: c_uint = 0x20B1;
pub const MAX98373_R20B2_BDE_L4_CFG_2: c_uint = 0x20B2;
pub const MAX98373_R20B3_BDE_L4_CFG_3: c_uint = 0x20B3;
pub const MAX98373_R20B4_BDE_INFINITE_HOLD_RELEASE: c_uint = 0x20B4;
pub const MAX98373_R20B5_BDE_EN: c_uint = 0x20B5;
pub const MAX98373_R20B6_BDE_CUR_STATE_READBACK: c_uint = 0x20B6;
pub const MAX98373_R20D1_DHT_CFG: c_uint = 0x20D1;
pub const MAX98373_R20D2_DHT_ATTACK_CFG: c_uint = 0x20D2;
pub const MAX98373_R20D3_DHT_RELEASE_CFG: c_uint = 0x20D3;
pub const MAX98373_R20D4_DHT_EN: c_uint = 0x20D4;
pub const MAX98373_R20E0_LIMITER_THRESH_CFG: c_uint = 0x20E0;
pub const MAX98373_R20E1_LIMITER_ATK_REL_RATES: c_uint = 0x20E1;
pub const MAX98373_R20E2_LIMITER_EN: c_uint = 0x20E2;
pub const MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG: c_uint = 0x20FE;
pub const MAX98373_R20FF_GLOBAL_SHDN: c_uint = 0x20FF;
pub const MAX98373_R21FF_REV_ID: c_uint = 0x21FF;
// MAX98373_R2022_PCM_TX_SRC_1

// MAX98373_R2024_PCM_DATA_FMT_CFG

// MAX98373_R2026_PCM_CLOCK_RATIO

// MAX98373_R2027_PCM_SR_SETUP_1

// MAX98373_R2028_PCM_SR_SETUP_2

// MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1

// MAX98373_R203E_AMP_PATH_GAIN

// MAX98373_R203F_AMP_DSP_CFG

// MAX98373_R2043_AMP_EN

// MAX98373_R2052_MEAS_ADC_PVDD_FLT_CFG

// MAX98373_R20B2_BDE_L4_CFG_2

// MAX98373_R20B5_BDE_EN

// MAX98373_R20D1_DHT_CFG

// MAX98373_R20D2_DHT_ATTACK_CFG

// MAX98373_R20D3_DHT_RELEASE_CFG

// MAX98373_R20D4_DHT_EN

// MAX98373_R20E0_LIMITER_THRESH_CFG

// MAX98373_R20E2_LIMITER_EN

// MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG

// MAX98373_R20FF_GLOBAL_SHDN

// MAX98373_R2000_SW_RESET

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98373_cache {
    pub reg: u32,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98373_priv {
    pub regmap: *mut regmap,
    pub reset: *mut gpio_desc,
    pub v_slot: c_uint,
    pub i_slot: c_uint,
    pub spkfb_slot: c_uint,
    pub interleave_mode: bool,
    pub ch_size: c_uint,
    pub tdm_mode: bool,
// cache for reading a valid fake feedback value
    pub cache: *mut max98373_cache,
    pub cache_num: c_int,
// variables to support soundwire
    pub slave: *mut sdw_slave,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub slot: c_int,
    pub rx_mask: c_uint,
}

extern "C" {
    pub fn max98373_reset(max98373: *mut max98373_priv, dev: *mut device);
}

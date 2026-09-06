//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98388.h
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
// max98388.h -- MAX98388 ALSA SoC audio driver header
//
// Copyright(c) 2022, Analog Devices Inc.
//
// Device Status Registers
pub const MAX98388_R2000_SW_RESET: c_uint = 0x2000;
pub const MAX98388_R2001_INT_RAW1: c_uint = 0x2001;
pub const MAX98388_R2002_INT_RAW2: c_uint = 0x2002;
pub const MAX98388_R2004_INT_STATE1: c_uint = 0x2004;
pub const MAX98388_R2005_INT_STATE2: c_uint = 0x2005;
// Thermal Protection Registers
pub const MAX98388_R2020_THERM_WARN_THRESH: c_uint = 0x2020;
// Error Monitor
pub const MAX98388_R2031_SPK_MON_THRESH: c_uint = 0x2031;
pub const MAX98388_R2032_SPK_MON_LD_SEL: c_uint = 0x2032;
pub const MAX98388_R2033_SPK_MON_DURATION: c_uint = 0x2033;
pub const MAX98388_R2037_ERR_MON_CTRL: c_uint = 0x2037;
// PCM Registers
pub const MAX98388_R2040_PCM_MODE_CFG: c_uint = 0x2040;
pub const MAX98388_R2041_PCM_CLK_SETUP: c_uint = 0x2041;
pub const MAX98388_R2042_PCM_SR_SETUP: c_uint = 0x2042;
pub const MAX98388_R2044_PCM_TX_CTRL1: c_uint = 0x2044;
pub const MAX98388_R2045_PCM_TX_CTRL2: c_uint = 0x2045;
pub const MAX98388_R2050_PCM_TX_HIZ_CTRL1: c_uint = 0x2050;
pub const MAX98388_R2051_PCM_TX_HIZ_CTRL2: c_uint = 0x2051;
pub const MAX98388_R2052_PCM_TX_HIZ_CTRL3: c_uint = 0x2052;
pub const MAX98388_R2053_PCM_TX_HIZ_CTRL4: c_uint = 0x2053;
pub const MAX98388_R2054_PCM_TX_HIZ_CTRL5: c_uint = 0x2054;
pub const MAX98388_R2055_PCM_TX_HIZ_CTRL6: c_uint = 0x2055;
pub const MAX98388_R2056_PCM_TX_HIZ_CTRL7: c_uint = 0x2056;
pub const MAX98388_R2057_PCM_TX_HIZ_CTRL8: c_uint = 0x2057;
pub const MAX98388_R2058_PCM_RX_SRC1: c_uint = 0x2058;
pub const MAX98388_R2059_PCM_RX_SRC2: c_uint = 0x2059;
pub const MAX98388_R205C_PCM_TX_DRIVE_STRENGTH: c_uint = 0x205C;
pub const MAX98388_R205D_PCM_TX_SRC_EN: c_uint = 0x205D;
pub const MAX98388_R205E_PCM_RX_EN: c_uint = 0x205E;
pub const MAX98388_R205F_PCM_TX_EN: c_uint = 0x205F;
// Speaker Channel Control
pub const MAX98388_R2090_SPK_CH_VOL_CTRL: c_uint = 0x2090;
pub const MAX98388_R2091_SPK_CH_CFG: c_uint = 0x2091;
pub const MAX98388_R2092_SPK_AMP_OUT_CFG: c_uint = 0x2092;
pub const MAX98388_R2093_SPK_AMP_SSM_CFG: c_uint = 0x2093;
pub const MAX98388_R2094_SPK_AMP_ER_CTRL: c_uint = 0x2094;
pub const MAX98388_R209E_SPK_CH_PINK_NOISE_EN: c_uint = 0x209E;
pub const MAX98388_R209F_SPK_CH_AMP_EN: c_uint = 0x209F;
pub const MAX98388_R20A0_IV_DATA_DSP_CTRL: c_uint = 0x20A0;
pub const MAX98388_R20A7_IV_DATA_EN: c_uint = 0x20A7;
pub const MAX98388_R20E0_BP_ALC_THRESH: c_uint = 0x20E0;
pub const MAX98388_R20E1_BP_ALC_RATES: c_uint = 0x20E1;
pub const MAX98388_R20E2_BP_ALC_ATTEN: c_uint = 0x20E2;
pub const MAX98388_R20E3_BP_ALC_REL: c_uint = 0x20E3;
pub const MAX98388_R20E4_BP_ALC_MUTE: c_uint = 0x20E4;
pub const MAX98388_R20EE_BP_INF_HOLD_REL: c_uint = 0x20EE;
pub const MAX98388_R20EF_BP_ALC_EN: c_uint = 0x20EF;
pub const MAX98388_R210E_AUTO_RESTART: c_uint = 0x210E;
pub const MAX98388_R210F_GLOBAL_EN: c_uint = 0x210F;
pub const MAX98388_R22FF_REV_ID: c_uint = 0x22FF;
// MAX98388_R2000_SW_RESET

// MAX98388_R2020_THERM_WARN_THRESH

// MAX98388_R2022_PCM_TX_SRC_1

// MAX98388_R2024_PCM_DATA_FMT_CFG

// MAX98388_R2031_SPK_MON_THRESH

// MAX98388_R2032_SPK_MON_LD_SEL

// MAX98388_R2033_SPK_MON_DURATION

// MAX98388_R2037_ERR_MON_CTRL

// MAX98388_R203E_AMP_PATH_GAIN

// MAX98388_R2041_PCM_CLK_SETUP

// MAX98388_R2042_PCM_SR_SETUP

// MAX98388_R2043_AMP_EN

// MAX98388_R2052_MEAS_ADC_PVDD_FLT_CFG

// MAX98388_R2058_PCM_RX_SRC1

// MAX98388_R2059_PCM_RX_SRC2

// MAX98388_R2091_SPK_CH_CFG

// MAX98388_R2092_SPK_AMP_OUT_CFG

// MAX98388_R2093_SPK_AMP_SSM_CFG

// MAX98388_R2094_SPK_AMP_ER_CTRL

// MAX98388_R209E_SPK_CH_PINK_NOISE_EN

// MAX98388_R20A0_IV_DATA_DSP_CTRL

// MAX98388_R20B2_BDE_L4_CFG_2

// MAX98388_R20B5_BDE_EN

// MAX98388_R20D1_DHT_CFG

// MAX98388_R20D2_DHT_ATTACK_CFG

// MAX98388_R20D3_DHT_RELEASE_CFG

// MAX98388_R20D4_DHT_EN

// MAX98388_R20E0_BP_ALC_THRESH

// MAX98388_R20E1_BP_ALC_RATES

// MAX98388_R20E2_BP_ALC_ATTEN

// MAX98388_R20E3_BP_ALC_REL

// MAX98388_R20E4_BP_ALC_MUTE

// MAX98388_R210E_AUTO_RESTART

// MAX98388_R210F_GLOBAL_EN

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98388_priv {
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub v_slot: c_uint,
    pub i_slot: c_uint,
    pub spkfb_slot: c_uint,
    pub interleave_mode: bool,
    pub ch_size: c_uint,
    pub tdm_mode: bool,
}

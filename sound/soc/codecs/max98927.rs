//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98927.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// max98927.h  --  MAX98927 ALSA Soc Audio driver
//
// Copyright (C) 2016-2017 Maxim Integrated Products
// Author: Ryan Lee <ryans.lee@maximintegrated.com>
//
// Register Values
pub const MAX98927_R0001_INT_RAW1: c_uint = 0x0001;
pub const MAX98927_R0002_INT_RAW2: c_uint = 0x0002;
pub const MAX98927_R0003_INT_RAW3: c_uint = 0x0003;
pub const MAX98927_R0004_INT_STATE1: c_uint = 0x0004;
pub const MAX98927_R0005_INT_STATE2: c_uint = 0x0005;
pub const MAX98927_R0006_INT_STATE3: c_uint = 0x0006;
pub const MAX98927_R0007_INT_FLAG1: c_uint = 0x0007;
pub const MAX98927_R0008_INT_FLAG2: c_uint = 0x0008;
pub const MAX98927_R0009_INT_FLAG3: c_uint = 0x0009;
pub const MAX98927_R000A_INT_EN1: c_uint = 0x000A;
pub const MAX98927_R000B_INT_EN2: c_uint = 0x000B;
pub const MAX98927_R000C_INT_EN3: c_uint = 0x000C;
pub const MAX98927_R000D_INT_FLAG_CLR1: c_uint = 0x000D;
pub const MAX98927_R000E_INT_FLAG_CLR2: c_uint = 0x000E;
pub const MAX98927_R000F_INT_FLAG_CLR3: c_uint = 0x000F;
pub const MAX98927_R0010_IRQ_CTRL: c_uint = 0x0010;
pub const MAX98927_R0011_CLK_MON: c_uint = 0x0011;
pub const MAX98927_R0012_WDOG_CTRL: c_uint = 0x0012;
pub const MAX98927_R0013_WDOG_RST: c_uint = 0x0013;
pub const MAX98927_R0014_MEAS_ADC_THERM_WARN_THRESH: c_uint = 0x0014;
pub const MAX98927_R0015_MEAS_ADC_THERM_SHDN_THRESH: c_uint = 0x0015;
pub const MAX98927_R0016_MEAS_ADC_THERM_HYSTERESIS: c_uint = 0x0016;
pub const MAX98927_R0017_PIN_CFG: c_uint = 0x0017;
pub const MAX98927_R0018_PCM_RX_EN_A: c_uint = 0x0018;
pub const MAX98927_R0019_PCM_RX_EN_B: c_uint = 0x0019;
pub const MAX98927_R001A_PCM_TX_EN_A: c_uint = 0x001A;
pub const MAX98927_R001B_PCM_TX_EN_B: c_uint = 0x001B;
pub const MAX98927_R001C_PCM_TX_HIZ_CTRL_A: c_uint = 0x001C;
pub const MAX98927_R001D_PCM_TX_HIZ_CTRL_B: c_uint = 0x001D;
pub const MAX98927_R001E_PCM_TX_CH_SRC_A: c_uint = 0x001E;
pub const MAX98927_R001F_PCM_TX_CH_SRC_B: c_uint = 0x001F;
pub const MAX98927_R0020_PCM_MODE_CFG: c_uint = 0x0020;
pub const MAX98927_R0021_PCM_MASTER_MODE: c_uint = 0x0021;
pub const MAX98927_R0022_PCM_CLK_SETUP: c_uint = 0x0022;
pub const MAX98927_R0023_PCM_SR_SETUP1: c_uint = 0x0023;
pub const MAX98927_R0024_PCM_SR_SETUP2: c_uint = 0x0024;
pub const MAX98927_R0025_PCM_TO_SPK_MONOMIX_A: c_uint = 0x0025;
pub const MAX98927_R0026_PCM_TO_SPK_MONOMIX_B: c_uint = 0x0026;
pub const MAX98927_R0027_ICC_RX_EN_A: c_uint = 0x0027;
pub const MAX98927_R0028_ICC_RX_EN_B: c_uint = 0x0028;
pub const MAX98927_R002B_ICC_TX_EN_A: c_uint = 0x002B;
pub const MAX98927_R002C_ICC_TX_EN_B: c_uint = 0x002C;
pub const MAX98927_R002E_ICC_HIZ_MANUAL_MODE: c_uint = 0x002E;
pub const MAX98927_R002F_ICC_TX_HIZ_EN_A: c_uint = 0x002F;
pub const MAX98927_R0030_ICC_TX_HIZ_EN_B: c_uint = 0x0030;
pub const MAX98927_R0031_ICC_LNK_EN: c_uint = 0x0031;
pub const MAX98927_R0032_PDM_TX_EN: c_uint = 0x0032;
pub const MAX98927_R0033_PDM_TX_HIZ_CTRL: c_uint = 0x0033;
pub const MAX98927_R0034_PDM_TX_CTRL: c_uint = 0x0034;
pub const MAX98927_R0035_PDM_RX_CTRL: c_uint = 0x0035;
pub const MAX98927_R0036_AMP_VOL_CTRL: c_uint = 0x0036;
pub const MAX98927_R0037_AMP_DSP_CFG: c_uint = 0x0037;
pub const MAX98927_R0038_TONE_GEN_DC_CFG: c_uint = 0x0038;
pub const MAX98927_R0039_DRE_CTRL: c_uint = 0x0039;
pub const MAX98927_R003A_AMP_EN: c_uint = 0x003A;
pub const MAX98927_R003B_SPK_SRC_SEL: c_uint = 0x003B;
pub const MAX98927_R003C_SPK_GAIN: c_uint = 0x003C;
pub const MAX98927_R003D_SSM_CFG: c_uint = 0x003D;
pub const MAX98927_R003E_MEAS_EN: c_uint = 0x003E;
pub const MAX98927_R003F_MEAS_DSP_CFG: c_uint = 0x003F;
pub const MAX98927_R0040_BOOST_CTRL0: c_uint = 0x0040;
pub const MAX98927_R0041_BOOST_CTRL3: c_uint = 0x0041;
pub const MAX98927_R0042_BOOST_CTRL1: c_uint = 0x0042;
pub const MAX98927_R0043_MEAS_ADC_CFG: c_uint = 0x0043;
pub const MAX98927_R0044_MEAS_ADC_BASE_MSB: c_uint = 0x0044;
pub const MAX98927_R0045_MEAS_ADC_BASE_LSB: c_uint = 0x0045;
pub const MAX98927_R0046_ADC_CH0_DIVIDE: c_uint = 0x0046;
pub const MAX98927_R0047_ADC_CH1_DIVIDE: c_uint = 0x0047;
pub const MAX98927_R0048_ADC_CH2_DIVIDE: c_uint = 0x0048;
pub const MAX98927_R0049_ADC_CH0_FILT_CFG: c_uint = 0x0049;
pub const MAX98927_R004A_ADC_CH1_FILT_CFG: c_uint = 0x004A;
pub const MAX98927_R004B_ADC_CH2_FILT_CFG: c_uint = 0x004B;
pub const MAX98927_R004C_MEAS_ADC_CH0_READ: c_uint = 0x004C;
pub const MAX98927_R004D_MEAS_ADC_CH1_READ: c_uint = 0x004D;
pub const MAX98927_R004E_MEAS_ADC_CH2_READ: c_uint = 0x004E;
pub const MAX98927_R0051_BROWNOUT_STATUS: c_uint = 0x0051;
pub const MAX98927_R0052_BROWNOUT_EN: c_uint = 0x0052;
pub const MAX98927_R0053_BROWNOUT_INFINITE_HOLD: c_uint = 0x0053;
pub const MAX98927_R0054_BROWNOUT_INFINITE_HOLD_CLR: c_uint = 0x0054;
pub const MAX98927_R0055_BROWNOUT_LVL_HOLD: c_uint = 0x0055;
pub const MAX98927_R005A_BROWNOUT_LVL1_THRESH: c_uint = 0x005A;
pub const MAX98927_R005B_BROWNOUT_LVL2_THRESH: c_uint = 0x005B;
pub const MAX98927_R005C_BROWNOUT_LVL3_THRESH: c_uint = 0x005C;
pub const MAX98927_R005D_BROWNOUT_LVL4_THRESH: c_uint = 0x005D;
pub const MAX98927_R005E_BROWNOUT_THRESH_HYSTERYSIS: c_uint = 0x005E;
pub const MAX98927_R005F_BROWNOUT_AMP_LIMITER_ATK_REL: c_uint = 0x005F;
pub const MAX98927_R0060_BROWNOUT_AMP_GAIN_ATK_REL: c_uint = 0x0060;
pub const MAX98927_R0061_BROWNOUT_AMP1_CLIP_MODE: c_uint = 0x0061;
pub const MAX98927_R0072_BROWNOUT_LVL1_CUR_LIMIT: c_uint = 0x0072;
pub const MAX98927_R0073_BROWNOUT_LVL1_AMP1_CTRL1: c_uint = 0x0073;
pub const MAX98927_R0074_BROWNOUT_LVL1_AMP1_CTRL2: c_uint = 0x0074;
pub const MAX98927_R0075_BROWNOUT_LVL1_AMP1_CTRL3: c_uint = 0x0075;
pub const MAX98927_R0076_BROWNOUT_LVL2_CUR_LIMIT: c_uint = 0x0076;
pub const MAX98927_R0077_BROWNOUT_LVL2_AMP1_CTRL1: c_uint = 0x0077;
pub const MAX98927_R0078_BROWNOUT_LVL2_AMP1_CTRL2: c_uint = 0x0078;
pub const MAX98927_R0079_BROWNOUT_LVL2_AMP1_CTRL3: c_uint = 0x0079;
pub const MAX98927_R007A_BROWNOUT_LVL3_CUR_LIMIT: c_uint = 0x007A;
pub const MAX98927_R007B_BROWNOUT_LVL3_AMP1_CTRL1: c_uint = 0x007B;
pub const MAX98927_R007C_BROWNOUT_LVL3_AMP1_CTRL2: c_uint = 0x007C;
pub const MAX98927_R007D_BROWNOUT_LVL3_AMP1_CTRL3: c_uint = 0x007D;
pub const MAX98927_R007E_BROWNOUT_LVL4_CUR_LIMIT: c_uint = 0x007E;
pub const MAX98927_R007F_BROWNOUT_LVL4_AMP1_CTRL1: c_uint = 0x007F;
pub const MAX98927_R0080_BROWNOUT_LVL4_AMP1_CTRL2: c_uint = 0x0080;
pub const MAX98927_R0081_BROWNOUT_LVL4_AMP1_CTRL3: c_uint = 0x0081;
pub const MAX98927_R0082_ENV_TRACK_VOUT_HEADROOM: c_uint = 0x0082;
pub const MAX98927_R0083_ENV_TRACK_BOOST_VOUT_DELAY: c_uint = 0x0083;
pub const MAX98927_R0084_ENV_TRACK_REL_RATE: c_uint = 0x0084;
pub const MAX98927_R0085_ENV_TRACK_HOLD_RATE: c_uint = 0x0085;
pub const MAX98927_R0086_ENV_TRACK_CTRL: c_uint = 0x0086;
pub const MAX98927_R0087_ENV_TRACK_BOOST_VOUT_READ: c_uint = 0x0087;
pub const MAX98927_R00FF_GLOBAL_SHDN: c_uint = 0x00FF;
pub const MAX98927_R0100_SOFT_RESET: c_uint = 0x0100;
pub const MAX98927_R01FF_REV_ID: c_uint = 0x01FF;
// MAX98927_R0018_PCM_RX_EN_A

// MAX98927_R001A_PCM_TX_EN_A

// MAX98927_R001E_PCM_TX_CH_SRC_A

// MAX98927_R001F_PCM_TX_CH_SRC_B

// MAX98927_R0020_PCM_MODE_CFG

// MAX98927_R0021_PCM_MASTER_MODE

// MAX98927_R0022_PCM_CLK_SETUP

// MAX98927_R0023_PCM_SR_SETUP1

// MAX98927_R0024_PCM_SR_SETUP2

// MAX98927_R0025_PCM_TO_SPK_MONOMIX_A

// MAX98927_R0035_PDM_RX_CTRL

// MAX98927_R0036_AMP_VOL_CTRL

// MAX98927_R0037_AMP_DSP_CFG

// MAX98927_R0039_DRE_CTRL

pub const MAX98927_DRE_EN_SHIFT: c_uint = 0x1;
// MAX98927_R003A_AMP_EN

// MAX98927_R003B_SPK_SRC_SEL

// MAX98927_R003C_SPK_GAIN

// MAX98927_R003E_MEAS_EN

// MAX98927_R0040_BOOST_CTRL0

// MAX98927_R0052_BROWNOUT_EN

// MAX98927_R0100_SOFT_RESET

// MAX98927_R00FF_GLOBAL_SHDN

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max98927_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub pdata: *mut max98927_pdata,
    pub reset_gpio: *mut gpio_desc,
    pub spk_gain: c_uint,
    pub sysclk: c_uint,
    pub v_l_slot: c_uint,
    pub i_l_slot: c_uint,
    pub interleave_mode: bool,
    pub ch_size: c_uint,
    pub rate: c_uint,
    pub iface: c_uint,
    pub provider: c_uint,
    pub digital_gain: c_uint,
    pub tdm_mode: bool,
}

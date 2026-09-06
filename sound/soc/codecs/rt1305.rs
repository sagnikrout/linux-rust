//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1305.h
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
// RT1305.h  --  RT1305 ALSA SoC amplifier component driver
//
// Copyright 2018 Realtek Semiconductor Corp.
// Author: Shuming Fan <shumingf@realtek.com>
//
pub const RT1305_DEVICE_ID_NUM: c_uint = 0x6251;
pub const RT1305_RESET: c_uint = 0x00;
pub const RT1305_CLK_1: c_uint = 0x04;
pub const RT1305_CLK_2: c_uint = 0x05;
pub const RT1305_CLK_3: c_uint = 0x06;
pub const RT1305_DFLL_REG: c_uint = 0x07;
pub const RT1305_CAL_EFUSE_CLOCK: c_uint = 0x08;
pub const RT1305_PLL0_1: c_uint = 0x0a;
pub const RT1305_PLL0_2: c_uint = 0x0b;
pub const RT1305_PLL1_1: c_uint = 0x0c;
pub const RT1305_PLL1_2: c_uint = 0x0d;
pub const RT1305_MIXER_CTRL_1: c_uint = 0x10;
pub const RT1305_MIXER_CTRL_2: c_uint = 0x11;
pub const RT1305_DAC_SET_1: c_uint = 0x12;
pub const RT1305_DAC_SET_2: c_uint = 0x14;
pub const RT1305_ADC_SET_1: c_uint = 0x16;
pub const RT1305_ADC_SET_2: c_uint = 0x17;
pub const RT1305_ADC_SET_3: c_uint = 0x18;
pub const RT1305_PATH_SET: c_uint = 0x20;
pub const RT1305_SPDIF_IN_SET_1: c_uint = 0x22;
pub const RT1305_SPDIF_IN_SET_2: c_uint = 0x24;
pub const RT1305_SPDIF_IN_SET_3: c_uint = 0x26;
pub const RT1305_SPDIF_OUT_SET_1: c_uint = 0x28;
pub const RT1305_SPDIF_OUT_SET_2: c_uint = 0x2a;
pub const RT1305_SPDIF_OUT_SET_3: c_uint = 0x2b;
pub const RT1305_I2S_SET_1: c_uint = 0x2d;
pub const RT1305_I2S_SET_2: c_uint = 0x2e;
pub const RT1305_PBTL_MONO_MODE_SRC: c_uint = 0x2f;
pub const RT1305_MANUALLY_I2C_DEVICE: c_uint = 0x32;
pub const RT1305_POWER_STATUS: c_uint = 0x39;
pub const RT1305_POWER_CTRL_1: c_uint = 0x3a;
pub const RT1305_POWER_CTRL_2: c_uint = 0x3b;
pub const RT1305_POWER_CTRL_3: c_uint = 0x3c;
pub const RT1305_POWER_CTRL_4: c_uint = 0x3d;
pub const RT1305_POWER_CTRL_5: c_uint = 0x3e;
pub const RT1305_CLOCK_DETECT: c_uint = 0x3f;
pub const RT1305_BIQUAD_SET_1: c_uint = 0x40;
pub const RT1305_BIQUAD_SET_2: c_uint = 0x42;
pub const RT1305_ADJUSTED_HPF_1: c_uint = 0x46;
pub const RT1305_ADJUSTED_HPF_2: c_uint = 0x47;
pub const RT1305_EQ_SET_1: c_uint = 0x4b;
pub const RT1305_EQ_SET_2: c_uint = 0x4c;
pub const RT1305_SPK_TEMP_PROTECTION_0: c_uint = 0x4f;
pub const RT1305_SPK_TEMP_PROTECTION_1: c_uint = 0x50;
pub const RT1305_SPK_TEMP_PROTECTION_2: c_uint = 0x51;
pub const RT1305_SPK_TEMP_PROTECTION_3: c_uint = 0x52;
pub const RT1305_SPK_DC_DETECT_1: c_uint = 0x53;
pub const RT1305_SPK_DC_DETECT_2: c_uint = 0x54;
pub const RT1305_LOUDNESS: c_uint = 0x58;
pub const RT1305_THERMAL_FOLD_BACK_1: c_uint = 0x5e;
pub const RT1305_THERMAL_FOLD_BACK_2: c_uint = 0x5f;
pub const RT1305_SILENCE_DETECT: c_uint = 0x60;
pub const RT1305_ALC_DRC_1: c_uint = 0x62;
pub const RT1305_ALC_DRC_2: c_uint = 0x63;
pub const RT1305_ALC_DRC_3: c_uint = 0x64;
pub const RT1305_ALC_DRC_4: c_uint = 0x65;
pub const RT1305_PRIV_INDEX: c_uint = 0x6a;
pub const RT1305_PRIV_DATA: c_uint = 0x6c;
pub const RT1305_SPK_EXCURSION_LIMITER_7: c_uint = 0x76;
pub const RT1305_VERSION_ID: c_uint = 0x7a;
pub const RT1305_VENDOR_ID: c_uint = 0x7c;
pub const RT1305_DEVICE_ID: c_uint = 0x7e;
pub const RT1305_EFUSE_1: c_uint = 0x80;
pub const RT1305_EFUSE_2: c_uint = 0x81;
pub const RT1305_EFUSE_3: c_uint = 0x82;
pub const RT1305_DC_CALIB_1: c_uint = 0x90;
pub const RT1305_DC_CALIB_2: c_uint = 0x91;
pub const RT1305_DC_CALIB_3: c_uint = 0x92;
pub const RT1305_DAC_OFFSET_1: c_uint = 0x93;
pub const RT1305_DAC_OFFSET_2: c_uint = 0x94;
pub const RT1305_DAC_OFFSET_3: c_uint = 0x95;
pub const RT1305_DAC_OFFSET_4: c_uint = 0x96;
pub const RT1305_DAC_OFFSET_5: c_uint = 0x97;
pub const RT1305_DAC_OFFSET_6: c_uint = 0x98;
pub const RT1305_DAC_OFFSET_7: c_uint = 0x99;
pub const RT1305_DAC_OFFSET_8: c_uint = 0x9a;
pub const RT1305_DAC_OFFSET_9: c_uint = 0x9b;
pub const RT1305_DAC_OFFSET_10: c_uint = 0x9c;
pub const RT1305_DAC_OFFSET_11: c_uint = 0x9d;
pub const RT1305_DAC_OFFSET_12: c_uint = 0x9e;
pub const RT1305_DAC_OFFSET_13: c_uint = 0x9f;
pub const RT1305_DAC_OFFSET_14: c_uint = 0xa0;
pub const RT1305_TRIM_1: c_uint = 0xb0;
pub const RT1305_TRIM_2: c_uint = 0xb1;
pub const RT1305_TUNE_INTERNAL_OSC: c_uint = 0xb2;
pub const RT1305_BIQUAD1_H0_L_28_16: c_uint = 0xc0;
pub const RT1305_BIQUAD3_A2_R_15_0: c_uint = 0xfb;
pub const RT1305_MAX_REG: c_uint = 0xff;
// CLOCK-1 (0x04)

pub const RT1305_SEL_PLL_SRC_2_SFT: c_int = 15;

pub const RT1305_DIV_PLL_SRC_2_SFT: c_int = 13;

pub const RT1305_SEL_PLL_SRC_1_SFT: c_int = 10;

pub const RT1305_SEL_FS_SYS_PRE_SFT: c_int = 8;

pub const RT1305_DIV_FS_SYS_SFT: c_int = 4;
// PLL1M/N/K Code-1 (0x0c)
pub const RT1305_PLL_1_M_SFT: c_int = 12;

pub const RT1305_PLL_1_M_BYPASS_SFT: c_int = 11;

// DAC Setting (0x14)
pub const RT1305_DVOL_MUTE_L_EN_SFT: c_int = 15;
pub const RT1305_DVOL_MUTE_R_EN_SFT: c_int = 14;
// I2S Setting-1 (0x2d)

pub const RT1305_SEL_I2S_OUT_MODE_SFT: c_int = 15;

// I2S Setting-2 (0x2e)

pub const RT1305_I2S_DF_SEL_SFT: c_int = 12;

pub const RT1305_I2S_DL_SEL_SFT: c_int = 10;

pub const RT1305_I2S_BCLK_SFT: c_int = 9;

// Power Control-1 (0x3a)

pub const RT1305_POW_PDB_JD_BIT: c_int = 12;

pub const RT1305_POW_PLL0_EN_BIT: c_int = 11;

pub const RT1305_POW_PLL1_EN_BIT: c_int = 10;

pub const RT1305_POW_PDB_JD_POLARITY_BIT: c_int = 9;

pub const RT1305_POW_MBIAS_LV_BIT: c_int = 8;

pub const RT1305_POW_BG_MBIAS_LV_BIT: c_int = 7;

pub const RT1305_POW_LDO2_BIT: c_int = 6;

pub const RT1305_POW_BG2_BIT: c_int = 5;

pub const RT1305_POW_LDO2_IB2_BIT: c_int = 4;

pub const RT1305_POW_VREF_BIT: c_int = 3;

pub const RT1305_POW_VREF1_BIT: c_int = 2;

pub const RT1305_POW_VREF2_BIT: c_int = 1;
// Power Control-2 (0x3b)

pub const RT1305_POW_DISC_VREF_BIT: c_int = 15;

pub const RT1305_POW_FASTB_VREF_BIT: c_int = 14;

pub const RT1305_POW_ULTRA_FAST_VREF_BIT: c_int = 13;

pub const RT1305_POW_CKXEN_DAC_BIT: c_int = 12;

pub const RT1305_POW_EN_CKGEN_DAC_BIT: c_int = 11;

pub const RT1305_POW_DAC1_L_BIT: c_int = 10;

pub const RT1305_POW_DAC1_R_BIT: c_int = 9;

pub const RT1305_POW_CLAMP_BIT: c_int = 8;

pub const RT1305_POW_BUFL_BIT: c_int = 7;

pub const RT1305_POW_BUFR_BIT: c_int = 6;

pub const RT1305_POW_EN_CKGEN_ADC_BIT: c_int = 5;

pub const RT1305_POW_ADC3_L_BIT: c_int = 4;

pub const RT1305_POW_ADC3_R_BIT: c_int = 3;

pub const RT1305_POW_TRIOSC_BIT: c_int = 2;

pub const RT1305_POR_AVDD1_BIT: c_int = 1;

pub const RT1305_POR_AVDD2_BIT: c_int = 0;
// Power Control-3 (0x3c)

pub const RT1305_POW_VSENSE_RCH_BIT: c_int = 15;

pub const RT1305_POW_VSENSE_LCH_BIT: c_int = 14;

pub const RT1305_POW_ISENSE_RCH_BIT: c_int = 13;

pub const RT1305_POW_ISENSE_LCH_BIT: c_int = 12;

pub const RT1305_POW_POR_AVDD1_BIT: c_int = 11;

pub const RT1305_POW_POR_AVDD2_BIT: c_int = 10;

pub const RT1305_EN_K_HV_BIT: c_int = 9;

pub const RT1305_EN_PRE_K_HV_BIT: c_int = 8;

pub const RT1305_EN_EFUSE_1P8V_BIT: c_int = 7;

pub const RT1305_EN_EFUSE_5V_BIT: c_int = 6;

pub const RT1305_EN_VCM_6172_BIT: c_int = 5;

pub const RT1305_POR_EFUSE_BIT: c_int = 4;
// Clock Detect (0x3f)

pub const RT1305_SEL_CLK_DET_SRC_SFT: c_int = 12;

// System Clock Source
// PLL Source 1/2
pub const R0_UPPER: c_uint = 0x2E8BA2 //5.5 ohm;
pub const R0_LOWER: c_uint = 0x666666 //2.5 ohm;

//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt5514.h
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
// rt5514.h  --  RT5514 ALSA SoC audio driver
//
// Copyright 2015 Realtek Microelectronics
// Author: Oder Chiou <oder_chiou@realtek.com>
//

pub const RT5514_DEVICE_ID: c_uint = 0x10ec5514;
pub const RT5514_RESET: c_uint = 0x2000;
pub const RT5514_PWR_ANA1: c_uint = 0x2004;
pub const RT5514_PWR_ANA2: c_uint = 0x2008;
pub const RT5514_I2S_CTRL1: c_uint = 0x2010;
pub const RT5514_I2S_CTRL2: c_uint = 0x2014;
pub const RT5514_VAD_CTRL6: c_uint = 0x2030;
pub const RT5514_EXT_VAD_CTRL: c_uint = 0x206c;
pub const RT5514_DIG_IO_CTRL: c_uint = 0x2070;
pub const RT5514_PAD_CTRL1: c_uint = 0x2080;
pub const RT5514_DMIC_DATA_CTRL: c_uint = 0x20a0;
pub const RT5514_DIG_SOURCE_CTRL: c_uint = 0x20a4;
pub const RT5514_SRC_CTRL: c_uint = 0x20ac;
pub const RT5514_DOWNFILTER2_CTRL1: c_uint = 0x20d0;
pub const RT5514_PLL_SOURCE_CTRL: c_uint = 0x2100;
pub const RT5514_CLK_CTRL1: c_uint = 0x2104;
pub const RT5514_CLK_CTRL2: c_uint = 0x2108;
pub const RT5514_PLL3_CALIB_CTRL1: c_uint = 0x2110;
pub const RT5514_PLL3_CALIB_CTRL4: c_uint = 0x2120;
pub const RT5514_PLL3_CALIB_CTRL5: c_uint = 0x2124;
pub const RT5514_PLL3_CALIB_CTRL6: c_uint = 0x2128;
pub const RT5514_DELAY_BUF_CTRL1: c_uint = 0x2140;
pub const RT5514_DELAY_BUF_CTRL3: c_uint = 0x2148;
pub const RT5514_ASRC_IN_CTRL1: c_uint = 0x2180;
pub const RT5514_DOWNFILTER0_CTRL1: c_uint = 0x2190;
pub const RT5514_DOWNFILTER0_CTRL2: c_uint = 0x2194;
pub const RT5514_DOWNFILTER0_CTRL3: c_uint = 0x2198;
pub const RT5514_DOWNFILTER1_CTRL1: c_uint = 0x21a0;
pub const RT5514_DOWNFILTER1_CTRL2: c_uint = 0x21a4;
pub const RT5514_DOWNFILTER1_CTRL3: c_uint = 0x21a8;
pub const RT5514_ANA_CTRL_LDO10: c_uint = 0x2200;
pub const RT5514_ANA_CTRL_LDO18_16: c_uint = 0x2204;
pub const RT5514_ANA_CTRL_ADC12: c_uint = 0x2210;
pub const RT5514_ANA_CTRL_ADC21: c_uint = 0x2214;
pub const RT5514_ANA_CTRL_ADC22: c_uint = 0x2218;
pub const RT5514_ANA_CTRL_ADC23: c_uint = 0x221c;
pub const RT5514_ANA_CTRL_MICBST: c_uint = 0x2220;
pub const RT5514_ANA_CTRL_ADCFED: c_uint = 0x2224;
pub const RT5514_ANA_CTRL_INBUF: c_uint = 0x2228;
pub const RT5514_ANA_CTRL_VREF: c_uint = 0x222c;
pub const RT5514_ANA_CTRL_PLL3: c_uint = 0x2240;
pub const RT5514_ANA_CTRL_PLL1_1: c_uint = 0x2260;
pub const RT5514_ANA_CTRL_PLL1_2: c_uint = 0x2264;
pub const RT5514_DMIC_LP_CTRL: c_uint = 0x2e00;
pub const RT5514_MISC_CTRL_DSP: c_uint = 0x2e04;
pub const RT5514_DSP_CTRL1: c_uint = 0x2f00;
pub const RT5514_DSP_CTRL3: c_uint = 0x2f08;
pub const RT5514_DSP_CTRL4: c_uint = 0x2f10;
pub const RT5514_VENDOR_ID1: c_uint = 0x2ff0;
pub const RT5514_VENDOR_ID2: c_uint = 0x2ff4;
pub const RT5514_DSP_MAPPING: c_uint = 0x18000000;
// RT5514_PWR_ANA1 (0x2004)

pub const RT5514_POW_LDO18_IN_BIT: c_int = 5;

pub const RT5514_POW_LDO18_ADC_BIT: c_int = 4;

pub const RT5514_POW_LDO21_BIT: c_int = 3;

pub const RT5514_POW_BG_LDO18_IN_BIT: c_int = 2;

pub const RT5514_POW_BG_LDO21_BIT: c_int = 1;
// RT5514_PWR_ANA2 (0x2008)

pub const RT5514_POW_PLL1_BIT: c_int = 18;

pub const RT5514_POW_PLL1_LDO_BIT: c_int = 16;

pub const RT5514_POW_BG_MBIAS_BIT: c_int = 15;

pub const RT5514_POW_MBIAS_BIT: c_int = 14;

pub const RT5514_POW_VREF2_BIT: c_int = 13;

pub const RT5514_POW_VREF1_BIT: c_int = 12;

pub const RT5514_POWR_LDO16_BIT: c_int = 11;

pub const RT5514_POWL_LDO16_BIT: c_int = 10;

pub const RT5514_POW_ADC2_BIT: c_int = 9;

pub const RT5514_POW_INPUT_BUF_BIT: c_int = 8;

pub const RT5514_POW_ADC1_R_BIT: c_int = 7;

pub const RT5514_POW_ADC1_L_BIT: c_int = 6;

pub const RT5514_POW2_BSTR_BIT: c_int = 5;

pub const RT5514_POW2_BSTL_BIT: c_int = 4;

pub const RT5514_POW_BSTR_BIT: c_int = 3;

pub const RT5514_POW_BSTL_BIT: c_int = 2;

pub const RT5514_POW_ADCFEDR_BIT: c_int = 1;

pub const RT5514_POW_ADCFEDL_BIT: c_int = 0;
// RT5514_I2S_CTRL1 (0x2010)

pub const RT5514_TDM_MODE2_SFT: c_int = 30;

pub const RT5514_TDM_MODE_SFT: c_int = 28;

pub const RT5514_I2S_LR_SFT: c_int = 26;

pub const RT5514_I2S_BP_SFT: c_int = 25;

pub const RT5514_I2S_DF_SFT: c_int = 16;

pub const RT5514_TDMSLOT_SEL_RX_SFT: c_int = 10;

pub const RT5514_CH_LEN_RX_SFT: c_int = 8;

pub const RT5514_TDMSLOT_SEL_TX_SFT: c_int = 6;

pub const RT5514_CH_LEN_TX_SFT: c_int = 4;

pub const RT5514_I2S_DL_SFT: c_int = 0;

// RT5514_I2S_CTRL2 (0x2014)

pub const RT5514_TDM_DOCKING_MODE_SFT: c_int = 31;

pub const RT5514_TDM_DOCKING_VALID_CH_SFT: c_int = 29;

pub const RT5514_TDM_DOCKING_START_SFT: c_int = 28;

// RT5514_DIG_SOURCE_CTRL (0x20a4)

pub const RT5514_AD1_DMIC_INPUT_SEL_SFT: c_int = 1;

pub const RT5514_AD0_DMIC_INPUT_SEL_SFT: c_int = 0;
// RT5514_PLL_SOURCE_CTRL (0x2100)

pub const RT5514_PLL_1_SEL_SFT: c_int = 12;

// RT5514_CLK_CTRL1 (0x2104)

pub const RT5514_CLK_AD_ANA1_EN_BIT: c_int = 31;

pub const RT5514_CLK_AD1_EN_BIT: c_int = 24;

pub const RT5514_CLK_AD0_EN_BIT: c_int = 23;

pub const RT5514_CLK_DMIC_OUT_SEL_SFT: c_int = 8;

pub const RT5514_CLK_AD_ANA1_SEL_SFT: c_int = 0;
// RT5514_CLK_CTRL2 (0x2108)

pub const RT5514_CLK_AD1_ASRC_EN_BIT: c_int = 17;

pub const RT5514_CLK_AD0_ASRC_EN_BIT: c_int = 16;

pub const RT5514_CLK_SYS_DIV_OUT_SFT: c_int = 8;

pub const RT5514_SEL_ADC_OSR_SFT: c_int = 4;

pub const RT5514_CLK_SYS_PRE_SEL_SFT: c_int = 0;

// RT5514_DOWNFILTER_CTRL (0x2190 0x2194 0x21a0 0x21a4)

pub const RT5514_AD_DMIC_MIX_BIT: c_int = 11;

pub const RT5514_AD_AD_MIX_BIT: c_int = 10;

pub const RT5514_AD_AD_MUTE_BIT: c_int = 7;

pub const RT5514_AD_GAIN_SFT: c_int = 1;
// RT5514_ANA_CTRL_MICBST (0x2220)

pub const RT5514_SEL_BSTL_SFT: c_int = 4;

pub const RT5514_SEL_BSTR_SFT: c_int = 0;
// RT5514_ANA_CTRL_PLL1_1 (0x2260)
pub const RT5514_PLL_K_MAX: c_uint = 0x1f;

pub const RT5514_PLL_K_SFT: c_int = 16;
pub const RT5514_PLL_N_MAX: c_uint = 0x1ff;

pub const RT5514_PLL_N_SFT: c_int = 4;
pub const RT5514_PLL_M_MAX: c_uint = 0xf;

pub const RT5514_PLL_M_SFT: c_int = 0;
// RT5514_ANA_CTRL_PLL1_2 (0x2264)

pub const RT5514_PLL_M_BP_SFT: c_int = 2;

pub const RT5514_PLL_K_BP_SFT: c_int = 1;

pub const RT5514_EN_LDO_PLL1_BIT: c_int = 0;
pub const RT5514_PLL_INP_MAX: c_int = 40000000;
pub const RT5514_PLL_INP_MIN: c_int = 256000;

// System Clock Source
// PLL1 Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt5514_priv {
    pub pdata: rt5514_platform_data,
    pub component: *mut snd_soc_component,
    pub regmap: *mut *mut regmap i2c_regmap,,
    pub dsp_calib_clk: *mut *mut clk mclk,,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: c_int,
    pub bclk: c_int,
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub dsp_enabled: c_int,
    pub pll3_cal_value: c_uint,
}

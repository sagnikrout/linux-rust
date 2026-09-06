//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1016.h
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
// rt1016.h  --  RT1016 ALSA SoC audio amplifier driver
//
// Copyright 2020 Realtek Semiconductor Corp.
// Author: Oder Chiou <oder_chiou@realtek.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
pub const RT1016_DEVICE_ID_VAL: c_uint = 0x6595;
pub const RT1016_RESET: c_uint = 0x00;
pub const RT1016_PADS_CTRL_1: c_uint = 0x01;
pub const RT1016_PADS_CTRL_2: c_uint = 0x02;
pub const RT1016_I2C_CTRL: c_uint = 0x03;
pub const RT1016_VOL_CTRL_1: c_uint = 0x04;
pub const RT1016_VOL_CTRL_2: c_uint = 0x05;
pub const RT1016_VOL_CTRL_3: c_uint = 0x06;
pub const RT1016_ANA_CTRL_1: c_uint = 0x07;
pub const RT1016_MUX_SEL: c_uint = 0x08;
pub const RT1016_RX_I2S_CTRL: c_uint = 0x09;
pub const RT1016_ANA_FLAG: c_uint = 0x0a;
pub const RT1016_VERSION2_ID: c_uint = 0x0c;
pub const RT1016_VERSION1_ID: c_uint = 0x0d;
pub const RT1016_VENDER_ID: c_uint = 0x0e;
pub const RT1016_DEVICE_ID: c_uint = 0x0f;
pub const RT1016_ANA_CTRL_2: c_uint = 0x11;
pub const RT1016_TEST_SIGNAL: c_uint = 0x1c;
pub const RT1016_TEST_CTRL_1: c_uint = 0x1d;
pub const RT1016_TEST_CTRL_2: c_uint = 0x1e;
pub const RT1016_TEST_CTRL_3: c_uint = 0x1f;
pub const RT1016_CLOCK_1: c_uint = 0x20;
pub const RT1016_CLOCK_2: c_uint = 0x21;
pub const RT1016_CLOCK_3: c_uint = 0x22;
pub const RT1016_CLOCK_4: c_uint = 0x23;
pub const RT1016_CLOCK_5: c_uint = 0x24;
pub const RT1016_CLOCK_6: c_uint = 0x25;
pub const RT1016_CLOCK_7: c_uint = 0x26;
pub const RT1016_I2S_CTRL: c_uint = 0x40;
pub const RT1016_DAC_CTRL_1: c_uint = 0x60;
pub const RT1016_SC_CTRL_1: c_uint = 0x80;
pub const RT1016_SC_CTRL_2: c_uint = 0x81;
pub const RT1016_SC_CTRL_3: c_uint = 0x82;
pub const RT1016_SC_CTRL_4: c_uint = 0x83;
pub const RT1016_SIL_DET: c_uint = 0xa0;
pub const RT1016_SYS_CLK: c_uint = 0xc0;
pub const RT1016_BIAS_CUR: c_uint = 0xc1;
pub const RT1016_DAC_CTRL_2: c_uint = 0xc2;
pub const RT1016_LDO_CTRL: c_uint = 0xc3;
pub const RT1016_CLASSD_1: c_uint = 0xc4;
pub const RT1016_PLL1: c_uint = 0xc5;
pub const RT1016_PLL2: c_uint = 0xc6;
pub const RT1016_PLL3: c_uint = 0xc7;
pub const RT1016_CLASSD_2: c_uint = 0xc8;
pub const RT1016_CLASSD_OUT: c_uint = 0xc9;
pub const RT1016_CLASSD_3: c_uint = 0xca;
pub const RT1016_CLASSD_4: c_uint = 0xcb;
pub const RT1016_CLASSD_5: c_uint = 0xcc;
pub const RT1016_PWR_CTRL: c_uint = 0xcf;
// global definition

pub const RT1016_L_VOL_SFT: c_int = 8;

pub const RT1016_R_VOL_SFT: c_int = 0;
// 0x04
pub const RT1016_DA_MUTE_L_SFT: c_int = 7;
pub const RT1016_DA_MUTE_R_SFT: c_int = 6;
// 0x20

pub const RT1016_CLK_SYS_SEL_SFT: c_int = 15;

pub const RT1016_PLL_SEL_SFT: c_int = 13;

// 0x21

pub const RT1016_FS_PD_SFT: c_int = 13;

pub const RT1016_OSR_PD_SFT: c_int = 10;
// 0x22

pub const RT1016_PWR_DAC_FILTER_BIT: c_int = 11;

pub const RT1016_PWR_DACMOD_BIT: c_int = 10;

pub const RT1016_PWR_CLK_FIFO_BIT: c_int = 9;

pub const RT1016_PWR_CLK_PUREDC_BIT: c_int = 8;

pub const RT1016_PWR_SIL_DET_BIT: c_int = 7;

pub const RT1016_PWR_RC_25M_BIT: c_int = 6;

pub const RT1016_PWR_PLL1_BIT: c_int = 5;

pub const RT1016_PWR_ANA_CTRL_BIT: c_int = 4;

pub const RT1016_PWR_CLK_SYS_BIT: c_int = 3;
// 0x23

pub const RT1016_PWR_LRCK_DET_BIT: c_int = 15;

pub const RT1016_PWR_BCLK_DET_BIT: c_int = 11;
// 0x40

pub const RT1016_I2S_BCLK_MS_SFT: c_int = 15;

pub const RT1016_I2S_BCLK_POL_SFT: c_int = 13;

pub const RT1016_I2S_DATA_SWAP_SFT: c_int = 10;

pub const RT1016_I2S_DL_SFT: c_int = 4;

pub const RT1016_I2S_MS_SFT: c_int = 3;

pub const RT1016_I2S_DF_SFT: c_int = 0;

// 0xa0

pub const RT1016_SIL_DET_EN_BIT: c_int = 15;
// 0xc2

pub const RT1016_CKGEN_DAC_BIT: c_int = 13;
// 0xc4

pub const RT1016_VCM_SLOW_BIT: c_int = 6;
// 0xc5
pub const RT1016_PLL_M_MAX: c_uint = 0xf;

pub const RT1016_PLL_M_SFT: c_int = 12;

pub const RT1016_PLL_M_BP_SFT: c_int = 11;
pub const RT1016_PLL_N_MAX: c_uint = 0x1ff;

pub const RT1016_PLL_N_SFT: c_int = 0;
// 0xc6

pub const RT1016_PLL2_EN_BIT: c_int = 15;

pub const RT1016_PLL_K_BP_SFT: c_int = 5;
pub const RT1016_PLL_K_MAX: c_uint = 0x1f;

pub const RT1016_PLL_K_SFT: c_int = 0;
// 0xcf

pub const RT1016_PWR_BG_1_2_BIT: c_int = 12;

pub const RT1016_PWR_MBIAS_BG_BIT: c_int = 11;

pub const RT1016_PWR_PLL_BIT: c_int = 9;

pub const RT1016_PWR_BASIC_BIT: c_int = 8;

pub const RT1016_PWR_CLSD_BIT: c_int = 7;

pub const RT1016_PWR_25M_BIT: c_int = 6;

pub const RT1016_PWR_DACL_BIT: c_int = 4;

pub const RT1016_PWR_DACR_BIT: c_int = 3;

pub const RT1016_PWR_LDO2_BIT: c_int = 2;

pub const RT1016_PWR_VREF_BIT: c_int = 1;

pub const RT1016_PWR_MBIAS_BIT: c_int = 0;
// System Clock Source
// PLL1 Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1016_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: c_int,
    pub bclk: c_int,
    pub master: c_int,
    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
}

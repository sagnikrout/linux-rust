//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs530x.h
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
// CS530x CODEC driver internal data
//
// Copyright (C) 2023-2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

// Devices
pub const CS530X_2CH_CODEC_DEV_ID: c_uint = 0x4282;
pub const CS530X_2CH_DAC_DEV_ID: c_uint = 0x4302;
pub const CS530X_4CH_DAC_DEV_ID: c_uint = 0x4304;
pub const CS530X_8CH_DAC_DEV_ID: c_uint = 0x4308;
pub const CS530X_2CH_ADC_DEV_ID: c_uint = 0x5302;
pub const CS530X_4CH_ADC_DEV_ID: c_uint = 0x5304;
pub const CS530X_8CH_ADC_DEV_ID: c_uint = 0x5308;
// Registers
pub const CS530X_DEVID: c_uint = 0x0000000;
pub const CS530X_REVID: c_uint = 0x0000004;
pub const CS530X_SW_RESET: c_uint = 0x0000022;
pub const CS530X_CLK_CFG_0: c_uint = 0x0000040;
pub const CS530X_CLK_CFG_1: c_uint = 0x0000042;
pub const CS530X_CHIP_ENABLE: c_uint = 0x0000044;
pub const CS530X_ASP_CFG: c_uint = 0x0000048;
pub const CS530X_SIGNAL_PATH_CFG: c_uint = 0x0000050;
pub const CS530X_IN_ENABLES: c_uint = 0x0000080;
pub const CS530X_IN_RAMP_SUM: c_uint = 0x0000082;
pub const CS530X_IN_FILTER: c_uint = 0x0000086;
pub const CS530X_IN_HIZ: c_uint = 0x0000088;
pub const CS530X_IN_INV: c_uint = 0x000008A;
pub const CS530X_IN_VOL_CTRL1_0: c_uint = 0x0000090;
pub const CS530X_IN_VOL_CTRL1_1: c_uint = 0x0000092;
pub const CS530X_IN_VOL_CTRL2_0: c_uint = 0x0000094;
pub const CS530X_IN_VOL_CTRL2_1: c_uint = 0x0000096;
pub const CS530X_IN_VOL_CTRL3_0: c_uint = 0x0000098;
pub const CS530X_IN_VOL_CTRL3_1: c_uint = 0x000009A;
pub const CS530X_IN_VOL_CTRL4_0: c_uint = 0x000009C;
pub const CS530X_IN_VOL_CTRL4_1: c_uint = 0x000009E;
pub const CS530X_IN_VOL_CTRL5: c_uint = 0x00000A0;
pub const CS530X_OUT_ENABLES: c_uint = 0x00000C0;
pub const CS530X_OUT_RAMP_SUM: c_uint = 0x00000C2;
pub const CS530X_OUT_DEEMPH: c_uint = 0x00000C4;
pub const CS530X_OUT_FILTER: c_uint = 0x00000C6;
pub const CS530X_OUT_INV: c_uint = 0x00000CA;
pub const CS530X_OUT_VOL_CTRL1_0: c_uint = 0x00000D0;
pub const CS530X_OUT_VOL_CTRL1_1: c_uint = 0x00000D2;
pub const CS530X_OUT_VOL_CTRL2_0: c_uint = 0x00000D4;
pub const CS530X_OUT_VOL_CTRL2_1: c_uint = 0x00000D6;
pub const CS530X_OUT_VOL_CTRL3_0: c_uint = 0x00000D8;
pub const CS530X_OUT_VOL_CTRL3_1: c_uint = 0x00000DA;
pub const CS530X_OUT_VOL_CTRL4_0: c_uint = 0x00000DC;
pub const CS530X_OUT_VOL_CTRL4_1: c_uint = 0x00000DE;
pub const CS530X_OUT_VOL_CTRL5: c_uint = 0x00000E0;
pub const CS530X_PAD_FN: c_uint = 0x0003D24;
pub const CS530X_PAD_LVL: c_uint = 0x0003D28;

// Register Fields
// REVID

// SW_RESET
pub const CS530X_SW_RST_SHIFT: c_int = 8;

// CLK_CFG_0

pub const CS530X_SYSCLK_SRC_SHIFT: c_int = 12;
pub const CS530X_REFCLK_2P822_3P072: c_int = 0;
pub const CS530X_REFCLK_5P6448_6P144: c_uint = 0x10;
pub const CS530X_REFCLK_11P2896_12P288: c_uint = 0x20;
pub const CS530X_REFCLK_24P5792_24P576: c_uint = 0x30;
// CLK_CFG_1

pub const CS530X_FS_32K: c_int = 0;
pub const CS530X_FS_44P1K_48K: c_int = 1;
pub const CS530X_FS_88P2K_96K: c_int = 2;
pub const CS530X_FS_176P4K_192K: c_int = 3;
pub const CS530X_FS_356P8K_384K: c_int = 4;
pub const CS530X_FS_705P6K_768K: c_int = 5;
// CHIP_ENABLE

// ASP_CFG

pub const CS530X_BCLK_2P822_3P072: c_int = 0;
pub const CS530X_BCLK_5P6448_6P144: c_int = 1;
pub const CS530X_BCLK_11P2896_12P288: c_int = 2;
pub const CS530X_BCLK_24P5792_24P576: c_int = 3;
// SIGNAL_PATH_CFG

pub const CS530X_ASP_TDM_SLOT_SHIFT: c_int = 3;

pub const CS530X_ASP_FMT_I2S: c_int = 0;
pub const CS530X_ASP_FMT_LJ: c_int = 1;
pub const CS530X_ASP_FMT_DSP_A: c_int = 6;
// TDM Slots

pub const CS530X_0_7_TDM_SLOT_VAL: c_int = 0;

pub const CS530X_2_3_TDM_SLOT_VAL: c_int = 1;

pub const CS530X_4_7_TDM_SLOT_VAL: c_int = 2;

pub const CS530X_6_7_TDM_SLOT_VAL: c_int = 3;

pub const CS530X_8_15_TDM_SLOT_VAL: c_int = 4;

pub const CS530X_10_11_TDM_SLOT_VAL: c_int = 5;

pub const CS530X_12_15_TDM_SLOT_VAL: c_int = 6;

pub const CS530X_14_15_TDM_SLOT_VAL: c_int = 7;
// IN_RAMP_SUM and OUT_RAMP_SUM
pub const CS530X_RAMP_RATE_INC_SHIFT: c_int = 0;
pub const CS530X_RAMP_RATE_DEC_SHIFT: c_int = 4;
pub const CS530X_INOUT_SUM_MODE_SHIFT: c_int = 13;
// IN_FILTER and OUT_FILTER
pub const CS530X_INOUT_FILTER_SHIFT: c_int = 8;
pub const CS530X_INOUT_HPF_EN_SHIFT: c_int = 12;
// IN_HIZ

// IN_INV and OUT_INV
pub const CS530X_INOUT1_INV_SHIFT: c_int = 0;
pub const CS530X_INOUT2_INV_SHIFT: c_int = 1;
pub const CS530X_INOUT3_INV_SHIFT: c_int = 2;
pub const CS530X_INOUT4_INV_SHIFT: c_int = 3;
pub const CS530X_INOUT5_INV_SHIFT: c_int = 4;
pub const CS530X_INOUT6_INV_SHIFT: c_int = 5;
pub const CS530X_INOUT7_INV_SHIFT: c_int = 6;
pub const CS530X_INOUT8_INV_SHIFT: c_int = 7;
// IN_VOL_CTLy_z and OUT_VOL_CTLy_z

// IN_VOL_CTL5

// PAD_FN

// PAD_LVL

// IN_VOL_CTL5 and OUT_VOL_CTL5

// System Clock Source
pub const CS530X_SYSCLK_SRC_MCLK: c_int = 0;
pub const CS530X_SYSCLK_SRC_PLL: c_int = 1;
// PLL Reference Clock Source
pub const CS530X_PLL_SRC_BCLK: c_int = 0;
pub const CS530X_PLL_SRC_MCLK: c_int = 1;
pub const CS530X_NUM_SUPPLIES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs530x_type {
    CS4282 = CS530X_2CH_CODEC_DEV_ID,
    CS4302 = CS530X_2CH_DAC_DEV_ID,
    CS4304 = CS530X_4CH_DAC_DEV_ID,
    CS4308 = CS530X_8CH_DAC_DEV_ID,
    CS5302 = CS530X_2CH_ADC_DEV_ID,
    CS5304 = CS530X_4CH_ADC_DEV_ID,
    CS5308 = CS530X_8CH_ADC_DEV_ID,
}

// codec private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs530x_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub dev_dai: *mut snd_soc_dai_driver,
    pub devtype: cs530x_type,
    pub num_adcs: c_int,
    pub num_dacs: c_int,
    pub supplies: [regulator_bulk_data; CS530X_NUM_SUPPLIES],
    pub tdm_width: c_int,
    pub tdm_slots: c_int,
    pub adc_pairs_count: c_int,
    pub dac_pairs_count: c_int,
    pub reset_gpio: *mut gpio_desc,
}

extern "C" {
    pub fn cs530x_probe(cs530x: *mut cs530x_priv) -> c_int;
}

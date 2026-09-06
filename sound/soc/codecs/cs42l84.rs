//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42l84.h
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
// Copyright (C) The Asahi Linux Contributors
//
// Based on sound/soc/codecs/cs42l42.h
//
// Copyright 2016 Cirrus Logic, Inc.
//

pub const CS42L84_CHIP_ID: c_uint = 0x42a84;
pub const CS42L84_DEVID: c_uint = 0x0000;
pub const CS42L84_REVID: c_uint = 0x73fe;
pub const CS42L84_FRZ_CTL: c_uint = 0x0006;

pub const CS42L84_TSRS_PLUG_INT_STATUS: c_uint = 0x0400;
pub const CS42L84_TSRS_PLUG_INT_MASK: c_uint = 0x0418;
pub const CS42L84_RS_PLUG_SHIFT: c_int = 0;

pub const CS42L84_TS_PLUG_SHIFT: c_int = 2;

pub const CS42L84_PLL_LOCK_STATUS: c_uint = 0x040e // probably bit 0x10;

pub const CS42L84_PLUG: c_int = 3;
pub const CS42L84_UNPLUG: c_int = 0;
pub const CS42L84_TRANS: c_int = 1;
pub const CS42L84_CCM_CTL1: c_uint = 0x0600;

pub const CS42L84_CCM_CTL1_MCLK_SRC_RCO: c_int = 0;
pub const CS42L84_CCM_CTL1_MCLK_SRC_MCLK: c_int = 1;
pub const CS42L84_CCM_CTL1_MCLK_SRC_BCLK: c_int = 2;
pub const CS42L84_CCM_CTL1_MCLK_SRC_PLL: c_int = 3;

pub const CS42L84_CCM_SAMP_RATE: c_uint = 0x0601;
pub const CS42L84_CCM_SAMP_RATE_RATE_48KHZ: c_int = 4;
pub const CS42L84_CCM_SAMP_RATE_RATE_96KHZ: c_int = 5;
pub const CS42L84_CCM_SAMP_RATE_RATE_192KHZ: c_int = 6;
pub const CS42L84_CCM_SAMP_RATE_RATE_44K1HZ: c_int = 12;
pub const CS42L84_CCM_SAMP_RATE_RATE_88K2HZ: c_int = 13;
pub const CS42L84_CCM_SAMP_RATE_RATE_176K4HZ: c_int = 14;
pub const CS42L84_CCM_CTL3: c_uint = 0x0602;

pub const CS42L84_CCM_CTL4: c_uint = 0x0603;

pub const CS42L84_CCM_ASP_CLK_CTRL: c_uint = 0x0608;
pub const CS42L84_PLL_CTL1: c_uint = 0x0800;

pub const CS42L84_PLL_DIV_FRAC0: c_uint = 0x0804;
pub const CS42L84_PLL_DIV_FRAC1: c_uint = 0x0805;
pub const CS42L84_PLL_DIV_FRAC2: c_uint = 0x0806;
pub const CS42L84_PLL_DIV_INT: c_uint = 0x0807;
pub const CS42L84_PLL_DIVOUT: c_uint = 0x0808;
pub const CS42L84_RING_SENSE_CTL: c_uint = 0x1282;

pub const CS42L84_TIP_SENSE_CTL: c_uint = 0x1283;

pub const CS42L84_TSRS_PLUG_STATUS: c_uint = 0x1288;
pub const CS42L84_TIP_SENSE_CTL2: c_uint = 0x1473;

pub const CS42L84_MISC_DET_CTL: c_uint = 0x1474;

pub const CS42L84_MIC_DET_CTL1: c_uint = 0x1475;

pub const CS42L84_MIC_DET_CTL4: c_uint = 0x1477;

pub const CS42L84_HS_DET_STATUS2: c_uint = 0x147d;
pub const CS42L84_MSM_BLOCK_EN1: c_uint = 0x1800;
pub const CS42L84_MSM_BLOCK_EN2: c_uint = 0x1801;
pub const CS42L84_MSM_BLOCK_EN2_ASP_SHIFT: c_int = 6;
pub const CS42L84_MSM_BLOCK_EN2_BUS_SHIFT: c_int = 5;
pub const CS42L84_MSM_BLOCK_EN2_DAC_SHIFT: c_int = 4;
pub const CS42L84_MSM_BLOCK_EN2_ADC_SHIFT: c_int = 3;
pub const CS42L84_MSM_BLOCK_EN3: c_uint = 0x1802;

pub const CS42L84_HS_DET_CTL2: c_uint = 0x1811;

pub const CS42L84_HS_SWITCH_CTL: c_uint = 0x1812;

pub const CS42L84_HS_CLAMP_DISABLE: c_uint = 0x1813;
pub const CS42L84_ADC_CTL1: c_uint = 0x2000;
pub const CS42L84_ADC_CTL1_PREAMP_GAIN_SHIFT: c_int = 6;
pub const CS42L84_ADC_CTL1_PGA_GAIN_SHIFT: c_int = 0;
pub const CS42L84_ADC_CTL4: c_uint = 0x2003;
pub const CS42L84_ADC_CTL4_WNF_CF_SHIFT: c_int = 4;
pub const CS42L84_ADC_CTL4_WNF_EN_SHIFT: c_int = 3;
pub const CS42L84_ADC_CTL4_HPF_CF_SHIFT: c_int = 1;
pub const CS42L84_ADC_CTL4_HPF_EN_SHIFT: c_int = 0;
pub const CS42L84_DAC_CTL1: c_uint = 0x3000;

// #define CS42L84_DAC_CTL1_DACB_INV_SHIFT 1
// #define CS42L84_DAC_CTL1_DACA_INV_SHIFT 0
pub const CS42L84_DAC_CTL2: c_uint = 0x3001;
pub const CS42L84_DAC_CHA_VOL_LSB: c_uint = 0x3004;
pub const CS42L84_DAC_CHA_VOL_MSB: c_uint = 0x3005;
pub const CS42L84_DAC_CHB_VOL_LSB: c_uint = 0x3006;
pub const CS42L84_DAC_CHB_VOL_MSB: c_uint = 0x3007;
pub const CS42L84_HP_VOL_CTL: c_uint = 0x3020;

pub const CS42L84_BUS_ASP_TX_SRC: c_uint = 0x4000;
pub const CS42L84_BUS_ASP_TX_SRC_CH1_SHIFT: c_int = 0;
pub const CS42L84_BUS_DAC_SRC: c_uint = 0x4001;
pub const CS42L84_BUS_DAC_SRC_DACA_SHIFT: c_int = 0;
pub const CS42L84_BUS_DAC_SRC_DACB_SHIFT: c_int = 4;
pub const CS42L84_ASP_CTL: c_uint = 0x5000;
pub const CS42L84_ASP_CTL_BCLK_EN_SHIFT: c_int = 1;

pub const CS42L84_ASP_FSYNC_CTL2: c_uint = 0x5010;

pub const CS42L84_ASP_FSYNC_CTL3: c_uint = 0x5011;

pub const CS42L84_ASP_DATA_CTL: c_uint = 0x5018;
pub const CS42L84_ASP_RX_EN: c_uint = 0x5020;
pub const CS42L84_ASP_RX_EN_CH1_SHIFT: c_int = 0;
pub const CS42L84_ASP_RX_EN_CH2_SHIFT: c_int = 1;
pub const CS42L84_ASP_TX_EN: c_uint = 0x5024;
pub const CS42L84_ASP_TX_EN_CH1_SHIFT: c_int = 0;
pub const CS42L84_ASP_RX_CH1_CTL1: c_uint = 0x5028;
pub const CS42L84_ASP_RX_CH1_CTL2: c_uint = 0x5029;
pub const CS42L84_ASP_RX_CH1_WIDTH: c_uint = 0x502a;
pub const CS42L84_ASP_RX_CH2_CTL1: c_uint = 0x502c;
pub const CS42L84_ASP_RX_CH2_CTL2: c_uint = 0x502d;
pub const CS42L84_ASP_RX_CH2_WIDTH: c_uint = 0x502e;

pub const CS42L84_ASP_TX_CH1_CTL1: c_uint = 0x5068;
pub const CS42L84_ASP_TX_CH1_CTL2: c_uint = 0x5069;
pub const CS42L84_ASP_TX_CH1_WIDTH: c_uint = 0x506a;
pub const CS42L84_ASP_TX_CH2_CTL1: c_uint = 0x506c;
pub const CS42L84_ASP_TX_CH2_CTL2: c_uint = 0x506d;
pub const CS42L84_ASP_TX_CH2_WIDTH: c_uint = 0x506e;

pub const CS42L84_BOOT_TIME_US: c_int = 3000;
pub const CS42L84_CLOCK_SWITCH_DELAY_US: c_int = 150;
pub const CS42L84_PLL_LOCK_POLL_US: c_int = 250;
pub const CS42L84_PLL_LOCK_TIMEOUT_US: c_int = 1250;

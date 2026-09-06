//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tlv320adcx140.h
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
// TLV320ADCX140 Sound driver
// Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com

pub const ADCX140_PAGE_SELECT: c_uint = 0x00;
pub const ADCX140_SW_RESET: c_uint = 0x01;
pub const ADCX140_SLEEP_CFG: c_uint = 0x02;
pub const ADCX140_SHDN_CFG: c_uint = 0x05;
pub const ADCX140_ASI_CFG0: c_uint = 0x07;
pub const ADCX140_ASI_CFG1: c_uint = 0x08;
pub const ADCX140_ASI_CFG2: c_uint = 0x09;
pub const ADCX140_ASI_CH1: c_uint = 0x0b;
pub const ADCX140_ASI_CH2: c_uint = 0x0c;
pub const ADCX140_ASI_CH3: c_uint = 0x0d;
pub const ADCX140_ASI_CH4: c_uint = 0x0e;
pub const ADCX140_ASI_CH5: c_uint = 0x0f;
pub const ADCX140_ASI_CH6: c_uint = 0x10;
pub const ADCX140_ASI_CH7: c_uint = 0x11;
pub const ADCX140_ASI_CH8: c_uint = 0x12;
pub const ADCX140_MST_CFG0: c_uint = 0x13;
pub const ADCX140_MST_CFG1: c_uint = 0x14;
pub const ADCX140_ASI_STS: c_uint = 0x15;
pub const ADCX140_CLK_SRC: c_uint = 0x16;
pub const ADCX140_PDMCLK_CFG: c_uint = 0x1f;
pub const ADCX140_PDM_CFG: c_uint = 0x20;
pub const ADCX140_GPIO_CFG0: c_uint = 0x21;
pub const ADCX140_GPO_CFG0: c_uint = 0x22;
pub const ADCX140_GPO_CFG1: c_uint = 0x23;
pub const ADCX140_GPO_CFG2: c_uint = 0x24;
pub const ADCX140_GPO_CFG3: c_uint = 0x25;
pub const ADCX140_GPO_VAL: c_uint = 0x29;
pub const ADCX140_GPIO_MON: c_uint = 0x2a;
pub const ADCX140_GPI_CFG0: c_uint = 0x2b;
pub const ADCX140_GPI_CFG1: c_uint = 0x2c;
pub const ADCX140_GPI_MON: c_uint = 0x2f;
pub const ADCX140_INT_CFG: c_uint = 0x32;
pub const ADCX140_INT_MASK0: c_uint = 0x33;
pub const ADCX140_INT_LTCH0: c_uint = 0x36;
pub const ADCX140_BIAS_CFG: c_uint = 0x3b;
pub const ADCX140_CH1_CFG0: c_uint = 0x3c;
pub const ADCX140_CH1_CFG1: c_uint = 0x3d;
pub const ADCX140_CH1_CFG2: c_uint = 0x3e;
pub const ADCX140_CH1_CFG3: c_uint = 0x3f;
pub const ADCX140_CH1_CFG4: c_uint = 0x40;
pub const ADCX140_CH2_CFG0: c_uint = 0x41;
pub const ADCX140_CH2_CFG1: c_uint = 0x42;
pub const ADCX140_CH2_CFG2: c_uint = 0x43;
pub const ADCX140_CH2_CFG3: c_uint = 0x44;
pub const ADCX140_CH2_CFG4: c_uint = 0x45;
pub const ADCX140_CH3_CFG0: c_uint = 0x46;
pub const ADCX140_CH3_CFG1: c_uint = 0x47;
pub const ADCX140_CH3_CFG2: c_uint = 0x48;
pub const ADCX140_CH3_CFG3: c_uint = 0x49;
pub const ADCX140_CH3_CFG4: c_uint = 0x4a;
pub const ADCX140_CH4_CFG0: c_uint = 0x4b;
pub const ADCX140_CH4_CFG1: c_uint = 0x4c;
pub const ADCX140_CH4_CFG2: c_uint = 0x4d;
pub const ADCX140_CH4_CFG3: c_uint = 0x4e;
pub const ADCX140_CH4_CFG4: c_uint = 0x4f;
pub const ADCX140_CH5_CFG2: c_uint = 0x52;
pub const ADCX140_CH5_CFG3: c_uint = 0x53;
pub const ADCX140_CH5_CFG4: c_uint = 0x54;
pub const ADCX140_CH6_CFG2: c_uint = 0x57;
pub const ADCX140_CH6_CFG3: c_uint = 0x58;
pub const ADCX140_CH6_CFG4: c_uint = 0x59;
pub const ADCX140_CH7_CFG2: c_uint = 0x5c;
pub const ADCX140_CH7_CFG3: c_uint = 0x5d;
pub const ADCX140_CH7_CFG4: c_uint = 0x5e;
pub const ADCX140_CH8_CFG2: c_uint = 0x61;
pub const ADCX140_CH8_CFG3: c_uint = 0x62;
pub const ADCX140_CH8_CFG4: c_uint = 0x63;
pub const ADCX140_DSP_CFG0: c_uint = 0x6b;
pub const ADCX140_DSP_CFG1: c_uint = 0x6c;
pub const ADCX140_DRE_CFG0: c_uint = 0x6d;
pub const ADCX140_AGC_CFG0: c_uint = 0x70;
pub const ADCX140_IN_CH_EN: c_uint = 0x73;
pub const ADCX140_ASI_OUT_CH_EN: c_uint = 0x74;
pub const ADCX140_PWR_CFG: c_uint = 0x75;
pub const ADCX140_DEV_STS0: c_uint = 0x76;
pub const ADCX140_DEV_STS1: c_uint = 0x77;

pub const ADCX140_16_BIT_WORD: c_uint = 0x0;

pub const ADCX140_WORD_LEN_MSK: c_uint = 0x30;
pub const ADCX140_MAX_CHANNELS: c_int = 8;
pub const ADCX140_MIC_BIAS_VAL_VREF: c_int = 0;
pub const ADCX140_MIC_BIAS_VAL_VREF_1096: c_int = 1;
pub const ADCX140_MIC_BIAS_VAL_AVDD: c_int = 6;

pub const ADCX140_MIC_BIAS_SHIFT: c_int = 4;
pub const ADCX140_MIC_BIAS_VREF_275V: c_int = 0;
pub const ADCX140_MIC_BIAS_VREF_25V: c_int = 1;
pub const ADCX140_MIC_BIAS_VREF_1375V: c_int = 2;

pub const ADCX140_NUM_PDM_EDGES: c_int = 4;
pub const ADCX140_PDM_EDGE_SHIFT: c_int = 7;
pub const ADCX140_NUM_GPI_PINS: c_int = 4;
pub const ADCX140_GPI_SHIFT: c_int = 4;
pub const ADCX140_GPI1_INDEX: c_int = 0;
pub const ADCX140_GPI2_INDEX: c_int = 1;
pub const ADCX140_GPI3_INDEX: c_int = 2;
pub const ADCX140_GPI4_INDEX: c_int = 3;
pub const ADCX140_NUM_GPOS: c_int = 4;
pub const ADCX140_NUM_GPO_CFGS: c_int = 2;
pub const ADCX140_GPO_SHIFT: c_int = 4;
pub const ADCX140_GPO_CFG_MAX: c_int = 4;
pub const ADCX140_GPO_DRV_MAX: c_int = 5;

pub const ADCX140_NUM_GPIO_CFGS: c_int = 2;
pub const ADCX140_GPIO_SHIFT: c_int = 4;
pub const ADCX140_GPIO_CFG_MAX: c_int = 15;
pub const ADCX140_GPIO_DRV_MAX: c_int = 5;

//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/lpass-lpaif-reg.h
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
// Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
//
// LPAIF I2S

pub const LPAIF_I2SCTL_LOOPBACK_DISABLE: c_int = 0;
pub const LPAIF_I2SCTL_LOOPBACK_ENABLE: c_int = 1;
pub const LPAIF_I2SCTL_SPKEN_DISABLE: c_int = 0;
pub const LPAIF_I2SCTL_SPKEN_ENABLE: c_int = 1;
pub const LPAIF_I2SCTL_MODE_NONE: c_int = 0;
pub const LPAIF_I2SCTL_MODE_SD0: c_int = 1;
pub const LPAIF_I2SCTL_MODE_SD1: c_int = 2;
pub const LPAIF_I2SCTL_MODE_SD2: c_int = 3;
pub const LPAIF_I2SCTL_MODE_SD3: c_int = 4;
pub const LPAIF_I2SCTL_MODE_QUAD01: c_int = 5;
pub const LPAIF_I2SCTL_MODE_QUAD23: c_int = 6;
pub const LPAIF_I2SCTL_MODE_6CH: c_int = 7;
pub const LPAIF_I2SCTL_MODE_8CH: c_int = 8;
pub const LPAIF_I2SCTL_MODE_10CH: c_int = 9;
pub const LPAIF_I2SCTL_MODE_12CH: c_int = 10;
pub const LPAIF_I2SCTL_MODE_14CH: c_int = 11;
pub const LPAIF_I2SCTL_MODE_16CH: c_int = 12;
pub const LPAIF_I2SCTL_MODE_SD4: c_int = 13;
pub const LPAIF_I2SCTL_MODE_SD5: c_int = 14;
pub const LPAIF_I2SCTL_MODE_SD6: c_int = 15;
pub const LPAIF_I2SCTL_MODE_SD7: c_int = 16;
pub const LPAIF_I2SCTL_MODE_QUAD45: c_int = 17;
pub const LPAIF_I2SCTL_MODE_QUAD47: c_int = 18;
pub const LPAIF_I2SCTL_MODE_8CH_2: c_int = 19;

pub const LPAIF_I2SCTL_SPKMONO_STEREO: c_int = 0;
pub const LPAIF_I2SCTL_SPKMONO_MONO: c_int = 1;
pub const LPAIF_I2SCTL_MICEN_DISABLE: c_int = 0;
pub const LPAIF_I2SCTL_MICEN_ENABLE: c_int = 1;

pub const LPAIF_I2SCTL_MICMONO_STEREO: c_int = 0;
pub const LPAIF_I2SCTL_MICMONO_MONO: c_int = 1;
pub const LPAIF_I2SCTL_WSSRC_INTERNAL: c_int = 0;
pub const LPAIF_I2SCTL_WSSRC_EXTERNAL: c_int = 1;
pub const LPAIF_I2SCTL_BITWIDTH_16: c_int = 0;
pub const LPAIF_I2SCTL_BITWIDTH_24: c_int = 1;
pub const LPAIF_I2SCTL_BITWIDTH_32: c_int = 2;
pub const LPAIF_I2SCTL_RESET_STATE: c_uint = 0x003C0004;
pub const LPAIF_DMACTL_RESET_STATE: c_uint = 0x00200000;
// LPAIF IRQ

pub const LPAIF_IRQ_PORT_HOST: c_int = 0;

// LPAIF RXTX IRQ

// LPAIF VA IRQ

pub const LPAIF_IRQ_BITSTRIDE: c_int = 3;

// LPAIF DMA

pub const LPAIF_DMACTL_BURSTEN_SINGLE: c_int = 0;
pub const LPAIF_DMACTL_BURSTEN_INCR4: c_int = 1;
pub const LPAIF_DMACTL_WPSCNT_ONE: c_int = 0;
pub const LPAIF_DMACTL_WPSCNT_TWO: c_int = 1;
pub const LPAIF_DMACTL_WPSCNT_THREE: c_int = 2;
pub const LPAIF_DMACTL_WPSCNT_FOUR: c_int = 3;
pub const LPAIF_DMACTL_WPSCNT_SIX: c_int = 5;
pub const LPAIF_DMACTL_WPSCNT_EIGHT: c_int = 7;
pub const LPAIF_DMACTL_WPSCNT_TEN: c_int = 9;
pub const LPAIF_DMACTL_WPSCNT_TWELVE: c_int = 11;
pub const LPAIF_DMACTL_WPSCNT_FOURTEEN: c_int = 13;
pub const LPAIF_DMACTL_WPSCNT_SIXTEEN: c_int = 15;

pub const LPAIF_DMACTL_FIFOWM_1: c_int = 0;
pub const LPAIF_DMACTL_FIFOWM_2: c_int = 1;
pub const LPAIF_DMACTL_FIFOWM_3: c_int = 2;
pub const LPAIF_DMACTL_FIFOWM_4: c_int = 3;
pub const LPAIF_DMACTL_FIFOWM_5: c_int = 4;
pub const LPAIF_DMACTL_FIFOWM_6: c_int = 5;
pub const LPAIF_DMACTL_FIFOWM_7: c_int = 6;
pub const LPAIF_DMACTL_FIFOWM_8: c_int = 7;
pub const LPAIF_DMACTL_FIFOWM_9: c_int = 8;
pub const LPAIF_DMACTL_FIFOWM_10: c_int = 9;
pub const LPAIF_DMACTL_FIFOWM_11: c_int = 10;
pub const LPAIF_DMACTL_FIFOWM_12: c_int = 11;
pub const LPAIF_DMACTL_FIFOWM_13: c_int = 12;
pub const LPAIF_DMACTL_FIFOWM_14: c_int = 13;
pub const LPAIF_DMACTL_FIFOWM_15: c_int = 14;
pub const LPAIF_DMACTL_FIFOWM_16: c_int = 15;
pub const LPAIF_DMACTL_FIFOWM_17: c_int = 16;
pub const LPAIF_DMACTL_FIFOWM_18: c_int = 17;
pub const LPAIF_DMACTL_FIFOWM_19: c_int = 18;
pub const LPAIF_DMACTL_FIFOWM_20: c_int = 19;
pub const LPAIF_DMACTL_FIFOWM_21: c_int = 20;
pub const LPAIF_DMACTL_FIFOWM_22: c_int = 21;
pub const LPAIF_DMACTL_FIFOWM_23: c_int = 22;
pub const LPAIF_DMACTL_FIFOWM_24: c_int = 23;
pub const LPAIF_DMACTL_FIFOWM_25: c_int = 24;
pub const LPAIF_DMACTL_FIFOWM_26: c_int = 25;
pub const LPAIF_DMACTL_FIFOWM_27: c_int = 26;
pub const LPAIF_DMACTL_FIFOWM_28: c_int = 27;
pub const LPAIF_DMACTL_FIFOWM_29: c_int = 28;
pub const LPAIF_DMACTL_FIFOWM_30: c_int = 29;
pub const LPAIF_DMACTL_FIFOWM_31: c_int = 30;
pub const LPAIF_DMACTL_FIFOWM_32: c_int = 31;
pub const LPAIF_DMACTL_ENABLE_OFF: c_int = 0;
pub const LPAIF_DMACTL_ENABLE_ON: c_int = 1;
pub const LPAIF_DMACTL_DYNCLK_OFF: c_int = 0;
pub const LPAIF_DMACTL_DYNCLK_ON: c_int = 1;

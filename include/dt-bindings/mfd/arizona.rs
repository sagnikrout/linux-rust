//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mfd/arizona.h
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
// Device Tree defines for Arizona devices
//
// Copyright 2015 Cirrus Logic Inc.
//
// Author: Charles Keepax <ckeepax@opensource.wolfsonmicro.com>
//
// GPIO Function Definitions
pub const ARIZONA_GP_FN_TXLRCLK: c_uint = 0x00;
pub const ARIZONA_GP_FN_GPIO: c_uint = 0x01;
pub const ARIZONA_GP_FN_IRQ1: c_uint = 0x02;
pub const ARIZONA_GP_FN_IRQ2: c_uint = 0x03;
pub const ARIZONA_GP_FN_OPCLK: c_uint = 0x04;
pub const ARIZONA_GP_FN_FLL1_OUT: c_uint = 0x05;
pub const ARIZONA_GP_FN_FLL2_OUT: c_uint = 0x06;
pub const ARIZONA_GP_FN_PWM1: c_uint = 0x08;
pub const ARIZONA_GP_FN_PWM2: c_uint = 0x09;
pub const ARIZONA_GP_FN_SYSCLK_UNDERCLOCKED: c_uint = 0x0A;
pub const ARIZONA_GP_FN_ASYNCCLK_UNDERCLOCKED: c_uint = 0x0B;
pub const ARIZONA_GP_FN_FLL1_LOCK: c_uint = 0x0C;
pub const ARIZONA_GP_FN_FLL2_LOCK: c_uint = 0x0D;
pub const ARIZONA_GP_FN_FLL1_CLOCK_OK: c_uint = 0x0F;
pub const ARIZONA_GP_FN_FLL2_CLOCK_OK: c_uint = 0x10;
pub const ARIZONA_GP_FN_HEADPHONE_DET: c_uint = 0x12;
pub const ARIZONA_GP_FN_MIC_DET: c_uint = 0x13;
pub const ARIZONA_GP_FN_WSEQ_STATUS: c_uint = 0x15;
pub const ARIZONA_GP_FN_CIF_ADDRESS_ERROR: c_uint = 0x16;
pub const ARIZONA_GP_FN_ASRC1_LOCK: c_uint = 0x1A;
pub const ARIZONA_GP_FN_ASRC2_LOCK: c_uint = 0x1B;
pub const ARIZONA_GP_FN_ASRC_CONFIG_ERROR: c_uint = 0x1C;
pub const ARIZONA_GP_FN_DRC1_SIGNAL_DETECT: c_uint = 0x1D;
pub const ARIZONA_GP_FN_DRC1_ANTICLIP: c_uint = 0x1E;
pub const ARIZONA_GP_FN_DRC1_DECAY: c_uint = 0x1F;
pub const ARIZONA_GP_FN_DRC1_NOISE: c_uint = 0x20;
pub const ARIZONA_GP_FN_DRC1_QUICK_RELEASE: c_uint = 0x21;
pub const ARIZONA_GP_FN_DRC2_SIGNAL_DETECT: c_uint = 0x22;
pub const ARIZONA_GP_FN_DRC2_ANTICLIP: c_uint = 0x23;
pub const ARIZONA_GP_FN_DRC2_DECAY: c_uint = 0x24;
pub const ARIZONA_GP_FN_DRC2_NOISE: c_uint = 0x25;
pub const ARIZONA_GP_FN_DRC2_QUICK_RELEASE: c_uint = 0x26;
pub const ARIZONA_GP_FN_MIXER_DROPPED_SAMPLE: c_uint = 0x27;
pub const ARIZONA_GP_FN_AIF1_CONFIG_ERROR: c_uint = 0x28;
pub const ARIZONA_GP_FN_AIF2_CONFIG_ERROR: c_uint = 0x29;
pub const ARIZONA_GP_FN_AIF3_CONFIG_ERROR: c_uint = 0x2A;
pub const ARIZONA_GP_FN_SPK_TEMP_SHUTDOWN: c_uint = 0x2B;
pub const ARIZONA_GP_FN_SPK_TEMP_WARNING: c_uint = 0x2C;
pub const ARIZONA_GP_FN_UNDERCLOCKED: c_uint = 0x2D;
pub const ARIZONA_GP_FN_OVERCLOCKED: c_uint = 0x2E;
pub const ARIZONA_GP_FN_DSP_IRQ1: c_uint = 0x35;
pub const ARIZONA_GP_FN_DSP_IRQ2: c_uint = 0x36;
pub const ARIZONA_GP_FN_ASYNC_OPCLK: c_uint = 0x3D;
pub const ARIZONA_GP_FN_BOOT_DONE: c_uint = 0x44;
pub const ARIZONA_GP_FN_DSP1_RAM_READY: c_uint = 0x45;
pub const ARIZONA_GP_FN_SYSCLK_ENA_STATUS: c_uint = 0x4B;
pub const ARIZONA_GP_FN_ASYNCCLK_ENA_STATUS: c_uint = 0x4C;
// GPIO Configuration Bits
pub const ARIZONA_GPN_DIR: c_uint = 0x8000;
pub const ARIZONA_GPN_PU: c_uint = 0x4000;
pub const ARIZONA_GPN_PD: c_uint = 0x2000;
pub const ARIZONA_GPN_LVL: c_uint = 0x0800;
pub const ARIZONA_GPN_POL: c_uint = 0x0400;
pub const ARIZONA_GPN_OP_CFG: c_uint = 0x0200;
pub const ARIZONA_GPN_DB: c_uint = 0x0100;
// Provide some defines for the most common configs
pub const ARIZONA_GP_DEFAULT: c_uint = 0xffffffff;

pub const ARIZONA_32KZ_MCLK1: c_int = 1;
pub const ARIZONA_32KZ_MCLK2: c_int = 2;
pub const ARIZONA_32KZ_NONE: c_int = 3;
pub const ARIZONA_DMIC_MICVDD: c_int = 0;
pub const ARIZONA_DMIC_MICBIAS1: c_int = 1;
pub const ARIZONA_DMIC_MICBIAS2: c_int = 2;
pub const ARIZONA_DMIC_MICBIAS3: c_int = 3;
pub const ARIZONA_INMODE_DIFF: c_int = 0;
pub const ARIZONA_INMODE_SE: c_int = 1;
pub const ARIZONA_INMODE_DMIC: c_int = 2;
pub const ARIZONA_MICD_TIME_CONTINUOUS: c_int = 0;
pub const ARIZONA_MICD_TIME_250US: c_int = 1;
pub const ARIZONA_MICD_TIME_500US: c_int = 2;
pub const ARIZONA_MICD_TIME_1MS: c_int = 3;
pub const ARIZONA_MICD_TIME_2MS: c_int = 4;
pub const ARIZONA_MICD_TIME_4MS: c_int = 5;
pub const ARIZONA_MICD_TIME_8MS: c_int = 6;
pub const ARIZONA_MICD_TIME_16MS: c_int = 7;
pub const ARIZONA_MICD_TIME_32MS: c_int = 8;
pub const ARIZONA_MICD_TIME_64MS: c_int = 9;
pub const ARIZONA_MICD_TIME_128MS: c_int = 10;
pub const ARIZONA_MICD_TIME_256MS: c_int = 11;
pub const ARIZONA_MICD_TIME_512MS: c_int = 12;
pub const ARIZONA_ACCDET_MODE_MIC: c_int = 0;
pub const ARIZONA_ACCDET_MODE_HPL: c_int = 1;
pub const ARIZONA_ACCDET_MODE_HPR: c_int = 2;
pub const ARIZONA_ACCDET_MODE_HPM: c_int = 4;
pub const ARIZONA_ACCDET_MODE_ADC: c_int = 7;
pub const ARIZONA_GPSW_OPEN: c_int = 0;
pub const ARIZONA_GPSW_CLOSED: c_int = 1;
pub const ARIZONA_GPSW_CLAMP_ENABLED: c_int = 2;
pub const ARIZONA_GPSW_CLAMP_DISABLED: c_int = 3;

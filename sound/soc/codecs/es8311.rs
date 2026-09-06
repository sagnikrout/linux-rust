//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/es8311.h
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
// es8311.c -- es8311 ALSA SoC audio driver
//
// Copyright (C) 2024 Matteo Martelli <matteomartelli3@gmail.com>
//
// Author: Matteo Martelli <matteomartelli3@gmail.com>
//

pub const ES8311_RESET: c_uint = 0x00;

// Clock Manager Registers
pub const ES8311_CLKMGR1: c_uint = 0x01;

pub const ES8311_CLKMGR1_CLKADC_ON_SHIFT: c_int = 3;
pub const ES8311_CLKMGR1_CLKDAC_ON_SHIFT: c_int = 2;
pub const ES8311_CLKMGR1_ANACLKADC_ON_SHIFT: c_int = 1;
pub const ES8311_CLKMGR1_ANACLKDAC_ON_SHIFT: c_int = 0;
pub const ES8311_CLKMGR2: c_uint = 0x02;

pub const ES8311_CLKMGR2_DIV_PRE_SHIFT: c_int = 5;
pub const ES8311_CLKMGR2_DIV_PRE_MAX: c_uint = 0x07;

pub const ES8311_CLKMGR2_MULT_PRE_SHIFT: c_int = 3;
pub const ES8311_CLKMGR3: c_uint = 0x03;
pub const ES8311_CLKMGR4: c_uint = 0x04;
pub const ES8311_CLKMGR5: c_uint = 0x05;

pub const ES8311_CLKMGR5_ADC_DIV_SHIFT: c_int = 4;

pub const ES8311_CLKMGR5_DAC_DIV_SHIFT: c_int = 0;
pub const ES8311_CLKMGR6: c_uint = 0x06;

pub const ES8311_CLKMGR7: c_uint = 0x07;

pub const ES8311_CLKMGR8: c_uint = 0x08;
pub const ES8311_CLKMGR_LRCLK_DIV_MAX: c_uint = 0x0FFF;
// SDP Mode Registers
pub const ES8311_SDP_IN: c_uint = 0x09;
pub const ES8311_SDP_IN_SEL_SHIFT: c_int = 7;
pub const ES8311_SDP_OUT: c_uint = 0x0A;
// Following values are the same for both SPD_IN and SDP_OUT
pub const ES8311_SDP_MUTE_SHIFT: c_int = 6;

pub const ES8311_SDP_WL_SHIFT: c_int = 2;
pub const ES8311_SDP_WL_24: c_uint = 0x00;
pub const ES8311_SDP_WL_20: c_uint = 0x01;
pub const ES8311_SDP_WL_18: c_uint = 0x02;
pub const ES8311_SDP_WL_16: c_uint = 0x03;
pub const ES8311_SDP_WL_32: c_uint = 0x04;

pub const ES8311_SDP_FMT_I2S: c_uint = 0x00;
pub const ES8311_SDP_FMT_LEFT_J: c_uint = 0x01;
pub const ES8311_SDP_FMT_DSP: c_uint = 0x03;
// System registers
pub const ES8311_SYS1: c_uint = 0x0B;
pub const ES8311_SYS2: c_uint = 0x0C;
pub const ES8311_SYS3: c_uint = 0x0D;
pub const ES8311_SYS3_PDN_ANA_SHIFT: c_int = 7;
pub const ES8311_SYS3_PDN_IBIASGEN_SHIFT: c_int = 6;
pub const ES8311_SYS3_PDN_ADCBIASGEN_SHIFT: c_int = 5;
pub const ES8311_SYS3_PDN_ADCVREFGEN_SHIFT: c_int = 4;
pub const ES8311_SYS3_PDN_DACVREFGEN_SHIFT: c_int = 3;
pub const ES8311_SYS3_PDN_VREF_SHIFT: c_int = 2;

pub const ES8311_SYS3_PDN_VMIDSEL_POWER_DOWN: c_int = 0;
pub const ES8311_SYS3_PDN_VMIDSEL_STARTUP_NORMAL_SPEED: c_int = 1;
pub const ES8311_SYS3_PDN_VMIDSEL_NORMAL_OPERATION: c_int = 2;
pub const ES8311_SYS3_PDN_VMIDSEL_STARTUP_FAST_SPEED: c_int = 3;
pub const ES8311_SYS4: c_uint = 0x0E;
pub const ES8311_SYS4_PDN_PGA_SHIFT: c_int = 6;
pub const ES8311_SYS4_PDN_MOD_SHIFT: c_int = 5;
pub const ES8311_SYS5: c_uint = 0x0F;
pub const ES8311_SYS6: c_uint = 0x10;
pub const ES8311_SYS7: c_uint = 0x11;
pub const ES8311_SYS8: c_uint = 0x12;
pub const ES8311_SYS8_PDN_DAC_SHIFT: c_int = 1;
pub const ES8311_SYS9: c_uint = 0x13;
pub const ES8311_SYS9_HPSW_SHIFT: c_int = 4;
pub const ES8311_SYS10: c_uint = 0x14;
pub const ES8311_SYS10_DMIC_ON_SHIFT: c_int = 6;
pub const ES8311_SYS10_LINESEL_SHIFT: c_int = 4;
pub const ES8311_SYS10_PGAGAIN_SHIFT: c_int = 0;
pub const ES8311_SYS10_PGAGAIN_MAX: c_uint = 0x0A;
// ADC Registers
pub const ES8311_ADC1: c_uint = 0x15;
pub const ES8311_ADC1_RAMPRATE_SHIFT: c_int = 4;
pub const ES8311_ADC2: c_uint = 0x16;
pub const ES8311_ADC2_INV_SHIFT: c_int = 4;
pub const ES8311_ADC2_SCALE_SHIFT: c_int = 0;
pub const ES8311_ADC2_SCALE_MAX: c_uint = 0x07;
pub const ES8311_ADC3: c_uint = 0x17;
pub const ES8311_ADC3_VOLUME_SHIFT: c_int = 0;
pub const ES8311_ADC3_VOLUME_MAX: c_uint = 0xFF;
pub const ES8311_ADC4: c_uint = 0x18;
pub const ES8311_ADC4_ALC_EN_SHIFT: c_int = 7;
pub const ES8311_ADC4_AUTOMUTE_EN_SHIFT: c_int = 6;
pub const ES8311_ADC4_ALC_WINSIZE_SHIFT: c_int = 0;
pub const ES8311_ADC5: c_uint = 0x19;
pub const ES8311_ADC5_ALC_MAXLEVEL_SHIFT: c_int = 4;
pub const ES8311_ADC5_ALC_MAXLEVEL_MAX: c_uint = 0x0F;
pub const ES8311_ADC5_ALC_MINLEVEL_SHIFT: c_int = 0;
pub const ES8311_ADC5_ALC_MINLEVEL_MAX: c_uint = 0x0F;
pub const ES8311_ADC6: c_uint = 0x1A;
pub const ES8311_ADC6_AUTOMUTE_WS_SHIFT: c_int = 4;
pub const ES8311_ADC6_AUTOMUTE_NG_SHIFT: c_int = 0;
pub const ES8311_ADC6_AUTOMUTE_NG_MAX: c_uint = 0x0F;
pub const ES8311_ADC7: c_uint = 0x1B;
pub const ES8311_ADC7_AUTOMUTE_VOL_SHIFT: c_int = 5;
pub const ES8311_ADC7_AUTOMUTE_VOL_MAX: c_uint = 0x07;
pub const ES8311_ADC8: c_uint = 0x1C;
pub const ES8311_ADC8_EQBYPASS_SHIFT: c_int = 6;
pub const ES8311_ADC8_HPF_SHIFT: c_int = 5;
// DAC Registers
pub const ES8311_DAC1: c_uint = 0x31;

pub const ES8311_DAC2: c_uint = 0x32;
pub const ES8311_DAC2_VOLUME_MAX: c_uint = 0xFF;
pub const ES8311_DAC3: c_uint = 0x33;
pub const ES8311_DAC4: c_uint = 0x34;
pub const ES8311_DAC4_DRC_EN_SHIFT: c_int = 7;
pub const ES8311_DAC4_DRC_WINSIZE_SHIFT: c_int = 0;
pub const ES8311_DAC5: c_uint = 0x35;
pub const ES8311_DAC5_DRC_MAXLEVEL_SHIFT: c_int = 4;
pub const ES8311_DAC5_DRC_MAXLEVEL_MAX: c_uint = 0x0F;
pub const ES8311_DAC5_DRC_MINLEVEL_SHIFT: c_int = 0;
pub const ES8311_DAC5_DRC_MINLEVEL_MAX: c_uint = 0x0F;
pub const ES8311_DAC6: c_uint = 0x37;
pub const ES8311_DAC6_RAMPRATE_SHIFT: c_int = 4;
pub const ES8311_DAC6_EQBYPASS_SHIFT: c_int = 3;
// GPIO Registers
pub const ES8311_GPIO: c_uint = 0x44;
pub const ES8311_GPIO_ADC2DAC_SEL_SHIFT: c_int = 7;
pub const ES8311_GPIO_ADCDAT_SEL_SHIFT: c_int = 4;
// Chip Info Registers
pub const ES8311_CHIPID1: c_uint = 0xFD /* 0x83 */;
pub const ES8311_CHIPID2: c_uint = 0xFE /* 0x11 */;
pub const ES8311_CHIPVER: c_uint = 0xFF;
pub const ES8311_REG_MAX: c_uint = 0xFF;

//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt286.h
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
// rt286.h  --  RT286 ALSA SoC audio driver
//
// Copyright 2011 Realtek Microelectronics
// Author: Johnny Hsu <johnnyhsu@realtek.com>
//

pub const RT286_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT286_DAC_OUT1: c_uint = 0x02;
pub const RT286_DAC_OUT2: c_uint = 0x03;
pub const RT286_ADC_IN1: c_uint = 0x09;
pub const RT286_ADC_IN2: c_uint = 0x08;
pub const RT286_MIXER_IN: c_uint = 0x0b;
pub const RT286_MIXER_OUT1: c_uint = 0x0c;
pub const RT286_MIXER_OUT2: c_uint = 0x0d;
pub const RT286_DMIC1: c_uint = 0x12;
pub const RT286_DMIC2: c_uint = 0x13;
pub const RT286_SPK_OUT: c_uint = 0x14;
pub const RT286_MIC1: c_uint = 0x18;
pub const RT286_LINE1: c_uint = 0x1a;
pub const RT286_BEEP: c_uint = 0x1d;
pub const RT286_SPDIF: c_uint = 0x1e;
pub const RT286_VENDOR_REGISTERS: c_uint = 0x20;
pub const RT286_HP_OUT: c_uint = 0x21;
pub const RT286_MIXER_IN1: c_uint = 0x22;
pub const RT286_MIXER_IN2: c_uint = 0x23;
pub const RT286_SET_PIN_SFT: c_int = 6;
pub const RT286_SET_PIN_ENABLE: c_uint = 0x40;
pub const RT286_SET_PIN_DISABLE: c_int = 0;
pub const RT286_SET_EAPD_HIGH: c_uint = 0x2;
pub const RT286_SET_EAPD_LOW: c_int = 0;
pub const RT286_MUTE_SFT: c_int = 7;
// Verb commands

// Macro flag: #define RT286_SPK_MUX\
// Macro flag: #define RT286_HPO_MUX\
// Macro flag: #define RT286_ADC0_MUX\
// Macro flag: #define RT286_ADC1_MUX\
// Macro flag: #define RT286_SET_MIC1\
// Macro flag: #define RT286_SET_PIN_HPO\
// Macro flag: #define RT286_SET_PIN_SPK\
// Macro flag: #define RT286_SET_PIN_DMIC1\
// Macro flag: #define RT286_SPK_EAPD\
// Macro flag: #define RT286_SET_AMP_GAIN_HPO\
// Macro flag: #define RT286_SET_AMP_GAIN_ADC_IN1\
// Macro flag: #define RT286_SET_AMP_GAIN_ADC_IN2\
// Macro flag: #define RT286_GET_HP_SENSE\
// Macro flag: #define RT286_GET_MIC1_SENSE\
// Macro flag: #define RT286_SET_DMIC2_DEFAULT\
// Macro flag: #define RT286_DACL_GAIN\
// Macro flag: #define RT286_DACR_GAIN\
// Macro flag: #define RT286_ADCL_GAIN\
// Macro flag: #define RT286_ADCR_GAIN\
// Macro flag: #define RT286_MIC_GAIN\
// Macro flag: #define RT286_SPOL_GAIN\
// Macro flag: #define RT286_SPOR_GAIN\
// Macro flag: #define RT286_HPOL_GAIN\
// Macro flag: #define RT286_HPOR_GAIN\
// Macro flag: #define RT286_F_DAC_SWITCH\
// Macro flag: #define RT286_F_RECMIX_SWITCH\
// Macro flag: #define RT286_REC_MIC_SWITCH\
// Macro flag: #define RT286_REC_I2S_SWITCH\
// Macro flag: #define RT286_REC_LINE_SWITCH\
// Macro flag: #define RT286_REC_BEEP_SWITCH\
// Macro flag: #define RT286_DAC_FORMAT\
// Macro flag: #define RT286_ADC_FORMAT\
// Macro flag: #define RT286_COEF_INDEX\
// Macro flag: #define RT286_PROC_COEF\
// Macro flag: #define RT286_SET_GPIO_MASK\
// Macro flag: #define RT286_SET_GPIO_DIRECTION\
// Macro flag: #define RT286_SET_GPIO_DATA\
// Index registers
pub const RT286_A_BIAS_CTRL1: c_uint = 0x01;
pub const RT286_A_BIAS_CTRL2: c_uint = 0x02;
pub const RT286_POWER_CTRL1: c_uint = 0x03;
pub const RT286_A_BIAS_CTRL3: c_uint = 0x04;
pub const RT286_POWER_CTRL2: c_uint = 0x08;
pub const RT286_I2S_CTRL1: c_uint = 0x09;
pub const RT286_I2S_CTRL2: c_uint = 0x0a;
pub const RT286_CLK_DIV: c_uint = 0x0b;
pub const RT286_DC_GAIN: c_uint = 0x0d;
pub const RT286_POWER_CTRL3: c_uint = 0x0f;
pub const RT286_MIC1_DET_CTRL: c_uint = 0x19;
pub const RT286_MISC_CTRL1: c_uint = 0x20;
pub const RT286_GPIO_CTRL: c_uint = 0x29;
pub const RT286_IRQ_CTRL: c_uint = 0x33;
pub const RT286_PLL_CTRL1: c_uint = 0x49;
pub const RT286_CBJ_CTRL1: c_uint = 0x4f;
pub const RT286_CBJ_CTRL2: c_uint = 0x50;
pub const RT286_PLL_CTRL: c_uint = 0x63;
pub const RT286_DEPOP_CTRL1: c_uint = 0x66;
pub const RT286_DEPOP_CTRL2: c_uint = 0x67;
pub const RT286_DEPOP_CTRL3: c_uint = 0x68;
pub const RT286_DEPOP_CTRL4: c_uint = 0x69;
// SPDIF (0x06)
pub const RT286_SPDIF_SEL_SFT: c_int = 0;
pub const RT286_SPDIF_SEL_PCM0: c_int = 0;
pub const RT286_SPDIF_SEL_PCM1: c_int = 1;
pub const RT286_SPDIF_SEL_SPOUT: c_int = 2;
pub const RT286_SPDIF_SEL_PP: c_int = 3;
// RECMIX (0x0b)
pub const RT286_M_REC_BEEP_SFT: c_int = 0;
pub const RT286_M_REC_LINE1_SFT: c_int = 1;
pub const RT286_M_REC_MIC1_SFT: c_int = 2;
pub const RT286_M_REC_I2S_SFT: c_int = 3;
// Front (0x0c)
pub const RT286_M_FRONT_DAC_SFT: c_int = 0;
pub const RT286_M_FRONT_REC_SFT: c_int = 1;
// SPK-OUT (0x14)
pub const RT286_M_SPK_MUX_SFT: c_int = 14;
pub const RT286_SPK_SEL_MASK: c_uint = 0x1;
pub const RT286_SPK_SEL_SFT: c_int = 0;
pub const RT286_SPK_SEL_F: c_int = 0;
pub const RT286_SPK_SEL_S: c_int = 1;
// HP-OUT (0x21)
pub const RT286_M_HP_MUX_SFT: c_int = 14;
pub const RT286_HP_SEL_MASK: c_uint = 0x1;
pub const RT286_HP_SEL_SFT: c_int = 0;
pub const RT286_HP_SEL_F: c_int = 0;
pub const RT286_HP_SEL_S: c_int = 1;
// ADC (0x22) (0x23)
pub const RT286_ADC_SEL_MASK: c_uint = 0x7;
pub const RT286_ADC_SEL_SFT: c_int = 0;
pub const RT286_ADC_SEL_SURR: c_int = 0;
pub const RT286_ADC_SEL_FRONT: c_int = 1;
pub const RT286_ADC_SEL_DMIC: c_int = 2;
pub const RT286_ADC_SEL_BEEP: c_int = 4;
pub const RT286_ADC_SEL_LINE1: c_int = 5;
pub const RT286_ADC_SEL_I2S: c_int = 6;
pub const RT286_ADC_SEL_MIC1: c_int = 7;
pub const RT286_SCLK_S_MCLK: c_int = 0;
pub const RT286_SCLK_S_PLL: c_int = 1;

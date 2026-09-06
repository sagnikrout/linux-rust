//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt298.h
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
// rt298.h  --  RT298 ALSA SoC audio driver
//
// Copyright 2011 Realtek Microelectronics
// Author: Johnny Hsu <johnnyhsu@realtek.com>
//

pub const RT298_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT298_DAC_OUT1: c_uint = 0x02;
pub const RT298_DAC_OUT2: c_uint = 0x03;
pub const RT298_DIG_CVT: c_uint = 0x06;
pub const RT298_ADC_IN1: c_uint = 0x09;
pub const RT298_ADC_IN2: c_uint = 0x08;
pub const RT298_MIXER_IN: c_uint = 0x0b;
pub const RT298_MIXER_OUT1: c_uint = 0x0c;
pub const RT298_MIXER_OUT2: c_uint = 0x0d;
pub const RT298_DMIC1: c_uint = 0x12;
pub const RT298_DMIC2: c_uint = 0x13;
pub const RT298_SPK_OUT: c_uint = 0x14;
pub const RT298_MIC1: c_uint = 0x18;
pub const RT298_LINE1: c_uint = 0x1a;
pub const RT298_BEEP: c_uint = 0x1d;
pub const RT298_SPDIF: c_uint = 0x1e;
pub const RT298_VENDOR_REGISTERS: c_uint = 0x20;
pub const RT298_HP_OUT: c_uint = 0x21;
pub const RT298_MIXER_IN1: c_uint = 0x22;
pub const RT298_MIXER_IN2: c_uint = 0x23;
pub const RT298_INLINE_CMD: c_uint = 0x55;
pub const RT298_SET_PIN_SFT: c_int = 6;
pub const RT298_SET_PIN_ENABLE: c_uint = 0x40;
pub const RT298_SET_PIN_DISABLE: c_int = 0;
pub const RT298_SET_EAPD_HIGH: c_uint = 0x2;
pub const RT298_SET_EAPD_LOW: c_int = 0;
pub const RT298_MUTE_SFT: c_int = 7;
// Verb commands

// Macro flag: #define RT298_SPK_MUX\
// Macro flag: #define RT298_HPO_MUX\
// Macro flag: #define RT298_ADC0_MUX\
// Macro flag: #define RT298_ADC1_MUX\
// Macro flag: #define RT298_SET_MIC1\
// Macro flag: #define RT298_SET_PIN_HPO\
// Macro flag: #define RT298_SET_PIN_SPK\
// Macro flag: #define RT298_SET_PIN_DMIC1\
// Macro flag: #define RT298_SET_PIN_SPDIF\
// Macro flag: #define RT298_SET_PIN_DIG_CVT\
// Macro flag: #define RT298_SPK_EAPD\
// Macro flag: #define RT298_SET_AMP_GAIN_HPO\
// Macro flag: #define RT298_SET_AMP_GAIN_ADC_IN1\
// Macro flag: #define RT298_SET_AMP_GAIN_ADC_IN2\
// Macro flag: #define RT298_GET_HP_SENSE\
// Macro flag: #define RT298_GET_MIC1_SENSE\
// Macro flag: #define RT298_SET_DMIC2_DEFAULT\
// Macro flag: #define RT298_SET_SPDIF_DEFAULT\
// Macro flag: #define RT298_DACL_GAIN\
// Macro flag: #define RT298_DACR_GAIN\
// Macro flag: #define RT298_ADCL_GAIN\
// Macro flag: #define RT298_ADCR_GAIN\
// Macro flag: #define RT298_MIC_GAIN\
// Macro flag: #define RT298_SPOL_GAIN\
// Macro flag: #define RT298_SPOR_GAIN\
// Macro flag: #define RT298_HPOL_GAIN\
// Macro flag: #define RT298_HPOR_GAIN\
// Macro flag: #define RT298_F_DAC_SWITCH\
// Macro flag: #define RT298_F_RECMIX_SWITCH\
// Macro flag: #define RT298_REC_MIC_SWITCH\
// Macro flag: #define RT298_REC_I2S_SWITCH\
// Macro flag: #define RT298_REC_LINE_SWITCH\
// Macro flag: #define RT298_REC_BEEP_SWITCH\
// Macro flag: #define RT298_DAC_FORMAT\
// Macro flag: #define RT298_ADC_FORMAT\
// Macro flag: #define RT298_COEF_INDEX\
// Macro flag: #define RT298_PROC_COEF\
// Macro flag: #define RT298_UNSOLICITED_INLINE_CMD\
// Macro flag: #define RT298_UNSOLICITED_HP_OUT\
// Macro flag: #define RT298_UNSOLICITED_MIC1\
// Index registers
pub const RT298_A_BIAS_CTRL1: c_uint = 0x01;
pub const RT298_A_BIAS_CTRL2: c_uint = 0x02;
pub const RT298_POWER_CTRL1: c_uint = 0x03;
pub const RT298_A_BIAS_CTRL3: c_uint = 0x04;
pub const RT298_D_FILTER_CTRL: c_uint = 0x05;
pub const RT298_POWER_CTRL2: c_uint = 0x08;
pub const RT298_I2S_CTRL1: c_uint = 0x09;
pub const RT298_I2S_CTRL2: c_uint = 0x0a;
pub const RT298_CLK_DIV: c_uint = 0x0b;
pub const RT298_DC_GAIN: c_uint = 0x0d;
pub const RT298_POWER_CTRL3: c_uint = 0x0f;
pub const RT298_MIC1_DET_CTRL: c_uint = 0x19;
pub const RT298_MISC_CTRL1: c_uint = 0x20;
pub const RT298_IRQ_CTRL: c_uint = 0x33;
pub const RT298_WIND_FILTER_CTRL: c_uint = 0x46;
pub const RT298_PLL_CTRL1: c_uint = 0x49;
pub const RT298_VAD_CTRL: c_uint = 0x4e;
pub const RT298_CBJ_CTRL1: c_uint = 0x4f;
pub const RT298_CBJ_CTRL2: c_uint = 0x50;
pub const RT298_PLL_CTRL: c_uint = 0x63;
pub const RT298_DEPOP_CTRL1: c_uint = 0x66;
pub const RT298_DEPOP_CTRL2: c_uint = 0x67;
pub const RT298_DEPOP_CTRL3: c_uint = 0x68;
pub const RT298_DEPOP_CTRL4: c_uint = 0x69;
pub const RT298_IRQ_FLAG_CTRL: c_uint = 0x7c;
// SPDIF (0x06)
pub const RT298_SPDIF_SEL_SFT: c_int = 0;
pub const RT298_SPDIF_SEL_PCM0: c_int = 0;
pub const RT298_SPDIF_SEL_PCM1: c_int = 1;
pub const RT298_SPDIF_SEL_SPOUT: c_int = 2;
pub const RT298_SPDIF_SEL_PP: c_int = 3;
// RECMIX (0x0b)
pub const RT298_M_REC_BEEP_SFT: c_int = 0;
pub const RT298_M_REC_LINE1_SFT: c_int = 1;
pub const RT298_M_REC_MIC1_SFT: c_int = 2;
pub const RT298_M_REC_I2S_SFT: c_int = 3;
// Front (0x0c)
pub const RT298_M_FRONT_DAC_SFT: c_int = 0;
pub const RT298_M_FRONT_REC_SFT: c_int = 1;
// SPK-OUT (0x14)
pub const RT298_M_SPK_MUX_SFT: c_int = 14;
pub const RT298_SPK_SEL_MASK: c_uint = 0x1;
pub const RT298_SPK_SEL_SFT: c_int = 0;
pub const RT298_SPK_SEL_F: c_int = 0;
pub const RT298_SPK_SEL_S: c_int = 1;
// HP-OUT (0x21)
pub const RT298_M_HP_MUX_SFT: c_int = 14;
pub const RT298_HP_SEL_MASK: c_uint = 0x1;
pub const RT298_HP_SEL_SFT: c_int = 0;
pub const RT298_HP_SEL_F: c_int = 0;
pub const RT298_HP_SEL_S: c_int = 1;
// ADC (0x22) (0x23)
pub const RT298_ADC_SEL_MASK: c_uint = 0x7;
pub const RT298_ADC_SEL_SFT: c_int = 0;
pub const RT298_ADC_SEL_SURR: c_int = 0;
pub const RT298_ADC_SEL_FRONT: c_int = 1;
pub const RT298_ADC_SEL_DMIC: c_int = 2;
pub const RT298_ADC_SEL_BEEP: c_int = 4;
pub const RT298_ADC_SEL_LINE1: c_int = 5;
pub const RT298_ADC_SEL_I2S: c_int = 6;
pub const RT298_ADC_SEL_MIC1: c_int = 7;
pub const RT298_SCLK_S_MCLK: c_int = 0;
pub const RT298_SCLK_S_PLL: c_int = 1;

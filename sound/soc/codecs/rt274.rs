//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt274.h
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
// rt274.h  --  RT274 ALSA SoC audio driver
//
// Copyright 2016 Realtek Microelectronics
// Author: Bard Liao <bardliao@realtek.com>
//

pub const RT274_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT274_DAC_OUT0: c_uint = 0x02;
pub const RT274_DAC_OUT1: c_uint = 0x03;
pub const RT274_ADC_IN2: c_uint = 0x08;
pub const RT274_ADC_IN1: c_uint = 0x09;
pub const RT274_DIG_CVT: c_uint = 0x0a;
pub const RT274_DMIC1: c_uint = 0x12;
pub const RT274_DMIC2: c_uint = 0x13;
pub const RT274_MIC: c_uint = 0x19;
pub const RT274_LINE1: c_uint = 0x1a;
pub const RT274_LINE2: c_uint = 0x1b;
pub const RT274_LINE3: c_uint = 0x16;
pub const RT274_SPDIF: c_uint = 0x1e;
pub const RT274_VENDOR_REGISTERS: c_uint = 0x20;
pub const RT274_HP_OUT: c_uint = 0x21;
pub const RT274_MIXER_IN1: c_uint = 0x22;
pub const RT274_MIXER_IN2: c_uint = 0x23;
pub const RT274_INLINE_CMD: c_uint = 0x55;
pub const RT274_SET_PIN_SFT: c_int = 6;
pub const RT274_SET_PIN_ENABLE: c_uint = 0x40;
pub const RT274_SET_PIN_DISABLE: c_int = 0;
pub const RT274_SET_EAPD_HIGH: c_uint = 0x2;
pub const RT274_SET_EAPD_LOW: c_int = 0;
pub const RT274_MUTE_SFT: c_int = 7;
// Verb commands
// Macro flag: #define RT274_RESET\

// Macro flag: #define RT274_LOUT_MUX\
// Macro flag: #define RT274_HPO_MUX\
// Macro flag: #define RT274_ADC0_MUX\
// Macro flag: #define RT274_ADC1_MUX\
// Macro flag: #define RT274_SET_MIC\
// Macro flag: #define RT274_SET_PIN_LOUT3\
// Macro flag: #define RT274_SET_PIN_HPO\
// Macro flag: #define RT274_SET_PIN_DMIC1\
// Macro flag: #define RT274_SET_PIN_SPDIF\
// Macro flag: #define RT274_SET_PIN_DIG_CVT\
// Macro flag: #define RT274_SET_AMP_GAIN_HPO\
// Macro flag: #define RT274_SET_AMP_GAIN_ADC_IN1\
// Macro flag: #define RT274_SET_AMP_GAIN_ADC_IN2\
// Macro flag: #define RT274_GET_HP_SENSE\
// Macro flag: #define RT274_GET_MIC_SENSE\
// Macro flag: #define RT274_SET_DMIC2_DEFAULT\
// Macro flag: #define RT274_SET_SPDIF_DEFAULT\
// Macro flag: #define RT274_DAC0L_GAIN\
// Macro flag: #define RT274_DAC0R_GAIN\
// Macro flag: #define RT274_DAC1L_GAIN\
// Macro flag: #define RT274_DAC1R_GAIN\
// Macro flag: #define RT274_ADCL_GAIN\
// Macro flag: #define RT274_ADCR_GAIN\
// Macro flag: #define RT274_MIC_GAIN\
// Macro flag: #define RT274_LOUTL_GAIN\
// Macro flag: #define RT274_LOUTR_GAIN\
// Macro flag: #define RT274_HPOL_GAIN\
// Macro flag: #define RT274_HPOR_GAIN\
// Macro flag: #define RT274_DAC_FORMAT\
// Macro flag: #define RT274_ADC_FORMAT\
// Macro flag: #define RT274_COEF_INDEX\
// Macro flag: #define RT274_PROC_COEF\
// Macro flag: #define RT274_UNSOLICITED_INLINE_CMD\
// Macro flag: #define RT274_UNSOLICITED_HP_OUT\
// Macro flag: #define RT274_UNSOLICITED_MIC\
// Macro flag: #define RT274_COEF58_INDEX\
// Macro flag: #define RT274_COEF58_COEF\
// Macro flag: #define RT274_COEF5b_INDEX\
// Macro flag: #define RT274_COEF5b_COEF\
// Macro flag: #define RT274_SET_STREAMID_DAC0\
// Macro flag: #define RT274_SET_STREAMID_DAC1\
// Macro flag: #define RT274_SET_STREAMID_ADC1\
// Macro flag: #define RT274_SET_STREAMID_ADC2\
// Index registers
pub const RT274_EAPD_GPIO_IRQ_CTRL: c_uint = 0x10;
pub const RT274_PAD_CTRL12: c_uint = 0x35;
pub const RT274_I2S_CTRL1: c_uint = 0x63;
pub const RT274_I2S_CTRL2: c_uint = 0x64;
pub const RT274_MCLK_CTRL: c_uint = 0x71;
pub const RT274_CLK_CTRL: c_uint = 0x72;
pub const RT274_PLL2_CTRL: c_uint = 0x7b;
// EAPD GPIO IRQ control (Index 0x10)

// Front I2S_Interface control 1 (Index 0x63)

// MCLK clock domain control (Index 0x71)

// Clock control (Index 0x72)

// PLL2 control (Index 0x7b)

// HP-OUT (0x21)
pub const RT274_M_HP_MUX_SFT: c_int = 14;
pub const RT274_HP_SEL_MASK: c_uint = 0x1;
pub const RT274_HP_SEL_SFT: c_int = 0;
pub const RT274_HP_SEL_F: c_int = 0;
pub const RT274_HP_SEL_S: c_int = 1;
// ADC (0x22) (0x23)
pub const RT274_ADC_SEL_MASK: c_uint = 0x7;
pub const RT274_ADC_SEL_SFT: c_int = 0;
pub const RT274_ADC_SEL_MIC: c_int = 0;
pub const RT274_ADC_SEL_LINE1: c_int = 1;
pub const RT274_ADC_SEL_LINE2: c_int = 2;
pub const RT274_ADC_SEL_DMIC: c_int = 3;
pub const RT274_SCLK_S_MCLK: c_int = 0;
pub const RT274_SCLK_S_PLL1: c_int = 1;
pub const RT274_SCLK_S_PLL2: c_int = 2;
pub const RT274_PLL2_S_MCLK: c_int = 0;
pub const RT274_PLL2_S_BCLK: c_int = 1;

//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/da7213.h
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
// da7213.h - DA7213 ASoC Codec Driver
//
// Copyright (c) 2013 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
// Author: David Rau <David.Rau.opensource@dm.renesas.com>
//

//
// Registers
//
// Status Registers
pub const DA7213_STATUS1: c_uint = 0x02;
pub const DA7213_PLL_STATUS: c_uint = 0x03;
pub const DA7213_AUX_L_GAIN_STATUS: c_uint = 0x04;
pub const DA7213_AUX_R_GAIN_STATUS: c_uint = 0x05;
pub const DA7213_MIC_1_GAIN_STATUS: c_uint = 0x06;
pub const DA7213_MIC_2_GAIN_STATUS: c_uint = 0x07;
pub const DA7213_MIXIN_L_GAIN_STATUS: c_uint = 0x08;
pub const DA7213_MIXIN_R_GAIN_STATUS: c_uint = 0x09;
pub const DA7213_ADC_L_GAIN_STATUS: c_uint = 0x0A;
pub const DA7213_ADC_R_GAIN_STATUS: c_uint = 0x0B;
pub const DA7213_DAC_L_GAIN_STATUS: c_uint = 0x0C;
pub const DA7213_DAC_R_GAIN_STATUS: c_uint = 0x0D;
pub const DA7213_HP_L_GAIN_STATUS: c_uint = 0x0E;
pub const DA7213_HP_R_GAIN_STATUS: c_uint = 0x0F;
pub const DA7213_LINE_GAIN_STATUS: c_uint = 0x10;
// System Initialisation Registers
pub const DA7213_DIG_ROUTING_DAI: c_uint = 0x21;
pub const DA7213_SR: c_uint = 0x22;
pub const DA7213_REFERENCES: c_uint = 0x23;
pub const DA7213_PLL_FRAC_TOP: c_uint = 0x24;
pub const DA7213_PLL_FRAC_BOT: c_uint = 0x25;
pub const DA7213_PLL_INTEGER: c_uint = 0x26;
pub const DA7213_PLL_CTRL: c_uint = 0x27;
pub const DA7213_DAI_CLK_MODE: c_uint = 0x28;
pub const DA7213_DAI_CTRL: c_uint = 0x29;
pub const DA7213_DIG_ROUTING_DAC: c_uint = 0x2A;
pub const DA7213_ALC_CTRL1: c_uint = 0x2B;
// Input - Gain, Select and Filter Registers
pub const DA7213_AUX_L_GAIN: c_uint = 0x30;
pub const DA7213_AUX_R_GAIN: c_uint = 0x31;
pub const DA7213_MIXIN_L_SELECT: c_uint = 0x32;
pub const DA7213_MIXIN_R_SELECT: c_uint = 0x33;
pub const DA7213_MIXIN_L_GAIN: c_uint = 0x34;
pub const DA7213_MIXIN_R_GAIN: c_uint = 0x35;
pub const DA7213_ADC_L_GAIN: c_uint = 0x36;
pub const DA7213_ADC_R_GAIN: c_uint = 0x37;
pub const DA7213_ADC_FILTERS1: c_uint = 0x38;
pub const DA7213_MIC_1_GAIN: c_uint = 0x39;
pub const DA7213_MIC_2_GAIN: c_uint = 0x3A;
// Output - Gain, Select and Filter Registers
pub const DA7213_DAC_FILTERS5: c_uint = 0x40;
pub const DA7213_DAC_FILTERS2: c_uint = 0x41;
pub const DA7213_DAC_FILTERS3: c_uint = 0x42;
pub const DA7213_DAC_FILTERS4: c_uint = 0x43;
pub const DA7213_DAC_FILTERS1: c_uint = 0x44;
pub const DA7213_DAC_L_GAIN: c_uint = 0x45;
pub const DA7213_DAC_R_GAIN: c_uint = 0x46;
pub const DA7213_CP_CTRL: c_uint = 0x47;
pub const DA7213_HP_L_GAIN: c_uint = 0x48;
pub const DA7213_HP_R_GAIN: c_uint = 0x49;
pub const DA7213_LINE_GAIN: c_uint = 0x4A;
pub const DA7213_MIXOUT_L_SELECT: c_uint = 0x4B;
pub const DA7213_MIXOUT_R_SELECT: c_uint = 0x4C;
// System Controller Registers
pub const DA7213_SYSTEM_MODES_INPUT: c_uint = 0x50;
pub const DA7213_SYSTEM_MODES_OUTPUT: c_uint = 0x51;
// Control Registers
pub const DA7213_AUX_L_CTRL: c_uint = 0x60;
pub const DA7213_AUX_R_CTRL: c_uint = 0x61;
pub const DA7213_MICBIAS_CTRL: c_uint = 0x62;
pub const DA7213_MIC_1_CTRL: c_uint = 0x63;
pub const DA7213_MIC_2_CTRL: c_uint = 0x64;
pub const DA7213_MIXIN_L_CTRL: c_uint = 0x65;
pub const DA7213_MIXIN_R_CTRL: c_uint = 0x66;
pub const DA7213_ADC_L_CTRL: c_uint = 0x67;
pub const DA7213_ADC_R_CTRL: c_uint = 0x68;
pub const DA7213_DAC_L_CTRL: c_uint = 0x69;
pub const DA7213_DAC_R_CTRL: c_uint = 0x6A;
pub const DA7213_HP_L_CTRL: c_uint = 0x6B;
pub const DA7213_HP_R_CTRL: c_uint = 0x6C;
pub const DA7213_LINE_CTRL: c_uint = 0x6D;
pub const DA7213_MIXOUT_L_CTRL: c_uint = 0x6E;
pub const DA7213_MIXOUT_R_CTRL: c_uint = 0x6F;
// Configuration Registers
pub const DA7213_LDO_CTRL: c_uint = 0x90;
pub const DA7213_IO_CTRL: c_uint = 0x91;
pub const DA7213_GAIN_RAMP_CTRL: c_uint = 0x92;
pub const DA7213_MIC_CONFIG: c_uint = 0x93;
pub const DA7213_PC_COUNT: c_uint = 0x94;
pub const DA7213_CP_VOL_THRESHOLD1: c_uint = 0x95;
pub const DA7213_CP_DELAY: c_uint = 0x96;
pub const DA7213_CP_DETECTOR: c_uint = 0x97;
pub const DA7213_DAI_OFFSET: c_uint = 0x98;
pub const DA7213_DIG_CTRL: c_uint = 0x99;
pub const DA7213_ALC_CTRL2: c_uint = 0x9A;
pub const DA7213_ALC_CTRL3: c_uint = 0x9B;
pub const DA7213_ALC_NOISE: c_uint = 0x9C;
pub const DA7213_ALC_TARGET_MIN: c_uint = 0x9D;
pub const DA7213_ALC_TARGET_MAX: c_uint = 0x9E;
pub const DA7213_ALC_GAIN_LIMITS: c_uint = 0x9F;
pub const DA7213_ALC_ANA_GAIN_LIMITS: c_uint = 0xA0;
pub const DA7213_ALC_ANTICLIP_CTRL: c_uint = 0xA1;
pub const DA7213_ALC_ANTICLIP_LEVEL: c_uint = 0xA2;
pub const DA7213_ALC_OFFSET_AUTO_M_L: c_uint = 0xA3;
pub const DA7213_ALC_OFFSET_AUTO_U_L: c_uint = 0xA4;
pub const DA7213_ALC_OFFSET_MAN_M_L: c_uint = 0xA6;
pub const DA7213_ALC_OFFSET_MAN_U_L: c_uint = 0xA7;
pub const DA7213_ALC_OFFSET_AUTO_M_R: c_uint = 0xA8;
pub const DA7213_ALC_OFFSET_AUTO_U_R: c_uint = 0xA9;
pub const DA7213_ALC_OFFSET_MAN_M_R: c_uint = 0xAB;
pub const DA7213_ALC_OFFSET_MAN_U_R: c_uint = 0xAC;
pub const DA7213_ALC_CIC_OP_LVL_CTRL: c_uint = 0xAD;
pub const DA7213_ALC_CIC_OP_LVL_DATA: c_uint = 0xAE;
pub const DA7213_DAC_NG_SETUP_TIME: c_uint = 0xAF;
pub const DA7213_DAC_NG_OFF_THRESHOLD: c_uint = 0xB0;
pub const DA7213_DAC_NG_ON_THRESHOLD: c_uint = 0xB1;
pub const DA7213_DAC_NG_CTRL: c_uint = 0xB2;
pub const DA7213_TONE_GEN_CFG1: c_uint = 0xB4;
pub const DA7213_TONE_GEN_CFG2: c_uint = 0xB5;
pub const DA7213_TONE_GEN_CYCLES: c_uint = 0xB6;
pub const DA7213_TONE_GEN_FREQ1_L: c_uint = 0xB7;
pub const DA7213_TONE_GEN_FREQ1_U: c_uint = 0xB8;
pub const DA7213_TONE_GEN_FREQ2_L: c_uint = 0xB9;
pub const DA7213_TONE_GEN_FREQ2_U: c_uint = 0xBA;
pub const DA7213_TONE_GEN_ON_PER: c_uint = 0xBB;
pub const DA7213_TONE_GEN_OFF_PER: c_uint = 0xBC;
//
// Bit fields
//
pub const DA7213_SWITCH_EN_MAX: c_uint = 0x1;
// DA7213_PLL_STATUS = 0x03

// DA7213_SR = 0x22

// DA7213_REFERENCES = 0x23

// DA7213_PLL_CTRL = 0x27

// DA7213_DAI_CLK_MODE = 0x28

// DA7213_DAI_CTRL = 0x29

pub const DA7213_DAI_EN_SHIFT: c_int = 7;
// DA7213_DIG_ROUTING_DAI = 0x21
pub const DA7213_DAI_L_SRC_SHIFT: c_int = 0;
pub const DA7213_DAI_R_SRC_SHIFT: c_int = 4;
pub const DA7213_DAI_SRC_MAX: c_int = 4;
// DA7213_DIG_ROUTING_DAC = 0x2A
pub const DA7213_DAC_L_SRC_SHIFT: c_int = 0;
pub const DA7213_DAC_L_MONO_SHIFT: c_int = 3;
pub const DA7213_DAC_R_SRC_SHIFT: c_int = 4;
pub const DA7213_DAC_R_MONO_SHIFT: c_int = 7;
pub const DA7213_DAC_SRC_MAX: c_int = 4;
pub const DA7213_DAC_MONO_MAX: c_uint = 0x1;
// DA7213_ALC_CTRL1 = 0x2B
pub const DA7213_ALC_OFFSET_EN_SHIFT: c_int = 0;
pub const DA7213_ALC_OFFSET_EN_MAX: c_uint = 0x1;

pub const DA7213_ALC_L_EN_SHIFT: c_int = 3;

pub const DA7213_ALC_R_EN_SHIFT: c_int = 7;
pub const DA7213_ALC_EN_MAX: c_uint = 0x1;
// DA7213_AUX_L/R_GAIN = 0x30/0x31
pub const DA7213_AUX_AMP_GAIN_SHIFT: c_int = 0;
pub const DA7213_AUX_AMP_GAIN_MAX: c_uint = 0x3F;
// DA7213_MIXIN_L/R_SELECT = 0x32/0x33
pub const DA7213_DMIC_EN_SHIFT: c_int = 7;
pub const DA7213_DMIC_EN_MAX: c_uint = 0x1;
// DA7213_MIXIN_L_SELECT = 0x32
pub const DA7213_MIXIN_L_MIX_SELECT_AUX_L_SHIFT: c_int = 0;
pub const DA7213_MIXIN_L_MIX_SELECT_MIC_1_SHIFT: c_int = 1;

pub const DA7213_MIXIN_L_MIX_SELECT_MIC_2_SHIFT: c_int = 2;

pub const DA7213_MIXIN_L_MIX_SELECT_MIXIN_R_SHIFT: c_int = 3;
pub const DA7213_MIXIN_L_MIX_SELECT_MAX: c_uint = 0x1;
// DA7213_MIXIN_R_SELECT =  0x33
pub const DA7213_MIXIN_R_MIX_SELECT_AUX_R_SHIFT: c_int = 0;
pub const DA7213_MIXIN_R_MIX_SELECT_MIC_2_SHIFT: c_int = 1;

pub const DA7213_MIXIN_R_MIX_SELECT_MIC_1_SHIFT: c_int = 2;

pub const DA7213_MIXIN_R_MIX_SELECT_MIXIN_L_SHIFT: c_int = 3;
pub const DA7213_MIXIN_R_MIX_SELECT_MAX: c_uint = 0x1;

// DA7213_MIXIN_L/R_GAIN = 0x34/0x35
pub const DA7213_MIXIN_AMP_GAIN_SHIFT: c_int = 0;
pub const DA7213_MIXIN_AMP_GAIN_MAX: c_uint = 0xF;
// DA7213_ADC_L/R_GAIN = 0x36/0x37
pub const DA7213_ADC_AMP_GAIN_SHIFT: c_int = 0;
pub const DA7213_ADC_AMP_GAIN_MAX: c_uint = 0x7F;
// DA7213_ADC/DAC_FILTERS1 = 0x38/0x44
pub const DA7213_VOICE_HPF_CORNER_SHIFT: c_int = 0;
pub const DA7213_VOICE_HPF_CORNER_MAX: c_int = 8;
pub const DA7213_VOICE_EN_SHIFT: c_int = 3;
pub const DA7213_VOICE_EN_MAX: c_uint = 0x1;
pub const DA7213_AUDIO_HPF_CORNER_SHIFT: c_int = 4;
pub const DA7213_AUDIO_HPF_CORNER_MAX: c_int = 4;
pub const DA7213_HPF_EN_SHIFT: c_int = 7;
pub const DA7213_HPF_EN_MAX: c_uint = 0x1;
// DA7213_MIC_1/2_GAIN = 0x39/0x3A
pub const DA7213_MIC_AMP_GAIN_SHIFT: c_int = 0;
pub const DA7213_MIC_AMP_GAIN_MAX: c_uint = 0x7;
// DA7213_DAC_FILTERS5 = 0x40
pub const DA7213_DAC_SOFTMUTE_EN_SHIFT: c_int = 7;
pub const DA7213_DAC_SOFTMUTE_EN_MAX: c_uint = 0x1;
pub const DA7213_DAC_SOFTMUTE_RATE_SHIFT: c_int = 4;
pub const DA7213_DAC_SOFTMUTE_RATE_MAX: c_int = 7;
// DA7213_DAC_FILTERS2/3/4 = 0x41/0x42/0x43
pub const DA7213_DAC_EQ_BAND_MAX: c_uint = 0xF;
// DA7213_DAC_FILTERS2 = 0x41
pub const DA7213_DAC_EQ_BAND1_SHIFT: c_int = 0;
pub const DA7213_DAC_EQ_BAND2_SHIFT: c_int = 4;
// DA7213_DAC_FILTERS2 = 0x42
pub const DA7213_DAC_EQ_BAND3_SHIFT: c_int = 0;
pub const DA7213_DAC_EQ_BAND4_SHIFT: c_int = 4;
// DA7213_DAC_FILTERS4 = 0x43
pub const DA7213_DAC_EQ_BAND5_SHIFT: c_int = 0;
pub const DA7213_DAC_EQ_EN_SHIFT: c_int = 7;
pub const DA7213_DAC_EQ_EN_MAX: c_uint = 0x1;
// DA7213_DAC_L/R_GAIN = 0x45/0x46
pub const DA7213_DAC_AMP_GAIN_SHIFT: c_int = 0;
pub const DA7213_DAC_AMP_GAIN_MAX: c_uint = 0x7F;
// DA7213_HP_L/R_GAIN = 0x45/0x46
pub const DA7213_HP_AMP_GAIN_SHIFT: c_int = 0;
pub const DA7213_HP_AMP_GAIN_MAX: c_uint = 0x3F;
// DA7213_CP_CTRL = 0x47
pub const DA7213_CP_EN_SHIFT: c_int = 7;
// DA7213_LINE_GAIN = 0x4A
pub const DA7213_LINE_AMP_GAIN_SHIFT: c_int = 0;
pub const DA7213_LINE_AMP_GAIN_MAX: c_uint = 0x3F;
// DA7213_MIXOUT_L_SELECT = 0x4B
pub const DA7213_MIXOUT_L_MIX_SELECT_AUX_L_SHIFT: c_int = 0;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_L_SHIFT: c_int = 1;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_R_SHIFT: c_int = 2;
pub const DA7213_MIXOUT_L_MIX_SELECT_DAC_L_SHIFT: c_int = 3;
pub const DA7213_MIXOUT_L_MIX_SELECT_AUX_L_INVERTED_SHIFT: c_int = 4;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_L_INVERTED_SHIFT: c_int = 5;
pub const DA7213_MIXOUT_L_MIX_SELECT_MIXIN_R_INVERTED_SHIFT: c_int = 6;
pub const DA7213_MIXOUT_L_MIX_SELECT_MAX: c_uint = 0x1;
// DA7213_MIXOUT_R_SELECT = 0x4C
pub const DA7213_MIXOUT_R_MIX_SELECT_AUX_R_SHIFT: c_int = 0;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_R_SHIFT: c_int = 1;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_L_SHIFT: c_int = 2;
pub const DA7213_MIXOUT_R_MIX_SELECT_DAC_R_SHIFT: c_int = 3;
pub const DA7213_MIXOUT_R_MIX_SELECT_AUX_R_INVERTED_SHIFT: c_int = 4;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_R_INVERTED_SHIFT: c_int = 5;
pub const DA7213_MIXOUT_R_MIX_SELECT_MIXIN_L_INVERTED_SHIFT: c_int = 6;
pub const DA7213_MIXOUT_R_MIX_SELECT_MAX: c_uint = 0x1;
//
// DA7213_AUX_L/R_CTRL = 0x60/0x61,
// DA7213_MIC_1/2_CTRL = 0x63/0x64,
// DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
// DA7213_ADC_L/R_CTRL = 0x65/0x66,
// DA7213_DAC_L/R_CTRL = 0x69/0x6A,
// DA7213_HP_L/R_CTRL = 0x6B/0x6C,
// DA7213_LINE_CTRL = 0x6D
//
pub const DA7213_MUTE_EN_SHIFT: c_int = 6;
pub const DA7213_MUTE_EN_MAX: c_uint = 0x1;

//
// DA7213_AUX_L/R_CTRL = 0x60/0x61,
// DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
// DA7213_ADC_L/R_CTRL = 0x65/0x66,
// DA7213_DAC_L/R_CTRL = 0x69/0x6A,
// DA7213_HP_L/R_CTRL = 0x6B/0x6C,
// DA7213_LINE_CTRL = 0x6D
//
pub const DA7213_GAIN_RAMP_EN_SHIFT: c_int = 5;
pub const DA7213_GAIN_RAMP_EN_MAX: c_uint = 0x1;

//
// DA7213_AUX_L/R_CTRL = 0x60/0x61,
// DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
// DA7213_HP_L/R_CTRL = 0x6B/0x6C,
// DA7213_LINE_CTRL = 0x6D
//
pub const DA7213_ZC_EN_SHIFT: c_int = 4;
pub const DA7213_ZC_EN_MAX: c_uint = 0x1;
//
// DA7213_AUX_L/R_CTRL = 0x60/0x61,
// DA7213_MIC_1/2_CTRL = 0x63/0x64,
// DA7213_MIXIN_L/R_CTRL = 0x65/0x66,
// DA7213_HP_L/R_CTRL = 0x6B/0x6C,
// DA7213_MIXOUT_L/R_CTRL = 0x6E/0x6F,
// DA7213_LINE_CTRL = 0x6D
//
pub const DA7213_AMP_EN_SHIFT: c_int = 7;
// DA7213_MIC_1/2_CTRL = 0x63/0x64
pub const DA7213_MIC_AMP_IN_SEL_SHIFT: c_int = 2;
pub const DA7213_MIC_AMP_IN_SEL_MAX: c_int = 3;
// DA7213_MICBIAS_CTRL = 0x62
pub const DA7213_MICBIAS1_LEVEL_SHIFT: c_int = 0;

pub const DA7213_MICBIAS1_EN_SHIFT: c_int = 3;
pub const DA7213_MICBIAS2_LEVEL_SHIFT: c_int = 4;

pub const DA7213_MICBIAS2_EN_SHIFT: c_int = 7;
// DA7213_MIXIN_L/R_CTRL = 0x65/0x66

// DA7213_ADC_L/R_CTRL = 0x67/0x68
pub const DA7213_ADC_EN_SHIFT: c_int = 7;

// DA7213_DAC_L/R_CTRL =  0x69/0x6A
pub const DA7213_DAC_EN_SHIFT: c_int = 7;
// DA7213_HP_L/R_CTRL = 0x6B/0x6C

// DA7213_LINE_CTRL = 0x6D

// DA7213_MIXOUT_L/R_CTRL = 0x6E/0x6F

// DA7213_GAIN_RAMP_CTRL = 0x92
pub const DA7213_GAIN_RAMP_RATE_SHIFT: c_int = 0;
pub const DA7213_GAIN_RAMP_RATE_MAX: c_int = 4;
// DA7213_MIC_CONFIG = 0x93
pub const DA7213_DMIC_DATA_SEL_SHIFT: c_int = 0;

pub const DA7213_DMIC_SAMPLEPHASE_SHIFT: c_int = 1;

pub const DA7213_DMIC_CLK_RATE_SHIFT: c_int = 2;

// DA7213_PC_COUNT = 0x94

// DA7213_DIG_CTRL = 0x99
pub const DA7213_DAC_L_INV_SHIFT: c_int = 3;
pub const DA7213_DAC_R_INV_SHIFT: c_int = 7;
pub const DA7213_DAC_INV_MAX: c_uint = 0x1;
// DA7213_ALC_CTRL2 = 0x9A
pub const DA7213_ALC_ATTACK_SHIFT: c_int = 0;
pub const DA7213_ALC_ATTACK_MAX: c_int = 13;
pub const DA7213_ALC_RELEASE_SHIFT: c_int = 4;
pub const DA7213_ALC_RELEASE_MAX: c_int = 11;
// DA7213_ALC_CTRL3 = 0x9B
pub const DA7213_ALC_HOLD_SHIFT: c_int = 0;
pub const DA7213_ALC_HOLD_MAX: c_int = 16;
pub const DA7213_ALC_INTEG_ATTACK_SHIFT: c_int = 4;
pub const DA7213_ALC_INTEG_RELEASE_SHIFT: c_int = 6;
pub const DA7213_ALC_INTEG_MAX: c_int = 4;
//
// DA7213_ALC_NOISE = 0x9C,
// DA7213_ALC_TARGET_MIN/MAX = 0x9D/0x9E
//
pub const DA7213_ALC_THRESHOLD_SHIFT: c_int = 0;
pub const DA7213_ALC_THRESHOLD_MAX: c_uint = 0x3F;
// DA7213_ALC_GAIN_LIMITS = 0x9F
pub const DA7213_ALC_ATTEN_MAX_SHIFT: c_int = 0;
pub const DA7213_ALC_GAIN_MAX_SHIFT: c_int = 4;
pub const DA7213_ALC_ATTEN_GAIN_MAX_MAX: c_uint = 0xF;
// DA7213_ALC_ANA_GAIN_LIMITS = 0xA0
pub const DA7213_ALC_ANA_GAIN_MIN_SHIFT: c_int = 0;
pub const DA7213_ALC_ANA_GAIN_MAX_SHIFT: c_int = 4;
pub const DA7213_ALC_ANA_GAIN_MAX: c_uint = 0x7;
// DA7213_ALC_ANTICLIP_CTRL = 0xA1
pub const DA7213_ALC_ANTICLIP_EN_SHIFT: c_int = 7;
pub const DA7213_ALC_ANTICLIP_EN_MAX: c_uint = 0x1;
// DA7213_ALC_ANTICLIP_LEVEL = 0xA2
pub const DA7213_ALC_ANTICLIP_LEVEL_SHIFT: c_int = 0;
pub const DA7213_ALC_ANTICLIP_LEVEL_MAX: c_uint = 0x7F;
// DA7213_ALC_CIC_OP_LVL_CTRL = 0xAD

// DA7213_DAC_NG_SETUP_TIME = 0xAF
pub const DA7213_DAC_NG_SETUP_TIME_SHIFT: c_int = 0;
pub const DA7213_DAC_NG_SETUP_TIME_MAX: c_int = 4;
pub const DA7213_DAC_NG_RAMPUP_RATE_SHIFT: c_int = 2;
pub const DA7213_DAC_NG_RAMPDN_RATE_SHIFT: c_int = 3;
pub const DA7213_DAC_NG_RAMP_RATE_MAX: c_int = 2;
// DA7213_DAC_NG_OFF/ON_THRESH = 0xB0/0xB1
pub const DA7213_DAC_NG_THRESHOLD_SHIFT: c_int = 0;
pub const DA7213_DAC_NG_THRESHOLD_MAX: c_uint = 0x7;
// DA7213_DAC_NG_CTRL = 0xB2
pub const DA7213_DAC_NG_EN_SHIFT: c_int = 7;
pub const DA7213_DAC_NG_EN_MAX: c_uint = 0x1;
// DA7213_TONE_GEN_CFG1 = 0xB4
pub const DA7213_DTMF_REG_SHIFT: c_int = 0;

pub const DA7213_DTMF_REG_MAX: c_int = 16;
pub const DA7213_DTMF_EN_SHIFT: c_int = 4;

pub const DA7213_START_STOPN_SHIFT: c_int = 7;

// DA7213_TONE_GEN_CFG2 = 0xB5
pub const DA7213_SWG_SEL_SHIFT: c_int = 0;

pub const DA7213_SWG_SEL_MAX: c_int = 4;

pub const DA7213_TONE_GEN_GAIN_SHIFT: c_int = 4;

pub const DA7213_TONE_GEN_GAIN_MAX: c_uint = 0xF;

// DA7213_TONE_GEN_CYCLES = 0xB6
pub const DA7213_BEEP_CYCLES_SHIFT: c_int = 0;

// DA7213_TONE_GEN_FREQ1_L = 0xB7
pub const DA7213_FREQ1_L_SHIFT: c_int = 0;

pub const DA7213_FREQ_MAX: c_uint = 0xFFFF;
// DA7213_TONE_GEN_FREQ1_U = 0xB8
pub const DA7213_FREQ1_U_SHIFT: c_int = 0;

// DA7213_TONE_GEN_FREQ2_L = 0xB9
pub const DA7213_FREQ2_L_SHIFT: c_int = 0;

// DA7213_TONE_GEN_FREQ2_U = 0xBA
pub const DA7213_FREQ2_U_SHIFT: c_int = 0;

// DA7213_TONE_GEN_ON_PER = 0xBB
pub const DA7213_BEEP_ON_PER_SHIFT: c_int = 0;

pub const DA7213_BEEP_ON_OFF_MAX: c_uint = 0x3F;
// DA7213_TONE_GEN_OFF_PER = 0xBC
pub const DA7213_BEEP_OFF_PER_SHIFT: c_int = 0;

//
// General defines
//
// Register inversion
pub const DA7213_NO_INVERT: c_int = 0;
pub const DA7213_INVERT: c_int = 1;
// Byte related defines
pub const DA7213_BYTE_SHIFT: c_int = 8;
pub const DA7213_BYTE_MASK: c_uint = 0xFF;
// ALC related
pub const DA7213_ALC_OFFSET_15_8: c_uint = 0x00FF00;
pub const DA7213_ALC_OFFSET_19_16: c_uint = 0x0F0000;
pub const DA7213_ALC_AVG_ITERATIONS: c_int = 5;
// PLL related
pub const DA7213_PLL_FREQ_OUT_90316800: c_int = 90316800;
pub const DA7213_PLL_FREQ_OUT_98304000: c_int = 98304000;
pub const DA7213_PLL_FREQ_OUT_94310400: c_int = 94310400;
pub const DA7213_PLL_INDIV_5_TO_9_MHZ_VAL: c_int = 2;
pub const DA7213_PLL_INDIV_9_TO_18_MHZ_VAL: c_int = 4;
pub const DA7213_PLL_INDIV_18_TO_36_MHZ_VAL: c_int = 8;
pub const DA7213_PLL_INDIV_36_TO_54_MHZ_VAL: c_int = 16;
pub const DA7213_SRM_CHECK_RETRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7213_clk_src {
    DA7213_CLKSRC_MCLK = 0,
    DA7213_CLKSRC_MCLK_SQR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7213_sys_clk {
    DA7213_SYSCLK_MCLK = 0,
    DA7213_SYSCLK_PLL,
    DA7213_SYSCLK_PLL_SRM,
    DA7213_SYSCLK_PLL_32KHZ
}

// Regulators
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7213_supplies {
    DA7213_SUPPLY_VDDA = 0,
    DA7213_SUPPLY_VDDIO,
    DA7213_NUM_SUPPLIES,
}

// Codec private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da7213_priv {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub ctrl_lock: mutex,
    pub supplies: [regulator_bulk_data; DA7213_NUM_SUPPLIES],
    pub mclk: *mut clk,
    pub mclk_rate: c_uint,
    pub out_rate: c_uint,
    pub fin_min_rate: c_uint,
    pub clk_src: c_int,
    pub master: bool,
    pub alc_calib_auto: bool,
    pub alc_en: bool,
    pub fixed_clk_auto_pll: bool,
    pub pdata: *mut da7213_platform_data,
    pub fmt: c_int,
}

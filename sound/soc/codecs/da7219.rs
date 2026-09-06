//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/da7219.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// da7219.h - DA7219 ALSA SoC Codec Driver
//
// Copyright (c) 2015 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//

//
// Registers
//
pub const DA7219_MIC_1_GAIN_STATUS: c_uint = 0x6;
pub const DA7219_MIXIN_L_GAIN_STATUS: c_uint = 0x8;
pub const DA7219_ADC_L_GAIN_STATUS: c_uint = 0xA;
pub const DA7219_DAC_L_GAIN_STATUS: c_uint = 0xC;
pub const DA7219_DAC_R_GAIN_STATUS: c_uint = 0xD;
pub const DA7219_HP_L_GAIN_STATUS: c_uint = 0xE;
pub const DA7219_HP_R_GAIN_STATUS: c_uint = 0xF;
pub const DA7219_MIC_1_SELECT: c_uint = 0x10;
pub const DA7219_CIF_TIMEOUT_CTRL: c_uint = 0x12;
pub const DA7219_CIF_CTRL: c_uint = 0x13;
pub const DA7219_SR_24_48: c_uint = 0x16;
pub const DA7219_SR: c_uint = 0x17;
pub const DA7219_CIF_I2C_ADDR_CFG: c_uint = 0x1B;
pub const DA7219_PLL_CTRL: c_uint = 0x20;
pub const DA7219_PLL_FRAC_TOP: c_uint = 0x22;
pub const DA7219_PLL_FRAC_BOT: c_uint = 0x23;
pub const DA7219_PLL_INTEGER: c_uint = 0x24;
pub const DA7219_PLL_SRM_STS: c_uint = 0x25;
pub const DA7219_DIG_ROUTING_DAI: c_uint = 0x2A;
pub const DA7219_DAI_CLK_MODE: c_uint = 0x2B;
pub const DA7219_DAI_CTRL: c_uint = 0x2C;
pub const DA7219_DAI_TDM_CTRL: c_uint = 0x2D;
pub const DA7219_DIG_ROUTING_DAC: c_uint = 0x2E;
pub const DA7219_ALC_CTRL1: c_uint = 0x2F;
pub const DA7219_DAI_OFFSET_LOWER: c_uint = 0x30;
pub const DA7219_DAI_OFFSET_UPPER: c_uint = 0x31;
pub const DA7219_REFERENCES: c_uint = 0x32;
pub const DA7219_MIXIN_L_SELECT: c_uint = 0x33;
pub const DA7219_MIXIN_L_GAIN: c_uint = 0x34;
pub const DA7219_ADC_L_GAIN: c_uint = 0x36;
pub const DA7219_ADC_FILTERS1: c_uint = 0x38;
pub const DA7219_MIC_1_GAIN: c_uint = 0x39;
pub const DA7219_SIDETONE_CTRL: c_uint = 0x3A;
pub const DA7219_SIDETONE_GAIN: c_uint = 0x3B;
pub const DA7219_DROUTING_ST_OUTFILT_1L: c_uint = 0x3C;
pub const DA7219_DROUTING_ST_OUTFILT_1R: c_uint = 0x3D;
pub const DA7219_DAC_FILTERS5: c_uint = 0x40;
pub const DA7219_DAC_FILTERS2: c_uint = 0x41;
pub const DA7219_DAC_FILTERS3: c_uint = 0x42;
pub const DA7219_DAC_FILTERS4: c_uint = 0x43;
pub const DA7219_DAC_FILTERS1: c_uint = 0x44;
pub const DA7219_DAC_L_GAIN: c_uint = 0x45;
pub const DA7219_DAC_R_GAIN: c_uint = 0x46;
pub const DA7219_CP_CTRL: c_uint = 0x47;
pub const DA7219_HP_L_GAIN: c_uint = 0x48;
pub const DA7219_HP_R_GAIN: c_uint = 0x49;
pub const DA7219_MIXOUT_L_SELECT: c_uint = 0x4B;
pub const DA7219_MIXOUT_R_SELECT: c_uint = 0x4C;
pub const DA7219_SYSTEM_MODES_INPUT: c_uint = 0x50;
pub const DA7219_SYSTEM_MODES_OUTPUT: c_uint = 0x51;
pub const DA7219_MICBIAS_CTRL: c_uint = 0x62;
pub const DA7219_MIC_1_CTRL: c_uint = 0x63;
pub const DA7219_MIXIN_L_CTRL: c_uint = 0x65;
pub const DA7219_ADC_L_CTRL: c_uint = 0x67;
pub const DA7219_DAC_L_CTRL: c_uint = 0x69;
pub const DA7219_DAC_R_CTRL: c_uint = 0x6A;
pub const DA7219_HP_L_CTRL: c_uint = 0x6B;
pub const DA7219_HP_R_CTRL: c_uint = 0x6C;
pub const DA7219_MIXOUT_L_CTRL: c_uint = 0x6E;
pub const DA7219_MIXOUT_R_CTRL: c_uint = 0x6F;
pub const DA7219_CHIP_ID1: c_uint = 0x81;
pub const DA7219_CHIP_ID2: c_uint = 0x82;
pub const DA7219_CHIP_REVISION: c_uint = 0x83;
pub const DA7219_IO_CTRL: c_uint = 0x91;
pub const DA7219_GAIN_RAMP_CTRL: c_uint = 0x92;
pub const DA7219_PC_COUNT: c_uint = 0x94;
pub const DA7219_CP_VOL_THRESHOLD1: c_uint = 0x95;
pub const DA7219_CP_DELAY: c_uint = 0x96;
pub const DA7219_DIG_CTRL: c_uint = 0x99;
pub const DA7219_ALC_CTRL2: c_uint = 0x9A;
pub const DA7219_ALC_CTRL3: c_uint = 0x9B;
pub const DA7219_ALC_NOISE: c_uint = 0x9C;
pub const DA7219_ALC_TARGET_MIN: c_uint = 0x9D;
pub const DA7219_ALC_TARGET_MAX: c_uint = 0x9E;
pub const DA7219_ALC_GAIN_LIMITS: c_uint = 0x9F;
pub const DA7219_ALC_ANA_GAIN_LIMITS: c_uint = 0xA0;
pub const DA7219_ALC_ANTICLIP_CTRL: c_uint = 0xA1;
pub const DA7219_ALC_ANTICLIP_LEVEL: c_uint = 0xA2;
pub const DA7219_ALC_OFFSET_AUTO_M_L: c_uint = 0xA3;
pub const DA7219_ALC_OFFSET_AUTO_U_L: c_uint = 0xA4;
pub const DA7219_DAC_NG_SETUP_TIME: c_uint = 0xAF;
pub const DA7219_DAC_NG_OFF_THRESH: c_uint = 0xB0;
pub const DA7219_DAC_NG_ON_THRESH: c_uint = 0xB1;
pub const DA7219_DAC_NG_CTRL: c_uint = 0xB2;
pub const DA7219_TONE_GEN_CFG1: c_uint = 0xB4;
pub const DA7219_TONE_GEN_CFG2: c_uint = 0xB5;
pub const DA7219_TONE_GEN_CYCLES: c_uint = 0xB6;
pub const DA7219_TONE_GEN_FREQ1_L: c_uint = 0xB7;
pub const DA7219_TONE_GEN_FREQ1_U: c_uint = 0xB8;
pub const DA7219_TONE_GEN_FREQ2_L: c_uint = 0xB9;
pub const DA7219_TONE_GEN_FREQ2_U: c_uint = 0xBA;
pub const DA7219_TONE_GEN_ON_PER: c_uint = 0xBB;
pub const DA7219_TONE_GEN_OFF_PER: c_uint = 0xBC;
pub const DA7219_SYSTEM_STATUS: c_uint = 0xE0;
pub const DA7219_SYSTEM_ACTIVE: c_uint = 0xFD;
//
// Bit Fields
//
pub const DA7219_SWITCH_EN_MAX: c_uint = 0x1;
// DA7219_MIC_1_GAIN_STATUS = 0x6
pub const DA7219_MIC_1_AMP_GAIN_STATUS_SHIFT: c_int = 0;

pub const DA7219_MIC_1_AMP_GAIN_MAX: c_uint = 0x7;
// DA7219_MIXIN_L_GAIN_STATUS = 0x8
pub const DA7219_MIXIN_L_AMP_GAIN_STATUS_SHIFT: c_int = 0;

// DA7219_ADC_L_GAIN_STATUS = 0xA
pub const DA7219_ADC_L_DIGITAL_GAIN_STATUS_SHIFT: c_int = 0;

// DA7219_DAC_L_GAIN_STATUS = 0xC
pub const DA7219_DAC_L_DIGITAL_GAIN_STATUS_SHIFT: c_int = 0;

// DA7219_DAC_R_GAIN_STATUS = 0xD
pub const DA7219_DAC_R_DIGITAL_GAIN_STATUS_SHIFT: c_int = 0;

// DA7219_HP_L_GAIN_STATUS = 0xE
pub const DA7219_HP_L_AMP_GAIN_STATUS_SHIFT: c_int = 0;

// DA7219_HP_R_GAIN_STATUS = 0xF
pub const DA7219_HP_R_AMP_GAIN_STATUS_SHIFT: c_int = 0;

// DA7219_MIC_1_SELECT = 0x10
pub const DA7219_MIC_1_AMP_IN_SEL_SHIFT: c_int = 0;

// DA7219_CIF_TIMEOUT_CTRL = 0x12
pub const DA7219_I2C_TIMEOUT_EN_SHIFT: c_int = 0;

// DA7219_CIF_CTRL = 0x13
pub const DA7219_CIF_I2C_WRITE_MODE_SHIFT: c_int = 0;

pub const DA7219_CIF_REG_SOFT_RESET_SHIFT: c_int = 7;

// DA7219_SR_24_48 = 0x16
pub const DA7219_SR_24_48_SHIFT: c_int = 0;

// DA7219_SR = 0x17
pub const DA7219_SR_SHIFT: c_int = 0;

// DA7219_CIF_I2C_ADDR_CFG = 0x1B
pub const DA7219_CIF_I2C_ADDR_CFG_SHIFT: c_int = 0;

// DA7219_PLL_CTRL = 0x20
pub const DA7219_PLL_INDIV_SHIFT: c_int = 2;

pub const DA7219_PLL_MCLK_SQR_EN_SHIFT: c_int = 5;

pub const DA7219_PLL_MODE_SHIFT: c_int = 6;

// DA7219_PLL_FRAC_TOP = 0x22
pub const DA7219_PLL_FBDIV_FRAC_TOP_SHIFT: c_int = 0;

// DA7219_PLL_FRAC_BOT = 0x23
pub const DA7219_PLL_FBDIV_FRAC_BOT_SHIFT: c_int = 0;

// DA7219_PLL_INTEGER = 0x24
pub const DA7219_PLL_FBDIV_INTEGER_SHIFT: c_int = 0;

// DA7219_PLL_SRM_STS = 0x25
pub const DA7219_PLL_SRM_STATE_SHIFT: c_int = 0;

pub const DA7219_PLL_SRM_STATUS_SHIFT: c_int = 4;

// DA7219_DIG_ROUTING_DAI = 0x2A
pub const DA7219_DAI_L_SRC_SHIFT: c_int = 0;

pub const DA7219_DAI_R_SRC_SHIFT: c_int = 4;

pub const DA7219_OUT_SRC_MAX: c_int = 4;
// DA7219_DAI_CLK_MODE = 0x2B
pub const DA7219_DAI_BCLKS_PER_WCLK_SHIFT: c_int = 0;

pub const DA7219_DAI_CLK_POL_SHIFT: c_int = 2;

pub const DA7219_DAI_WCLK_POL_SHIFT: c_int = 3;

pub const DA7219_DAI_WCLK_TRI_STATE_SHIFT: c_int = 4;

pub const DA7219_DAI_CLK_EN_SHIFT: c_int = 7;

// DA7219_DAI_CTRL = 0x2C
pub const DA7219_DAI_FORMAT_SHIFT: c_int = 0;

pub const DA7219_DAI_WORD_LENGTH_SHIFT: c_int = 2;

pub const DA7219_DAI_CH_NUM_SHIFT: c_int = 4;

pub const DA7219_DAI_CH_NUM_MAX: c_int = 2;
pub const DA7219_DAI_EN_SHIFT: c_int = 7;

// DA7219_DAI_TDM_CTRL = 0x2D
pub const DA7219_DAI_TDM_CH_EN_SHIFT: c_int = 0;

pub const DA7219_DAI_OE_SHIFT: c_int = 6;

pub const DA7219_DAI_TDM_MODE_EN_SHIFT: c_int = 7;

pub const DA7219_DAI_TDM_MAX_SLOTS: c_int = 2;
// DA7219_DIG_ROUTING_DAC = 0x2E
pub const DA7219_DAC_L_SRC_SHIFT: c_int = 0;

pub const DA7219_DAC_L_MONO_SHIFT: c_int = 3;

pub const DA7219_DAC_R_SRC_SHIFT: c_int = 4;

pub const DA7219_DAC_R_MONO_SHIFT: c_int = 7;

// DA7219_ALC_CTRL1 = 0x2F
pub const DA7219_ALC_OFFSET_EN_SHIFT: c_int = 0;

pub const DA7219_ALC_SYNC_MODE_SHIFT: c_int = 1;

pub const DA7219_ALC_EN_SHIFT: c_int = 3;

pub const DA7219_ALC_AUTO_CALIB_EN_SHIFT: c_int = 4;

pub const DA7219_ALC_CALIB_OVERFLOW_SHIFT: c_int = 5;

// DA7219_DAI_OFFSET_LOWER = 0x30
pub const DA7219_DAI_OFFSET_LOWER_SHIFT: c_int = 0;

// DA7219_DAI_OFFSET_UPPER = 0x31
pub const DA7219_DAI_OFFSET_UPPER_SHIFT: c_int = 0;

pub const DA7219_DAI_OFFSET_MAX: c_uint = 0x2FF;
// DA7219_REFERENCES = 0x32
pub const DA7219_BIAS_EN_SHIFT: c_int = 3;

pub const DA7219_VMID_FAST_CHARGE_SHIFT: c_int = 4;

// DA7219_MIXIN_L_SELECT = 0x33
pub const DA7219_MIXIN_L_MIX_SELECT_SHIFT: c_int = 0;

// DA7219_MIXIN_L_GAIN = 0x34
pub const DA7219_MIXIN_L_AMP_GAIN_SHIFT: c_int = 0;

pub const DA7219_MIXIN_L_AMP_GAIN_MAX: c_uint = 0xF;
// DA7219_ADC_L_GAIN = 0x36
pub const DA7219_ADC_L_DIGITAL_GAIN_SHIFT: c_int = 0;

pub const DA7219_ADC_L_DIGITAL_GAIN_MAX: c_uint = 0x7F;
// DA7219_ADC_FILTERS1 = 0x38
pub const DA7219_ADC_VOICE_HPF_CORNER_SHIFT: c_int = 0;

pub const DA7219_VOICE_HPF_CORNER_MAX: c_int = 8;
pub const DA7219_ADC_VOICE_EN_SHIFT: c_int = 3;

pub const DA7219_ADC_AUDIO_HPF_CORNER_SHIFT: c_int = 4;

pub const DA7219_AUDIO_HPF_CORNER_MAX: c_int = 4;
pub const DA7219_ADC_HPF_EN_SHIFT: c_int = 7;

pub const DA7219_HPF_MODE_SHIFT: c_int = 0;

pub const DA7219_HPF_MODE_MAX: c_int = 3;
// DA7219_MIC_1_GAIN = 0x39
pub const DA7219_MIC_1_AMP_GAIN_SHIFT: c_int = 0;

// DA7219_SIDETONE_CTRL = 0x3A
pub const DA7219_SIDETONE_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_SIDETONE_EN_SHIFT: c_int = 7;

// DA7219_SIDETONE_GAIN = 0x3B
pub const DA7219_SIDETONE_GAIN_SHIFT: c_int = 0;

pub const DA7219_SIDETONE_GAIN_MAX: c_uint = 0xE;
// DA7219_DROUTING_ST_OUTFILT_1L = 0x3C
pub const DA7219_OUTFILT_ST_1L_SRC_SHIFT: c_int = 0;

pub const DA7219_DMIX_ST_SRC_OUTFILT1L_SHIFT: c_int = 0;
pub const DA7219_DMIX_ST_SRC_OUTFILT1R_SHIFT: c_int = 1;
pub const DA7219_DMIX_ST_SRC_SIDETONE_SHIFT: c_int = 2;

// DA7219_DROUTING_ST_OUTFILT_1R = 0x3D
pub const DA7219_OUTFILT_ST_1R_SRC_SHIFT: c_int = 0;

// DA7219_DAC_FILTERS5 = 0x40
pub const DA7219_DAC_SOFTMUTE_RATE_SHIFT: c_int = 4;

pub const DA7219_DAC_SOFTMUTE_RATE_MAX: c_int = 7;
pub const DA7219_DAC_SOFTMUTE_EN_SHIFT: c_int = 7;

// DA7219_DAC_FILTERS2 = 0x41
pub const DA7219_DAC_EQ_BAND1_SHIFT: c_int = 0;

pub const DA7219_DAC_EQ_BAND2_SHIFT: c_int = 4;

pub const DA7219_DAC_EQ_BAND_MAX: c_uint = 0xF;
// DA7219_DAC_FILTERS3 = 0x42
pub const DA7219_DAC_EQ_BAND3_SHIFT: c_int = 0;

pub const DA7219_DAC_EQ_BAND4_SHIFT: c_int = 4;

// DA7219_DAC_FILTERS4 = 0x43
pub const DA7219_DAC_EQ_BAND5_SHIFT: c_int = 0;

pub const DA7219_DAC_EQ_EN_SHIFT: c_int = 7;

// DA7219_DAC_FILTERS1 = 0x44
pub const DA7219_DAC_VOICE_HPF_CORNER_SHIFT: c_int = 0;

pub const DA7219_DAC_VOICE_EN_SHIFT: c_int = 3;

pub const DA7219_DAC_AUDIO_HPF_CORNER_SHIFT: c_int = 4;

pub const DA7219_DAC_HPF_EN_SHIFT: c_int = 7;

// DA7219_DAC_L_GAIN = 0x45
pub const DA7219_DAC_L_DIGITAL_GAIN_SHIFT: c_int = 0;

pub const DA7219_DAC_DIGITAL_GAIN_MAX: c_uint = 0x7F;

// DA7219_DAC_R_GAIN = 0x46
pub const DA7219_DAC_R_DIGITAL_GAIN_SHIFT: c_int = 0;

// DA7219_CP_CTRL = 0x47
pub const DA7219_CP_MCHANGE_SHIFT: c_int = 4;

pub const DA7219_CP_MCHANGE_REL_MASK: c_uint = 0x3;
pub const DA7219_CP_MCHANGE_MAX: c_int = 3;
pub const DA7219_CP_MCHANGE_LARGEST_VOL: c_uint = 0x1;
pub const DA7219_CP_MCHANGE_DAC_VOL: c_uint = 0x2;
pub const DA7219_CP_MCHANGE_SIG_MAG: c_uint = 0x3;
pub const DA7219_CP_EN_SHIFT: c_int = 7;

// DA7219_HP_L_GAIN = 0x48
pub const DA7219_HP_L_AMP_GAIN_SHIFT: c_int = 0;

pub const DA7219_HP_AMP_GAIN_MAX: c_uint = 0x3F;

// DA7219_HP_R_GAIN = 0x49
pub const DA7219_HP_R_AMP_GAIN_SHIFT: c_int = 0;

// DA7219_MIXOUT_L_SELECT = 0x4B
pub const DA7219_MIXOUT_L_MIX_SELECT_SHIFT: c_int = 0;

// DA7219_MIXOUT_R_SELECT = 0x4C
pub const DA7219_MIXOUT_R_MIX_SELECT_SHIFT: c_int = 0;

// DA7219_SYSTEM_MODES_INPUT = 0x50
pub const DA7219_MODE_SUBMIT_SHIFT: c_int = 0;

pub const DA7219_ADC_MODE_SHIFT: c_int = 1;

// DA7219_SYSTEM_MODES_OUTPUT = 0x51
pub const DA7219_MODE_SUBMIT_SHIFT: c_int = 0;

pub const DA7219_DAC_MODE_SHIFT: c_int = 1;

// DA7219_MICBIAS_CTRL = 0x62
pub const DA7219_MICBIAS1_LEVEL_SHIFT: c_int = 0;

pub const DA7219_MICBIAS1_EN_SHIFT: c_int = 3;

// DA7219_MIC_1_CTRL = 0x63
pub const DA7219_MIC_1_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7219_MIC_1_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_MIC_1_AMP_EN_SHIFT: c_int = 7;

// DA7219_MIXIN_L_CTRL = 0x65
pub const DA7219_MIXIN_L_MIX_EN_SHIFT: c_int = 3;

pub const DA7219_MIXIN_L_AMP_ZC_EN_SHIFT: c_int = 4;

pub const DA7219_MIXIN_L_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7219_MIXIN_L_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_MIXIN_L_AMP_EN_SHIFT: c_int = 7;

// DA7219_ADC_L_CTRL = 0x67
pub const DA7219_ADC_L_BIAS_SHIFT: c_int = 0;

pub const DA7219_ADC_L_RAMP_EN_SHIFT: c_int = 5;

pub const DA7219_ADC_L_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_ADC_L_EN_SHIFT: c_int = 7;

// DA7219_DAC_L_CTRL = 0x69
pub const DA7219_DAC_L_RAMP_EN_SHIFT: c_int = 5;

pub const DA7219_DAC_L_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_DAC_L_EN_SHIFT: c_int = 7;

// DA7219_DAC_R_CTRL = 0x6A
pub const DA7219_DAC_R_RAMP_EN_SHIFT: c_int = 5;

pub const DA7219_DAC_R_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_DAC_R_EN_SHIFT: c_int = 7;

// DA7219_HP_L_CTRL = 0x6B
pub const DA7219_HP_L_AMP_MIN_GAIN_EN_SHIFT: c_int = 2;

pub const DA7219_HP_L_AMP_OE_SHIFT: c_int = 3;

pub const DA7219_HP_L_AMP_ZC_EN_SHIFT: c_int = 4;

pub const DA7219_HP_L_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7219_HP_L_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_HP_L_AMP_EN_SHIFT: c_int = 7;

// DA7219_HP_R_CTRL = 0x6C
pub const DA7219_HP_R_AMP_MIN_GAIN_EN_SHIFT: c_int = 2;

pub const DA7219_HP_R_AMP_OE_SHIFT: c_int = 3;

pub const DA7219_HP_R_AMP_ZC_EN_SHIFT: c_int = 4;

pub const DA7219_HP_R_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7219_HP_R_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7219_HP_R_AMP_EN_SHIFT: c_int = 7;

// DA7219_MIXOUT_L_CTRL = 0x6E
pub const DA7219_MIXOUT_L_AMP_EN_SHIFT: c_int = 7;

// DA7219_MIXOUT_R_CTRL = 0x6F
pub const DA7219_MIXOUT_R_AMP_EN_SHIFT: c_int = 7;

// DA7219_CHIP_ID1 = 0x81
pub const DA7219_CHIP_ID1_SHIFT: c_int = 0;

// DA7219_CHIP_ID2 = 0x82
pub const DA7219_CHIP_ID2_SHIFT: c_int = 0;

// DA7219_CHIP_REVISION = 0x83
pub const DA7219_CHIP_MINOR_SHIFT: c_int = 0;

pub const DA7219_CHIP_MAJOR_SHIFT: c_int = 4;

// DA7219_IO_CTRL = 0x91
pub const DA7219_IO_VOLTAGE_LEVEL_SHIFT: c_int = 0;

pub const DA7219_IO_VOLTAGE_LEVEL_2_5V_3_6V: c_int = 0;
pub const DA7219_IO_VOLTAGE_LEVEL_1_2V_2_8V: c_int = 1;
// DA7219_GAIN_RAMP_CTRL = 0x92
pub const DA7219_GAIN_RAMP_RATE_SHIFT: c_int = 0;

pub const DA7219_GAIN_RAMP_RATE_MAX: c_int = 4;
// DA7219_PC_COUNT = 0x94
pub const DA7219_PC_FREERUN_SHIFT: c_int = 0;

pub const DA7219_PC_RESYNC_AUTO_SHIFT: c_int = 1;

// DA7219_CP_VOL_THRESHOLD1 = 0x95
pub const DA7219_CP_THRESH_VDD2_SHIFT: c_int = 0;

pub const DA7219_CP_THRESH_VDD2_MAX: c_uint = 0x3F;
// DA7219_DIG_CTRL = 0x99
pub const DA7219_DAC_L_INV_SHIFT: c_int = 3;

pub const DA7219_DAC_R_INV_SHIFT: c_int = 7;

// DA7219_ALC_CTRL2 = 0x9A
pub const DA7219_ALC_ATTACK_SHIFT: c_int = 0;

pub const DA7219_ALC_ATTACK_MAX: c_int = 13;
pub const DA7219_ALC_RELEASE_SHIFT: c_int = 4;

pub const DA7219_ALC_RELEASE_MAX: c_int = 11;
// DA7219_ALC_CTRL3 = 0x9B
pub const DA7219_ALC_HOLD_SHIFT: c_int = 0;

pub const DA7219_ALC_HOLD_MAX: c_int = 16;
pub const DA7219_ALC_INTEG_ATTACK_SHIFT: c_int = 4;

pub const DA7219_ALC_INTEG_RELEASE_SHIFT: c_int = 6;

pub const DA7219_ALC_INTEG_MAX: c_int = 4;
// DA7219_ALC_NOISE = 0x9C
pub const DA7219_ALC_NOISE_SHIFT: c_int = 0;

pub const DA7219_ALC_THRESHOLD_MAX: c_uint = 0x3F;
// DA7219_ALC_TARGET_MIN = 0x9D
pub const DA7219_ALC_THRESHOLD_MIN_SHIFT: c_int = 0;

// DA7219_ALC_TARGET_MAX = 0x9E
pub const DA7219_ALC_THRESHOLD_MAX_SHIFT: c_int = 0;

// DA7219_ALC_GAIN_LIMITS = 0x9F
pub const DA7219_ALC_ATTEN_MAX_SHIFT: c_int = 0;

pub const DA7219_ALC_GAIN_MAX_SHIFT: c_int = 4;

pub const DA7219_ALC_ATTEN_GAIN_MAX: c_uint = 0xF;
// DA7219_ALC_ANA_GAIN_LIMITS = 0xA0
pub const DA7219_ALC_ANA_GAIN_MIN_SHIFT: c_int = 0;

pub const DA7219_ALC_ANA_GAIN_MIN: c_uint = 0x1;
pub const DA7219_ALC_ANA_GAIN_MAX_SHIFT: c_int = 4;

pub const DA7219_ALC_ANA_GAIN_MAX: c_uint = 0x7;
// DA7219_ALC_ANTICLIP_CTRL = 0xA1
pub const DA7219_ALC_ANTICLIP_STEP_SHIFT: c_int = 0;

pub const DA7219_ALC_ANTICLIP_STEP_MAX: c_int = 4;
pub const DA7219_ALC_ANTIPCLIP_EN_SHIFT: c_int = 7;

// DA7219_ALC_ANTICLIP_LEVEL = 0xA2
pub const DA7219_ALC_ANTICLIP_LEVEL_SHIFT: c_int = 0;

// DA7219_ALC_OFFSET_AUTO_M_L = 0xA3
pub const DA7219_ALC_OFFSET_AUTO_M_L_SHIFT: c_int = 0;

// DA7219_ALC_OFFSET_AUTO_U_L = 0xA4
pub const DA7219_ALC_OFFSET_AUTO_U_L_SHIFT: c_int = 0;

// DA7219_DAC_NG_SETUP_TIME = 0xAF
pub const DA7219_DAC_NG_SETUP_TIME_SHIFT: c_int = 0;

pub const DA7219_DAC_NG_SETUP_TIME_MAX: c_int = 4;
pub const DA7219_DAC_NG_RAMPUP_RATE_SHIFT: c_int = 2;

pub const DA7219_DAC_NG_RAMPDN_RATE_SHIFT: c_int = 3;

pub const DA7219_DAC_NG_RAMP_RATE_MAX: c_int = 2;
// DA7219_DAC_NG_OFF_THRESH = 0xB0
pub const DA7219_DAC_NG_OFF_THRESHOLD_SHIFT: c_int = 0;

pub const DA7219_DAC_NG_THRESHOLD_MAX: c_uint = 0x7;
// DA7219_DAC_NG_ON_THRESH = 0xB1
pub const DA7219_DAC_NG_ON_THRESHOLD_SHIFT: c_int = 0;

// DA7219_DAC_NG_CTRL = 0xB2
pub const DA7219_DAC_NG_EN_SHIFT: c_int = 7;

// DA7219_TONE_GEN_CFG1 = 0xB4
pub const DA7219_DTMF_REG_SHIFT: c_int = 0;

pub const DA7219_DTMF_REG_MAX: c_int = 16;
pub const DA7219_DTMF_EN_SHIFT: c_int = 4;

pub const DA7219_START_STOPN_SHIFT: c_int = 7;

// DA7219_TONE_GEN_CFG2 = 0xB5
pub const DA7219_SWG_SEL_SHIFT: c_int = 0;

pub const DA7219_SWG_SEL_MAX: c_int = 4;

pub const DA7219_TONE_GEN_GAIN_SHIFT: c_int = 4;

pub const DA7219_TONE_GEN_GAIN_MAX: c_uint = 0xF;

// DA7219_TONE_GEN_CYCLES = 0xB6
pub const DA7219_BEEP_CYCLES_SHIFT: c_int = 0;

// DA7219_TONE_GEN_FREQ1_L = 0xB7
pub const DA7219_FREQ1_L_SHIFT: c_int = 0;

pub const DA7219_FREQ_MAX: c_uint = 0xFFFF;
// DA7219_TONE_GEN_FREQ1_U = 0xB8
pub const DA7219_FREQ1_U_SHIFT: c_int = 0;

// DA7219_TONE_GEN_FREQ2_L = 0xB9
pub const DA7219_FREQ2_L_SHIFT: c_int = 0;

// DA7219_TONE_GEN_FREQ2_U = 0xBA
pub const DA7219_FREQ2_U_SHIFT: c_int = 0;

// DA7219_TONE_GEN_ON_PER = 0xBB
pub const DA7219_BEEP_ON_PER_SHIFT: c_int = 0;

pub const DA7219_BEEP_ON_OFF_MAX: c_uint = 0x3F;
// DA7219_TONE_GEN_OFF_PER = 0xBC
pub const DA7219_BEEP_OFF_PER_SHIFT: c_int = 0;

// DA7219_SYSTEM_STATUS = 0xE0
pub const DA7219_SC1_BUSY_SHIFT: c_int = 0;

pub const DA7219_SC2_BUSY_SHIFT: c_int = 1;

// DA7219_SYSTEM_ACTIVE = 0xFD
pub const DA7219_SYSTEM_ACTIVE_SHIFT: c_int = 0;

//
// General defines & data
//
// Register inversion
pub const DA7219_NO_INVERT: c_int = 0;
pub const DA7219_INVERT: c_int = 1;
// Byte related defines
pub const DA7219_BYTE_SHIFT: c_int = 8;
pub const DA7219_BYTE_MASK: c_uint = 0xFF;
// PLL Output Frequencies
pub const DA7219_PLL_FREQ_OUT_90316: c_int = 90316800;
pub const DA7219_PLL_FREQ_OUT_98304: c_int = 98304000;
// PLL Frequency Dividers
pub const DA7219_PLL_INDIV_2_TO_4_5_MHZ_VAL: c_int = 1;
pub const DA7219_PLL_INDIV_4_5_TO_9_MHZ_VAL: c_int = 2;
pub const DA7219_PLL_INDIV_9_TO_18_MHZ_VAL: c_int = 4;
pub const DA7219_PLL_INDIV_18_TO_36_MHZ_VAL: c_int = 8;
pub const DA7219_PLL_INDIV_36_TO_54_MHZ_VAL: c_int = 16;
// SRM
pub const DA7219_SRM_CHECK_RETRIES: c_int = 8;
// System Controller
pub const DA7219_SYS_STAT_CHECK_RETRIES: c_int = 6;
pub const DA7219_SYS_STAT_CHECK_DELAY: c_int = 50;
// Power up/down Delays
pub const DA7219_SETTLING_DELAY: c_int = 40;
pub const DA7219_MIN_GAIN_DELAY: c_int = 30;
pub const DA7219_MIC_PGA_BASE_DELAY: c_int = 100;
pub const DA7219_MIC_PGA_OFFSET_DELAY: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_clk_src {
    DA7219_CLKSRC_MCLK = 0,
    DA7219_CLKSRC_MCLK_SQR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_sys_clk {
    DA7219_SYSCLK_MCLK = 0,
    DA7219_SYSCLK_PLL,
    DA7219_SYSCLK_PLL_SRM,
}

// Regulators
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7219_supplies {
    DA7219_SUPPLY_VDD = 0,
    DA7219_SUPPLY_VDDMIC,
    DA7219_SUPPLY_VDDIO,
    DA7219_NUM_SUPPLIES,
}

// Private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da7219_priv {
    pub component: *mut snd_soc_component,
    pub aad: *mut da7219_aad_priv,
    pub pdata: *mut da7219_pdata,
    pub wakeup_source: bool,
    pub supplies: [regulator_bulk_data; DA7219_NUM_SUPPLIES],
    pub regmap: *mut regmap,
    pub ctrl_lock: mutex,
    pub pll_lock: mutex,
    pub dai_clks_hw: [clk_hw; DA7219_DAI_NUM_CLKS],
    pub clk_hw_data: *mut clk_hw_onecell_data,
    pub dai_clks_lookup: [*mut clk_lookup; DA7219_DAI_NUM_CLKS],
    pub dai_clks: [*mut clk; DA7219_DAI_NUM_CLKS],
    pub mclk: *mut clk,
    pub mclk_rate: c_uint,
    pub clk_src: c_int,
    pub master: bool,
    pub tdm_en: bool,
    pub alc_en: bool,
    pub micbias_on_event: bool,
    pub mic_pga_delay: c_uint,
    pub gain_ramp_ctrl: u8,
}

extern "C" {
    pub fn da7219_set_pll(component: *mut snd_soc_component, source: c_int, fout: c_uint) -> c_int;
}

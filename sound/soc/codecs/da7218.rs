//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/da7218.h
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
// da7218.h - DA7218 ALSA SoC Codec Driver
//
// Copyright (c) 2015 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//

//
// Registers
//
pub const DA7218_SYSTEM_ACTIVE: c_uint = 0x0;
pub const DA7218_CIF_CTRL: c_uint = 0x1;
pub const DA7218_CHIP_ID1: c_uint = 0x4;
pub const DA7218_CHIP_ID2: c_uint = 0x5;
pub const DA7218_CHIP_REVISION: c_uint = 0x6;
pub const DA7218_SPARE1: c_uint = 0x7;
pub const DA7218_STATUS1: c_uint = 0x8;
pub const DA7218_SOFT_RESET: c_uint = 0x9;
pub const DA7218_SR: c_uint = 0xB;
pub const DA7218_PC_COUNT: c_uint = 0xC;
pub const DA7218_GAIN_RAMP_CTRL: c_uint = 0xD;
pub const DA7218_CIF_TIMEOUT_CTRL: c_uint = 0x10;
pub const DA7218_SYSTEM_MODES_INPUT: c_uint = 0x14;
pub const DA7218_SYSTEM_MODES_OUTPUT: c_uint = 0x15;
pub const DA7218_SYSTEM_STATUS: c_uint = 0x16;
pub const DA7218_IN_1L_FILTER_CTRL: c_uint = 0x18;
pub const DA7218_IN_1R_FILTER_CTRL: c_uint = 0x19;
pub const DA7218_IN_2L_FILTER_CTRL: c_uint = 0x1A;
pub const DA7218_IN_2R_FILTER_CTRL: c_uint = 0x1B;
pub const DA7218_OUT_1L_FILTER_CTRL: c_uint = 0x20;
pub const DA7218_OUT_1R_FILTER_CTRL: c_uint = 0x21;
pub const DA7218_OUT_1_HPF_FILTER_CTRL: c_uint = 0x24;
pub const DA7218_OUT_1_EQ_12_FILTER_CTRL: c_uint = 0x25;
pub const DA7218_OUT_1_EQ_34_FILTER_CTRL: c_uint = 0x26;
pub const DA7218_OUT_1_EQ_5_FILTER_CTRL: c_uint = 0x27;
pub const DA7218_OUT_1_BIQ_5STAGE_CTRL: c_uint = 0x28;
pub const DA7218_OUT_1_BIQ_5STAGE_DATA: c_uint = 0x29;
pub const DA7218_OUT_1_BIQ_5STAGE_ADDR: c_uint = 0x2A;
pub const DA7218_MIXIN_1_CTRL: c_uint = 0x2C;
pub const DA7218_MIXIN_1_GAIN: c_uint = 0x2D;
pub const DA7218_MIXIN_2_CTRL: c_uint = 0x2E;
pub const DA7218_MIXIN_2_GAIN: c_uint = 0x2F;
pub const DA7218_ALC_CTRL1: c_uint = 0x30;
pub const DA7218_ALC_CTRL2: c_uint = 0x31;
pub const DA7218_ALC_CTRL3: c_uint = 0x32;
pub const DA7218_ALC_NOISE: c_uint = 0x33;
pub const DA7218_ALC_TARGET_MIN: c_uint = 0x34;
pub const DA7218_ALC_TARGET_MAX: c_uint = 0x35;
pub const DA7218_ALC_GAIN_LIMITS: c_uint = 0x36;
pub const DA7218_ALC_ANA_GAIN_LIMITS: c_uint = 0x37;
pub const DA7218_ALC_ANTICLIP_CTRL: c_uint = 0x38;
pub const DA7218_AGS_ENABLE: c_uint = 0x3C;
pub const DA7218_AGS_TRIGGER: c_uint = 0x3D;
pub const DA7218_AGS_ATT_MAX: c_uint = 0x3E;
pub const DA7218_AGS_TIMEOUT: c_uint = 0x3F;
pub const DA7218_AGS_ANTICLIP_CTRL: c_uint = 0x40;
pub const DA7218_CALIB_CTRL: c_uint = 0x44;
pub const DA7218_CALIB_OFFSET_AUTO_M_1: c_uint = 0x45;
pub const DA7218_CALIB_OFFSET_AUTO_U_1: c_uint = 0x46;
pub const DA7218_CALIB_OFFSET_AUTO_M_2: c_uint = 0x47;
pub const DA7218_CALIB_OFFSET_AUTO_U_2: c_uint = 0x48;
pub const DA7218_ENV_TRACK_CTRL: c_uint = 0x4C;
pub const DA7218_LVL_DET_CTRL: c_uint = 0x50;
pub const DA7218_LVL_DET_LEVEL: c_uint = 0x51;
pub const DA7218_DGS_TRIGGER: c_uint = 0x54;
pub const DA7218_DGS_ENABLE: c_uint = 0x55;
pub const DA7218_DGS_RISE_FALL: c_uint = 0x56;
pub const DA7218_DGS_SYNC_DELAY: c_uint = 0x57;
pub const DA7218_DGS_SYNC_DELAY2: c_uint = 0x58;
pub const DA7218_DGS_SYNC_DELAY3: c_uint = 0x59;
pub const DA7218_DGS_LEVELS: c_uint = 0x5A;
pub const DA7218_DGS_GAIN_CTRL: c_uint = 0x5B;
pub const DA7218_DROUTING_OUTDAI_1L: c_uint = 0x5C;
pub const DA7218_DMIX_OUTDAI_1L_INFILT_1L_GAIN: c_uint = 0x5D;
pub const DA7218_DMIX_OUTDAI_1L_INFILT_1R_GAIN: c_uint = 0x5E;
pub const DA7218_DMIX_OUTDAI_1L_INFILT_2L_GAIN: c_uint = 0x5F;
pub const DA7218_DMIX_OUTDAI_1L_INFILT_2R_GAIN: c_uint = 0x60;
pub const DA7218_DMIX_OUTDAI_1L_TONEGEN_GAIN: c_uint = 0x61;
pub const DA7218_DMIX_OUTDAI_1L_INDAI_1L_GAIN: c_uint = 0x62;
pub const DA7218_DMIX_OUTDAI_1L_INDAI_1R_GAIN: c_uint = 0x63;
pub const DA7218_DROUTING_OUTDAI_1R: c_uint = 0x64;
pub const DA7218_DMIX_OUTDAI_1R_INFILT_1L_GAIN: c_uint = 0x65;
pub const DA7218_DMIX_OUTDAI_1R_INFILT_1R_GAIN: c_uint = 0x66;
pub const DA7218_DMIX_OUTDAI_1R_INFILT_2L_GAIN: c_uint = 0x67;
pub const DA7218_DMIX_OUTDAI_1R_INFILT_2R_GAIN: c_uint = 0x68;
pub const DA7218_DMIX_OUTDAI_1R_TONEGEN_GAIN: c_uint = 0x69;
pub const DA7218_DMIX_OUTDAI_1R_INDAI_1L_GAIN: c_uint = 0x6A;
pub const DA7218_DMIX_OUTDAI_1R_INDAI_1R_GAIN: c_uint = 0x6B;
pub const DA7218_DROUTING_OUTFILT_1L: c_uint = 0x6C;
pub const DA7218_DMIX_OUTFILT_1L_INFILT_1L_GAIN: c_uint = 0x6D;
pub const DA7218_DMIX_OUTFILT_1L_INFILT_1R_GAIN: c_uint = 0x6E;
pub const DA7218_DMIX_OUTFILT_1L_INFILT_2L_GAIN: c_uint = 0x6F;
pub const DA7218_DMIX_OUTFILT_1L_INFILT_2R_GAIN: c_uint = 0x70;
pub const DA7218_DMIX_OUTFILT_1L_TONEGEN_GAIN: c_uint = 0x71;
pub const DA7218_DMIX_OUTFILT_1L_INDAI_1L_GAIN: c_uint = 0x72;
pub const DA7218_DMIX_OUTFILT_1L_INDAI_1R_GAIN: c_uint = 0x73;
pub const DA7218_DROUTING_OUTFILT_1R: c_uint = 0x74;
pub const DA7218_DMIX_OUTFILT_1R_INFILT_1L_GAIN: c_uint = 0x75;
pub const DA7218_DMIX_OUTFILT_1R_INFILT_1R_GAIN: c_uint = 0x76;
pub const DA7218_DMIX_OUTFILT_1R_INFILT_2L_GAIN: c_uint = 0x77;
pub const DA7218_DMIX_OUTFILT_1R_INFILT_2R_GAIN: c_uint = 0x78;
pub const DA7218_DMIX_OUTFILT_1R_TONEGEN_GAIN: c_uint = 0x79;
pub const DA7218_DMIX_OUTFILT_1R_INDAI_1L_GAIN: c_uint = 0x7A;
pub const DA7218_DMIX_OUTFILT_1R_INDAI_1R_GAIN: c_uint = 0x7B;
pub const DA7218_DROUTING_OUTDAI_2L: c_uint = 0x7C;
pub const DA7218_DMIX_OUTDAI_2L_INFILT_1L_GAIN: c_uint = 0x7D;
pub const DA7218_DMIX_OUTDAI_2L_INFILT_1R_GAIN: c_uint = 0x7E;
pub const DA7218_DMIX_OUTDAI_2L_INFILT_2L_GAIN: c_uint = 0x7F;
pub const DA7218_DMIX_OUTDAI_2L_INFILT_2R_GAIN: c_uint = 0x80;
pub const DA7218_DMIX_OUTDAI_2L_TONEGEN_GAIN: c_uint = 0x81;
pub const DA7218_DMIX_OUTDAI_2L_INDAI_1L_GAIN: c_uint = 0x82;
pub const DA7218_DMIX_OUTDAI_2L_INDAI_1R_GAIN: c_uint = 0x83;
pub const DA7218_DROUTING_OUTDAI_2R: c_uint = 0x84;
pub const DA7218_DMIX_OUTDAI_2R_INFILT_1L_GAIN: c_uint = 0x85;
pub const DA7218_DMIX_OUTDAI_2R_INFILT_1R_GAIN: c_uint = 0x86;
pub const DA7218_DMIX_OUTDAI_2R_INFILT_2L_GAIN: c_uint = 0x87;
pub const DA7218_DMIX_OUTDAI_2R_INFILT_2R_GAIN: c_uint = 0x88;
pub const DA7218_DMIX_OUTDAI_2R_TONEGEN_GAIN: c_uint = 0x89;
pub const DA7218_DMIX_OUTDAI_2R_INDAI_1L_GAIN: c_uint = 0x8A;
pub const DA7218_DMIX_OUTDAI_2R_INDAI_1R_GAIN: c_uint = 0x8B;
pub const DA7218_DAI_CTRL: c_uint = 0x8C;
pub const DA7218_DAI_TDM_CTRL: c_uint = 0x8D;
pub const DA7218_DAI_OFFSET_LOWER: c_uint = 0x8E;
pub const DA7218_DAI_OFFSET_UPPER: c_uint = 0x8F;
pub const DA7218_DAI_CLK_MODE: c_uint = 0x90;
pub const DA7218_PLL_CTRL: c_uint = 0x91;
pub const DA7218_PLL_FRAC_TOP: c_uint = 0x92;
pub const DA7218_PLL_FRAC_BOT: c_uint = 0x93;
pub const DA7218_PLL_INTEGER: c_uint = 0x94;
pub const DA7218_PLL_STATUS: c_uint = 0x95;
pub const DA7218_PLL_REFOSC_CAL: c_uint = 0x98;
pub const DA7218_DAC_NG_CTRL: c_uint = 0x9C;
pub const DA7218_DAC_NG_SETUP_TIME: c_uint = 0x9D;
pub const DA7218_DAC_NG_OFF_THRESH: c_uint = 0x9E;
pub const DA7218_DAC_NG_ON_THRESH: c_uint = 0x9F;
pub const DA7218_TONE_GEN_CFG1: c_uint = 0xA0;
pub const DA7218_TONE_GEN_CFG2: c_uint = 0xA1;
pub const DA7218_TONE_GEN_FREQ1_L: c_uint = 0xA2;
pub const DA7218_TONE_GEN_FREQ1_U: c_uint = 0xA3;
pub const DA7218_TONE_GEN_FREQ2_L: c_uint = 0xA4;
pub const DA7218_TONE_GEN_FREQ2_U: c_uint = 0xA5;
pub const DA7218_TONE_GEN_CYCLES: c_uint = 0xA6;
pub const DA7218_TONE_GEN_ON_PER: c_uint = 0xA7;
pub const DA7218_TONE_GEN_OFF_PER: c_uint = 0xA8;
pub const DA7218_CP_CTRL: c_uint = 0xAC;
pub const DA7218_CP_DELAY: c_uint = 0xAD;
pub const DA7218_CP_VOL_THRESHOLD1: c_uint = 0xAE;
pub const DA7218_MIC_1_CTRL: c_uint = 0xB4;
pub const DA7218_MIC_1_GAIN: c_uint = 0xB5;
pub const DA7218_MIC_1_SELECT: c_uint = 0xB7;
pub const DA7218_MIC_2_CTRL: c_uint = 0xB8;
pub const DA7218_MIC_2_GAIN: c_uint = 0xB9;
pub const DA7218_MIC_2_SELECT: c_uint = 0xBB;
pub const DA7218_IN_1_HPF_FILTER_CTRL: c_uint = 0xBC;
pub const DA7218_IN_2_HPF_FILTER_CTRL: c_uint = 0xBD;
pub const DA7218_ADC_1_CTRL: c_uint = 0xC0;
pub const DA7218_ADC_2_CTRL: c_uint = 0xC1;
pub const DA7218_ADC_MODE: c_uint = 0xC2;
pub const DA7218_MIXOUT_L_CTRL: c_uint = 0xCC;
pub const DA7218_MIXOUT_L_GAIN: c_uint = 0xCD;
pub const DA7218_MIXOUT_R_CTRL: c_uint = 0xCE;
pub const DA7218_MIXOUT_R_GAIN: c_uint = 0xCF;
pub const DA7218_HP_L_CTRL: c_uint = 0xD0;
pub const DA7218_HP_L_GAIN: c_uint = 0xD1;
pub const DA7218_HP_R_CTRL: c_uint = 0xD2;
pub const DA7218_HP_R_GAIN: c_uint = 0xD3;
pub const DA7218_HP_SNGL_CTRL: c_uint = 0xD4;
pub const DA7218_HP_DIFF_CTRL: c_uint = 0xD5;
pub const DA7218_HP_DIFF_UNLOCK: c_uint = 0xD7;
pub const DA7218_HPLDET_JACK: c_uint = 0xD8;
pub const DA7218_HPLDET_CTRL: c_uint = 0xD9;
pub const DA7218_HPLDET_TEST: c_uint = 0xDA;
pub const DA7218_REFERENCES: c_uint = 0xDC;
pub const DA7218_IO_CTRL: c_uint = 0xE0;
pub const DA7218_LDO_CTRL: c_uint = 0xE1;
pub const DA7218_SIDETONE_CTRL: c_uint = 0xE4;
pub const DA7218_SIDETONE_IN_SELECT: c_uint = 0xE5;
pub const DA7218_SIDETONE_GAIN: c_uint = 0xE6;
pub const DA7218_DROUTING_ST_OUTFILT_1L: c_uint = 0xE8;
pub const DA7218_DROUTING_ST_OUTFILT_1R: c_uint = 0xE9;
pub const DA7218_SIDETONE_BIQ_3STAGE_DATA: c_uint = 0xEA;
pub const DA7218_SIDETONE_BIQ_3STAGE_ADDR: c_uint = 0xEB;
pub const DA7218_EVENT_STATUS: c_uint = 0xEC;
pub const DA7218_EVENT: c_uint = 0xED;
pub const DA7218_EVENT_MASK: c_uint = 0xEE;
pub const DA7218_DMIC_1_CTRL: c_uint = 0xF0;
pub const DA7218_DMIC_2_CTRL: c_uint = 0xF1;
pub const DA7218_IN_1L_GAIN: c_uint = 0xF4;
pub const DA7218_IN_1R_GAIN: c_uint = 0xF5;
pub const DA7218_IN_2L_GAIN: c_uint = 0xF6;
pub const DA7218_IN_2R_GAIN: c_uint = 0xF7;
pub const DA7218_OUT_1L_GAIN: c_uint = 0xF8;
pub const DA7218_OUT_1R_GAIN: c_uint = 0xF9;
pub const DA7218_MICBIAS_CTRL: c_uint = 0xFC;
pub const DA7218_MICBIAS_EN: c_uint = 0xFD;
//
// Bit Fields
//
pub const DA7218_SWITCH_EN_MAX: c_uint = 0x1;
// DA7218_SYSTEM_ACTIVE = 0x0
pub const DA7218_SYSTEM_ACTIVE_SHIFT: c_int = 0;

// DA7218_CIF_CTRL = 0x1
pub const DA7218_CIF_I2C_WRITE_MODE_SHIFT: c_int = 0;

// DA7218_CHIP_ID1 = 0x4
pub const DA7218_CHIP_ID1_SHIFT: c_int = 0;

// DA7218_CHIP_ID2 = 0x5
pub const DA7218_CHIP_ID2_SHIFT: c_int = 0;

// DA7218_CHIP_REVISION = 0x6
pub const DA7218_CHIP_MINOR_SHIFT: c_int = 0;

pub const DA7218_CHIP_MAJOR_SHIFT: c_int = 4;

// DA7218_SPARE1 = 0x7
pub const DA7218_SPARE1_SHIFT: c_int = 0;

// DA7218_STATUS1 = 0x8
pub const DA7218_STATUS_SPARE1_SHIFT: c_int = 0;

// DA7218_SOFT_RESET = 0x9
pub const DA7218_CIF_REG_SOFT_RESET_SHIFT: c_int = 7;

// DA7218_SR = 0xB
pub const DA7218_SR_ADC_SHIFT: c_int = 0;

pub const DA7218_SR_DAC_SHIFT: c_int = 4;

pub const DA7218_SR_8000: c_uint = 0x01;
pub const DA7218_SR_11025: c_uint = 0x02;
pub const DA7218_SR_12000: c_uint = 0x03;
pub const DA7218_SR_16000: c_uint = 0x05;
pub const DA7218_SR_22050: c_uint = 0x06;
pub const DA7218_SR_24000: c_uint = 0x07;
pub const DA7218_SR_32000: c_uint = 0x09;
pub const DA7218_SR_44100: c_uint = 0x0A;
pub const DA7218_SR_48000: c_uint = 0x0B;
pub const DA7218_SR_88200: c_uint = 0x0E;
pub const DA7218_SR_96000: c_uint = 0x0F;
// DA7218_PC_COUNT = 0xC
pub const DA7218_PC_FREERUN_SHIFT: c_int = 0;

pub const DA7218_PC_RESYNC_AUTO_SHIFT: c_int = 1;

// DA7218_GAIN_RAMP_CTRL = 0xD
pub const DA7218_GAIN_RAMP_RATE_SHIFT: c_int = 0;

pub const DA7218_GAIN_RAMP_RATE_MAX: c_int = 4;
// DA7218_CIF_TIMEOUT_CTRL = 0x10
pub const DA7218_I2C_TIMEOUT_EN_SHIFT: c_int = 0;

// DA7218_SYSTEM_MODES_INPUT = 0x14
pub const DA7218_MODE_SUBMIT_SHIFT: c_int = 0;

pub const DA7218_ADC_MODE_SHIFT: c_int = 1;

// DA7218_SYSTEM_MODES_OUTPUT = 0x15
pub const DA7218_MODE_SUBMIT_SHIFT: c_int = 0;

pub const DA7218_DAC_MODE_SHIFT: c_int = 1;

// DA7218_SYSTEM_STATUS = 0x16
pub const DA7218_SC1_BUSY_SHIFT: c_int = 0;

pub const DA7218_SC2_BUSY_SHIFT: c_int = 1;

// DA7218_IN_1L_FILTER_CTRL = 0x18
pub const DA7218_IN_1L_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_IN_1L_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_IN_1L_FILTER_EN_SHIFT: c_int = 7;

// DA7218_IN_1R_FILTER_CTRL = 0x19
pub const DA7218_IN_1R_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_IN_1R_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_IN_1R_FILTER_EN_SHIFT: c_int = 7;

// DA7218_IN_2L_FILTER_CTRL = 0x1A
pub const DA7218_IN_2L_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_IN_2L_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_IN_2L_FILTER_EN_SHIFT: c_int = 7;

// DA7218_IN_2R_FILTER_CTRL = 0x1B
pub const DA7218_IN_2R_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_IN_2R_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_IN_2R_FILTER_EN_SHIFT: c_int = 7;

// DA7218_OUT_1L_FILTER_CTRL = 0x20
pub const DA7218_OUT_1L_BIQ_5STAGE_SEL_SHIFT: c_int = 3;

pub const DA7218_OUT_BIQ_5STAGE_SEL_MAX: c_int = 2;
pub const DA7218_OUT_1L_SUBRANGE_EN_SHIFT: c_int = 4;

pub const DA7218_OUT_1L_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_OUT_1L_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_OUT_1L_FILTER_EN_SHIFT: c_int = 7;

// DA7218_OUT_1R_FILTER_CTRL = 0x21
pub const DA7218_OUT_1R_BIQ_5STAGE_SEL_SHIFT: c_int = 3;

pub const DA7218_OUT_1R_SUBRANGE_EN_SHIFT: c_int = 4;

pub const DA7218_OUT_1R_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_OUT_1R_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_OUT_1R_FILTER_EN_SHIFT: c_int = 7;

// DA7218_OUT_1_HPF_FILTER_CTRL = 0x24
pub const DA7218_OUT_1_VOICE_HPF_CORNER_SHIFT: c_int = 0;

pub const DA7218_VOICE_HPF_CORNER_MAX: c_int = 8;
pub const DA7218_OUT_1_VOICE_EN_SHIFT: c_int = 3;

pub const DA7218_OUT_1_AUDIO_HPF_CORNER_SHIFT: c_int = 4;

pub const DA7218_AUDIO_HPF_CORNER_MAX: c_int = 4;
pub const DA7218_OUT_1_HPF_EN_SHIFT: c_int = 7;

pub const DA7218_HPF_MODE_SHIFT: c_int = 0;

pub const DA7218_HPF_MODE_MAX: c_int = 3;
// DA7218_OUT_1_EQ_12_FILTER_CTRL = 0x25
pub const DA7218_OUT_1_EQ_BAND1_SHIFT: c_int = 0;

pub const DA7218_OUT_EQ_BAND_MAX: c_uint = 0xF;
pub const DA7218_OUT_1_EQ_BAND2_SHIFT: c_int = 4;

// DA7218_OUT_1_EQ_34_FILTER_CTRL = 0x26
pub const DA7218_OUT_1_EQ_BAND3_SHIFT: c_int = 0;

pub const DA7218_OUT_1_EQ_BAND4_SHIFT: c_int = 4;

// DA7218_OUT_1_EQ_5_FILTER_CTRL = 0x27
pub const DA7218_OUT_1_EQ_BAND5_SHIFT: c_int = 0;

pub const DA7218_OUT_1_EQ_EN_SHIFT: c_int = 7;

// DA7218_OUT_1_BIQ_5STAGE_CTRL = 0x28
pub const DA7218_OUT_1_BIQ_5STAGE_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_OUT_1_BIQ_5STAGE_FILTER_EN_SHIFT: c_int = 7;

// DA7218_OUT_1_BIQ_5STAGE_DATA = 0x29
pub const DA7218_OUT_1_BIQ_5STAGE_DATA_SHIFT: c_int = 0;

// DA7218_OUT_1_BIQ_5STAGE_ADDR = 0x2A
pub const DA7218_OUT_1_BIQ_5STAGE_ADDR_SHIFT: c_int = 0;

pub const DA7218_OUT_1_BIQ_5STAGE_CFG_SIZE: c_int = 50;
// DA7218_MIXIN_1_CTRL = 0x2C
pub const DA7218_MIXIN_1_MIX_SEL_SHIFT: c_int = 3;

pub const DA7218_MIXIN_1_AMP_ZC_EN_SHIFT: c_int = 4;

pub const DA7218_MIXIN_1_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_MIXIN_1_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_MIXIN_1_AMP_EN_SHIFT: c_int = 7;

// DA7218_MIXIN_1_GAIN = 0x2D
pub const DA7218_MIXIN_1_AMP_GAIN_SHIFT: c_int = 0;

pub const DA7218_MIXIN_AMP_GAIN_MAX: c_uint = 0xF;
// DA7218_MIXIN_2_CTRL = 0x2E
pub const DA7218_MIXIN_2_MIX_SEL_SHIFT: c_int = 3;

pub const DA7218_MIXIN_2_AMP_ZC_EN_SHIFT: c_int = 4;

pub const DA7218_MIXIN_2_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_MIXIN_2_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_MIXIN_2_AMP_EN_SHIFT: c_int = 7;

// DA7218_MIXIN_2_GAIN = 0x2F
pub const DA7218_MIXIN_2_AMP_GAIN_SHIFT: c_int = 0;

// DA7218_ALC_CTRL1 = 0x30
pub const DA7218_ALC_EN_SHIFT: c_int = 0;

pub const DA7218_ALC_CHAN1_L_EN_SHIFT: c_int = 0;
pub const DA7218_ALC_CHAN1_R_EN_SHIFT: c_int = 1;
pub const DA7218_ALC_CHAN2_L_EN_SHIFT: c_int = 2;
pub const DA7218_ALC_CHAN2_R_EN_SHIFT: c_int = 3;
pub const DA7218_ALC_SYNC_MODE_SHIFT: c_int = 4;

// DA7218_ALC_CTRL2 = 0x31
pub const DA7218_ALC_ATTACK_SHIFT: c_int = 0;

pub const DA7218_ALC_ATTACK_MAX: c_int = 13;
pub const DA7218_ALC_RELEASE_SHIFT: c_int = 4;

pub const DA7218_ALC_RELEASE_MAX: c_int = 11;
// DA7218_ALC_CTRL3 = 0x32
pub const DA7218_ALC_HOLD_SHIFT: c_int = 0;

pub const DA7218_ALC_HOLD_MAX: c_int = 16;
// DA7218_ALC_NOISE = 0x33
pub const DA7218_ALC_NOISE_SHIFT: c_int = 0;

pub const DA7218_ALC_THRESHOLD_MAX: c_uint = 0x3F;
// DA7218_ALC_TARGET_MIN = 0x34
pub const DA7218_ALC_THRESHOLD_MIN_SHIFT: c_int = 0;

// DA7218_ALC_TARGET_MAX = 0x35
pub const DA7218_ALC_THRESHOLD_MAX_SHIFT: c_int = 0;

// DA7218_ALC_GAIN_LIMITS = 0x36
pub const DA7218_ALC_ATTEN_MAX_SHIFT: c_int = 0;

pub const DA7218_ALC_ATTEN_GAIN_MAX: c_uint = 0xF;
pub const DA7218_ALC_GAIN_MAX_SHIFT: c_int = 4;

// DA7218_ALC_ANA_GAIN_LIMITS = 0x37
pub const DA7218_ALC_ANA_GAIN_MIN_SHIFT: c_int = 0;

pub const DA7218_ALC_ANA_GAIN_MIN: c_uint = 0x1;
pub const DA7218_ALC_ANA_GAIN_MAX: c_uint = 0x7;
pub const DA7218_ALC_ANA_GAIN_MAX_SHIFT: c_int = 4;

// DA7218_ALC_ANTICLIP_CTRL = 0x38
pub const DA7218_ALC_ANTICLIP_STEP_SHIFT: c_int = 0;

pub const DA7218_ALC_ANTICLIP_STEP_MAX: c_int = 4;
pub const DA7218_ALC_ANTICLIP_EN_SHIFT: c_int = 7;

// DA7218_AGS_ENABLE = 0x3C
pub const DA7218_AGS_ENABLE_SHIFT: c_int = 0;

pub const DA7218_AGS_ENABLE_CHAN1_SHIFT: c_int = 0;
pub const DA7218_AGS_ENABLE_CHAN2_SHIFT: c_int = 1;
// DA7218_AGS_TRIGGER = 0x3D
pub const DA7218_AGS_TRIGGER_SHIFT: c_int = 0;

pub const DA7218_AGS_TRIGGER_MAX: c_uint = 0xF;
// DA7218_AGS_ATT_MAX = 0x3E
pub const DA7218_AGS_ATT_MAX_SHIFT: c_int = 0;

pub const DA7218_AGS_ATT_MAX_MAX: c_uint = 0x7;
// DA7218_AGS_TIMEOUT = 0x3F
pub const DA7218_AGS_TIMEOUT_EN_SHIFT: c_int = 0;

// DA7218_AGS_ANTICLIP_CTRL = 0x40
pub const DA7218_AGS_ANTICLIP_EN_SHIFT: c_int = 7;

// DA7218_CALIB_CTRL = 0x44
pub const DA7218_CALIB_OFFSET_EN_SHIFT: c_int = 0;

pub const DA7218_CALIB_AUTO_EN_SHIFT: c_int = 2;

pub const DA7218_CALIB_OVERFLOW_SHIFT: c_int = 3;

// DA7218_CALIB_OFFSET_AUTO_M_1 = 0x45
pub const DA7218_CALIB_OFFSET_AUTO_M_1_SHIFT: c_int = 0;

// DA7218_CALIB_OFFSET_AUTO_U_1 = 0x46
pub const DA7218_CALIB_OFFSET_AUTO_U_1_SHIFT: c_int = 0;

// DA7218_CALIB_OFFSET_AUTO_M_2 = 0x47
pub const DA7218_CALIB_OFFSET_AUTO_M_2_SHIFT: c_int = 0;

// DA7218_CALIB_OFFSET_AUTO_U_2 = 0x48
pub const DA7218_CALIB_OFFSET_AUTO_U_2_SHIFT: c_int = 0;

// DA7218_ENV_TRACK_CTRL = 0x4C
pub const DA7218_INTEG_ATTACK_SHIFT: c_int = 0;

pub const DA7218_INTEG_RELEASE_SHIFT: c_int = 4;

pub const DA7218_INTEG_MAX: c_int = 4;
// DA7218_LVL_DET_CTRL = 0x50
pub const DA7218_LVL_DET_EN_SHIFT: c_int = 0;

pub const DA7218_LVL_DET_EN_CHAN1L_SHIFT: c_int = 0;
pub const DA7218_LVL_DET_EN_CHAN1R_SHIFT: c_int = 1;
pub const DA7218_LVL_DET_EN_CHAN2L_SHIFT: c_int = 2;
pub const DA7218_LVL_DET_EN_CHAN2R_SHIFT: c_int = 3;
// DA7218_LVL_DET_LEVEL = 0x51
pub const DA7218_LVL_DET_LEVEL_SHIFT: c_int = 0;

pub const DA7218_LVL_DET_LEVEL_MAX: c_uint = 0x7F;
// DA7218_DGS_TRIGGER = 0x54
pub const DA7218_DGS_TRIGGER_LVL_SHIFT: c_int = 0;

pub const DA7218_DGS_TRIGGER_MAX: c_uint = 0x3F;
// DA7218_DGS_ENABLE = 0x55
pub const DA7218_DGS_ENABLE_SHIFT: c_int = 0;

pub const DA7218_DGS_ENABLE_L_SHIFT: c_int = 0;
pub const DA7218_DGS_ENABLE_R_SHIFT: c_int = 1;
// DA7218_DGS_RISE_FALL = 0x56
pub const DA7218_DGS_RISE_COEFF_SHIFT: c_int = 0;

pub const DA7218_DGS_RISE_COEFF_MAX: c_int = 7;
pub const DA7218_DGS_FALL_COEFF_SHIFT: c_int = 4;

pub const DA7218_DGS_FALL_COEFF_MAX: c_int = 8;
// DA7218_DGS_SYNC_DELAY = 0x57
pub const DA7218_DGS_SYNC_DELAY_SHIFT: c_int = 0;

pub const DA7218_DGS_SYNC_DELAY_MAX: c_uint = 0xFF;
// DA7218_DGS_SYNC_DELAY2 = 0x58
pub const DA7218_DGS_SYNC_DELAY2_SHIFT: c_int = 0;

// DA7218_DGS_SYNC_DELAY3 = 0x59
pub const DA7218_DGS_SYNC_DELAY3_SHIFT: c_int = 0;

pub const DA7218_DGS_SYNC_DELAY3_MAX: c_uint = 0x7F;
// DA7218_DGS_LEVELS = 0x5A
pub const DA7218_DGS_ANTICLIP_LVL_SHIFT: c_int = 0;

pub const DA7218_DGS_ANTICLIP_LVL_MAX: c_uint = 0x7;
pub const DA7218_DGS_SIGNAL_LVL_SHIFT: c_int = 4;

pub const DA7218_DGS_SIGNAL_LVL_MAX: c_uint = 0xF;
// DA7218_DGS_GAIN_CTRL = 0x5B
pub const DA7218_DGS_STEPS_SHIFT: c_int = 0;

pub const DA7218_DGS_STEPS_MAX: c_uint = 0x1F;
pub const DA7218_DGS_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_DGS_SUBR_EN_SHIFT: c_int = 6;

// DA7218_DROUTING_OUTDAI_1L = 0x5C
pub const DA7218_OUTDAI_1L_SRC_SHIFT: c_int = 0;

pub const DA7218_DMIX_SRC_INFILT1L: c_int = 0;
pub const DA7218_DMIX_SRC_INFILT1R: c_int = 1;
pub const DA7218_DMIX_SRC_INFILT2L: c_int = 2;
pub const DA7218_DMIX_SRC_INFILT2R: c_int = 3;
pub const DA7218_DMIX_SRC_TONEGEN: c_int = 4;
pub const DA7218_DMIX_SRC_DAIL: c_int = 5;
pub const DA7218_DMIX_SRC_DAIR: c_int = 6;
// DA7218_DMIX_OUTDAI_1L_INFILT_1L_GAIN = 0x5D
pub const DA7218_OUTDAI_1L_INFILT_1L_GAIN_SHIFT: c_int = 0;

pub const DA7218_DMIX_GAIN_MAX: c_uint = 0x1F;
// DA7218_DMIX_OUTDAI_1L_INFILT_1R_GAIN = 0x5E
pub const DA7218_OUTDAI_1L_INFILT_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1L_INFILT_2L_GAIN = 0x5F
pub const DA7218_OUTDAI_1L_INFILT_2L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1L_INFILT_2R_GAIN = 0x60
pub const DA7218_OUTDAI_1L_INFILT_2R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1L_TONEGEN_GAIN = 0x61
pub const DA7218_OUTDAI_1L_TONEGEN_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1L_INDAI_1L_GAIN = 0x62
pub const DA7218_OUTDAI_1L_INDAI_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1L_INDAI_1R_GAIN = 0x63
pub const DA7218_OUTDAI_1L_INDAI_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DROUTING_OUTDAI_1R = 0x64
pub const DA7218_OUTDAI_1R_SRC_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1R_INFILT_1L_GAIN = 0x65
pub const DA7218_OUTDAI_1R_INFILT_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1R_INFILT_1R_GAIN = 0x66
pub const DA7218_OUTDAI_1R_INFILT_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1R_INFILT_2L_GAIN = 0x67
pub const DA7218_OUTDAI_1R_INFILT_2L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1R_INFILT_2R_GAIN = 0x68
pub const DA7218_OUTDAI_1R_INFILT_2R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1R_TONEGEN_GAIN = 0x69
pub const DA7218_OUTDAI_1R_TONEGEN_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1R_INDAI_1L_GAIN = 0x6A
pub const DA7218_OUTDAI_1R_INDAI_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_1R_INDAI_1R_GAIN = 0x6B
pub const DA7218_OUTDAI_1R_INDAI_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DROUTING_OUTFILT_1L = 0x6C
pub const DA7218_OUTFILT_1L_SRC_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1L_INFILT_1L_GAIN = 0x6D
pub const DA7218_OUTFILT_1L_INFILT_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1L_INFILT_1R_GAIN = 0x6E
pub const DA7218_OUTFILT_1L_INFILT_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1L_INFILT_2L_GAIN = 0x6F
pub const DA7218_OUTFILT_1L_INFILT_2L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1L_INFILT_2R_GAIN = 0x70
pub const DA7218_OUTFILT_1L_INFILT_2R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1L_TONEGEN_GAIN = 0x71
pub const DA7218_OUTFILT_1L_TONEGEN_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1L_INDAI_1L_GAIN = 0x72
pub const DA7218_OUTFILT_1L_INDAI_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1L_INDAI_1R_GAIN = 0x73
pub const DA7218_OUTFILT_1L_INDAI_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DROUTING_OUTFILT_1R = 0x74
pub const DA7218_OUTFILT_1R_SRC_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1R_INFILT_1L_GAIN = 0x75
pub const DA7218_OUTFILT_1R_INFILT_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1R_INFILT_1R_GAIN = 0x76
pub const DA7218_OUTFILT_1R_INFILT_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1R_INFILT_2L_GAIN = 0x77
pub const DA7218_OUTFILT_1R_INFILT_2L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1R_INFILT_2R_GAIN = 0x78
pub const DA7218_OUTFILT_1R_INFILT_2R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1R_TONEGEN_GAIN = 0x79
pub const DA7218_OUTFILT_1R_TONEGEN_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1R_INDAI_1L_GAIN = 0x7A
pub const DA7218_OUTFILT_1R_INDAI_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTFILT_1R_INDAI_1R_GAIN = 0x7B
pub const DA7218_OUTFILT_1R_INDAI_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DROUTING_OUTDAI_2L = 0x7C
pub const DA7218_OUTDAI_2L_SRC_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2L_INFILT_1L_GAIN = 0x7D
pub const DA7218_OUTDAI_2L_INFILT_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2L_INFILT_1R_GAIN = 0x7E
pub const DA7218_OUTDAI_2L_INFILT_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2L_INFILT_2L_GAIN = 0x7F
pub const DA7218_OUTDAI_2L_INFILT_2L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2L_INFILT_2R_GAIN = 0x80
pub const DA7218_OUTDAI_2L_INFILT_2R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2L_TONEGEN_GAIN = 0x81
pub const DA7218_OUTDAI_2L_TONEGEN_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2L_INDAI_1L_GAIN = 0x82
pub const DA7218_OUTDAI_2L_INDAI_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2L_INDAI_1R_GAIN = 0x83
pub const DA7218_OUTDAI_2L_INDAI_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DROUTING_OUTDAI_2R = 0x84
pub const DA7218_OUTDAI_2R_SRC_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2R_INFILT_1L_GAIN = 0x85
pub const DA7218_OUTDAI_2R_INFILT_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2R_INFILT_1R_GAIN = 0x86
pub const DA7218_OUTDAI_2R_INFILT_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2R_INFILT_2L_GAIN = 0x87
pub const DA7218_OUTDAI_2R_INFILT_2L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2R_INFILT_2R_GAIN = 0x88
pub const DA7218_OUTDAI_2R_INFILT_2R_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2R_TONEGEN_GAIN = 0x89
pub const DA7218_OUTDAI_2R_TONEGEN_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2R_INDAI_1L_GAIN = 0x8A
pub const DA7218_OUTDAI_2R_INDAI_1L_GAIN_SHIFT: c_int = 0;

// DA7218_DMIX_OUTDAI_2R_INDAI_1R_GAIN = 0x8B
pub const DA7218_OUTDAI_2R_INDAI_1R_GAIN_SHIFT: c_int = 0;

// DA7218_DAI_CTRL = 0x8C
pub const DA7218_DAI_FORMAT_SHIFT: c_int = 0;

pub const DA7218_DAI_WORD_LENGTH_SHIFT: c_int = 2;

pub const DA7218_DAI_CH_NUM_SHIFT: c_int = 4;

pub const DA7218_DAI_CH_NUM_MAX: c_int = 4;
pub const DA7218_DAI_EN_SHIFT: c_int = 7;

// DA7218_DAI_TDM_CTRL = 0x8D
pub const DA7218_DAI_TDM_CH_EN_SHIFT: c_int = 0;

pub const DA7218_DAI_TDM_MAX_SLOTS: c_int = 4;
pub const DA7218_DAI_OE_SHIFT: c_int = 6;

pub const DA7218_DAI_TDM_MODE_EN_SHIFT: c_int = 7;

// DA7218_DAI_OFFSET_LOWER = 0x8E
pub const DA7218_DAI_OFFSET_LOWER_SHIFT: c_int = 0;

// DA7218_DAI_OFFSET_UPPER = 0x8F
pub const DA7218_DAI_OFFSET_UPPER_SHIFT: c_int = 0;

// DA7218_DAI_CLK_MODE = 0x90
pub const DA7218_DAI_BCLKS_PER_WCLK_SHIFT: c_int = 0;

pub const DA7218_DAI_CLK_POL_SHIFT: c_int = 2;

pub const DA7218_DAI_WCLK_POL_SHIFT: c_int = 3;

pub const DA7218_DAI_WCLK_TRI_STATE_SHIFT: c_int = 4;

pub const DA7218_DAI_CLK_EN_SHIFT: c_int = 7;

// DA7218_PLL_CTRL = 0x91
pub const DA7218_PLL_INDIV_SHIFT: c_int = 0;

pub const DA7218_PLL_MCLK_SQR_EN_SHIFT: c_int = 4;

pub const DA7218_PLL_MODE_SHIFT: c_int = 6;

// DA7218_PLL_FRAC_TOP = 0x92
pub const DA7218_PLL_FBDIV_FRAC_TOP_SHIFT: c_int = 0;

// DA7218_PLL_FRAC_BOT = 0x93
pub const DA7218_PLL_FBDIV_FRAC_BOT_SHIFT: c_int = 0;

// DA7218_PLL_INTEGER = 0x94
pub const DA7218_PLL_FBDIV_INTEGER_SHIFT: c_int = 0;

// DA7218_PLL_STATUS = 0x95
pub const DA7218_PLL_SRM_STATUS_SHIFT: c_int = 0;

// DA7218_PLL_REFOSC_CAL = 0x98
pub const DA7218_PLL_REFOSC_CAL_CTRL_SHIFT: c_int = 0;

pub const DA7218_PLL_REFOSC_CAL_START_SHIFT: c_int = 6;

pub const DA7218_PLL_REFOSC_CAL_EN_SHIFT: c_int = 7;

// DA7218_DAC_NG_CTRL = 0x9C
pub const DA7218_DAC_NG_EN_SHIFT: c_int = 7;

// DA7218_DAC_NG_SETUP_TIME = 0x9D
pub const DA7218_DAC_NG_SETUP_TIME_SHIFT: c_int = 0;

pub const DA7218_DAC_NG_SETUP_TIME_MAX: c_int = 4;
pub const DA7218_DAC_NG_RAMPUP_RATE_SHIFT: c_int = 2;

pub const DA7218_DAC_NG_RAMPUP_RATE_MAX: c_int = 2;
pub const DA7218_DAC_NG_RAMPDN_RATE_SHIFT: c_int = 3;

pub const DA7218_DAC_NG_RAMPDN_RATE_MAX: c_int = 2;
// DA7218_DAC_NG_OFF_THRESH = 0x9E
pub const DA7218_DAC_NG_OFF_THRESHOLD_SHIFT: c_int = 0;

pub const DA7218_DAC_NG_THRESHOLD_MAX: c_uint = 0x7;
// DA7218_DAC_NG_ON_THRESH = 0x9F
pub const DA7218_DAC_NG_ON_THRESHOLD_SHIFT: c_int = 0;

// DA7218_TONE_GEN_CFG1 = 0xA0
pub const DA7218_DTMF_REG_SHIFT: c_int = 0;

pub const DA7218_DTMF_REG_MAX: c_int = 16;
pub const DA7218_DTMF_EN_SHIFT: c_int = 4;

pub const DA7218_START_STOPN_SHIFT: c_int = 7;

// DA7218_TONE_GEN_CFG2 = 0xA1
pub const DA7218_SWG_SEL_SHIFT: c_int = 0;

pub const DA7218_SWG_SEL_MAX: c_int = 4;
// DA7218_TONE_GEN_FREQ1_L = 0xA2
pub const DA7218_FREQ1_L_SHIFT: c_int = 0;

pub const DA7218_FREQ_MAX: c_uint = 0xFFFF;
// DA7218_TONE_GEN_FREQ1_U = 0xA3
pub const DA7218_FREQ1_U_SHIFT: c_int = 0;

// DA7218_TONE_GEN_FREQ2_L = 0xA4
pub const DA7218_FREQ2_L_SHIFT: c_int = 0;

// DA7218_TONE_GEN_FREQ2_U = 0xA5
pub const DA7218_FREQ2_U_SHIFT: c_int = 0;

// DA7218_TONE_GEN_CYCLES = 0xA6
pub const DA7218_BEEP_CYCLES_SHIFT: c_int = 0;

// DA7218_TONE_GEN_ON_PER = 0xA7
pub const DA7218_BEEP_ON_PER_SHIFT: c_int = 0;

// DA7218_TONE_GEN_OFF_PER = 0xA8
pub const DA7218_BEEP_OFF_PER_SHIFT: c_int = 0;

pub const DA7218_BEEP_ON_OFF_MAX: c_uint = 0x3F;
// DA7218_CP_CTRL = 0xAC
pub const DA7218_CP_MOD_SHIFT: c_int = 2;

pub const DA7218_CP_MCHANGE_SHIFT: c_int = 4;

pub const DA7218_CP_MCHANGE_REL_MASK: c_uint = 0x3;
pub const DA7218_CP_MCHANGE_MAX: c_int = 3;
pub const DA7218_CP_MCHANGE_LARGEST_VOL: c_uint = 0x1;
pub const DA7218_CP_MCHANGE_DAC_VOL: c_uint = 0x2;
pub const DA7218_CP_MCHANGE_SIG_MAG: c_uint = 0x3;
pub const DA7218_CP_SMALL_SWITCH_FREQ_EN_SHIFT: c_int = 6;

pub const DA7218_CP_EN_SHIFT: c_int = 7;

// DA7218_CP_DELAY = 0xAD
pub const DA7218_CP_FCONTROL_SHIFT: c_int = 0;

pub const DA7218_CP_FCONTROL_MAX: c_int = 6;
pub const DA7218_CP_TAU_DELAY_SHIFT: c_int = 3;

pub const DA7218_CP_TAU_DELAY_MAX: c_int = 8;
// DA7218_CP_VOL_THRESHOLD1 = 0xAE
pub const DA7218_CP_THRESH_VDD2_SHIFT: c_int = 0;

pub const DA7218_CP_THRESH_VDD2_MAX: c_uint = 0x3F;
// DA7218_MIC_1_CTRL = 0xB4
pub const DA7218_MIC_1_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_MIC_1_AMP_EN_SHIFT: c_int = 7;

// DA7218_MIC_1_GAIN = 0xB5
pub const DA7218_MIC_1_AMP_GAIN_SHIFT: c_int = 0;

pub const DA7218_MIC_AMP_GAIN_MAX: c_uint = 0x7;
// DA7218_MIC_1_SELECT = 0xB7
pub const DA7218_MIC_1_AMP_IN_SEL_SHIFT: c_int = 0;

// DA7218_MIC_2_CTRL = 0xB8
pub const DA7218_MIC_2_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_MIC_2_AMP_EN_SHIFT: c_int = 7;

// DA7218_MIC_2_GAIN = 0xB9
pub const DA7218_MIC_2_AMP_GAIN_SHIFT: c_int = 0;

// DA7218_MIC_2_SELECT = 0xBB
pub const DA7218_MIC_2_AMP_IN_SEL_SHIFT: c_int = 0;

// DA7218_IN_1_HPF_FILTER_CTRL = 0xBC
pub const DA7218_IN_1_VOICE_HPF_CORNER_SHIFT: c_int = 0;

pub const DA7218_IN_VOICE_HPF_CORNER_MAX: c_int = 8;
pub const DA7218_IN_1_VOICE_EN_SHIFT: c_int = 3;

pub const DA7218_IN_1_AUDIO_HPF_CORNER_SHIFT: c_int = 4;

pub const DA7218_IN_1_HPF_EN_SHIFT: c_int = 7;

// DA7218_IN_2_HPF_FILTER_CTRL = 0xBD
pub const DA7218_IN_2_VOICE_HPF_CORNER_SHIFT: c_int = 0;

pub const DA7218_IN_2_VOICE_EN_SHIFT: c_int = 3;

pub const DA7218_IN_2_AUDIO_HPF_CORNER_SHIFT: c_int = 4;

pub const DA7218_IN_2_HPF_EN_SHIFT: c_int = 7;

// DA7218_ADC_1_CTRL = 0xC0
pub const DA7218_ADC_1_AAF_EN_SHIFT: c_int = 2;

// DA7218_ADC_2_CTRL = 0xC1
pub const DA7218_ADC_2_AAF_EN_SHIFT: c_int = 2;

// DA7218_ADC_MODE = 0xC2
pub const DA7218_ADC_LP_MODE_SHIFT: c_int = 0;

pub const DA7218_ADC_LVLDET_MODE_SHIFT: c_int = 1;

pub const DA7218_ADC_LVLDET_AUTO_EXIT_SHIFT: c_int = 2;

// DA7218_MIXOUT_L_CTRL = 0xCC
pub const DA7218_MIXOUT_L_AMP_EN_SHIFT: c_int = 7;

// DA7218_MIXOUT_L_GAIN = 0xCD
pub const DA7218_MIXOUT_L_AMP_GAIN_SHIFT: c_int = 0;

pub const DA7218_MIXOUT_AMP_GAIN_MIN: c_uint = 0x1;
pub const DA7218_MIXOUT_AMP_GAIN_MAX: c_uint = 0x3;
// DA7218_MIXOUT_R_CTRL = 0xCE
pub const DA7218_MIXOUT_R_AMP_EN_SHIFT: c_int = 7;

// DA7218_MIXOUT_R_GAIN = 0xCF
pub const DA7218_MIXOUT_R_AMP_GAIN_SHIFT: c_int = 0;

// DA7218_HP_L_CTRL = 0xD0
pub const DA7218_HP_L_AMP_MIN_GAIN_EN_SHIFT: c_int = 2;

pub const DA7218_HP_L_AMP_OE_SHIFT: c_int = 3;

pub const DA7218_HP_L_AMP_ZC_EN_SHIFT: c_int = 4;

pub const DA7218_HP_L_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_HP_L_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_HP_L_AMP_EN_SHIFT: c_int = 7;

// DA7218_HP_L_GAIN = 0xD1
pub const DA7218_HP_L_AMP_GAIN_SHIFT: c_int = 0;

pub const DA7218_HP_AMP_GAIN_MIN: c_uint = 0x15;
pub const DA7218_HP_AMP_GAIN_MAX: c_uint = 0x3F;
// DA7218_HP_R_CTRL = 0xD2
pub const DA7218_HP_R_AMP_MIN_GAIN_EN_SHIFT: c_int = 2;

pub const DA7218_HP_R_AMP_OE_SHIFT: c_int = 3;

pub const DA7218_HP_R_AMP_ZC_EN_SHIFT: c_int = 4;

pub const DA7218_HP_R_AMP_RAMP_EN_SHIFT: c_int = 5;

pub const DA7218_HP_R_AMP_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_HP_R_AMP_EN_SHIFT: c_int = 7;

// DA7218_HP_R_GAIN = 0xD3
pub const DA7218_HP_R_AMP_GAIN_SHIFT: c_int = 0;

// DA7218_HP_SNGL_CTRL = 0xD4
pub const DA7218_HP_AMP_STEREO_DETECT_STATUS_SHIFT: c_int = 0;

pub const DA7218_HPL_AMP_LOAD_DETECT_STATUS_SHIFT: c_int = 1;

pub const DA7218_HPR_AMP_LOAD_DETECT_STATUS_SHIFT: c_int = 2;

pub const DA7218_HP_AMP_LOAD_DETECT_EN_SHIFT: c_int = 6;

pub const DA7218_HP_AMP_STEREO_DETECT_EN_SHIFT: c_int = 7;

// DA7218_HP_DIFF_CTRL = 0xD5
pub const DA7218_HP_AMP_DIFF_MODE_EN_SHIFT: c_int = 0;

pub const DA7218_HP_AMP_SINGLE_SUPPLY_EN_SHIFT: c_int = 4;

// DA7218_HP_DIFF_UNLOCK = 0xD7
pub const DA7218_HP_DIFF_UNLOCK_SHIFT: c_int = 0;

pub const DA7218_HP_DIFF_UNLOCK_VAL: c_uint = 0xC3;
// DA7218_HPLDET_JACK = 0xD8
pub const DA7218_HPLDET_JACK_RATE_SHIFT: c_int = 0;

pub const DA7218_HPLDET_JACK_DEBOUNCE_SHIFT: c_int = 3;

pub const DA7218_HPLDET_JACK_THR_SHIFT: c_int = 5;

pub const DA7218_HPLDET_JACK_EN_SHIFT: c_int = 7;

// DA7218_HPLDET_CTRL = 0xD9
pub const DA7218_HPLDET_COMP_INV_SHIFT: c_int = 0;

pub const DA7218_HPLDET_HYST_EN_SHIFT: c_int = 1;

pub const DA7218_HPLDET_DISCHARGE_EN_SHIFT: c_int = 7;

// DA7218_HPLDET_TEST = 0xDA
pub const DA7218_HPLDET_COMP_STS_SHIFT: c_int = 4;

// DA7218_REFERENCES = 0xDC
pub const DA7218_BIAS_EN_SHIFT: c_int = 3;

// DA7218_IO_CTRL = 0xE0
pub const DA7218_IO_VOLTAGE_LEVEL_SHIFT: c_int = 0;

pub const DA7218_IO_VOLTAGE_LEVEL_2_5V_3_6V: c_int = 0;
pub const DA7218_IO_VOLTAGE_LEVEL_1_5V_2_5V: c_int = 1;
// DA7218_LDO_CTRL = 0xE1
pub const DA7218_LDO_LEVEL_SELECT_SHIFT: c_int = 4;

pub const DA7218_LDO_EN_SHIFT: c_int = 7;

// DA7218_SIDETONE_CTRL = 0xE4
pub const DA7218_SIDETONE_MUTE_EN_SHIFT: c_int = 6;

pub const DA7218_SIDETONE_FILTER_EN_SHIFT: c_int = 7;

// DA7218_SIDETONE_IN_SELECT = 0xE5
pub const DA7218_SIDETONE_IN_SELECT_SHIFT: c_int = 0;

pub const DA7218_SIDETONE_IN_SELECT_MAX: c_int = 4;
// DA7218_SIDETONE_GAIN = 0xE6
pub const DA7218_SIDETONE_GAIN_SHIFT: c_int = 0;

// DA7218_DROUTING_ST_OUTFILT_1L = 0xE8
pub const DA7218_OUTFILT_ST_1L_SRC_SHIFT: c_int = 0;

pub const DA7218_DMIX_ST_SRC_OUTFILT1L: c_int = 0;
pub const DA7218_DMIX_ST_SRC_OUTFILT1R: c_int = 1;
pub const DA7218_DMIX_ST_SRC_SIDETONE: c_int = 2;
// DA7218_DROUTING_ST_OUTFILT_1R = 0xE9
pub const DA7218_OUTFILT_ST_1R_SRC_SHIFT: c_int = 0;

// DA7218_SIDETONE_BIQ_3STAGE_DATA = 0xEA
pub const DA7218_SIDETONE_BIQ_3STAGE_DATA_SHIFT: c_int = 0;

// DA7218_SIDETONE_BIQ_3STAGE_ADDR = 0xEB
pub const DA7218_SIDETONE_BIQ_3STAGE_ADDR_SHIFT: c_int = 0;

pub const DA7218_SIDETONE_BIQ_3STAGE_CFG_SIZE: c_int = 30;
// DA7218_EVENT_STATUS = 0xEC
pub const DA7218_HPLDET_JACK_STS_SHIFT: c_int = 7;

// DA7218_EVENT = 0xED
pub const DA7218_LVL_DET_EVENT_SHIFT: c_int = 0;

pub const DA7218_HPLDET_JACK_EVENT_SHIFT: c_int = 7;

// DA7218_EVENT_MASK	= 0xEE
pub const DA7218_LVL_DET_EVENT_MSK_SHIFT: c_int = 0;

pub const DA7218_HPLDET_JACK_EVENT_IRQ_MSK_SHIFT: c_int = 7;

// DA7218_DMIC_1_CTRL = 0xF0
pub const DA7218_DMIC_1_DATA_SEL_SHIFT: c_int = 0;

pub const DA7218_DMIC_1_SAMPLEPHASE_SHIFT: c_int = 1;

pub const DA7218_DMIC_1_CLK_RATE_SHIFT: c_int = 2;

pub const DA7218_DMIC_1L_EN_SHIFT: c_int = 6;

pub const DA7218_DMIC_1R_EN_SHIFT: c_int = 7;

// DA7218_DMIC_2_CTRL = 0xF1
pub const DA7218_DMIC_2_DATA_SEL_SHIFT: c_int = 0;

pub const DA7218_DMIC_2_SAMPLEPHASE_SHIFT: c_int = 1;

pub const DA7218_DMIC_2_CLK_RATE_SHIFT: c_int = 2;

pub const DA7218_DMIC_2L_EN_SHIFT: c_int = 6;

pub const DA7218_DMIC_2R_EN_SHIFT: c_int = 7;

// DA7218_IN_1L_GAIN = 0xF4
pub const DA7218_IN_1L_DIGITAL_GAIN_SHIFT: c_int = 0;

pub const DA7218_IN_DIGITAL_GAIN_MAX: c_uint = 0x7F;
// DA7218_IN_1R_GAIN = 0xF5
pub const DA7218_IN_1R_DIGITAL_GAIN_SHIFT: c_int = 0;

// DA7218_IN_2L_GAIN = 0xF6
pub const DA7218_IN_2L_DIGITAL_GAIN_SHIFT: c_int = 0;

// DA7218_IN_2R_GAIN = 0xF7
pub const DA7218_IN_2R_DIGITAL_GAIN_SHIFT: c_int = 0;

// DA7218_OUT_1L_GAIN = 0xF8
pub const DA7218_OUT_1L_DIGITAL_GAIN_SHIFT: c_int = 0;

pub const DA7218_OUT_DIGITAL_GAIN_MIN: c_uint = 0x0;
pub const DA7218_OUT_DIGITAL_GAIN_MAX: c_uint = 0x97;
// DA7218_OUT_1R_GAIN = 0xF9
pub const DA7218_OUT_1R_DIGITAL_GAIN_SHIFT: c_int = 0;

// DA7218_MICBIAS_CTRL = 0xFC
pub const DA7218_MICBIAS_1_LEVEL_SHIFT: c_int = 0;

pub const DA7218_MICBIAS_1_LP_MODE_SHIFT: c_int = 3;

pub const DA7218_MICBIAS_2_LEVEL_SHIFT: c_int = 4;

pub const DA7218_MICBIAS_2_LP_MODE_SHIFT: c_int = 7;

// DA7218_MICBIAS_EN = 0xFD
pub const DA7218_MICBIAS_1_EN_SHIFT: c_int = 0;

pub const DA7218_MICBIAS_2_EN_SHIFT: c_int = 4;

//
// General defines & data
//
// Register inversion
pub const DA7218_NO_INVERT: c_int = 0;
pub const DA7218_INVERT: c_int = 1;
// Byte related defines
pub const DA7218_BYTE_SHIFT: c_int = 8;
pub const DA7218_BYTE_MASK: c_uint = 0xFF;
pub const DA7218_2BYTE_SHIFT: c_int = 16;
pub const DA7218_2BYTE_MASK: c_uint = 0xFFFF;
// PLL Output Frequencies
pub const DA7218_PLL_FREQ_OUT_90316: c_int = 90316800;
pub const DA7218_PLL_FREQ_OUT_98304: c_int = 98304000;
// PLL Frequency Dividers
pub const DA7218_PLL_INDIV_2_TO_4_5_MHZ_VAL: c_int = 1;
pub const DA7218_PLL_INDIV_4_5_TO_9_MHZ_VAL: c_int = 2;
pub const DA7218_PLL_INDIV_9_TO_18_MHZ_VAL: c_int = 4;
pub const DA7218_PLL_INDIV_18_TO_36_MHZ_VAL: c_int = 8;
pub const DA7218_PLL_INDIV_36_TO_54_MHZ_VAL: c_int = 16;
// ALC Calibration
pub const DA7218_ALC_CALIB_DELAY_MIN: c_int = 2500;
pub const DA7218_ALC_CALIB_DELAY_MAX: c_int = 5000;
pub const DA7218_ALC_CALIB_MAX_TRIES: c_int = 5;
// Ref Oscillator
pub const DA7218_REF_OSC_CHECK_DELAY_MIN: c_int = 5000;
pub const DA7218_REF_OSC_CHECK_DELAY_MAX: c_int = 10000;
pub const DA7218_REF_OSC_CHECK_TRIES: c_int = 4;
// SRM
pub const DA7218_SRM_CHECK_DELAY: c_int = 50;
pub const DA7218_SRM_CHECK_TRIES: c_int = 8;
// Mic Level Detect
pub const DA7218_MIC_LVL_DET_DELAY: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7218_biq_cfg {
    DA7218_BIQ_CFG_DATA = 0,
    DA7218_BIQ_CFG_ADDR,
    DA7218_BIQ_CFG_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7218_clk_src {
    DA7218_CLKSRC_MCLK = 0,
    DA7218_CLKSRC_MCLK_SQR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7218_sys_clk {
    DA7218_SYSCLK_MCLK = 0,
    DA7218_SYSCLK_PLL,
    DA7218_SYSCLK_PLL_SRM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7218_dev_id {
    DA7217_DEV_ID = 1,
    DA7218_DEV_ID,
}

// Regulators
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da7218_supplies {
    DA7218_SUPPLY_VDD = 0,
    DA7218_SUPPLY_VDDMIC,
    DA7218_SUPPLY_VDDIO,
    DA7218_NUM_SUPPLIES,
}

// Private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da7218_priv {
    pub pdata: *mut da7218_pdata,
    pub supplies: [regulator_bulk_data; DA7218_NUM_SUPPLIES],
    pub regmap: *mut regmap,
    pub dev_id: c_int,
    pub jack: *mut snd_soc_jack,
    pub irq: c_int,
    pub mclk: *mut clk,
    pub mclk_rate: c_uint,
    pub hp_single_supply: bool,
    pub master: bool,
    pub alc_en: u8,
    pub in_filt_en: u8,
    pub mic_lvl_det_en: u8,
    pub biq_5stage_coeff: [u8; DA7218_OUT_1_BIQ_5STAGE_CFG_SIZE],
    pub stbiq_3stage_coeff: [u8; DA7218_SIDETONE_BIQ_3STAGE_CFG_SIZE],
}

// HP detect control
extern "C" {
    pub fn da7218_hpldet(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int;
}

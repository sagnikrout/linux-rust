//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/hisilicon/hi6210-i2s.h
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
// linux/sound/soc/hisilicon/hi6210-i2s.h
//
// Copyright (C) 2015 Linaro, Ltd
// Author: Andy Green <andy.green@linaro.org>
//
// Note at least on 6220, S2 == BT, S1 == Digital FM Radio IF
//
pub const HII2S_SW_RST_N: c_int = 0;
pub const HII2S_SW_RST_N__STEREO_UPLINK_WORDLEN_SHIFT: c_int = 28;
pub const HII2S_SW_RST_N__STEREO_UPLINK_WORDLEN_MASK: c_int = 3;
pub const HII2S_SW_RST_N__THIRDMD_UPLINK_WORDLEN_SHIFT: c_int = 26;
pub const HII2S_SW_RST_N__THIRDMD_UPLINK_WORDLEN_MASK: c_int = 3;
pub const HII2S_SW_RST_N__VOICE_UPLINK_WORDLEN_SHIFT: c_int = 24;
pub const HII2S_SW_RST_N__VOICE_UPLINK_WORDLEN_MASK: c_int = 3;
pub const HII2S_SW_RST_N__ST_DL_WORDLEN_SHIFT: c_int = 20;
pub const HII2S_SW_RST_N__ST_DL_WORDLEN_MASK: c_int = 3;
pub const HII2S_SW_RST_N__THIRDMD_DLINK_WORDLEN_SHIFT: c_int = 18;
pub const HII2S_SW_RST_N__THIRDMD_DLINK_WORDLEN_MASK: c_int = 3;
pub const HII2S_SW_RST_N__VOICE_DLINK_WORDLEN_SHIFT: c_int = 16;
pub const HII2S_SW_RST_N__VOICE_DLINK_WORDLEN_MASK: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hi6210_bits {
    HII2S_BITS_16,
    HII2S_BITS_18,
    HII2S_BITS_20,
    HII2S_BITS_24,
}

pub const HII2S_IF_CLK_EN_CFG: c_int = 4;

pub const HII2S_DIG_FILTER_CLK_EN_CFG: c_int = 8;

pub const HII2S_FS_CFG: c_uint = 0xc;
pub const HII2S_FS_CFG__FS_S2_SHIFT: c_int = 28;
pub const HII2S_FS_CFG__FS_S2_MASK: c_int = 7;
pub const HII2S_FS_CFG__FS_S1_SHIFT: c_int = 24;
pub const HII2S_FS_CFG__FS_S1_MASK: c_int = 7;
pub const HII2S_FS_CFG__FS_ADCLR_SHIFT: c_int = 20;
pub const HII2S_FS_CFG__FS_ADCLR_MASK: c_int = 7;
pub const HII2S_FS_CFG__FS_DACLR_SHIFT: c_int = 16;
pub const HII2S_FS_CFG__FS_DACLR_MASK: c_int = 7;
pub const HII2S_FS_CFG__FS_ST_DL_R_SHIFT: c_int = 8;
pub const HII2S_FS_CFG__FS_ST_DL_R_MASK: c_int = 7;
pub const HII2S_FS_CFG__FS_ST_DL_L_SHIFT: c_int = 4;
pub const HII2S_FS_CFG__FS_ST_DL_L_MASK: c_int = 7;
pub const HII2S_FS_CFG__FS_VOICE_DLINK_SHIFT: c_int = 0;
pub const HII2S_FS_CFG__FS_VOICE_DLINK_MASK: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hi6210_i2s_rates {
    HII2S_FS_RATE_8KHZ = 0,
    HII2S_FS_RATE_16KHZ = 1,
    HII2S_FS_RATE_32KHZ = 2,
    HII2S_FS_RATE_48KHZ = 4,
    HII2S_FS_RATE_96KHZ = 5,
    HII2S_FS_RATE_192KHZ = 6,
}

pub const HII2S_I2S_CFG: c_uint = 0x10;

pub const HII2S_I2S_CFG__S2_CODEC_IO_WORDLENGTH_SHIFT: c_int = 24;
pub const HII2S_I2S_CFG__S2_CODEC_IO_WORDLENGTH_MASK: c_int = 3;
pub const HII2S_I2S_CFG__S2_DIRECT_LOOP_SHIFT: c_int = 22;
pub const HII2S_I2S_CFG__S2_DIRECT_LOOP_MASK: c_int = 3;

pub const HII2S_I2S_CFG__S2_FUNC_MODE_SHIFT: c_int = 16;
pub const HII2S_I2S_CFG__S2_FUNC_MODE_MASK: c_int = 7;

pub const HII2S_I2S_CFG__S1_CODEC_IO_WORDLENGTH_SHIFT: c_int = 8;
pub const HII2S_I2S_CFG__S1_CODEC_IO_WORDLENGTH_MASK: c_int = 3;
pub const HII2S_I2S_CFG__S1_DIRECT_LOOP_SHIFT: c_int = 6;
pub const HII2S_I2S_CFG__S1_DIRECT_LOOP_MASK: c_int = 3;

pub const HII2S_I2S_CFG__S1_FUNC_MODE_SHIFT: c_int = 0;
pub const HII2S_I2S_CFG__S1_FUNC_MODE_MASK: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hi6210_i2s_formats {
    HII2S_FORMAT_I2S,
    HII2S_FORMAT_PCM_STD,
    HII2S_FORMAT_PCM_USER,
    HII2S_FORMAT_LEFT_JUST,
    HII2S_FORMAT_RIGHT_JUST,
}

pub const HII2S_DIG_FILTER_MODULE_CFG: c_uint = 0x14;
pub const HII2S_DIG_FILTER_MODULE_CFG__DACR_MIXER_GAIN_SHIFT: c_int = 28;
pub const HII2S_DIG_FILTER_MODULE_CFG__DACR_MIXER_GAIN_MASK: c_int = 3;

pub const HII2S_DIG_FILTER_MODULE_CFG__DACL_MIXER_GAIN_SHIFT: c_int = 20;
pub const HII2S_DIG_FILTER_MODULE_CFG__DACL_MIXER_GAIN_MASK: c_int = 3;

pub const HII2S_DIG_FILTER_MODULE_CFG__LM_CODEC_DAC2ADC_SHIFT: c_int = 4;
pub const HII2S_DIG_FILTER_MODULE_CFG__LM_CODEC_DAC2ADC_MASK: c_int = 7;
pub const HII2S_DIG_FILTER_MODULE_CFG__RM_CODEC_DAC2ADC_SHIFT: c_int = 0;
pub const HII2S_DIG_FILTER_MODULE_CFG__RM_CODEC_DAC2ADC_MASK: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hi6210_gains {
    HII2S_GAIN_100PC,
    HII2S_GAIN_50PC,
    HII2S_GAIN_25PC,
}

pub const HII2S_MUX_TOP_MODULE_CFG: c_uint = 0x18;
pub const HII2S_MUX_TOP_MODULE_CFG__VOICE_DLINK_MIXER_GAIN_SHIFT: c_int = 14;
pub const HII2S_MUX_TOP_MODULE_CFG__VOICE_DLINK_MIXER_GAIN_MASK: c_int = 3;

pub const HII2S_MUX_TOP_MODULE_CFG__S2_OL_MIXER_GAIN_SHIFT: c_int = 10;
pub const HII2S_MUX_TOP_MODULE_CFG__S2_OL_MIXER_GAIN_MASK: c_int = 3;

pub const HII2S_MUX_TOP_MODULE_CFG__S2_OL_SRC_MODE_SHIFT: c_int = 4;
pub const HII2S_MUX_TOP_MODULE_CFG__S2_OL_SRC_MODE_MASK: c_int = 3;

pub const HII2S_MUX_TOP_MODULE_CFG__VOICE_DLINK_SRC_MODE_SHIFT: c_int = 0;
pub const HII2S_MUX_TOP_MODULE_CFG__VOICE_DLINK_SRC_MODE_MASK: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hi6210_s2_src_mode {
    HII2S_S2_SRC_MODE_3,
    HII2S_S2_SRC_MODE_12,
    HII2S_S2_SRC_MODE_6,
    HII2S_S2_SRC_MODE_2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hi6210_voice_dlink_src_mode {
    HII2S_VOICE_DL_SRC_MODE_12 = 1,
    HII2S_VOICE_DL_SRC_MODE_6,
    HII2S_VOICE_DL_SRC_MODE_2,
    HII2S_VOICE_DL_SRC_MODE_3,
}

pub const HII2S_ADC_PGA_CFG: c_uint = 0x1c;
pub const HII2S_S1_INPUT_PGA_CFG: c_uint = 0x20;
pub const HII2S_S2_INPUT_PGA_CFG: c_uint = 0x24;
pub const HII2S_ST_DL_PGA_CFG: c_uint = 0x28;
pub const HII2S_VOICE_SIDETONE_DLINK_PGA_CFG: c_uint = 0x2c;
pub const HII2S_APB_AFIFO_CFG_1: c_uint = 0x30;
pub const HII2S_APB_AFIFO_CFG_2: c_uint = 0x34;
pub const HII2S_ST_DL_FIFO_TH_CFG: c_uint = 0x38;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AEMPTY_SHIFT: c_int = 24;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AEMPTY_MASK: c_uint = 0x1f;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AFULL_SHIFT: c_int = 16;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_R_AFULL_MASK: c_uint = 0x1f;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AEMPTY_SHIFT: c_int = 8;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AEMPTY_MASK: c_uint = 0x1f;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AFULL_SHIFT: c_int = 0;
pub const HII2S_ST_DL_FIFO_TH_CFG__ST_DL_L_AFULL_MASK: c_uint = 0x1f;
pub const HII2S_STEREO_UPLINK_FIFO_TH_CFG: c_uint = 0x3c;
pub const HII2S_VOICE_UPLINK_FIFO_TH_CFG: c_uint = 0x40;
pub const HII2S_CODEC_IRQ_MASK: c_uint = 0x44;
pub const HII2S_CODEC_IRQ: c_uint = 0x48;
pub const HII2S_DACL_AGC_CFG_1: c_uint = 0x4c;
pub const HII2S_DACL_AGC_CFG_2: c_uint = 0x50;
pub const HII2S_DACR_AGC_CFG_1: c_uint = 0x54;
pub const HII2S_DACR_AGC_CFG_2: c_uint = 0x58;
pub const HII2S_DMIC_SIF_CFG: c_uint = 0x5c;
pub const HII2S_MISC_CFG: c_uint = 0x60;

pub const HII2S_S2_SRC_CFG: c_uint = 0x64;
pub const HII2S_MEM_CFG: c_uint = 0x68;
pub const HII2S_THIRDMD_PCM_PGA_CFG: c_uint = 0x6c;
pub const HII2S_THIRD_MODEM_FIFO_TH: c_uint = 0x70;
pub const HII2S_S3_ANTI_FREQ_JITTER_TX_INC_CNT: c_uint = 0x74;
pub const HII2S_S3_ANTI_FREQ_JITTER_TX_DEC_CNT: c_uint = 0x78;
pub const HII2S_S3_ANTI_FREQ_JITTER_RX_INC_CNT: c_uint = 0x7c;
pub const HII2S_S3_ANTI_FREQ_JITTER_RX_DEC_CNT: c_uint = 0x80;
pub const HII2S_ANTI_FREQ_JITTER_EN: c_uint = 0x84;
pub const HII2S_CLK_SEL: c_uint = 0x88;
// 0 = BT owns the i2s

// 0 = internal source, 1 = ext

pub const HII2S_THIRDMD_DLINK_CHANNEL: c_uint = 0xe8;
pub const HII2S_THIRDMD_ULINK_CHANNEL: c_uint = 0xec;
pub const HII2S_VOICE_DLINK_CHANNEL: c_uint = 0xf0;
// shovel data in here for playback
pub const HII2S_ST_DL_CHANNEL: c_uint = 0xf4;
pub const HII2S_STEREO_UPLINK_CHANNEL: c_uint = 0xf8;
pub const HII2S_VOICE_UPLINK_CHANNEL: c_uint = 0xfc;

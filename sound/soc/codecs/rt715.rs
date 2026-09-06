//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt715.h
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


// SPDX-License-Identifier: GPL-2.0
//
// rt715.h -- RT715 ALSA SoC audio driver header
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt715_priv {
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub codec: *mut snd_soc_codec,
    pub slave: *mut sdw_slave,
    pub dbg_nid: c_int,
    pub dbg_vid: c_int,
    pub dbg_payload: c_int,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub kctl_2ch_vol_ori: [c_uint; 2],
    pub kctl_8ch_switch_ori: [c_uint; 8],
    pub kctl_8ch_vol_ori: [c_uint; 8],
}

// NID
pub const RT715_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT715_MIC_ADC: c_uint = 0x07;
pub const RT715_LINE_ADC: c_uint = 0x08;
pub const RT715_MIX_ADC: c_uint = 0x09;
pub const RT715_DMIC1: c_uint = 0x12;
pub const RT715_DMIC2: c_uint = 0x13;
pub const RT715_MIC1: c_uint = 0x18;
pub const RT715_MIC2: c_uint = 0x19;
pub const RT715_LINE1: c_uint = 0x1a;
pub const RT715_LINE2: c_uint = 0x1b;
pub const RT715_DMIC3: c_uint = 0x1d;
pub const RT715_DMIC4: c_uint = 0x29;
pub const RT715_VENDOR_REGISTERS: c_uint = 0x20;
pub const RT715_MUX_IN1: c_uint = 0x22;
pub const RT715_MUX_IN2: c_uint = 0x23;
pub const RT715_MUX_IN3: c_uint = 0x24;
pub const RT715_MUX_IN4: c_uint = 0x25;
pub const RT715_MIX_ADC2: c_uint = 0x27;
pub const RT715_INLINE_CMD: c_uint = 0x55;
// Index (NID:20h)
pub const RT715_VD_CLEAR_CTRL: c_uint = 0x01;
pub const RT715_SDW_INPUT_SEL: c_uint = 0x39;
pub const RT715_EXT_DMIC_CLK_CTRL2: c_uint = 0x54;
// Verb
pub const RT715_VERB_SET_CONNECT_SEL: c_uint = 0x3100;
pub const RT715_VERB_GET_CONNECT_SEL: c_uint = 0xb100;
pub const RT715_VERB_SET_EAPD_BTLENABLE: c_uint = 0x3c00;
pub const RT715_VERB_SET_POWER_STATE: c_uint = 0x3500;
pub const RT715_VERB_SET_CHANNEL_STREAMID: c_uint = 0x3600;
pub const RT715_VERB_SET_PIN_WIDGET_CONTROL: c_uint = 0x3700;
pub const RT715_VERB_SET_CONFIG_DEFAULT1: c_uint = 0x4c00;
pub const RT715_VERB_SET_CONFIG_DEFAULT2: c_uint = 0x4d00;
pub const RT715_VERB_SET_CONFIG_DEFAULT3: c_uint = 0x4e00;
pub const RT715_VERB_SET_CONFIG_DEFAULT4: c_uint = 0x4f00;
pub const RT715_VERB_SET_UNSOLICITED_ENABLE: c_uint = 0x3800;
pub const RT715_SET_AMP_GAIN_MUTE_H: c_uint = 0x7300;
pub const RT715_SET_AMP_GAIN_MUTE_L: c_uint = 0x8380;
pub const RT715_READ_HDA_3: c_uint = 0x2012;
pub const RT715_READ_HDA_2: c_uint = 0x2013;
pub const RT715_READ_HDA_1: c_uint = 0x2014;
pub const RT715_READ_HDA_0: c_uint = 0x2015;
pub const RT715_PRIV_INDEX_W_H: c_uint = 0x7520;
pub const RT715_PRIV_INDEX_W_L: c_uint = 0x85a0;
pub const RT715_PRIV_INDEX_W_H_2: c_uint = 0x7500;
pub const RT715_PRIV_INDEX_W_L_2: c_uint = 0x8580;
pub const RT715_PRIV_DATA_W_H: c_uint = 0x7420;
pub const RT715_PRIV_DATA_W_L: c_uint = 0x84a0;
pub const RT715_PRIV_INDEX_R_H: c_uint = 0x9d20;
pub const RT715_PRIV_INDEX_R_L: c_uint = 0xada0;
pub const RT715_PRIV_DATA_R_H: c_uint = 0x9c20;
pub const RT715_PRIV_DATA_R_L: c_uint = 0xaca0;
pub const RT715_MIC_ADC_FORMAT_H: c_uint = 0x7207;
pub const RT715_MIC_ADC_FORMAT_L: c_uint = 0x8287;
pub const RT715_MIC_LINE_FORMAT_H: c_uint = 0x7208;
pub const RT715_MIC_LINE_FORMAT_L: c_uint = 0x8288;
pub const RT715_MIX_ADC_FORMAT_H: c_uint = 0x7209;
pub const RT715_MIX_ADC_FORMAT_L: c_uint = 0x8289;
pub const RT715_MIX_ADC2_FORMAT_H: c_uint = 0x7227;
pub const RT715_MIX_ADC2_FORMAT_L: c_uint = 0x82a7;
pub const RT715_FUNC_RESET: c_uint = 0xff01;
// Macro flag: #define RT715_SET_AUDIO_POWER_STATE\
// Macro flag: #define RT715_SET_PIN_DMIC1\
// Macro flag: #define RT715_SET_PIN_DMIC2\
// Macro flag: #define RT715_SET_PIN_DMIC3\
// Macro flag: #define RT715_SET_PIN_DMIC4\
// Macro flag: #define RT715_SET_PIN_MIC1\
// Macro flag: #define RT715_SET_PIN_MIC2\
// Macro flag: #define RT715_SET_PIN_LINE1\
// Macro flag: #define RT715_SET_PIN_LINE2\
// Macro flag: #define RT715_SET_MIC1_UNSOLICITED_ENABLE\
// Macro flag: #define RT715_SET_MIC2_UNSOLICITED_ENABLE\
// Macro flag: #define RT715_SET_STREAMID_MIC_ADC\
// Macro flag: #define RT715_SET_STREAMID_LINE_ADC\
// Macro flag: #define RT715_SET_STREAMID_MIX_ADC\
// Macro flag: #define RT715_SET_STREAMID_MIX_ADC2\
// Macro flag: #define RT715_SET_GAIN_MIC_ADC_L\
// Macro flag: #define RT715_SET_GAIN_MIC_ADC_H\
// Macro flag: #define RT715_SET_GAIN_LINE_ADC_L\
// Macro flag: #define RT715_SET_GAIN_LINE_ADC_H\
// Macro flag: #define RT715_SET_GAIN_MIX_ADC_L\
// Macro flag: #define RT715_SET_GAIN_MIX_ADC_H\
// Macro flag: #define RT715_SET_GAIN_MIX_ADC2_L\
// Macro flag: #define RT715_SET_GAIN_MIX_ADC2_H\
// Macro flag: #define RT715_SET_GAIN_DMIC1_L\
// Macro flag: #define RT715_SET_GAIN_DMIC1_H\
// Macro flag: #define RT715_SET_GAIN_DMIC2_L\
// Macro flag: #define RT715_SET_GAIN_DMIC2_H\
// Macro flag: #define RT715_SET_GAIN_DMIC3_L\
// Macro flag: #define RT715_SET_GAIN_DMIC3_H\
// Macro flag: #define RT715_SET_GAIN_DMIC4_L\
// Macro flag: #define RT715_SET_GAIN_DMIC4_H\
// Macro flag: #define RT715_SET_GAIN_MIC1_L\
// Macro flag: #define RT715_SET_GAIN_MIC1_H\
// Macro flag: #define RT715_SET_GAIN_MIC2_L\
// Macro flag: #define RT715_SET_GAIN_MIC2_H\
// Macro flag: #define RT715_SET_GAIN_LINE1_L\
// Macro flag: #define RT715_SET_GAIN_LINE1_H\
// Macro flag: #define RT715_SET_GAIN_LINE2_L\
// Macro flag: #define RT715_SET_GAIN_LINE2_H\
// Macro flag: #define RT715_SET_DMIC1_CONFIG_DEFAULT1\
// Macro flag: #define RT715_SET_DMIC2_CONFIG_DEFAULT1\
// Macro flag: #define RT715_SET_DMIC1_CONFIG_DEFAULT2\
// Macro flag: #define RT715_SET_DMIC2_CONFIG_DEFAULT2\
// Macro flag: #define RT715_SET_DMIC1_CONFIG_DEFAULT3\
// Macro flag: #define RT715_SET_DMIC2_CONFIG_DEFAULT3\
// Macro flag: #define RT715_SET_DMIC1_CONFIG_DEFAULT4\
// Macro flag: #define RT715_SET_DMIC2_CONFIG_DEFAULT4\
// Macro flag: #define RT715_SET_DMIC3_CONFIG_DEFAULT1\
// Macro flag: #define RT715_SET_DMIC4_CONFIG_DEFAULT1\
// Macro flag: #define RT715_SET_DMIC3_CONFIG_DEFAULT2\
// Macro flag: #define RT715_SET_DMIC4_CONFIG_DEFAULT2\
// Macro flag: #define RT715_SET_DMIC3_CONFIG_DEFAULT3\
// Macro flag: #define RT715_SET_DMIC4_CONFIG_DEFAULT3\
// Macro flag: #define RT715_SET_DMIC3_CONFIG_DEFAULT4\
// Macro flag: #define RT715_SET_DMIC4_CONFIG_DEFAULT4\
// vendor register clear ctrl-1    (0x01)(NID:20h)

pub const RT715_MUTE_SFT: c_int = 7;
pub const RT715_DIR_IN_SFT: c_int = 6;
pub const RT715_DIR_OUT_SFT: c_int = 7;
pub const RT715_POWER_UP_DELAY_MS: c_int = 400;
extern "C" {
    pub fn rt715_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn rt715_clock_config(dev: *mut device) -> c_int;
}

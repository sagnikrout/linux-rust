//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt700.h
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
// rt700.h -- RT700 ALSA SoC audio driver header
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt700_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub jack_type: c_int,
    pub /: *mut *mut mutex disable_irq_lock; / imp-def irq lock protection,
    pub disable_irq: bool,
}

// NID
pub const RT700_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT700_DAC_OUT1: c_uint = 0x02;
pub const RT700_DAC_OUT2: c_uint = 0x03;
pub const RT700_ADC_IN1: c_uint = 0x09;
pub const RT700_ADC_IN2: c_uint = 0x08;
pub const RT700_DMIC1: c_uint = 0x12;
pub const RT700_DMIC2: c_uint = 0x13;
pub const RT700_SPK_OUT: c_uint = 0x14;
pub const RT700_MIC2: c_uint = 0x19;
pub const RT700_LINE1: c_uint = 0x1a;
pub const RT700_LINE2: c_uint = 0x1b;
pub const RT700_BEEP: c_uint = 0x1d;
pub const RT700_SPDIF: c_uint = 0x1e;
pub const RT700_VENDOR_REGISTERS: c_uint = 0x20;
pub const RT700_HP_OUT: c_uint = 0x21;
pub const RT700_MIXER_IN1: c_uint = 0x22;
pub const RT700_MIXER_IN2: c_uint = 0x23;
pub const RT700_INLINE_CMD: c_uint = 0x55;
// Index (NID:20h)
pub const RT700_DAC_DC_CALI_CTL1: c_uint = 0x00;
pub const RT700_PARA_VERB_CTL: c_uint = 0x1a;
pub const RT700_COMBO_JACK_AUTO_CTL1: c_uint = 0x45;
pub const RT700_COMBO_JACK_AUTO_CTL2: c_uint = 0x46;
pub const RT700_INLINE_CMD_CTL: c_uint = 0x48;
pub const RT700_DIGITAL_MISC_CTRL4: c_uint = 0x4a;
pub const RT700_VREFOUT_CTL: c_uint = 0x6b;
pub const RT700_FSM_CTL: c_uint = 0x6f;
pub const RT700_IRQ_FLAG_TABLE1: c_uint = 0x80;
pub const RT700_IRQ_FLAG_TABLE2: c_uint = 0x81;
pub const RT700_IRQ_FLAG_TABLE3: c_uint = 0x82;
// Verb
pub const RT700_VERB_SET_CONNECT_SEL: c_uint = 0x3100;
pub const RT700_VERB_SET_EAPD_BTLENABLE: c_uint = 0x3c00;
pub const RT700_VERB_GET_CONNECT_SEL: c_uint = 0xb100;
pub const RT700_VERB_SET_POWER_STATE: c_uint = 0x3500;
pub const RT700_VERB_SET_CHANNEL_STREAMID: c_uint = 0x3600;
pub const RT700_VERB_SET_PIN_WIDGET_CONTROL: c_uint = 0x3700;
pub const RT700_VERB_SET_UNSOLICITED_ENABLE: c_uint = 0x3800;
pub const RT700_SET_AMP_GAIN_MUTE_H: c_uint = 0x7300;
pub const RT700_SET_AMP_GAIN_MUTE_L: c_uint = 0x8380;
pub const RT700_VERB_GET_PIN_SENSE: c_uint = 0xb900;
pub const RT700_READ_HDA_3: c_uint = 0x2012;
pub const RT700_READ_HDA_2: c_uint = 0x2013;
pub const RT700_READ_HDA_1: c_uint = 0x2014;
pub const RT700_READ_HDA_0: c_uint = 0x2015;
pub const RT700_PRIV_INDEX_W_H: c_uint = 0x7520;
pub const RT700_PRIV_INDEX_W_L: c_uint = 0x85a0;
pub const RT700_PRIV_DATA_W_H: c_uint = 0x7420;
pub const RT700_PRIV_DATA_W_L: c_uint = 0x84a0;
pub const RT700_PRIV_INDEX_R_H: c_uint = 0x9d20;
pub const RT700_PRIV_INDEX_R_L: c_uint = 0xada0;
pub const RT700_PRIV_DATA_R_H: c_uint = 0x9c20;
pub const RT700_PRIV_DATA_R_L: c_uint = 0xaca0;
pub const RT700_DAC_FORMAT_H: c_uint = 0x7203;
pub const RT700_DAC_FORMAT_L: c_uint = 0x8283;
pub const RT700_ADC_FORMAT_H: c_uint = 0x7209;
pub const RT700_ADC_FORMAT_L: c_uint = 0x8289;
// Macro flag: #define RT700_SET_AUDIO_POWER_STATE\
// Macro flag: #define RT700_SET_PIN_DMIC1\
// Macro flag: #define RT700_SET_PIN_DMIC2\
// Macro flag: #define RT700_SET_PIN_SPK\
// Macro flag: #define RT700_SET_PIN_HP\
// Macro flag: #define RT700_SET_PIN_MIC2\
// Macro flag: #define RT700_SET_PIN_LINE1\
// Macro flag: #define RT700_SET_PIN_LINE2\
// Macro flag: #define RT700_SET_MIC2_UNSOLICITED_ENABLE\
// Macro flag: #define RT700_SET_HP_UNSOLICITED_ENABLE\
// Macro flag: #define RT700_SET_INLINE_UNSOLICITED_ENABLE\
// Macro flag: #define RT700_SET_STREAMID_DAC1\
// Macro flag: #define RT700_SET_STREAMID_DAC2\
// Macro flag: #define RT700_SET_STREAMID_ADC1\
// Macro flag: #define RT700_SET_STREAMID_ADC2\
// Macro flag: #define RT700_SET_GAIN_DAC1_L\
// Macro flag: #define RT700_SET_GAIN_DAC1_H\
// Macro flag: #define RT700_SET_GAIN_ADC1_L\
// Macro flag: #define RT700_SET_GAIN_ADC1_H\
// Macro flag: #define RT700_SET_GAIN_ADC2_L\
// Macro flag: #define RT700_SET_GAIN_ADC2_H\
// Macro flag: #define RT700_SET_GAIN_AMIC_L\
// Macro flag: #define RT700_SET_GAIN_AMIC_H\
// Macro flag: #define RT700_SET_GAIN_HP_L\
// Macro flag: #define RT700_SET_GAIN_HP_H\
// Macro flag: #define RT700_SET_GAIN_SPK_L\
// Macro flag: #define RT700_SET_GAIN_SPK_H\
// Macro flag: #define RT700_SET_EAPD_SPK\
// combo jack auto switch control 2 (0x46)(NID:20h)

pub const RT700_EAPD_HIGH: c_uint = 0x2;
pub const RT700_EAPD_LOW: c_uint = 0x0;
pub const RT700_MUTE_SFT: c_int = 7;
pub const RT700_DIR_IN_SFT: c_int = 6;
pub const RT700_DIR_OUT_SFT: c_int = 7;
extern "C" {
    pub fn rt700_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn rt700_jack_detect(rt700: *mut rt700_priv, hp: *mut bool, mic: *mut bool) -> c_int;
}
extern "C" {
    pub fn rt700_clock_config(dev: *mut device) -> c_int;
}

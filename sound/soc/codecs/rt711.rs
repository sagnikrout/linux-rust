//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt711.h
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
// rt711.h -- RT711 ALSA SoC audio driver header
//
// Copyright(c) 2019 Realtek Semiconductor Corp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt711_priv {
    pub regmap: *mut regmap,
    pub sdw_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub jack_btn_check_work: delayed_work,
    pub calibration_work: work_struct,
    pub /: *mut *mut mutex calibrate_mutex; / for headset calibration,
    pub jd_src: int jack_type,,
    pub /: *mut *mut mutex disable_irq_lock; / imp-def irq lock protection,
    pub disable_irq: bool,
}

// NID
pub const RT711_AUDIO_FUNCTION_GROUP: c_uint = 0x01;
pub const RT711_DAC_OUT2: c_uint = 0x03;
pub const RT711_ADC_IN1: c_uint = 0x09;
pub const RT711_ADC_IN2: c_uint = 0x08;
pub const RT711_DMIC1: c_uint = 0x12;
pub const RT711_DMIC2: c_uint = 0x13;
pub const RT711_MIC2: c_uint = 0x19;
pub const RT711_LINE1: c_uint = 0x1a;
pub const RT711_LINE2: c_uint = 0x1b;
pub const RT711_BEEP: c_uint = 0x1d;
pub const RT711_VENDOR_REG: c_uint = 0x20;
pub const RT711_HP_OUT: c_uint = 0x21;
pub const RT711_MIXER_IN1: c_uint = 0x22;
pub const RT711_MIXER_IN2: c_uint = 0x23;
pub const RT711_INLINE_CMD: c_uint = 0x55;
pub const RT711_VENDOR_CALI: c_uint = 0x58;
pub const RT711_VENDOR_IMS_DRE: c_uint = 0x5b;
// Index (NID:20h)
pub const RT711_DAC_DC_CALI_CTL1: c_uint = 0x00;
pub const RT711_JD_CTL1: c_uint = 0x08;
pub const RT711_JD_CTL2: c_uint = 0x09;
pub const RT711_JD_CTL4: c_uint = 0x0b;
pub const RT711_CC_DET1: c_uint = 0x11;
pub const RT711_PARA_VERB_CTL: c_uint = 0x1a;
pub const RT711_COMBO_JACK_AUTO_CTL1: c_uint = 0x45;
pub const RT711_COMBO_JACK_AUTO_CTL2: c_uint = 0x46;
pub const RT711_INLINE_CMD_CTL: c_uint = 0x48;
pub const RT711_DIGITAL_MISC_CTRL4: c_uint = 0x4a;
pub const RT711_VREFOUT_CTL: c_uint = 0x6b;
pub const RT711_FSM_CTL: c_uint = 0x6f;
pub const RT711_IRQ_FLAG_TABLE1: c_uint = 0x80;
pub const RT711_IRQ_FLAG_TABLE2: c_uint = 0x81;
pub const RT711_IRQ_FLAG_TABLE3: c_uint = 0x82;
pub const RT711_TX_RX_MUX_CTL: c_uint = 0x91;
// Index (NID:5bh)
pub const RT711_IMS_DIGITAL_CTL1: c_uint = 0x00;
pub const RT711_HP_IMS_RESULT_L: c_uint = 0x20;
pub const RT711_HP_IMS_RESULT_R: c_uint = 0x21;
// Verb
pub const RT711_VERB_SET_CONNECT_SEL: c_uint = 0x3100;
pub const RT711_VERB_SET_EAPD_BTLENABLE: c_uint = 0x3c00;
pub const RT711_VERB_GET_CONNECT_SEL: c_uint = 0xb100;
pub const RT711_VERB_SET_POWER_STATE: c_uint = 0x3500;
pub const RT711_VERB_SET_CHANNEL_STREAMID: c_uint = 0x3600;
pub const RT711_VERB_SET_PIN_WIDGET_CONTROL: c_uint = 0x3700;
pub const RT711_VERB_SET_UNSOLICITED_ENABLE: c_uint = 0x3800;
pub const RT711_SET_AMP_GAIN_MUTE_H: c_uint = 0x7300;
pub const RT711_SET_AMP_GAIN_MUTE_L: c_uint = 0x8380;
pub const RT711_VERB_GET_POWER_STATE: c_uint = 0xb500;
pub const RT711_VERB_GET_CHANNEL_STREAMID: c_uint = 0xb600;
pub const RT711_VERB_GET_PIN_SENSE: c_uint = 0xb900;
pub const RT711_FUNC_RESET: c_uint = 0xff01;
pub const RT711_READ_HDA_3: c_uint = 0x2012;
pub const RT711_READ_HDA_2: c_uint = 0x2013;
pub const RT711_READ_HDA_1: c_uint = 0x2014;
pub const RT711_READ_HDA_0: c_uint = 0x2015;
pub const RT711_PRIV_INDEX_W_H: c_uint = 0x7500;
pub const RT711_PRIV_INDEX_W_L: c_uint = 0x8580;
pub const RT711_PRIV_DATA_W_H: c_uint = 0x7400;
pub const RT711_PRIV_DATA_W_L: c_uint = 0x8480;
pub const RT711_PRIV_INDEX_R_H: c_uint = 0x9d00;
pub const RT711_PRIV_INDEX_R_L: c_uint = 0xad80;
pub const RT711_PRIV_DATA_R_H: c_uint = 0x9c00;
pub const RT711_PRIV_DATA_R_L: c_uint = 0xac80;
pub const RT711_DAC_FORMAT_H: c_uint = 0x7203;
pub const RT711_DAC_FORMAT_L: c_uint = 0x8283;
pub const RT711_ADC1_FORMAT_H: c_uint = 0x7209;
pub const RT711_ADC1_FORMAT_L: c_uint = 0x8289;
pub const RT711_ADC2_FORMAT_H: c_uint = 0x7208;
pub const RT711_ADC2_FORMAT_L: c_uint = 0x8288;
// Macro flag: #define RT711_SET_AUDIO_POWER_STATE\
// Macro flag: #define RT711_GET_AUDIO_POWER_STATE\
// Macro flag: #define RT711_SET_PIN_DMIC1\
// Macro flag: #define RT711_SET_PIN_DMIC2\
// Macro flag: #define RT711_SET_PIN_HP\
// Macro flag: #define RT711_SET_PIN_MIC2\
// Macro flag: #define RT711_SET_PIN_LINE1\
// Macro flag: #define RT711_SET_PIN_LINE2\
// Macro flag: #define RT711_SET_MIC2_UNSOLICITED_ENABLE\
// Macro flag: #define RT711_SET_HP_UNSOLICITED_ENABLE\
// Macro flag: #define RT711_SET_INLINE_UNSOLICITED_ENABLE\
// Macro flag: #define RT711_SET_STREAMID_DAC2\
// Macro flag: #define RT711_SET_STREAMID_ADC1\
// Macro flag: #define RT711_SET_STREAMID_ADC2\
// Macro flag: #define RT711_GET_STREAMID_DAC2\
// Macro flag: #define RT711_GET_STREAMID_ADC1\
// Macro flag: #define RT711_GET_STREAMID_ADC2\
// Macro flag: #define RT711_SET_GAIN_DAC2_L\
// Macro flag: #define RT711_SET_GAIN_DAC2_H\
// Macro flag: #define RT711_SET_GAIN_ADC1_L\
// Macro flag: #define RT711_SET_GAIN_ADC1_H\
// Macro flag: #define RT711_SET_GAIN_ADC2_L\
// Macro flag: #define RT711_SET_GAIN_ADC2_H\
// Macro flag: #define RT711_SET_GAIN_AMIC_L\
// Macro flag: #define RT711_SET_GAIN_AMIC_H\
// Macro flag: #define RT711_SET_GAIN_DMIC1_L\
// Macro flag: #define RT711_SET_GAIN_DMIC1_H\
// Macro flag: #define RT711_SET_GAIN_DMIC2_L\
// Macro flag: #define RT711_SET_GAIN_DMIC2_H\
// Macro flag: #define RT711_SET_GAIN_HP_L\
// Macro flag: #define RT711_SET_GAIN_HP_H\
// DAC DC offset calibration control-1 (0x00)(NID:20h)

// jack detect control 1 (0x08)(NID:20h)

// jack detect control 2 (0x09)(NID:20h)

// jack detect control 4 (0x0b)(NID:20h)

// CC DET1 (0x11)(NID:20h)

// Parameter & Verb control (0x1a)(NID:20h)

// combo jack auto switch control 2 (0x46)(NID:20h)

// FSM control (0x6f)(NID:20h)

// Impedance Sense Digital Control 1 (0x00)(NID:5bh)

pub const RT711_EAPD_HIGH: c_uint = 0x2;
pub const RT711_EAPD_LOW: c_uint = 0x0;
pub const RT711_MUTE_SFT: c_int = 7;
// set input/output mapping to payload[14][15] separately
pub const RT711_DIR_IN_SFT: c_int = 6;
pub const RT711_DIR_OUT_SFT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt711_jd_src {
    RT711_JD_NULL,
    RT711_JD1,
    RT711_JD2,
    RT711_JD2_100K,
    RT711_JD2_1P8V_1PORT
}

extern "C" {
    pub fn rt711_io_init(dev: *mut device, slave: *mut sdw_slave) -> c_int;
}
extern "C" {
    pub fn rt711_jack_detect(rt711: *mut rt711_priv, hp: *mut bool, mic: *mut bool) -> c_int;
}
extern "C" {
    pub fn rt711_clock_config(dev: *mut device) -> c_int;
}

//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/alc5632.h
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
// alc5632.h  --  ALC5632 ALSA SoC Audio Codec
//
// Copyright (C) 2011 The AC100 Kernel Team <ac100@lists.lauchpad.net>
//
// Authors:  Leon Romanovsky <leon@leon.nu>
// Andrey Danin <danindrey@mail.ru>
// Ilya Petrov <ilya.muromec@gmail.com>
// Marc Dietrich <marvin24@gmx.de>
//
// Based on alc5623.h by Arnaud Patard
//
pub const ALC5632_RESET: c_uint = 0x00;
// speaker output vol		   2    2
// line output vol                      4    2
// HP output vol		   4    0    4
pub const ALC5632_SPK_OUT_VOL: c_uint = 0x02 /* spe out vol */;

pub const ALC5632_HP_OUT_VOL: c_uint = 0x04 /* hp out vol */;
pub const ALC5632_AUX_OUT_VOL: c_uint = 0x06 /* aux out vol */;
pub const ALC5632_PHONE_IN_VOL: c_uint = 0x08 /* phone in vol */;
pub const ALC5632_LINE_IN_VOL: c_uint = 0x0A /* line in vol */;
pub const ALC5632_STEREO_DAC_IN_VOL: c_uint = 0x0C /* stereo dac in vol */;
pub const ALC5632_MIC_VOL: c_uint = 0x0E /* mic in vol */;
// stero dac/mic routing
pub const ALC5632_MIC_ROUTING_CTRL: c_uint = 0x10;

pub const ALC5632_ADC_REC_GAIN: c_uint = 0x12 /* rec gain */;
pub const ALC5632_ADC_REC_GAIN_RANGE: c_uint = 0x1F1F;

pub const ALC5632_ADC_REC_MIXER: c_uint = 0x14 /* mixer control */;

pub const ALC5632_VOICE_DAC_VOL: c_uint = 0x18 /* voice dac vol */;
pub const ALC5632_I2S_OUT_CTL: c_uint = 0x1A /* undocumented reg. found in path scheme */;
// ALC5632_OUTPUT_MIXER_CTRL :
// same remark as for reg 2 line vs speaker
pub const ALC5632_OUTPUT_MIXER_CTRL: c_uint = 0x1C /* out mix ctrl */;

pub const ALC5632_MIC_CTRL: c_uint = 0x22 /* mic phone ctrl */;
pub const ALC5632_MIC_BOOST_BYPASS: c_int = 0;
pub const ALC5632_MIC_BOOST_20DB: c_int = 1;
pub const ALC5632_MIC_BOOST_30DB: c_int = 2;
pub const ALC5632_MIC_BOOST_40DB: c_int = 3;
pub const ALC5632_DIGI_BOOST_CTRL: c_uint = 0x24 /* digi mic / bost ctl */;
pub const ALC5632_MIC_BOOST_RANGE: c_int = 7;
pub const ALC5632_MIC_BOOST_STEP: c_int = 6;
pub const ALC5632_PWR_DOWN_CTRL_STATUS: c_uint = 0x26;
pub const ALC5632_PWR_DOWN_CTRL_STATUS_MASK: c_uint = 0xEF00;

// stereo/voice DAC / stereo adc func ctrl
pub const ALC5632_DAC_FUNC_SELECT: c_uint = 0x2E;
// Main serial data port ctrl (i2s)
pub const ALC5632_DAI_CONTROL: c_uint = 0x34;

// 0:voice, 1:main

// 0:normal, 1:invert

// extend serial data port control (VoDAC_i2c/pcm)
pub const ALC5632_DAI_CONTROL2: c_uint = 0x36;
// 0:gpio func, 1:voice pcm

// 0:master, 1:slave

// 0:disable, 1:enable

// 0:main, 1:voice

// 0:normal, 1:invert

// 0:normal, 1:invert

pub const ALC5632_PWR_MANAG_ADD1: c_uint = 0x3A;
pub const ALC5632_PWR_MANAG_ADD1_MASK: c_uint = 0xEFFF;

pub const ALC5632_PWR_MANAG_ADD2: c_uint = 0x3C;
pub const ALC5632_PWR_MANAG_ADD2_MASK: c_uint = 0x7FFF;

pub const ALC5632_PWR_MANAG_ADD3: c_uint = 0x3E;
pub const ALC5632_PWR_MANAG_ADD3_MASK: c_uint = 0x7CFF;

pub const ALC5632_GPCR1: c_uint = 0x40;

pub const ALC5632_GPCR2: c_uint = 0x42;

pub const ALC5632_PLL1_CTRL: c_uint = 0x44;

pub const ALC5632_PLL2_CTRL: c_uint = 0x46;

pub const ALC5632_GPIO_PIN_CONFIG: c_uint = 0x4C;
pub const ALC5632_GPIO_PIN_POLARITY: c_uint = 0x4E;
pub const ALC5632_GPIO_PIN_STICKY: c_uint = 0x50;
pub const ALC5632_GPIO_PIN_WAKEUP: c_uint = 0x52;
pub const ALC5632_GPIO_PIN_STATUS: c_uint = 0x54;
pub const ALC5632_GPIO_PIN_SHARING: c_uint = 0x56;
pub const ALC5632_OVER_CURR_STATUS: c_uint = 0x58;
pub const ALC5632_SOFTVOL_CTRL: c_uint = 0x5A;
pub const ALC5632_GPIO_OUPUT_PIN_CTRL: c_uint = 0x5C;
pub const ALC5632_MISC_CTRL: c_uint = 0x5E;

pub const ALC5632_DAC_CLK_CTRL1: c_uint = 0x60;
pub const ALC5632_DAC_CLK_CTRL2: c_uint = 0x62;

pub const ALC5632_VOICE_DAC_PCM_CLK_CTRL1: c_uint = 0x64;
pub const ALC5632_PSEUDO_SPATIAL_CTRL: c_uint = 0x68;
pub const ALC5632_HID_CTRL_INDEX: c_uint = 0x6A;
pub const ALC5632_HID_CTRL_DATA: c_uint = 0x6C;
pub const ALC5632_EQ_CTRL: c_uint = 0x6E;
// undocumented
pub const ALC5632_VENDOR_ID1: c_uint = 0x7C;
pub const ALC5632_VENDOR_ID2: c_uint = 0x7E;
pub const ALC5632_MAX_REGISTER: c_uint = 0x7E;

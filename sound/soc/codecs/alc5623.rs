//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/alc5623.h
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
// alc5623.h  --  alc562[123] ALSA Soc Audio driver
//
// Copyright 2008 Realtek Microelectronics
// Copyright 2010 Arnaud Patard <arnaud.patard@rtp-net.org>
//
// Author: flove <flove@realtek.com>
// Arnaud Patard <arnaud.patard@rtp-net.org>
//
pub const ALC5623_RESET: c_uint = 0x00;
// 5621 5622 5623
// speaker output vol		   2    2
// line output vol                      4    2
// HP output vol		   4    0    4
pub const ALC5623_SPK_OUT_VOL: c_uint = 0x02;
pub const ALC5623_HP_OUT_VOL: c_uint = 0x04;
pub const ALC5623_MONO_AUX_OUT_VOL: c_uint = 0x06;
pub const ALC5623_AUXIN_VOL: c_uint = 0x08;
pub const ALC5623_LINE_IN_VOL: c_uint = 0x0A;
pub const ALC5623_STEREO_DAC_VOL: c_uint = 0x0C;
pub const ALC5623_MIC_VOL: c_uint = 0x0E;
pub const ALC5623_MIC_ROUTING_CTRL: c_uint = 0x10;
pub const ALC5623_ADC_REC_GAIN: c_uint = 0x12;
pub const ALC5623_ADC_REC_MIXER: c_uint = 0x14;
pub const ALC5623_SOFT_VOL_CTRL_TIME: c_uint = 0x16;
// ALC5623_OUTPUT_MIXER_CTRL :
// same remark as for reg 2 line vs speaker
pub const ALC5623_OUTPUT_MIXER_CTRL: c_uint = 0x1C;
pub const ALC5623_MIC_CTRL: c_uint = 0x22;
pub const ALC5623_DAI_CONTROL: c_uint = 0x34;

pub const ALC5623_STEREO_AD_DA_CLK_CTRL: c_uint = 0x36;
pub const ALC5623_COMPANDING_CTRL: c_uint = 0x38;
pub const ALC5623_PWR_MANAG_ADD1: c_uint = 0x3A;

pub const ALC5623_PWR_MANAG_ADD2: c_uint = 0x3C;

pub const ALC5623_PWR_MANAG_ADD3: c_uint = 0x3E;

pub const ALC5623_ADD_CTRL_REG: c_uint = 0x40;
pub const ALC5623_GLOBAL_CLK_CTRL_REG: c_uint = 0x42;

pub const ALC5623_PLL_CTRL: c_uint = 0x44;

pub const ALC5623_GPIO_OUTPUT_PIN_CTRL: c_uint = 0x4A;
pub const ALC5623_GPIO_PIN_CONFIG: c_uint = 0x4C;
pub const ALC5623_GPIO_PIN_POLARITY: c_uint = 0x4E;
pub const ALC5623_GPIO_PIN_STICKY: c_uint = 0x50;
pub const ALC5623_GPIO_PIN_WAKEUP: c_uint = 0x52;
pub const ALC5623_GPIO_PIN_STATUS: c_uint = 0x54;
pub const ALC5623_GPIO_PIN_SHARING: c_uint = 0x56;
pub const ALC5623_OVER_CURR_STATUS: c_uint = 0x58;
pub const ALC5623_JACK_DET_CTRL: c_uint = 0x5A;
pub const ALC5623_MISC_CTRL: c_uint = 0x5E;

pub const ALC5623_PSEDUEO_SPATIAL_CTRL: c_uint = 0x60;
pub const ALC5623_EQ_CTRL: c_uint = 0x62;
pub const ALC5623_EQ_MODE_ENABLE: c_uint = 0x66;
pub const ALC5623_AVC_CTRL: c_uint = 0x68;
pub const ALC5623_HID_CTRL_INDEX: c_uint = 0x6A;
pub const ALC5623_HID_CTRL_DATA: c_uint = 0x6C;
pub const ALC5623_VENDOR_ID1: c_uint = 0x7C;
pub const ALC5623_VENDOR_ID2: c_uint = 0x7E;
pub const ALC5623_PLL_FR_MCLK: c_int = 0;
pub const ALC5623_PLL_FR_BCK: c_int = 1;

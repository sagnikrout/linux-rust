//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max98504.h
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
// MAX98504 ALSA SoC Audio driver
//
// Copyright 2011 - 2012 Maxim Integrated Products
// Copyright 2016 Samsung Electronics Co., Ltd.
//
// MAX98504 Register Definitions
//
pub const MAX98504_INTERRUPT_STATUS: c_uint = 0x01;
pub const MAX98504_INTERRUPT_FLAGS: c_uint = 0x02;
pub const MAX98504_INTERRUPT_ENABLE: c_uint = 0x03;
pub const MAX98504_INTERRUPT_FLAG_CLEARS: c_uint = 0x04;
pub const MAX98504_GPIO_ENABLE: c_uint = 0x10;
pub const MAX98504_GPIO_CONFIG: c_uint = 0x11;
pub const MAX98504_WATCHDOG_ENABLE: c_uint = 0x12;
pub const MAX98504_WATCHDOG_CONFIG: c_uint = 0x13;
pub const MAX98504_WATCHDOG_CLEAR: c_uint = 0x14;
pub const MAX98504_CLOCK_MONITOR_ENABLE: c_uint = 0x15;
pub const MAX98504_PVDD_BROWNOUT_ENABLE: c_uint = 0x16;
pub const MAX98504_PVDD_BROWNOUT_CONFIG_1: c_uint = 0x17;
pub const MAX98504_PVDD_BROWNOUT_CONFIG_2: c_uint = 0x18;
pub const MAX98504_PVDD_BROWNOUT_CONFIG_3: c_uint = 0x19;
pub const MAX98504_PVDD_BROWNOUT_CONFIG_4: c_uint = 0x1a;
pub const MAX98504_PCM_RX_ENABLE: c_uint = 0x20;
pub const MAX98504_PCM_TX_ENABLE: c_uint = 0x21;
pub const MAX98504_PCM_TX_HIZ_CONTROL: c_uint = 0x22;
pub const MAX98504_PCM_TX_CHANNEL_SOURCES: c_uint = 0x23;
pub const MAX98504_PCM_MODE_CONFIG: c_uint = 0x24;
pub const MAX98504_PCM_DSP_CONFIG: c_uint = 0x25;
pub const MAX98504_PCM_CLOCK_SETUP: c_uint = 0x26;
pub const MAX98504_PCM_SAMPLE_RATE_SETUP: c_uint = 0x27;
pub const MAX98504_PCM_TO_SPEAKER_MONOMIX: c_uint = 0x28;
pub const MAX98504_PDM_TX_ENABLE: c_uint = 0x30;
pub const MAX98504_PDM_TX_HIZ_CONTROL: c_uint = 0x31;
pub const MAX98504_PDM_TX_CONTROL: c_uint = 0x32;
pub const MAX98504_PDM_RX_ENABLE: c_uint = 0x33;
pub const MAX98504_SPEAKER_ENABLE: c_uint = 0x34;
pub const MAX98504_SPEAKER_SOURCE_SELECT: c_uint = 0x35;
pub const MAX98504_MEASUREMENT_ENABLE: c_uint = 0x36;
pub const MAX98504_ANALOGUE_INPUT_GAIN: c_uint = 0x37;
pub const MAX98504_TEMPERATURE_LIMIT_CONFIG: c_uint = 0x38;
pub const MAX98504_GLOBAL_ENABLE: c_uint = 0x40;
pub const MAX98504_SOFTWARE_RESET: c_uint = 0x41;
pub const MAX98504_REV_ID: c_uint = 0x7fff;
pub const MAX98504_MAX_REGISTER: c_uint = 0x7fff;
pub const MAX98504_DAI_ID_PCM: c_int = 1;
pub const MAX98504_DAI_ID_PDM: c_int = 2;

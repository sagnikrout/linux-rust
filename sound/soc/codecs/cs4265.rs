//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs4265.h
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
// cs4265.h -- CS4265 ALSA SoC audio driver
//
// Copyright 2014 Cirrus Logic, Inc.
//
// Author: Paul Handrigan <paul.handrigan@cirrus.com>
//
pub const CS4265_CHIP_ID: c_uint = 0x1;
pub const CS4265_CHIP_ID_VAL: c_uint = 0xD0;
pub const CS4265_CHIP_ID_MASK: c_uint = 0xF0;
pub const CS4265_REV_ID_MASK: c_uint = 0x0F;
pub const CS4265_PWRCTL: c_uint = 0x02;
pub const CS4265_PWRCTL_PDN: c_int = 1;
pub const CS4265_DAC_CTL: c_uint = 0x3;

pub const CS4265_ADC_CTL: c_uint = 0x4;
pub const CS4265_ADC_MASTER: c_int = 1;

pub const CS4265_MCLK_FREQ: c_uint = 0x5;

pub const CS4265_SIG_SEL: c_uint = 0x6;

pub const CS4265_CHB_PGA_CTL: c_uint = 0x7;
pub const CS4265_CHA_PGA_CTL: c_uint = 0x8;
pub const CS4265_ADC_CTL2: c_uint = 0x9;
pub const CS4265_DAC_CHA_VOL: c_uint = 0xA;
pub const CS4265_DAC_CHB_VOL: c_uint = 0xB;
pub const CS4265_DAC_CTL2: c_uint = 0xC;
pub const CS4265_INT_STATUS: c_uint = 0xD;
pub const CS4265_INT_MASK: c_uint = 0xE;
pub const CS4265_STATUS_MODE_MSB: c_uint = 0xF;
pub const CS4265_STATUS_MODE_LSB: c_uint = 0x10;
pub const CS4265_SPDIF_CTL1: c_uint = 0x11;
pub const CS4265_SPDIF_CTL2: c_uint = 0x12;

pub const CS4265_C_DATA_BUFF: c_uint = 0x13;
pub const CS4265_MAX_REGISTER: c_uint = 0x2A;

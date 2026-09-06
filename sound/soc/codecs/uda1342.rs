//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/uda1342.h
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
// Audio support for NXP UDA1342
//
// Copyright (c) 2005 Giorgio Padrin <giorgio@mandarinlogiq.org>
// Copyright (c) 2024 Binbin Zhou <zhoubinbin@loongson.cn>
//
pub const UDA1342_CLK: c_uint = 0x00;
pub const UDA1342_IFACE: c_uint = 0x01;
pub const UDA1342_PM: c_uint = 0x02;
pub const UDA1342_AMIX: c_uint = 0x03;
pub const UDA1342_HP: c_uint = 0x04;
pub const UDA1342_MVOL: c_uint = 0x11;
pub const UDA1342_MIXVOL: c_uint = 0x12;
pub const UDA1342_MODE: c_uint = 0x12;
pub const UDA1342_DEEMP: c_uint = 0x13;
pub const UDA1342_MIXER: c_uint = 0x14;
pub const UDA1342_INTSTAT: c_uint = 0x18;
pub const UDA1342_DEC: c_uint = 0x20;
pub const UDA1342_PGA: c_uint = 0x21;
pub const UDA1342_ADC: c_uint = 0x22;
pub const UDA1342_AGC: c_uint = 0x23;
pub const UDA1342_DECSTAT: c_uint = 0x28;
pub const UDA1342_RESET: c_uint = 0x7f;
// Register flags
pub const R00_EN_ADC: c_uint = 0x0800;
pub const R00_EN_DEC: c_uint = 0x0400;
pub const R00_EN_DAC: c_uint = 0x0200;
pub const R00_EN_INT: c_uint = 0x0100;
pub const R00_DAC_CLK: c_uint = 0x0010;
pub const R01_SFORI_I2S: c_uint = 0x0000;
pub const R01_SFORI_LSB16: c_uint = 0x0100;
pub const R01_SFORI_LSB18: c_uint = 0x0200;
pub const R01_SFORI_LSB20: c_uint = 0x0300;
pub const R01_SFORI_MSB: c_uint = 0x0500;
pub const R01_SFORI_MASK: c_uint = 0x0700;
pub const R01_SFORO_I2S: c_uint = 0x0000;
pub const R01_SFORO_LSB16: c_uint = 0x0001;
pub const R01_SFORO_LSB18: c_uint = 0x0002;
pub const R01_SFORO_LSB20: c_uint = 0x0003;
pub const R01_SFORO_LSB24: c_uint = 0x0004;
pub const R01_SFORO_MSB: c_uint = 0x0005;
pub const R01_SFORO_MASK: c_uint = 0x0007;
pub const R01_SEL_SOURCE: c_uint = 0x0040;
pub const R01_SIM: c_uint = 0x0010;
pub const R02_PON_PLL: c_uint = 0x8000;
pub const R02_PON_HP: c_uint = 0x2000;
pub const R02_PON_DAC: c_uint = 0x0400;
pub const R02_PON_BIAS: c_uint = 0x0100;
pub const R02_EN_AVC: c_uint = 0x0080;
pub const R02_PON_AVC: c_uint = 0x0040;
pub const R02_PON_LNA: c_uint = 0x0010;
pub const R02_PON_PGAL: c_uint = 0x0008;
pub const R02_PON_ADCL: c_uint = 0x0004;
pub const R02_PON_PGAR: c_uint = 0x0002;
pub const R02_PON_ADCR: c_uint = 0x0001;
pub const R13_MTM: c_uint = 0x4000;
pub const R14_SILENCE: c_uint = 0x0080;
pub const R14_SDET_ON: c_uint = 0x0040;
pub const R21_MT_ADC: c_uint = 0x8000;
pub const R22_SEL_LNA: c_uint = 0x0008;
pub const R22_SEL_MIC: c_uint = 0x0004;
pub const R22_SKIP_DCFIL: c_uint = 0x0002;
pub const R23_AGC_EN: c_uint = 0x0001;


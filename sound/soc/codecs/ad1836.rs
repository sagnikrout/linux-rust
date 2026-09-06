//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ad1836.h
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
// Audio Codec driver supporting:
// AD1835A, AD1836, AD1837A, AD1838A, AD1839A
//
// Copyright 2009-2011 Analog Devices Inc.
//
pub const AD1836_DAC_CTRL1: c_int = 0;
pub const AD1836_DAC_POWERDOWN: c_int = 2;
pub const AD1836_DAC_SERFMT_MASK: c_uint = 0xE0;

pub const AD1836_DAC_WORD_LEN_MASK: c_uint = 0x18;
pub const AD1836_DAC_WORD_LEN_OFFSET: c_int = 3;
pub const AD1836_DAC_CTRL2: c_int = 1;
// These macros are one-based. So AD183X_MUTE_LEFT(1) will return the mute bit
// for the first ADC/DAC

pub const AD1836_ADC_CTRL1: c_int = 12;
pub const AD1836_ADC_POWERDOWN: c_int = 7;
pub const AD1836_ADC_HIGHPASS_FILTER: c_int = 8;
pub const AD1836_ADC_CTRL2: c_int = 13;
pub const AD1836_ADC_WORD_LEN_MASK: c_uint = 0x30;
pub const AD1836_ADC_WORD_OFFSET: c_int = 4;

pub const AD1836_ADC_CTRL3: c_int = 14;
pub const AD1836_NUM_REGS: c_int = 16;
pub const AD1836_WORD_LEN_24: c_uint = 0x0;
pub const AD1836_WORD_LEN_20: c_uint = 0x1;
pub const AD1836_WORD_LEN_16: c_uint = 0x2;

//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/ad193x.h
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
// AD193X Audio Codec driver
//
// Copyright 2010 Analog Devices Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad193x_type {
    AD193X,
    AD1933,
    AD1934,
}

pub const AD193X_PLL_CLK_CTRL0: c_uint = 0x00;
pub const AD193X_PLL_POWERDOWN: c_uint = 0x01;
pub const AD193X_PLL_INPUT_MASK: c_uint = 0x6;

pub const AD193X_PLL_CLK_CTRL1: c_uint = 0x01;
pub const AD193X_PLL_SRC_MASK: c_uint = 0x03;
pub const AD193X_PLL_DAC_SRC_PLL: c_int = 0;
pub const AD193X_PLL_DAC_SRC_MCLK: c_int = 1;

pub const AD193X_DAC_CTRL0: c_uint = 0x02;
pub const AD193X_DAC_POWERDOWN: c_uint = 0x01;
pub const AD193X_DAC_SR_MASK: c_uint = 0x06;

pub const AD193X_DAC_SERFMT_MASK: c_uint = 0xC0;

pub const AD193X_DAC_CTRL1: c_uint = 0x03;
pub const AD193X_DAC_CHAN_SHFT: c_int = 1;

pub const AD193X_DAC_CTRL2: c_uint = 0x04;
pub const AD193X_DAC_WORD_LEN_SHFT: c_int = 3;
pub const AD193X_DAC_WORD_LEN_MASK: c_uint = 0x18;
pub const AD193X_DAC_MASTER_MUTE: c_int = 1;
pub const AD193X_DAC_CHNL_MUTE: c_uint = 0x05;
pub const AD193X_DACL1_MUTE: c_int = 0;
pub const AD193X_DACR1_MUTE: c_int = 1;
pub const AD193X_DACL2_MUTE: c_int = 2;
pub const AD193X_DACR2_MUTE: c_int = 3;
pub const AD193X_DACL3_MUTE: c_int = 4;
pub const AD193X_DACR3_MUTE: c_int = 5;
pub const AD193X_DACL4_MUTE: c_int = 6;
pub const AD193X_DACR4_MUTE: c_int = 7;
pub const AD193X_DAC_L1_VOL: c_uint = 0x06;
pub const AD193X_DAC_R1_VOL: c_uint = 0x07;
pub const AD193X_DAC_L2_VOL: c_uint = 0x08;
pub const AD193X_DAC_R2_VOL: c_uint = 0x09;
pub const AD193X_DAC_L3_VOL: c_uint = 0x0a;
pub const AD193X_DAC_R3_VOL: c_uint = 0x0b;
pub const AD193X_DAC_L4_VOL: c_uint = 0x0c;
pub const AD193X_DAC_R4_VOL: c_uint = 0x0d;
pub const AD193X_ADC_CTRL0: c_uint = 0x0e;
pub const AD193X_ADC_POWERDOWN: c_uint = 0x01;
pub const AD193X_ADC_HIGHPASS_FILTER: c_int = 1;
pub const AD193X_ADCL1_MUTE: c_int = 2;
pub const AD193X_ADCR1_MUTE: c_int = 3;
pub const AD193X_ADCL2_MUTE: c_int = 4;
pub const AD193X_ADCR2_MUTE: c_int = 5;
pub const AD193X_ADC_CTRL1: c_uint = 0x0f;
pub const AD193X_ADC_SERFMT_MASK: c_uint = 0x60;

pub const AD193X_ADC_WORD_LEN_MASK: c_uint = 0x3;
pub const AD193X_ADC_CTRL2: c_uint = 0x10;
pub const AD193X_ADC_CHAN_SHFT: c_int = 4;

pub const AD193X_2_CHANNELS: c_int = 0;
pub const AD193X_4_CHANNELS: c_int = 1;
pub const AD193X_8_CHANNELS: c_int = 2;
pub const AD193X_16_CHANNELS: c_int = 3;
pub const AD193X_NUM_REGS: c_int = 17;
pub const AD193X_SYSCLK_PLL: c_int = 0;
pub const AD193X_SYSCLK_MCLK: c_int = 1;

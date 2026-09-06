//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8978.h
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
// wm8978.h		--  codec driver for WM8978
//
// Copyright 2009 Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//
// Register values.
//
pub const WM8978_RESET: c_uint = 0x00;
pub const WM8978_POWER_MANAGEMENT_1: c_uint = 0x01;
pub const WM8978_POWER_MANAGEMENT_2: c_uint = 0x02;
pub const WM8978_POWER_MANAGEMENT_3: c_uint = 0x03;
pub const WM8978_AUDIO_INTERFACE: c_uint = 0x04;
pub const WM8978_COMPANDING_CONTROL: c_uint = 0x05;
pub const WM8978_CLOCKING: c_uint = 0x06;
pub const WM8978_ADDITIONAL_CONTROL: c_uint = 0x07;
pub const WM8978_GPIO_CONTROL: c_uint = 0x08;
pub const WM8978_JACK_DETECT_CONTROL_1: c_uint = 0x09;
pub const WM8978_DAC_CONTROL: c_uint = 0x0A;
pub const WM8978_LEFT_DAC_DIGITAL_VOLUME: c_uint = 0x0B;
pub const WM8978_RIGHT_DAC_DIGITAL_VOLUME: c_uint = 0x0C;
pub const WM8978_JACK_DETECT_CONTROL_2: c_uint = 0x0D;
pub const WM8978_ADC_CONTROL: c_uint = 0x0E;
pub const WM8978_LEFT_ADC_DIGITAL_VOLUME: c_uint = 0x0F;
pub const WM8978_RIGHT_ADC_DIGITAL_VOLUME: c_uint = 0x10;
pub const WM8978_EQ1: c_uint = 0x12;
pub const WM8978_EQ2: c_uint = 0x13;
pub const WM8978_EQ3: c_uint = 0x14;
pub const WM8978_EQ4: c_uint = 0x15;
pub const WM8978_EQ5: c_uint = 0x16;
pub const WM8978_DAC_LIMITER_1: c_uint = 0x18;
pub const WM8978_DAC_LIMITER_2: c_uint = 0x19;
pub const WM8978_NOTCH_FILTER_1: c_uint = 0x1b;
pub const WM8978_NOTCH_FILTER_2: c_uint = 0x1c;
pub const WM8978_NOTCH_FILTER_3: c_uint = 0x1d;
pub const WM8978_NOTCH_FILTER_4: c_uint = 0x1e;
pub const WM8978_ALC_CONTROL_1: c_uint = 0x20;
pub const WM8978_ALC_CONTROL_2: c_uint = 0x21;
pub const WM8978_ALC_CONTROL_3: c_uint = 0x22;
pub const WM8978_NOISE_GATE: c_uint = 0x23;
pub const WM8978_PLL_N: c_uint = 0x24;
pub const WM8978_PLL_K1: c_uint = 0x25;
pub const WM8978_PLL_K2: c_uint = 0x26;
pub const WM8978_PLL_K3: c_uint = 0x27;
pub const WM8978_3D_CONTROL: c_uint = 0x29;
pub const WM8978_BEEP_CONTROL: c_uint = 0x2b;
pub const WM8978_INPUT_CONTROL: c_uint = 0x2c;
pub const WM8978_LEFT_INP_PGA_CONTROL: c_uint = 0x2d;
pub const WM8978_RIGHT_INP_PGA_CONTROL: c_uint = 0x2e;
pub const WM8978_LEFT_ADC_BOOST_CONTROL: c_uint = 0x2f;
pub const WM8978_RIGHT_ADC_BOOST_CONTROL: c_uint = 0x30;
pub const WM8978_OUTPUT_CONTROL: c_uint = 0x31;
pub const WM8978_LEFT_MIXER_CONTROL: c_uint = 0x32;
pub const WM8978_RIGHT_MIXER_CONTROL: c_uint = 0x33;
pub const WM8978_LOUT1_HP_CONTROL: c_uint = 0x34;
pub const WM8978_ROUT1_HP_CONTROL: c_uint = 0x35;
pub const WM8978_LOUT2_SPK_CONTROL: c_uint = 0x36;
pub const WM8978_ROUT2_SPK_CONTROL: c_uint = 0x37;
pub const WM8978_OUT3_MIXER_CONTROL: c_uint = 0x38;
pub const WM8978_OUT4_MIXER_CONTROL: c_uint = 0x39;
pub const WM8978_MAX_REGISTER: c_uint = 0x39;
pub const WM8978_CACHEREGNUM: c_int = 58;
// Clock divider Id's
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm8978_clk_id {
    WM8978_OPCLKRATE,
    WM8978_BCLKDIV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm8978_sysclk_src {
    WM8978_MCLK = 0,
    WM8978_PLL,
}

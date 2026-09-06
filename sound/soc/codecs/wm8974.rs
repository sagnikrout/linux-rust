//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8974.h
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
// wm8974.h  --  WM8974 Soc Audio driver
//
// WM8974 register space
pub const WM8974_RESET: c_uint = 0x0;
pub const WM8974_POWER1: c_uint = 0x1;
pub const WM8974_POWER2: c_uint = 0x2;
pub const WM8974_POWER3: c_uint = 0x3;
pub const WM8974_IFACE: c_uint = 0x4;
pub const WM8974_COMP: c_uint = 0x5;
pub const WM8974_CLOCK: c_uint = 0x6;
pub const WM8974_ADD: c_uint = 0x7;
pub const WM8974_GPIO: c_uint = 0x8;
pub const WM8974_DAC: c_uint = 0xa;
pub const WM8974_DACVOL: c_uint = 0xb;
pub const WM8974_ADC: c_uint = 0xe;
pub const WM8974_ADCVOL: c_uint = 0xf;
pub const WM8974_EQ1: c_uint = 0x12;
pub const WM8974_EQ2: c_uint = 0x13;
pub const WM8974_EQ3: c_uint = 0x14;
pub const WM8974_EQ4: c_uint = 0x15;
pub const WM8974_EQ5: c_uint = 0x16;
pub const WM8974_DACLIM1: c_uint = 0x18;
pub const WM8974_DACLIM2: c_uint = 0x19;
pub const WM8974_NOTCH1: c_uint = 0x1b;
pub const WM8974_NOTCH2: c_uint = 0x1c;
pub const WM8974_NOTCH3: c_uint = 0x1d;
pub const WM8974_NOTCH4: c_uint = 0x1e;
pub const WM8974_ALC1: c_uint = 0x20;
pub const WM8974_ALC2: c_uint = 0x21;
pub const WM8974_ALC3: c_uint = 0x22;
pub const WM8974_NGATE: c_uint = 0x23;
pub const WM8974_PLLN: c_uint = 0x24;
pub const WM8974_PLLK1: c_uint = 0x25;
pub const WM8974_PLLK2: c_uint = 0x26;
pub const WM8974_PLLK3: c_uint = 0x27;
pub const WM8974_ATTEN: c_uint = 0x28;
pub const WM8974_INPUT: c_uint = 0x2c;
pub const WM8974_INPPGA: c_uint = 0x2d;
pub const WM8974_ADCBOOST: c_uint = 0x2f;
pub const WM8974_OUTPUT: c_uint = 0x31;
pub const WM8974_SPKMIX: c_uint = 0x32;
pub const WM8974_SPKVOL: c_uint = 0x36;
pub const WM8974_MONOMIX: c_uint = 0x38;
pub const WM8974_CACHEREGNUM: c_int = 57;
// Clock divider Id's
pub const WM8974_OPCLKDIV: c_int = 0;
pub const WM8974_MCLKDIV: c_int = 1;
pub const WM8974_BCLKDIV: c_int = 2;
// PLL Out dividers

// BCLK clock dividers

// MCLK clock dividers


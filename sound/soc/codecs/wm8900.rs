//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8900.h
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
// wm8900.h  --  WM890 Soc Audio driver
//
pub const WM8900_FLL: c_int = 1;
pub const WM8900_BCLK_DIV: c_int = 1;
pub const WM8900_ADC_CLKDIV: c_int = 2;
pub const WM8900_DAC_CLKDIV: c_int = 3;
pub const WM8900_ADC_LRCLK: c_int = 4;
pub const WM8900_DAC_LRCLK: c_int = 5;
pub const WM8900_OPCLK_DIV: c_int = 6;
pub const WM8900_LRCLK_MODE: c_int = 7;
pub const WM8900_BCLK_DIV_1: c_uint = 0x00;
pub const WM8900_BCLK_DIV_1_5: c_uint = 0x02;
pub const WM8900_BCLK_DIV_2: c_uint = 0x04;
pub const WM8900_BCLK_DIV_3: c_uint = 0x06;
pub const WM8900_BCLK_DIV_4: c_uint = 0x08;
pub const WM8900_BCLK_DIV_5_5: c_uint = 0x0a;
pub const WM8900_BCLK_DIV_6: c_uint = 0x0c;
pub const WM8900_BCLK_DIV_8: c_uint = 0x0e;
pub const WM8900_BCLK_DIV_11: c_uint = 0x10;
pub const WM8900_BCLK_DIV_12: c_uint = 0x12;
pub const WM8900_BCLK_DIV_16: c_uint = 0x14;
pub const WM8900_BCLK_DIV_22: c_uint = 0x16;
pub const WM8900_BCLK_DIV_24: c_uint = 0x18;
pub const WM8900_BCLK_DIV_32: c_uint = 0x1a;
pub const WM8900_BCLK_DIV_44: c_uint = 0x1c;
pub const WM8900_BCLK_DIV_48: c_uint = 0x1e;
pub const WM8900_ADC_CLKDIV_1: c_uint = 0x00;
pub const WM8900_ADC_CLKDIV_1_5: c_uint = 0x20;
pub const WM8900_ADC_CLKDIV_2: c_uint = 0x40;
pub const WM8900_ADC_CLKDIV_3: c_uint = 0x60;
pub const WM8900_ADC_CLKDIV_4: c_uint = 0x80;
pub const WM8900_ADC_CLKDIV_5_5: c_uint = 0xa0;
pub const WM8900_ADC_CLKDIV_6: c_uint = 0xc0;
pub const WM8900_DAC_CLKDIV_1: c_uint = 0x00;
pub const WM8900_DAC_CLKDIV_1_5: c_uint = 0x04;
pub const WM8900_DAC_CLKDIV_2: c_uint = 0x08;
pub const WM8900_DAC_CLKDIV_3: c_uint = 0x0c;
pub const WM8900_DAC_CLKDIV_4: c_uint = 0x10;
pub const WM8900_DAC_CLKDIV_5_5: c_uint = 0x14;
pub const WM8900_DAC_CLKDIV_6: c_uint = 0x18;

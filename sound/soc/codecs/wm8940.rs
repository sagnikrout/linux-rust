//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8940.h
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
// wm8940.h -- WM8940 Soc Audio driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8940_setup_data {
// Vref to analogue output resistance
pub const WM8940_VROI_1K: c_int = 0;
pub const WM8940_VROI_30K: c_int = 1;
    pub vroi:1: c_uint,
}

// WM8940 register space
pub const WM8940_SOFTRESET: c_uint = 0x00;
pub const WM8940_POWER1: c_uint = 0x01;
pub const WM8940_POWER2: c_uint = 0x02;
pub const WM8940_POWER3: c_uint = 0x03;
pub const WM8940_IFACE: c_uint = 0x04;
pub const WM8940_COMPANDINGCTL: c_uint = 0x05;
pub const WM8940_CLOCK: c_uint = 0x06;
pub const WM8940_ADDCNTRL: c_uint = 0x07;
pub const WM8940_GPIO: c_uint = 0x08;
pub const WM8940_CTLINT: c_uint = 0x09;
pub const WM8940_DAC: c_uint = 0x0A;
pub const WM8940_DACVOL: c_uint = 0x0B;
pub const WM8940_ADC: c_uint = 0x0E;
pub const WM8940_ADCVOL: c_uint = 0x0F;
pub const WM8940_NOTCH1: c_uint = 0x10;
pub const WM8940_NOTCH2: c_uint = 0x11;
pub const WM8940_NOTCH3: c_uint = 0x12;
pub const WM8940_NOTCH4: c_uint = 0x13;
pub const WM8940_NOTCH5: c_uint = 0x14;
pub const WM8940_NOTCH6: c_uint = 0x15;
pub const WM8940_NOTCH7: c_uint = 0x16;
pub const WM8940_NOTCH8: c_uint = 0x17;
pub const WM8940_DACLIM1: c_uint = 0x18;
pub const WM8940_DACLIM2: c_uint = 0x19;
pub const WM8940_ALC1: c_uint = 0x20;
pub const WM8940_ALC2: c_uint = 0x21;
pub const WM8940_ALC3: c_uint = 0x22;
pub const WM8940_NOISEGATE: c_uint = 0x23;
pub const WM8940_PLLN: c_uint = 0x24;
pub const WM8940_PLLK1: c_uint = 0x25;
pub const WM8940_PLLK2: c_uint = 0x26;
pub const WM8940_PLLK3: c_uint = 0x27;
pub const WM8940_ALC4: c_uint = 0x2A;
pub const WM8940_INPUTCTL: c_uint = 0x2C;
pub const WM8940_PGAGAIN: c_uint = 0x2D;
pub const WM8940_ADCBOOST: c_uint = 0x2F;
pub const WM8940_OUTPUTCTL: c_uint = 0x31;
pub const WM8940_SPKMIX: c_uint = 0x32;
pub const WM8940_SPKVOL: c_uint = 0x36;
pub const WM8940_MONOMIX: c_uint = 0x38;
pub const WM8940_CACHEREGNUM: c_uint = 0x57;
// Clock divider Id's
pub const WM8940_BCLKDIV: c_int = 0;
pub const WM8940_MCLKDIV: c_int = 1;
pub const WM8940_OPCLKDIV: c_int = 2;
// MCLK clock dividers
pub const WM8940_MCLKDIV_1: c_int = 0;
pub const WM8940_MCLKDIV_1_5: c_int = 1;
pub const WM8940_MCLKDIV_2: c_int = 2;
pub const WM8940_MCLKDIV_3: c_int = 3;
pub const WM8940_MCLKDIV_4: c_int = 4;
pub const WM8940_MCLKDIV_6: c_int = 5;
pub const WM8940_MCLKDIV_8: c_int = 6;
pub const WM8940_MCLKDIV_12: c_int = 7;
// BCLK clock dividers
pub const WM8940_BCLKDIV_1: c_int = 0;
pub const WM8940_BCLKDIV_2: c_int = 1;
pub const WM8940_BCLKDIV_4: c_int = 2;
pub const WM8940_BCLKDIV_8: c_int = 3;
pub const WM8940_BCLKDIV_16: c_int = 4;
pub const WM8940_BCLKDIV_32: c_int = 5;
// PLL Out Dividers
pub const WM8940_OPCLKDIV_1: c_int = 0;
pub const WM8940_OPCLKDIV_2: c_int = 1;
pub const WM8940_OPCLKDIV_3: c_int = 2;
pub const WM8940_OPCLKDIV_4: c_int = 3;
// Chip ID
pub const WM8940_CHIP_ID: c_uint = 0x8940;

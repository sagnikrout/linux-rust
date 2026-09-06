//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8510.h
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
// wm8510.h  --  WM8510 Soc Audio driver
//
// WM8510 register space
pub const WM8510_RESET: c_uint = 0x0;
pub const WM8510_POWER1: c_uint = 0x1;
pub const WM8510_POWER2: c_uint = 0x2;
pub const WM8510_POWER3: c_uint = 0x3;
pub const WM8510_IFACE: c_uint = 0x4;
pub const WM8510_COMP: c_uint = 0x5;
pub const WM8510_CLOCK: c_uint = 0x6;
pub const WM8510_ADD: c_uint = 0x7;
pub const WM8510_GPIO: c_uint = 0x8;
pub const WM8510_DAC: c_uint = 0xa;
pub const WM8510_DACVOL: c_uint = 0xb;
pub const WM8510_ADC: c_uint = 0xe;
pub const WM8510_ADCVOL: c_uint = 0xf;
pub const WM8510_EQ1: c_uint = 0x12;
pub const WM8510_EQ2: c_uint = 0x13;
pub const WM8510_EQ3: c_uint = 0x14;
pub const WM8510_EQ4: c_uint = 0x15;
pub const WM8510_EQ5: c_uint = 0x16;
pub const WM8510_DACLIM1: c_uint = 0x18;
pub const WM8510_DACLIM2: c_uint = 0x19;
pub const WM8510_NOTCH1: c_uint = 0x1b;
pub const WM8510_NOTCH2: c_uint = 0x1c;
pub const WM8510_NOTCH3: c_uint = 0x1d;
pub const WM8510_NOTCH4: c_uint = 0x1e;
pub const WM8510_ALC1: c_uint = 0x20;
pub const WM8510_ALC2: c_uint = 0x21;
pub const WM8510_ALC3: c_uint = 0x22;
pub const WM8510_NGATE: c_uint = 0x23;
pub const WM8510_PLLN: c_uint = 0x24;
pub const WM8510_PLLK1: c_uint = 0x25;
pub const WM8510_PLLK2: c_uint = 0x26;
pub const WM8510_PLLK3: c_uint = 0x27;
pub const WM8510_ATTEN: c_uint = 0x28;
pub const WM8510_INPUT: c_uint = 0x2c;
pub const WM8510_INPPGA: c_uint = 0x2d;
pub const WM8510_ADCBOOST: c_uint = 0x2f;
pub const WM8510_OUTPUT: c_uint = 0x31;
pub const WM8510_SPKMIX: c_uint = 0x32;
pub const WM8510_SPKVOL: c_uint = 0x36;
pub const WM8510_MONOMIX: c_uint = 0x38;
pub const WM8510_CACHEREGNUM: c_int = 57;
// Clock divider Id's
pub const WM8510_OPCLKDIV: c_int = 0;
pub const WM8510_MCLKDIV: c_int = 1;
pub const WM8510_ADCCLK: c_int = 2;
pub const WM8510_DACCLK: c_int = 3;
pub const WM8510_BCLKDIV: c_int = 4;
// DAC clock dividers

// ADC clock dividers

// PLL Out dividers

// BCLK clock dividers

// MCLK clock dividers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8510_setup_data {
    pub spi: c_int,
    pub i2c_bus: c_int,
    pub i2c_address: c_ushort,
}

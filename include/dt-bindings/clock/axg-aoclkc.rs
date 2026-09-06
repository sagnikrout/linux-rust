//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/axg-aoclkc.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright (c) 2016 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
//
// Copyright (c) 2018 Amlogic, inc.
// Author: Qiufang Dai <qiufang.dai@amlogic.com>
//

// Macro flag: #define DT_BINDINGS_CLOCK_AMLOGIC_MESON_AXG_AOCLK
pub const CLKID_AO_REMOTE: c_int = 0;
pub const CLKID_AO_I2C_MASTER: c_int = 1;
pub const CLKID_AO_I2C_SLAVE: c_int = 2;
pub const CLKID_AO_UART1: c_int = 3;
pub const CLKID_AO_UART2: c_int = 4;
pub const CLKID_AO_IR_BLASTER: c_int = 5;
pub const CLKID_AO_SAR_ADC: c_int = 6;
pub const CLKID_AO_CLK81: c_int = 7;
pub const CLKID_AO_SAR_ADC_SEL: c_int = 8;
pub const CLKID_AO_SAR_ADC_DIV: c_int = 9;
pub const CLKID_AO_SAR_ADC_CLK: c_int = 10;
pub const CLKID_AO_CTS_OSCIN: c_int = 11;
pub const CLKID_AO_32K_PRE: c_int = 12;
pub const CLKID_AO_32K_DIV: c_int = 13;
pub const CLKID_AO_32K_SEL: c_int = 14;
pub const CLKID_AO_32K: c_int = 15;
pub const CLKID_AO_CTS_RTC_OSCIN: c_int = 16;

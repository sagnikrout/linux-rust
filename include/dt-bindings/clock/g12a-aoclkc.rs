//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/g12a-aoclkc.h
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

// Macro flag: #define DT_BINDINGS_CLOCK_AMLOGIC_MESON_G12A_AOCLK
pub const CLKID_AO_AHB: c_int = 0;
pub const CLKID_AO_IR_IN: c_int = 1;
pub const CLKID_AO_I2C_M0: c_int = 2;
pub const CLKID_AO_I2C_S0: c_int = 3;
pub const CLKID_AO_UART: c_int = 4;
pub const CLKID_AO_PROD_I2C: c_int = 5;
pub const CLKID_AO_UART2: c_int = 6;
pub const CLKID_AO_IR_OUT: c_int = 7;
pub const CLKID_AO_SAR_ADC: c_int = 8;
pub const CLKID_AO_MAILBOX: c_int = 9;
pub const CLKID_AO_M3: c_int = 10;
pub const CLKID_AO_AHB_SRAM: c_int = 11;
pub const CLKID_AO_RTI: c_int = 12;
pub const CLKID_AO_M4_FCLK: c_int = 13;
pub const CLKID_AO_M4_HCLK: c_int = 14;
pub const CLKID_AO_CLK81: c_int = 15;
pub const CLKID_AO_SAR_ADC_DIV: c_int = 17;
pub const CLKID_AO_SAR_ADC_SEL: c_int = 16;
pub const CLKID_AO_SAR_ADC_CLK: c_int = 18;
pub const CLKID_AO_CTS_OSCIN: c_int = 19;
pub const CLKID_AO_32K_PRE: c_int = 20;
pub const CLKID_AO_32K_DIV: c_int = 21;
pub const CLKID_AO_32K_SEL: c_int = 22;
pub const CLKID_AO_32K: c_int = 23;
pub const CLKID_AO_CEC_PRE: c_int = 24;
pub const CLKID_AO_CEC_DIV: c_int = 25;
pub const CLKID_AO_CEC_SEL: c_int = 26;
pub const CLKID_AO_CEC: c_int = 27;
pub const CLKID_AO_CTS_RTC_OSCIN: c_int = 28;

//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/cirrus,ep9301-syscon.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
pub const EP93XX_CLK_PLL1: c_int = 0;
pub const EP93XX_CLK_PLL2: c_int = 1;
pub const EP93XX_CLK_FCLK: c_int = 2;
pub const EP93XX_CLK_HCLK: c_int = 3;
pub const EP93XX_CLK_PCLK: c_int = 4;
pub const EP93XX_CLK_UART: c_int = 5;
pub const EP93XX_CLK_SPI: c_int = 6;
pub const EP93XX_CLK_PWM: c_int = 7;
pub const EP93XX_CLK_USB: c_int = 8;
pub const EP93XX_CLK_M2M0: c_int = 9;
pub const EP93XX_CLK_M2M1: c_int = 10;
pub const EP93XX_CLK_M2P0: c_int = 11;
pub const EP93XX_CLK_M2P1: c_int = 12;
pub const EP93XX_CLK_M2P2: c_int = 13;
pub const EP93XX_CLK_M2P3: c_int = 14;
pub const EP93XX_CLK_M2P4: c_int = 15;
pub const EP93XX_CLK_M2P5: c_int = 16;
pub const EP93XX_CLK_M2P6: c_int = 17;
pub const EP93XX_CLK_M2P7: c_int = 18;
pub const EP93XX_CLK_M2P8: c_int = 19;
pub const EP93XX_CLK_M2P9: c_int = 20;
pub const EP93XX_CLK_UART1: c_int = 21;
pub const EP93XX_CLK_UART2: c_int = 22;
pub const EP93XX_CLK_UART3: c_int = 23;
pub const EP93XX_CLK_ADC: c_int = 24;
pub const EP93XX_CLK_ADC_EN: c_int = 25;
pub const EP93XX_CLK_KEYPAD: c_int = 26;
pub const EP93XX_CLK_VIDEO: c_int = 27;
pub const EP93XX_CLK_I2S_MCLK: c_int = 28;
pub const EP93XX_CLK_I2S_SCLK: c_int = 29;
pub const EP93XX_CLK_I2S_LRCLK: c_int = 30;

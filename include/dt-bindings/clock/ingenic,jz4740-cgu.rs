//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/ingenic,jz4740-cgu.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This header provides clock numbers for the ingenic,jz4740-cgu DT binding.
//
// They are roughly ordered as:
// - external clocks
// - PLLs
// - muxes/dividers in the order they appear in the jz4740 programmers manual
// - gates in order of their bit in the CLKGR* registers
//
pub const JZ4740_CLK_EXT: c_int = 0;
pub const JZ4740_CLK_RTC: c_int = 1;
pub const JZ4740_CLK_PLL: c_int = 2;
pub const JZ4740_CLK_PLL_HALF: c_int = 3;
pub const JZ4740_CLK_CCLK: c_int = 4;
pub const JZ4740_CLK_HCLK: c_int = 5;
pub const JZ4740_CLK_PCLK: c_int = 6;
pub const JZ4740_CLK_MCLK: c_int = 7;
pub const JZ4740_CLK_LCD: c_int = 8;
pub const JZ4740_CLK_LCD_PCLK: c_int = 9;
pub const JZ4740_CLK_I2S: c_int = 10;
pub const JZ4740_CLK_SPI: c_int = 11;
pub const JZ4740_CLK_MMC: c_int = 12;
pub const JZ4740_CLK_UHC: c_int = 13;
pub const JZ4740_CLK_UDC: c_int = 14;
pub const JZ4740_CLK_UART0: c_int = 15;
pub const JZ4740_CLK_UART1: c_int = 16;
pub const JZ4740_CLK_DMA: c_int = 17;
pub const JZ4740_CLK_IPU: c_int = 18;
pub const JZ4740_CLK_ADC: c_int = 19;
pub const JZ4740_CLK_I2C: c_int = 20;
pub const JZ4740_CLK_AIC: c_int = 21;
pub const JZ4740_CLK_TCU: c_int = 22;

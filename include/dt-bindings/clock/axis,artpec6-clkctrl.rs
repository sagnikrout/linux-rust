//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/axis,artpec6-clkctrl.h
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
// ARTPEC-6 clock controller indexes
//
// Copyright 2016 Axis Communications AB.
//
pub const ARTPEC6_CLK_CPU: c_int = 0;
pub const ARTPEC6_CLK_CPU_PERIPH: c_int = 1;
pub const ARTPEC6_CLK_NAND_CLKA: c_int = 2;
pub const ARTPEC6_CLK_NAND_CLKB: c_int = 3;
pub const ARTPEC6_CLK_ETH_ACLK: c_int = 4;
pub const ARTPEC6_CLK_DMA_ACLK: c_int = 5;
pub const ARTPEC6_CLK_PTP_REF: c_int = 6;
pub const ARTPEC6_CLK_SD_PCLK: c_int = 7;
pub const ARTPEC6_CLK_SD_IMCLK: c_int = 8;
pub const ARTPEC6_CLK_I2S_HST: c_int = 9;
pub const ARTPEC6_CLK_I2S0_CLK: c_int = 10;
pub const ARTPEC6_CLK_I2S1_CLK: c_int = 11;
pub const ARTPEC6_CLK_UART_PCLK: c_int = 12;
pub const ARTPEC6_CLK_UART_REFCLK: c_int = 13;
pub const ARTPEC6_CLK_I2C: c_int = 14;
pub const ARTPEC6_CLK_SPI_PCLK: c_int = 15;
pub const ARTPEC6_CLK_SPI_SSPCLK: c_int = 16;
pub const ARTPEC6_CLK_SYS_TIMER: c_int = 17;
pub const ARTPEC6_CLK_FRACDIV_IN: c_int = 18;
pub const ARTPEC6_CLK_DBG_PCLK: c_int = 19;
// This must be the highest clock index plus one.
pub const ARTPEC6_CLK_NUMCLOCKS: c_int = 20;

//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/k210-clk.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2019-20 Sean Anderson <seanga2@gmail.com>
// Copyright (c) 2020 Western Digital Corporation or its affiliates.
//
// Kendryte K210 SoC clock identifiers (arbitrary values).
//
pub const K210_CLK_CPU: c_int = 0;
pub const K210_CLK_SRAM0: c_int = 1;
pub const K210_CLK_SRAM1: c_int = 2;
pub const K210_CLK_AI: c_int = 3;
pub const K210_CLK_DMA: c_int = 4;
pub const K210_CLK_FFT: c_int = 5;
pub const K210_CLK_ROM: c_int = 6;
pub const K210_CLK_DVP: c_int = 7;
pub const K210_CLK_APB0: c_int = 8;
pub const K210_CLK_APB1: c_int = 9;
pub const K210_CLK_APB2: c_int = 10;
pub const K210_CLK_I2S0: c_int = 11;
pub const K210_CLK_I2S1: c_int = 12;
pub const K210_CLK_I2S2: c_int = 13;
pub const K210_CLK_I2S0_M: c_int = 14;
pub const K210_CLK_I2S1_M: c_int = 15;
pub const K210_CLK_I2S2_M: c_int = 16;
pub const K210_CLK_WDT0: c_int = 17;
pub const K210_CLK_WDT1: c_int = 18;
pub const K210_CLK_SPI0: c_int = 19;
pub const K210_CLK_SPI1: c_int = 20;
pub const K210_CLK_SPI2: c_int = 21;
pub const K210_CLK_I2C0: c_int = 22;
pub const K210_CLK_I2C1: c_int = 23;
pub const K210_CLK_I2C2: c_int = 24;
pub const K210_CLK_SPI3: c_int = 25;
pub const K210_CLK_TIMER0: c_int = 26;
pub const K210_CLK_TIMER1: c_int = 27;
pub const K210_CLK_TIMER2: c_int = 28;
pub const K210_CLK_GPIO: c_int = 29;
pub const K210_CLK_UART1: c_int = 30;
pub const K210_CLK_UART2: c_int = 31;
pub const K210_CLK_UART3: c_int = 32;
pub const K210_CLK_FPIOA: c_int = 33;
pub const K210_CLK_SHA: c_int = 34;
pub const K210_CLK_AES: c_int = 35;
pub const K210_CLK_OTP: c_int = 36;
pub const K210_CLK_RTC: c_int = 37;
pub const K210_NUM_CLKS: c_int = 38;

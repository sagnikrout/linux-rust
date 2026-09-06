//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/exynos5410.h
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
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Copyright (c) 2016 Krzysztof Kozlowski
//
// Device Tree binding constants for Exynos5421 clock controller.
//
// core clocks
pub const CLK_FIN_PLL: c_int = 1;
pub const CLK_FOUT_APLL: c_int = 2;
pub const CLK_FOUT_CPLL: c_int = 3;
pub const CLK_FOUT_MPLL: c_int = 4;
pub const CLK_FOUT_BPLL: c_int = 5;
pub const CLK_FOUT_KPLL: c_int = 6;
pub const CLK_FOUT_EPLL: c_int = 7;
// gate for special clocks (sclk)
pub const CLK_SCLK_UART0: c_int = 128;
pub const CLK_SCLK_UART1: c_int = 129;
pub const CLK_SCLK_UART2: c_int = 130;
pub const CLK_SCLK_UART3: c_int = 131;
pub const CLK_SCLK_MMC0: c_int = 132;
pub const CLK_SCLK_MMC1: c_int = 133;
pub const CLK_SCLK_MMC2: c_int = 134;
pub const CLK_SCLK_USBD300: c_int = 150;
pub const CLK_SCLK_USBD301: c_int = 151;
pub const CLK_SCLK_USBPHY300: c_int = 152;
pub const CLK_SCLK_USBPHY301: c_int = 153;
pub const CLK_SCLK_PWM: c_int = 155;
// gate clocks
pub const CLK_UART0: c_int = 257;
pub const CLK_UART1: c_int = 258;
pub const CLK_UART2: c_int = 259;
pub const CLK_UART3: c_int = 260;
pub const CLK_I2C0: c_int = 261;
pub const CLK_I2C1: c_int = 262;
pub const CLK_I2C2: c_int = 263;
pub const CLK_I2C3: c_int = 264;
pub const CLK_USI0: c_int = 265;
pub const CLK_USI1: c_int = 266;
pub const CLK_USI2: c_int = 267;
pub const CLK_USI3: c_int = 268;
pub const CLK_TSADC: c_int = 270;
pub const CLK_PWM: c_int = 279;
pub const CLK_MCT: c_int = 315;
pub const CLK_WDT: c_int = 316;
pub const CLK_RTC: c_int = 317;
pub const CLK_TMU: c_int = 318;
pub const CLK_MMC0: c_int = 351;
pub const CLK_MMC1: c_int = 352;
pub const CLK_MMC2: c_int = 353;
pub const CLK_PDMA0: c_int = 362;
pub const CLK_PDMA1: c_int = 363;
pub const CLK_USBH20: c_int = 365;
pub const CLK_USBD300: c_int = 366;
pub const CLK_USBD301: c_int = 367;
pub const CLK_SSS: c_int = 471;

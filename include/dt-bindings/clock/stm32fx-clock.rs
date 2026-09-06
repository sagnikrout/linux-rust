//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/stm32fx-clock.h
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
// stm32fx-clock.h
//
// Copyright (C) 2016 STMicroelectronics
// Author: Gabriel Fernandez for STMicroelectronics.
//
// List of clocks which are not derived from system clock (SYSCLOCK)
//
// The index of these clocks is the secondary index of DT bindings
// (see Documentation/devicetree/bindings/clock/st,stm32-rcc.yaml)
//
// e.g:
//
pub const SYSTICK: c_int = 0;
pub const FCLK: c_int = 1;
pub const CLK_LSI: c_int = 2;
pub const CLK_LSE: c_int = 3;
pub const CLK_HSE_RTC: c_int = 4;
pub const CLK_RTC: c_int = 5;
pub const PLL_VCO_I2S: c_int = 6;
pub const PLL_VCO_SAI: c_int = 7;
pub const CLK_LCD: c_int = 8;
pub const CLK_I2S: c_int = 9;
pub const CLK_SAI1: c_int = 10;
pub const CLK_SAI2: c_int = 11;
pub const CLK_I2SQ_PDIV: c_int = 12;
pub const CLK_SAIQ_PDIV: c_int = 13;
pub const CLK_HSI: c_int = 14;
pub const CLK_SYSCLK: c_int = 15;
pub const CLK_F469_DSI: c_int = 16;
pub const END_PRIMARY_CLK: c_int = 17;
pub const CLK_HDMI_CEC: c_int = 16;
pub const CLK_SPDIF: c_int = 17;
pub const CLK_USART1: c_int = 18;
pub const CLK_USART2: c_int = 19;
pub const CLK_USART3: c_int = 20;
pub const CLK_UART4: c_int = 21;
pub const CLK_UART5: c_int = 22;
pub const CLK_USART6: c_int = 23;
pub const CLK_UART7: c_int = 24;
pub const CLK_UART8: c_int = 25;
pub const CLK_I2C1: c_int = 26;
pub const CLK_I2C2: c_int = 27;
pub const CLK_I2C3: c_int = 28;
pub const CLK_I2C4: c_int = 29;
pub const CLK_LPTIMER: c_int = 30;
pub const CLK_PLL_SRC: c_int = 31;
pub const CLK_DFSDM1: c_int = 32;
pub const CLK_ADFSDM1: c_int = 33;
pub const CLK_F769_DSI: c_int = 34;
pub const END_PRIMARY_CLK_F7: c_int = 35;

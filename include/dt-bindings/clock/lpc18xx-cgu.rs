//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/lpc18xx-cgu.h
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


//
// Copyright (c) 2015 Joachim Eastwood <manabian@gmail.com>
//
// This code is released using a dual license strategy: BSD/GPL
// You can choose the licence that better fits your requirements.
//
// Released under the terms of 3-clause BSD License
// Released under the terms of GNU General Public License Version 2.0
//
// LPC18xx/43xx base clock ids
pub const BASE_SAFE_CLK: c_int = 0;
pub const BASE_USB0_CLK: c_int = 1;
pub const BASE_PERIPH_CLK: c_int = 2;
pub const BASE_USB1_CLK: c_int = 3;
pub const BASE_CPU_CLK: c_int = 4;
pub const BASE_SPIFI_CLK: c_int = 5;
pub const BASE_SPI_CLK: c_int = 6;
pub const BASE_PHY_RX_CLK: c_int = 7;
pub const BASE_PHY_TX_CLK: c_int = 8;
pub const BASE_APB1_CLK: c_int = 9;
pub const BASE_APB3_CLK: c_int = 10;
pub const BASE_LCD_CLK: c_int = 11;
pub const BASE_ADCHS_CLK: c_int = 12;
pub const BASE_SDIO_CLK: c_int = 13;
pub const BASE_SSP0_CLK: c_int = 14;
pub const BASE_SSP1_CLK: c_int = 15;
pub const BASE_UART0_CLK: c_int = 16;
pub const BASE_UART1_CLK: c_int = 17;
pub const BASE_UART2_CLK: c_int = 18;
pub const BASE_UART3_CLK: c_int = 19;
pub const BASE_OUT_CLK: c_int = 20;
pub const BASE_RES1_CLK: c_int = 21;
pub const BASE_RES2_CLK: c_int = 22;
pub const BASE_RES3_CLK: c_int = 23;
pub const BASE_RES4_CLK: c_int = 24;
pub const BASE_AUDIO_CLK: c_int = 25;
pub const BASE_CGU_OUT0_CLK: c_int = 26;
pub const BASE_CGU_OUT1_CLK: c_int = 27;

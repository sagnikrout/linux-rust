//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imx1-clock.h
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
// Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
//
pub const IMX1_CLK_DUMMY: c_int = 0;
pub const IMX1_CLK_CLK32: c_int = 1;
pub const IMX1_CLK_CLK16M_EXT: c_int = 2;
pub const IMX1_CLK_CLK16M: c_int = 3;
pub const IMX1_CLK_CLK32_PREMULT: c_int = 4;
pub const IMX1_CLK_PREM: c_int = 5;
pub const IMX1_CLK_MPLL: c_int = 6;
pub const IMX1_CLK_MPLL_GATE: c_int = 7;
pub const IMX1_CLK_SPLL: c_int = 8;
pub const IMX1_CLK_SPLL_GATE: c_int = 9;
pub const IMX1_CLK_MCU: c_int = 10;
pub const IMX1_CLK_FCLK: c_int = 11;
pub const IMX1_CLK_HCLK: c_int = 12;
pub const IMX1_CLK_CLK48M: c_int = 13;
pub const IMX1_CLK_PER1: c_int = 14;
pub const IMX1_CLK_PER2: c_int = 15;
pub const IMX1_CLK_PER3: c_int = 16;
pub const IMX1_CLK_CLKO: c_int = 17;
pub const IMX1_CLK_UART3_GATE: c_int = 18;
pub const IMX1_CLK_SSI2_GATE: c_int = 19;
pub const IMX1_CLK_BROM_GATE: c_int = 20;
pub const IMX1_CLK_DMA_GATE: c_int = 21;
pub const IMX1_CLK_CSI_GATE: c_int = 22;
pub const IMX1_CLK_MMA_GATE: c_int = 23;
pub const IMX1_CLK_USBD_GATE: c_int = 24;
pub const IMX1_CLK_MAX: c_int = 25;

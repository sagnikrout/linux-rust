//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/s5pv210-audss.h
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
// Copyright (c) 2014 Tomasz Figa <tomasz.figa@gmail.com>
//
// This header provides constants for Samsung audio subsystem
// clock controller.
//
// The constants defined in this header are being used in dts
// and s5pv210 audss driver.
//
pub const CLK_MOUT_AUDSS: c_int = 0;
pub const CLK_MOUT_I2S_A: c_int = 1;
pub const CLK_DOUT_AUD_BUS: c_int = 2;
pub const CLK_DOUT_I2S_A: c_int = 3;
pub const CLK_I2S: c_int = 4;
pub const CLK_HCLK_I2S: c_int = 5;
pub const CLK_HCLK_UART: c_int = 6;
pub const CLK_HCLK_HWA: c_int = 7;
pub const CLK_HCLK_DMA: c_int = 8;
pub const CLK_HCLK_BUF: c_int = 9;
pub const CLK_HCLK_RP: c_int = 10;
pub const AUDSS_MAX_CLKS: c_int = 11;

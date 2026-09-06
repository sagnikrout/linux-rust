//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/renesas,r9a09g087-cpg-mssr.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) 2025 Renesas Electronics Corp.
//

// R9A09G087 CPG Core Clocks
pub const R9A09G087_CLK_CA55C0: c_int = 0;
pub const R9A09G087_CLK_CA55C1: c_int = 1;
pub const R9A09G087_CLK_CA55C2: c_int = 2;
pub const R9A09G087_CLK_CA55C3: c_int = 3;
pub const R9A09G087_CLK_CA55S: c_int = 4;
pub const R9A09G087_CLK_CR52_CPU0: c_int = 5;
pub const R9A09G087_CLK_CR52_CPU1: c_int = 6;
pub const R9A09G087_CLK_CKIO: c_int = 7;
pub const R9A09G087_CLK_PCLKAH: c_int = 8;
pub const R9A09G087_CLK_PCLKAM: c_int = 9;
pub const R9A09G087_CLK_PCLKAL: c_int = 10;
pub const R9A09G087_CLK_PCLKGPTL: c_int = 11;
pub const R9A09G087_CLK_PCLKH: c_int = 12;
pub const R9A09G087_CLK_PCLKM: c_int = 13;
pub const R9A09G087_CLK_PCLKL: c_int = 14;
pub const R9A09G087_SDHI_CLKHS: c_int = 15;
pub const R9A09G087_USB_CLK: c_int = 16;
pub const R9A09G087_ETCLKA: c_int = 17;
pub const R9A09G087_ETCLKB: c_int = 18;
pub const R9A09G087_ETCLKC: c_int = 19;
pub const R9A09G087_ETCLKD: c_int = 20;
pub const R9A09G087_ETCLKE: c_int = 21;
pub const R9A09G087_XSPI_CLK0: c_int = 22;
pub const R9A09G087_XSPI_CLK1: c_int = 23;
pub const R9A09G087_PCLKCAN: c_int = 24;
pub const R9A09G087_LCDC_CLKD: c_int = 25;
pub const R9A09G087_PCLKRTC: c_int = 26;

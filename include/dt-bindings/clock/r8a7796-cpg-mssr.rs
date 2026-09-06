//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r8a7796-cpg-mssr.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2016 Renesas Electronics Corp.
//

// r8a7796 CPG Core Clocks
pub const R8A7796_CLK_Z: c_int = 0;
pub const R8A7796_CLK_Z2: c_int = 1;
pub const R8A7796_CLK_ZR: c_int = 2;
pub const R8A7796_CLK_ZG: c_int = 3;
pub const R8A7796_CLK_ZTR: c_int = 4;
pub const R8A7796_CLK_ZTRD2: c_int = 5;
pub const R8A7796_CLK_ZT: c_int = 6;
pub const R8A7796_CLK_ZX: c_int = 7;
pub const R8A7796_CLK_S0D1: c_int = 8;
pub const R8A7796_CLK_S0D2: c_int = 9;
pub const R8A7796_CLK_S0D3: c_int = 10;
pub const R8A7796_CLK_S0D4: c_int = 11;
pub const R8A7796_CLK_S0D6: c_int = 12;
pub const R8A7796_CLK_S0D8: c_int = 13;
pub const R8A7796_CLK_S0D12: c_int = 14;
pub const R8A7796_CLK_S1D1: c_int = 15;
pub const R8A7796_CLK_S1D2: c_int = 16;
pub const R8A7796_CLK_S1D4: c_int = 17;
pub const R8A7796_CLK_S2D1: c_int = 18;
pub const R8A7796_CLK_S2D2: c_int = 19;
pub const R8A7796_CLK_S2D4: c_int = 20;
pub const R8A7796_CLK_S3D1: c_int = 21;
pub const R8A7796_CLK_S3D2: c_int = 22;
pub const R8A7796_CLK_S3D4: c_int = 23;
pub const R8A7796_CLK_LB: c_int = 24;
pub const R8A7796_CLK_CL: c_int = 25;
pub const R8A7796_CLK_ZB3: c_int = 26;
pub const R8A7796_CLK_ZB3D2: c_int = 27;
pub const R8A7796_CLK_ZB3D4: c_int = 28;
pub const R8A7796_CLK_CR: c_int = 29;
pub const R8A7796_CLK_CRD2: c_int = 30;
pub const R8A7796_CLK_SD0H: c_int = 31;
pub const R8A7796_CLK_SD0: c_int = 32;
pub const R8A7796_CLK_SD1H: c_int = 33;
pub const R8A7796_CLK_SD1: c_int = 34;
pub const R8A7796_CLK_SD2H: c_int = 35;
pub const R8A7796_CLK_SD2: c_int = 36;
pub const R8A7796_CLK_SD3H: c_int = 37;
pub const R8A7796_CLK_SD3: c_int = 38;
pub const R8A7796_CLK_SSP2: c_int = 39;
pub const R8A7796_CLK_SSP1: c_int = 40;
pub const R8A7796_CLK_SSPRS: c_int = 41;
pub const R8A7796_CLK_RPC: c_int = 42;
pub const R8A7796_CLK_RPCD2: c_int = 43;
pub const R8A7796_CLK_MSO: c_int = 44;
pub const R8A7796_CLK_CANFD: c_int = 45;
pub const R8A7796_CLK_HDMI: c_int = 46;
pub const R8A7796_CLK_CSI0: c_int = 47;
// CLK_CSIREF was removed
pub const R8A7796_CLK_CP: c_int = 49;
pub const R8A7796_CLK_CPEX: c_int = 50;
pub const R8A7796_CLK_R: c_int = 51;
pub const R8A7796_CLK_OSC: c_int = 52;

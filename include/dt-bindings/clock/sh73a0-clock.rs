//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sh73a0-clock.h
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
// Copyright 2014 Ulrich Hecht
//
// CPG
pub const SH73A0_CLK_MAIN: c_int = 0;
pub const SH73A0_CLK_PLL0: c_int = 1;
pub const SH73A0_CLK_PLL1: c_int = 2;
pub const SH73A0_CLK_PLL2: c_int = 3;
pub const SH73A0_CLK_PLL3: c_int = 4;
pub const SH73A0_CLK_DSI0PHY: c_int = 5;
pub const SH73A0_CLK_DSI1PHY: c_int = 6;
pub const SH73A0_CLK_ZG: c_int = 7;
pub const SH73A0_CLK_M3: c_int = 8;
pub const SH73A0_CLK_B: c_int = 9;
pub const SH73A0_CLK_M1: c_int = 10;
pub const SH73A0_CLK_M2: c_int = 11;
pub const SH73A0_CLK_Z: c_int = 12;
pub const SH73A0_CLK_ZX: c_int = 13;
pub const SH73A0_CLK_HP: c_int = 14;
// MSTP0
pub const SH73A0_CLK_IIC2: c_int = 1;
pub const SH73A0_CLK_MSIOF0: c_int = 0;
// MSTP1
pub const SH73A0_CLK_CEU1: c_int = 29;
pub const SH73A0_CLK_CSI2_RX1: c_int = 28;
pub const SH73A0_CLK_CEU0: c_int = 27;
pub const SH73A0_CLK_CSI2_RX0: c_int = 26;
pub const SH73A0_CLK_TMU0: c_int = 25;
pub const SH73A0_CLK_DSITX0: c_int = 18;
pub const SH73A0_CLK_IIC0: c_int = 16;
pub const SH73A0_CLK_SGX: c_int = 12;
pub const SH73A0_CLK_LCDC0: c_int = 0;
// MSTP2
pub const SH73A0_CLK_SCIFA7: c_int = 19;
pub const SH73A0_CLK_SY_DMAC: c_int = 18;
pub const SH73A0_CLK_MP_DMAC: c_int = 17;
pub const SH73A0_CLK_MSIOF3: c_int = 15;
pub const SH73A0_CLK_MSIOF1: c_int = 8;
pub const SH73A0_CLK_SCIFA5: c_int = 7;
pub const SH73A0_CLK_SCIFB: c_int = 6;
pub const SH73A0_CLK_MSIOF2: c_int = 5;
pub const SH73A0_CLK_SCIFA0: c_int = 4;
pub const SH73A0_CLK_SCIFA1: c_int = 3;
pub const SH73A0_CLK_SCIFA2: c_int = 2;
pub const SH73A0_CLK_SCIFA3: c_int = 1;
pub const SH73A0_CLK_SCIFA4: c_int = 0;
// MSTP3
pub const SH73A0_CLK_SCIFA6: c_int = 31;
pub const SH73A0_CLK_CMT1: c_int = 29;
pub const SH73A0_CLK_FSI: c_int = 28;
pub const SH73A0_CLK_IRDA: c_int = 25;
pub const SH73A0_CLK_IIC1: c_int = 23;
pub const SH73A0_CLK_USB: c_int = 22;
pub const SH73A0_CLK_FLCTL: c_int = 15;
pub const SH73A0_CLK_SDHI0: c_int = 14;
pub const SH73A0_CLK_SDHI1: c_int = 13;
pub const SH73A0_CLK_MMCIF0: c_int = 12;
pub const SH73A0_CLK_SDHI2: c_int = 11;
pub const SH73A0_CLK_TPU0: c_int = 4;
pub const SH73A0_CLK_TPU1: c_int = 3;
pub const SH73A0_CLK_TPU2: c_int = 2;
pub const SH73A0_CLK_TPU3: c_int = 1;
pub const SH73A0_CLK_TPU4: c_int = 0;
// MSTP4
pub const SH73A0_CLK_IIC3: c_int = 11;
pub const SH73A0_CLK_IIC4: c_int = 10;
pub const SH73A0_CLK_KEYSC: c_int = 3;
// MSTP5
pub const SH73A0_CLK_INTCA0: c_int = 8;

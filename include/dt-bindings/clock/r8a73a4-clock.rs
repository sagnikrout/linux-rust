//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r8a73a4-clock.h
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
pub const R8A73A4_CLK_MAIN: c_int = 0;
pub const R8A73A4_CLK_PLL0: c_int = 1;
pub const R8A73A4_CLK_PLL1: c_int = 2;
pub const R8A73A4_CLK_PLL2: c_int = 3;
pub const R8A73A4_CLK_PLL2S: c_int = 4;
pub const R8A73A4_CLK_PLL2H: c_int = 5;
pub const R8A73A4_CLK_Z: c_int = 6;
pub const R8A73A4_CLK_Z2: c_int = 7;
pub const R8A73A4_CLK_I: c_int = 8;
pub const R8A73A4_CLK_M3: c_int = 9;
pub const R8A73A4_CLK_B: c_int = 10;
pub const R8A73A4_CLK_M1: c_int = 11;
pub const R8A73A4_CLK_M2: c_int = 12;
pub const R8A73A4_CLK_ZX: c_int = 13;
pub const R8A73A4_CLK_ZS: c_int = 14;
pub const R8A73A4_CLK_HP: c_int = 15;
pub const R8A73A4_CLK_ZTR: c_int = 16;
pub const R8A73A4_CLK_ZT: c_int = 17;
// MSTP1
pub const R8A73A4_CLK_TMU0: c_int = 25;
pub const R8A73A4_CLK_TMU3: c_int = 21;
// MSTP2
pub const R8A73A4_CLK_DMAC: c_int = 18;
pub const R8A73A4_CLK_SCIFB3: c_int = 17;
pub const R8A73A4_CLK_SCIFB2: c_int = 16;
pub const R8A73A4_CLK_SCIFB1: c_int = 7;
pub const R8A73A4_CLK_SCIFB0: c_int = 6;
pub const R8A73A4_CLK_SCIFA0: c_int = 4;
pub const R8A73A4_CLK_SCIFA1: c_int = 3;
// MSTP3
pub const R8A73A4_CLK_CMT1: c_int = 29;
pub const R8A73A4_CLK_IIC1: c_int = 23;
pub const R8A73A4_CLK_IIC0: c_int = 18;
pub const R8A73A4_CLK_IIC7: c_int = 17;
pub const R8A73A4_CLK_IIC6: c_int = 16;
pub const R8A73A4_CLK_MMCIF0: c_int = 15;
pub const R8A73A4_CLK_SDHI0: c_int = 14;
pub const R8A73A4_CLK_SDHI1: c_int = 13;
pub const R8A73A4_CLK_SDHI2: c_int = 12;
pub const R8A73A4_CLK_MMCIF1: c_int = 5;
pub const R8A73A4_CLK_IIC2: c_int = 0;
// MSTP4
pub const R8A73A4_CLK_IIC3: c_int = 11;
pub const R8A73A4_CLK_IIC4: c_int = 10;
pub const R8A73A4_CLK_IIC5: c_int = 9;
pub const R8A73A4_CLK_INTC_SYS: c_int = 8;
pub const R8A73A4_CLK_IRQC: c_int = 7;
// MSTP5
pub const R8A73A4_CLK_THERMAL: c_int = 22;
pub const R8A73A4_CLK_IIC8: c_int = 15;

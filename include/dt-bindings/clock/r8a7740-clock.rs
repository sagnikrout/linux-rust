//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r8a7740-clock.h
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
pub const R8A7740_CLK_SYSTEM: c_int = 0;
pub const R8A7740_CLK_PLLC0: c_int = 1;
pub const R8A7740_CLK_PLLC1: c_int = 2;
pub const R8A7740_CLK_PLLC2: c_int = 3;
pub const R8A7740_CLK_R: c_int = 4;
pub const R8A7740_CLK_USB24S: c_int = 5;
pub const R8A7740_CLK_I: c_int = 6;
pub const R8A7740_CLK_ZG: c_int = 7;
pub const R8A7740_CLK_B: c_int = 8;
pub const R8A7740_CLK_M1: c_int = 9;
pub const R8A7740_CLK_HP: c_int = 10;
pub const R8A7740_CLK_HPP: c_int = 11;
pub const R8A7740_CLK_USBP: c_int = 12;
pub const R8A7740_CLK_S: c_int = 13;
pub const R8A7740_CLK_ZB: c_int = 14;
pub const R8A7740_CLK_M3: c_int = 15;
pub const R8A7740_CLK_CP: c_int = 16;
pub const R8A7740_CLK_ZTR: c_int = 17;
pub const R8A7740_CLK_ZT: c_int = 18;
// MSTP1
pub const R8A7740_CLK_CEU21: c_int = 28;
pub const R8A7740_CLK_CEU20: c_int = 27;
pub const R8A7740_CLK_TMU0: c_int = 25;
pub const R8A7740_CLK_LCDC1: c_int = 17;
pub const R8A7740_CLK_IIC0: c_int = 16;
pub const R8A7740_CLK_TMU1: c_int = 11;
pub const R8A7740_CLK_LCDC0: c_int = 0;
// MSTP2
pub const R8A7740_CLK_SCIFA6: c_int = 30;
pub const R8A7740_CLK_INTCA: c_int = 29;
pub const R8A7740_CLK_SCIFA7: c_int = 22;
pub const R8A7740_CLK_DMAC1: c_int = 18;
pub const R8A7740_CLK_DMAC2: c_int = 17;
pub const R8A7740_CLK_DMAC3: c_int = 16;
pub const R8A7740_CLK_USBDMAC: c_int = 14;
pub const R8A7740_CLK_SCIFA5: c_int = 7;
pub const R8A7740_CLK_SCIFB: c_int = 6;
pub const R8A7740_CLK_SCIFA0: c_int = 4;
pub const R8A7740_CLK_SCIFA1: c_int = 3;
pub const R8A7740_CLK_SCIFA2: c_int = 2;
pub const R8A7740_CLK_SCIFA3: c_int = 1;
pub const R8A7740_CLK_SCIFA4: c_int = 0;
// MSTP3
pub const R8A7740_CLK_CMT1: c_int = 29;
pub const R8A7740_CLK_FSI: c_int = 28;
pub const R8A7740_CLK_IIC1: c_int = 23;
pub const R8A7740_CLK_USBF: c_int = 20;
pub const R8A7740_CLK_SDHI0: c_int = 14;
pub const R8A7740_CLK_SDHI1: c_int = 13;
pub const R8A7740_CLK_MMC: c_int = 12;
pub const R8A7740_CLK_GETHER: c_int = 9;
pub const R8A7740_CLK_TPU0: c_int = 4;
// MSTP4
pub const R8A7740_CLK_USBH: c_int = 16;
pub const R8A7740_CLK_SDHI2: c_int = 15;
pub const R8A7740_CLK_USBFUNC: c_int = 7;
pub const R8A7740_CLK_USBPHY: c_int = 6;
// SUBCK*
pub const R8A7740_CLK_SUBCK: c_int = 9;
pub const R8A7740_CLK_SUBCK2: c_int = 10;

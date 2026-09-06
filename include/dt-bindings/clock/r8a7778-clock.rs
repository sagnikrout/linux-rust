//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r8a7778-clock.h
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
// Copyright (C) 2014 Ulrich Hecht
//
// CPG
pub const R8A7778_CLK_PLLA: c_int = 0;
pub const R8A7778_CLK_PLLB: c_int = 1;
pub const R8A7778_CLK_B: c_int = 2;
pub const R8A7778_CLK_OUT: c_int = 3;
pub const R8A7778_CLK_P: c_int = 4;
pub const R8A7778_CLK_S: c_int = 5;
pub const R8A7778_CLK_S1: c_int = 6;
// MSTP0
pub const R8A7778_CLK_I2C0: c_int = 30;
pub const R8A7778_CLK_I2C1: c_int = 29;
pub const R8A7778_CLK_I2C2: c_int = 28;
pub const R8A7778_CLK_I2C3: c_int = 27;
pub const R8A7778_CLK_SCIF0: c_int = 26;
pub const R8A7778_CLK_SCIF1: c_int = 25;
pub const R8A7778_CLK_SCIF2: c_int = 24;
pub const R8A7778_CLK_SCIF3: c_int = 23;
pub const R8A7778_CLK_SCIF4: c_int = 22;
pub const R8A7778_CLK_SCIF5: c_int = 21;
pub const R8A7778_CLK_HSCIF0: c_int = 19;
pub const R8A7778_CLK_HSCIF1: c_int = 18;
pub const R8A7778_CLK_TMU0: c_int = 16;
pub const R8A7778_CLK_TMU1: c_int = 15;
pub const R8A7778_CLK_TMU2: c_int = 14;
pub const R8A7778_CLK_SSI0: c_int = 12;
pub const R8A7778_CLK_SSI1: c_int = 11;
pub const R8A7778_CLK_SSI2: c_int = 10;
pub const R8A7778_CLK_SSI3: c_int = 9;
pub const R8A7778_CLK_SRU: c_int = 8;
pub const R8A7778_CLK_HSPI: c_int = 7;
// MSTP1
pub const R8A7778_CLK_ETHER: c_int = 14;
pub const R8A7778_CLK_VIN0: c_int = 10;
pub const R8A7778_CLK_VIN1: c_int = 9;
pub const R8A7778_CLK_USB: c_int = 0;
// MSTP3
pub const R8A7778_CLK_MMC: c_int = 31;
pub const R8A7778_CLK_SDHI0: c_int = 23;
pub const R8A7778_CLK_SDHI1: c_int = 22;
pub const R8A7778_CLK_SDHI2: c_int = 21;
pub const R8A7778_CLK_SSI4: c_int = 11;
pub const R8A7778_CLK_SSI5: c_int = 10;
pub const R8A7778_CLK_SSI6: c_int = 9;
pub const R8A7778_CLK_SSI7: c_int = 8;
pub const R8A7778_CLK_SSI8: c_int = 7;
// MSTP5
pub const R8A7778_CLK_SRU_SRC0: c_int = 31;
pub const R8A7778_CLK_SRU_SRC1: c_int = 30;
pub const R8A7778_CLK_SRU_SRC2: c_int = 29;
pub const R8A7778_CLK_SRU_SRC3: c_int = 28;
pub const R8A7778_CLK_SRU_SRC4: c_int = 27;
pub const R8A7778_CLK_SRU_SRC5: c_int = 26;
pub const R8A7778_CLK_SRU_SRC6: c_int = 25;
pub const R8A7778_CLK_SRU_SRC7: c_int = 24;
pub const R8A7778_CLK_SRU_SRC8: c_int = 23;

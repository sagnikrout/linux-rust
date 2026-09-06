//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/renesas,r9a09g047-cpg.h
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
// Copyright (C) 2024 Renesas Electronics Corp.
//

// Core Clock list
pub const R9A09G047_SYS_0_PCLK: c_int = 0;
pub const R9A09G047_CA55_0_CORECLK0: c_int = 1;
pub const R9A09G047_CA55_0_CORECLK1: c_int = 2;
pub const R9A09G047_CA55_0_CORECLK2: c_int = 3;
pub const R9A09G047_CA55_0_CORECLK3: c_int = 4;
pub const R9A09G047_CA55_0_PERIPHCLK: c_int = 5;
pub const R9A09G047_CM33_CLK0: c_int = 6;
pub const R9A09G047_CST_0_SWCLKTCK: c_int = 7;
pub const R9A09G047_IOTOP_0_SHCLK: c_int = 8;
pub const R9A09G047_SPI_CLK_SPI: c_int = 9;
pub const R9A09G047_GBETH_0_CLK_PTP_REF_I: c_int = 10;
pub const R9A09G047_GBETH_1_CLK_PTP_REF_I: c_int = 11;
pub const R9A09G047_USB3_0_REF_ALT_CLK_P: c_int = 12;
pub const R9A09G047_USB3_0_CLKCORE: c_int = 13;
pub const R9A09G047_USB2_0_CLK_CORE0: c_int = 14;
pub const R9A09G047_USB2_0_CLK_CORE1: c_int = 15;

//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sophgo,sg2044-pll.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (C) 2024 Inochi Amaoto <inochiama@gmail.com>
//
pub const CLK_FPLL0: c_int = 0;
pub const CLK_FPLL1: c_int = 1;
pub const CLK_FPLL2: c_int = 2;
pub const CLK_DPLL0: c_int = 3;
pub const CLK_DPLL1: c_int = 4;
pub const CLK_DPLL2: c_int = 5;
pub const CLK_DPLL3: c_int = 6;
pub const CLK_DPLL4: c_int = 7;
pub const CLK_DPLL5: c_int = 8;
pub const CLK_DPLL6: c_int = 9;
pub const CLK_DPLL7: c_int = 10;
pub const CLK_MPLL0: c_int = 11;
pub const CLK_MPLL1: c_int = 12;
pub const CLK_MPLL2: c_int = 13;
pub const CLK_MPLL3: c_int = 14;
pub const CLK_MPLL4: c_int = 15;
pub const CLK_MPLL5: c_int = 16;

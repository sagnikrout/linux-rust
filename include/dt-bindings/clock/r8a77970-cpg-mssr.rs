//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r8a77970-cpg-mssr.h
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
// Copyright (C) 2017 Cogent Embedded, Inc.
//

// r8a77970 CPG Core Clocks
pub const R8A77970_CLK_Z2: c_int = 0;
pub const R8A77970_CLK_ZR: c_int = 1;
pub const R8A77970_CLK_ZTR: c_int = 2;
pub const R8A77970_CLK_ZTRD2: c_int = 3;
pub const R8A77970_CLK_ZT: c_int = 4;
pub const R8A77970_CLK_ZX: c_int = 5;
pub const R8A77970_CLK_S1D1: c_int = 6;
pub const R8A77970_CLK_S1D2: c_int = 7;
pub const R8A77970_CLK_S1D4: c_int = 8;
pub const R8A77970_CLK_S2D1: c_int = 9;
pub const R8A77970_CLK_S2D2: c_int = 10;
pub const R8A77970_CLK_S2D4: c_int = 11;
pub const R8A77970_CLK_LB: c_int = 12;
pub const R8A77970_CLK_CL: c_int = 13;
pub const R8A77970_CLK_ZB3: c_int = 14;
pub const R8A77970_CLK_ZB3D2: c_int = 15;
pub const R8A77970_CLK_DDR: c_int = 16;
pub const R8A77970_CLK_CR: c_int = 17;
pub const R8A77970_CLK_CRD2: c_int = 18;
pub const R8A77970_CLK_SD0H: c_int = 19;
pub const R8A77970_CLK_SD0: c_int = 20;
pub const R8A77970_CLK_RPC: c_int = 21;
pub const R8A77970_CLK_RPCD2: c_int = 22;
pub const R8A77970_CLK_MSO: c_int = 23;
pub const R8A77970_CLK_CANFD: c_int = 24;
pub const R8A77970_CLK_CSI0: c_int = 25;
pub const R8A77970_CLK_FRAY: c_int = 26;
pub const R8A77970_CLK_CP: c_int = 27;
pub const R8A77970_CLK_CPEX: c_int = 28;
pub const R8A77970_CLK_R: c_int = 29;
pub const R8A77970_CLK_OSC: c_int = 30;

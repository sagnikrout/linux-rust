//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/ti-dra7-atl.h
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
// This header provides constants for DRA7 ATL (Audio Tracking Logic)
//
// The constants defined in this header are used in dts files
//
// Copyright (C) 2013 Texas Instruments, Inc.
//
// Peter Ujfalusi <peter.ujfalusi@ti.com>
//
pub const DRA7_ATL_WS_MCASP1_FSR: c_int = 0;
pub const DRA7_ATL_WS_MCASP1_FSX: c_int = 1;
pub const DRA7_ATL_WS_MCASP2_FSR: c_int = 2;
pub const DRA7_ATL_WS_MCASP2_FSX: c_int = 3;
pub const DRA7_ATL_WS_MCASP3_FSX: c_int = 4;
pub const DRA7_ATL_WS_MCASP4_FSX: c_int = 5;
pub const DRA7_ATL_WS_MCASP5_FSX: c_int = 6;
pub const DRA7_ATL_WS_MCASP6_FSX: c_int = 7;
pub const DRA7_ATL_WS_MCASP7_FSX: c_int = 8;
pub const DRA7_ATL_WS_MCASP8_FSX: c_int = 9;
pub const DRA7_ATL_WS_MCASP8_AHCLKX: c_int = 10;
pub const DRA7_ATL_WS_XREF_CLK3: c_int = 11;
pub const DRA7_ATL_WS_XREF_CLK0: c_int = 12;
pub const DRA7_ATL_WS_XREF_CLK1: c_int = 13;
pub const DRA7_ATL_WS_XREF_CLK2: c_int = 14;
pub const DRA7_ATL_WS_OSC1_X1: c_int = 15;

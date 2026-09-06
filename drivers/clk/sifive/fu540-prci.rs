//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sifive/fu540-prci.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018-2021 SiFive, Inc.
// Copyright (C) 2018-2019 Wesley Terpstra
// Copyright (C) 2018-2019 Paul Walmsley
// Copyright (C) 2020-2021 Zong Li
//
// The FU540 PRCI implements clock and reset control for the SiFive
// FU540-C000 chip.  This driver assumes that it has sole control
// over all PRCI resources.
//
// This driver is based on the PRCI driver written by Wesley Terpstra:
// https://github.com/riscv/riscv-linux/commit/999529edf517ed75b56659d456d221b2ee56bb60
//
// References:
// - SiFive FU540-C000 manual v1p0, Chapter 7 "Clocking and Reset"
//

// PRCI integration data for each WRPLL instance
// Linux clock framework integration
// List of clock controls provided by the PRCI

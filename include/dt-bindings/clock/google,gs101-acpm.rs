//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/google,gs101-acpm.h
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
// Copyright 2025 Linaro Ltd.
//
// Device Tree binding constants for Google gs101 ACPM clock controller.
//
pub const GS101_CLK_ACPM_DVFS_MIF: c_int = 0;
pub const GS101_CLK_ACPM_DVFS_INT: c_int = 1;
pub const GS101_CLK_ACPM_DVFS_CPUCL0: c_int = 2;
pub const GS101_CLK_ACPM_DVFS_CPUCL1: c_int = 3;
pub const GS101_CLK_ACPM_DVFS_CPUCL2: c_int = 4;
pub const GS101_CLK_ACPM_DVFS_G3D: c_int = 5;
pub const GS101_CLK_ACPM_DVFS_G3DL2: c_int = 6;
pub const GS101_CLK_ACPM_DVFS_TPU: c_int = 7;
pub const GS101_CLK_ACPM_DVFS_INTCAM: c_int = 8;
pub const GS101_CLK_ACPM_DVFS_TNR: c_int = 9;
pub const GS101_CLK_ACPM_DVFS_CAM: c_int = 10;
pub const GS101_CLK_ACPM_DVFS_MFC: c_int = 11;
pub const GS101_CLK_ACPM_DVFS_DISP: c_int = 12;
pub const GS101_CLK_ACPM_DVFS_BO: c_int = 13;

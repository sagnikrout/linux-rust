//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/tegra186-powergate.h
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
// Copyright (c) 2015-2016, NVIDIA CORPORATION.  All rights reserved.
//
pub const TEGRA186_POWER_DOMAIN_AUD: c_int = 0;
pub const TEGRA186_POWER_DOMAIN_DFD: c_int = 1;
pub const TEGRA186_POWER_DOMAIN_DISP: c_int = 2;
pub const TEGRA186_POWER_DOMAIN_DISPB: c_int = 3;
pub const TEGRA186_POWER_DOMAIN_DISPC: c_int = 4;
pub const TEGRA186_POWER_DOMAIN_ISPA: c_int = 5;
pub const TEGRA186_POWER_DOMAIN_NVDEC: c_int = 6;
pub const TEGRA186_POWER_DOMAIN_NVJPG: c_int = 7;
pub const TEGRA186_POWER_DOMAIN_MPE: c_int = 8;
pub const TEGRA186_POWER_DOMAIN_PCX: c_int = 9;
pub const TEGRA186_POWER_DOMAIN_SAX: c_int = 10;
pub const TEGRA186_POWER_DOMAIN_VE: c_int = 11;
pub const TEGRA186_POWER_DOMAIN_VIC: c_int = 12;
pub const TEGRA186_POWER_DOMAIN_XUSBA: c_int = 13;
pub const TEGRA186_POWER_DOMAIN_XUSBB: c_int = 14;
pub const TEGRA186_POWER_DOMAIN_XUSBC: c_int = 15;
pub const TEGRA186_POWER_DOMAIN_GPU: c_int = 43;
pub const TEGRA186_POWER_DOMAIN_MAX: c_int = 44;

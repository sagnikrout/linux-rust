//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/nvidia,tegra264-bpmp.h
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
// Copyright (c) 2022-2024, NVIDIA CORPORATION.  All rights reserved.
pub const TEGRA264_POWER_DOMAIN_DISP: c_int = 1;
pub const TEGRA264_POWER_DOMAIN_AUD: c_int = 2;
// reserved 3:9
pub const TEGRA264_POWER_DOMAIN_XUSB_SS: c_int = 10;
pub const TEGRA264_POWER_DOMAIN_XUSB_DEV: c_int = 11;
pub const TEGRA264_POWER_DOMAIN_XUSB_HOST: c_int = 12;
pub const TEGRA264_POWER_DOMAIN_MGBE0: c_int = 13;
pub const TEGRA264_POWER_DOMAIN_MGBE1: c_int = 14;
pub const TEGRA264_POWER_DOMAIN_MGBE2: c_int = 15;
pub const TEGRA264_POWER_DOMAIN_MGBE3: c_int = 16;
pub const TEGRA264_POWER_DOMAIN_VI: c_int = 17;
pub const TEGRA264_POWER_DOMAIN_VIC: c_int = 18;
pub const TEGRA264_POWER_DOMAIN_ISP0: c_int = 19;
pub const TEGRA264_POWER_DOMAIN_ISP1: c_int = 20;
pub const TEGRA264_POWER_DOMAIN_PVA0: c_int = 21;
pub const TEGRA264_POWER_DOMAIN_GPU: c_int = 22;

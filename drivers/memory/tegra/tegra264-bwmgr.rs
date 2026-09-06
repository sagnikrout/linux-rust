//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/tegra/tegra264-bwmgr.h
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
// Copyright (C) 2025 NVIDIA CORPORATION.  All rights reserved.
pub const TEGRA264_BWMGR_ICC_PRIMARY: c_int = 1;
pub const TEGRA264_BWMGR_DEBUG: c_int = 2;
pub const TEGRA264_BWMGR_CPU_CLUSTER0: c_int = 3;
pub const TEGRA264_BWMGR_CPU_CLUSTER1: c_int = 4;
pub const TEGRA264_BWMGR_CPU_CLUSTER2: c_int = 5;
pub const TEGRA264_BWMGR_CPU_CLUSTER3: c_int = 6;
pub const TEGRA264_BWMGR_CPU_CLUSTER4: c_int = 7;
pub const TEGRA264_BWMGR_CPU_CLUSTER5: c_int = 8;
pub const TEGRA264_BWMGR_CPU_CLUSTER6: c_int = 9;
pub const TEGRA264_BWMGR_CACTMON: c_int = 10;
pub const TEGRA264_BWMGR_DISPLAY: c_int = 11;
pub const TEGRA264_BWMGR_VI: c_int = 12;
pub const TEGRA264_BWMGR_APE: c_int = 13;
pub const TEGRA264_BWMGR_VIFAL: c_int = 14;
pub const TEGRA264_BWMGR_GPU: c_int = 15;
pub const TEGRA264_BWMGR_EQOS: c_int = 16;
pub const TEGRA264_BWMGR_PCIE_0: c_int = 17;
pub const TEGRA264_BWMGR_PCIE_1: c_int = 18;
pub const TEGRA264_BWMGR_PCIE_2: c_int = 19;
pub const TEGRA264_BWMGR_PCIE_3: c_int = 20;
pub const TEGRA264_BWMGR_PCIE_4: c_int = 21;
pub const TEGRA264_BWMGR_PCIE_5: c_int = 22;
pub const TEGRA264_BWMGR_SDMMC_1: c_int = 23;
pub const TEGRA264_BWMGR_SDMMC_2: c_int = 24;
pub const TEGRA264_BWMGR_NVDEC: c_int = 25;
pub const TEGRA264_BWMGR_NVENC: c_int = 26;
pub const TEGRA264_BWMGR_NVJPG_0: c_int = 27;
pub const TEGRA264_BWMGR_NVJPG_1: c_int = 28;
pub const TEGRA264_BWMGR_OFAA: c_int = 29;
pub const TEGRA264_BWMGR_XUSB_HOST: c_int = 30;
pub const TEGRA264_BWMGR_XUSB_DEV: c_int = 31;
pub const TEGRA264_BWMGR_TSEC: c_int = 32;
pub const TEGRA264_BWMGR_VIC: c_int = 33;
pub const TEGRA264_BWMGR_APEDMA: c_int = 34;
pub const TEGRA264_BWMGR_SE: c_int = 35;
pub const TEGRA264_BWMGR_ISP: c_int = 36;
pub const TEGRA264_BWMGR_HDA: c_int = 37;
pub const TEGRA264_BWMGR_VI2FAL: c_int = 38;
pub const TEGRA264_BWMGR_VI2: c_int = 39;
pub const TEGRA264_BWMGR_RCE: c_int = 40;
pub const TEGRA264_BWMGR_PVA: c_int = 41;
pub const TEGRA264_BWMGR_NVPMODEL: c_int = 42;

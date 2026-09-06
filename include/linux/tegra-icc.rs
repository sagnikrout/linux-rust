//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tegra-icc.h
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
// Copyright (C) 2022-2023 NVIDIA CORPORATION.  All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_icc_client_type {
    TEGRA_ICC_NONE,
    TEGRA_ICC_NISO,
    TEGRA_ICC_ISO_DISPLAY,
    TEGRA_ICC_ISO_VI,
    TEGRA_ICC_ISO_AUDIO,
    TEGRA_ICC_ISO_VIFAL,
}

// ICC ID's for MC client's used in BPMP
pub const TEGRA_ICC_BPMP_DEBUG: c_int = 1;
pub const TEGRA_ICC_BPMP_CPU_CLUSTER0: c_int = 2;
pub const TEGRA_ICC_BPMP_CPU_CLUSTER1: c_int = 3;
pub const TEGRA_ICC_BPMP_CPU_CLUSTER2: c_int = 4;
pub const TEGRA_ICC_BPMP_GPU: c_int = 5;
pub const TEGRA_ICC_BPMP_CACTMON: c_int = 6;
pub const TEGRA_ICC_BPMP_DISPLAY: c_int = 7;
pub const TEGRA_ICC_BPMP_VI: c_int = 8;
pub const TEGRA_ICC_BPMP_EQOS: c_int = 9;
pub const TEGRA_ICC_BPMP_PCIE_0: c_int = 10;
pub const TEGRA_ICC_BPMP_PCIE_1: c_int = 11;
pub const TEGRA_ICC_BPMP_PCIE_2: c_int = 12;
pub const TEGRA_ICC_BPMP_PCIE_3: c_int = 13;
pub const TEGRA_ICC_BPMP_PCIE_4: c_int = 14;
pub const TEGRA_ICC_BPMP_PCIE_5: c_int = 15;
pub const TEGRA_ICC_BPMP_PCIE_6: c_int = 16;
pub const TEGRA_ICC_BPMP_PCIE_7: c_int = 17;
pub const TEGRA_ICC_BPMP_PCIE_8: c_int = 18;
pub const TEGRA_ICC_BPMP_PCIE_9: c_int = 19;
pub const TEGRA_ICC_BPMP_PCIE_10: c_int = 20;
pub const TEGRA_ICC_BPMP_DLA_0: c_int = 21;
pub const TEGRA_ICC_BPMP_DLA_1: c_int = 22;
pub const TEGRA_ICC_BPMP_SDMMC_1: c_int = 23;
pub const TEGRA_ICC_BPMP_SDMMC_2: c_int = 24;
pub const TEGRA_ICC_BPMP_SDMMC_3: c_int = 25;
pub const TEGRA_ICC_BPMP_SDMMC_4: c_int = 26;
pub const TEGRA_ICC_BPMP_NVDEC: c_int = 27;
pub const TEGRA_ICC_BPMP_NVENC: c_int = 28;
pub const TEGRA_ICC_BPMP_NVJPG_0: c_int = 29;
pub const TEGRA_ICC_BPMP_NVJPG_1: c_int = 30;
pub const TEGRA_ICC_BPMP_OFAA: c_int = 31;
pub const TEGRA_ICC_BPMP_XUSB_HOST: c_int = 32;
pub const TEGRA_ICC_BPMP_XUSB_DEV: c_int = 33;
pub const TEGRA_ICC_BPMP_TSEC: c_int = 34;
pub const TEGRA_ICC_BPMP_VIC: c_int = 35;
pub const TEGRA_ICC_BPMP_APE: c_int = 36;
pub const TEGRA_ICC_BPMP_APEDMA: c_int = 37;
pub const TEGRA_ICC_BPMP_SE: c_int = 38;
pub const TEGRA_ICC_BPMP_ISP: c_int = 39;
pub const TEGRA_ICC_BPMP_HDA: c_int = 40;
pub const TEGRA_ICC_BPMP_VIFAL: c_int = 41;
pub const TEGRA_ICC_BPMP_VI2FAL: c_int = 42;
pub const TEGRA_ICC_BPMP_VI2: c_int = 43;
pub const TEGRA_ICC_BPMP_RCE: c_int = 44;
pub const TEGRA_ICC_BPMP_PVA: c_int = 45;

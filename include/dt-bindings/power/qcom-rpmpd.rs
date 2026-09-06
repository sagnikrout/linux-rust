//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/qcom-rpmpd.h
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.

// Generic RPM Power Domain Indexes
pub const RPMPD_VDDCX: c_int = 0;
pub const RPMPD_VDDCX_AO: c_int = 1;
// VFC and VFL are mutually exclusive and can not be present on the same platform
pub const RPMPD_VDDCX_VFC: c_int = 2;
pub const RPMPD_VDDCX_VFL: c_int = 2;
pub const RPMPD_VDDMX: c_int = 3;
pub const RPMPD_VDDMX_AO: c_int = 4;
pub const RPMPD_VDDMX_VFL: c_int = 5;
pub const RPMPD_SSCCX: c_int = 6;
pub const RPMPD_SSCCX_VFL: c_int = 7;
pub const RPMPD_SSCMX: c_int = 8;
pub const RPMPD_SSCMX_VFL: c_int = 9;
//
// Platform-specific power domain bindings. Don't add new entries here, use
// RPMPD_* above.
//
// MDM9607 Power Domains

// MSM8226 Power Domain Indexes

// MSM8939 Power Domains
pub const MSM8939_VDDMDCX: c_int = 0;
pub const MSM8939_VDDMDCX_AO: c_int = 1;
pub const MSM8939_VDDMDCX_VFC: c_int = 2;
pub const MSM8939_VDDCX: c_int = 3;
pub const MSM8939_VDDCX_AO: c_int = 4;
pub const MSM8939_VDDCX_VFC: c_int = 5;
pub const MSM8939_VDDMX: c_int = 6;
pub const MSM8939_VDDMX_AO: c_int = 7;
// MSM8916 Power Domain Indexes

// MSM8909 Power Domain Indexes

// MSM8917 Power Domain Indexes

// MSM8937 Power Domain Indexes

// QM215 Power Domain Indexes

// MSM8953 Power Domain Indexes
pub const MSM8953_VDDMD: c_int = 0;
pub const MSM8953_VDDMD_AO: c_int = 1;
pub const MSM8953_VDDCX: c_int = 2;
pub const MSM8953_VDDCX_AO: c_int = 3;
pub const MSM8953_VDDCX_VFL: c_int = 4;
pub const MSM8953_VDDMX: c_int = 5;
pub const MSM8953_VDDMX_AO: c_int = 6;
// MSM8974 Power Domain Indexes
pub const MSM8974_VDDCX: c_int = 0;
pub const MSM8974_VDDCX_AO: c_int = 1;
pub const MSM8974_VDDCX_VFC: c_int = 2;
pub const MSM8974_VDDGFX: c_int = 3;
pub const MSM8974_VDDGFX_VFC: c_int = 4;
// MSM8976 Power Domain Indexes

// MSM8994 Power Domain Indexes
pub const MSM8994_VDDCX: c_int = 0;
pub const MSM8994_VDDCX_AO: c_int = 1;
pub const MSM8994_VDDCX_VFC: c_int = 2;
pub const MSM8994_VDDMX: c_int = 3;
pub const MSM8994_VDDMX_AO: c_int = 4;
pub const MSM8994_VDDGFX: c_int = 5;
pub const MSM8994_VDDGFX_VFC: c_int = 6;
// MSM8996 Power Domain Indexes
pub const MSM8996_VDDCX: c_int = 0;
pub const MSM8996_VDDCX_AO: c_int = 1;
pub const MSM8996_VDDCX_VFC: c_int = 2;
pub const MSM8996_VDDMX: c_int = 3;
pub const MSM8996_VDDMX_AO: c_int = 4;
pub const MSM8996_VDDSSCX: c_int = 5;
pub const MSM8996_VDDSSCX_VFC: c_int = 6;
// MSM8998 Power Domain Indexes

// QCM2290 Power Domains
pub const QCM2290_VDDCX: c_int = 0;
pub const QCM2290_VDDCX_AO: c_int = 1;
pub const QCM2290_VDDCX_VFL: c_int = 2;
pub const QCM2290_VDDMX: c_int = 3;
pub const QCM2290_VDDMX_AO: c_int = 4;
pub const QCM2290_VDDMX_VFL: c_int = 5;
pub const QCM2290_VDD_LPI_CX: c_int = 6;
pub const QCM2290_VDD_LPI_MX: c_int = 7;
// QCS404 Power Domains
pub const QCS404_VDDMX: c_int = 0;
pub const QCS404_VDDMX_AO: c_int = 1;
pub const QCS404_VDDMX_VFL: c_int = 2;
pub const QCS404_LPICX: c_int = 3;
pub const QCS404_LPICX_VFL: c_int = 4;
pub const QCS404_LPIMX: c_int = 5;
pub const QCS404_LPIMX_VFL: c_int = 6;
// SDM660 Power Domains

// SM6115 Power Domains
pub const SM6115_VDDCX: c_int = 0;
pub const SM6115_VDDCX_AO: c_int = 1;
pub const SM6115_VDDCX_VFL: c_int = 2;
pub const SM6115_VDDMX: c_int = 3;
pub const SM6115_VDDMX_AO: c_int = 4;
pub const SM6115_VDDMX_VFL: c_int = 5;
pub const SM6115_VDD_LPI_CX: c_int = 6;
pub const SM6115_VDD_LPI_MX: c_int = 7;
// SM6125 Power Domains

// SM6375 Power Domain Indexes
pub const SM6375_VDDCX: c_int = 0;
pub const SM6375_VDDCX_AO: c_int = 1;
pub const SM6375_VDDCX_VFL: c_int = 2;
pub const SM6375_VDDMX: c_int = 3;
pub const SM6375_VDDMX_AO: c_int = 4;
pub const SM6375_VDDMX_VFL: c_int = 5;
pub const SM6375_VDDGX: c_int = 6;
pub const SM6375_VDDGX_AO: c_int = 7;
pub const SM6375_VDD_LPI_CX: c_int = 8;
pub const SM6375_VDD_LPI_MX: c_int = 9;
// RPM SMD Power Domain performance levels
pub const RPM_SMD_LEVEL_RETENTION: c_int = 16;
pub const RPM_SMD_LEVEL_RETENTION_PLUS: c_int = 32;
pub const RPM_SMD_LEVEL_MIN_SVS: c_int = 48;
pub const RPM_SMD_LEVEL_LOW_SVS: c_int = 64;
pub const RPM_SMD_LEVEL_SVS: c_int = 128;
pub const RPM_SMD_LEVEL_SVS_PLUS: c_int = 192;
pub const RPM_SMD_LEVEL_NOM: c_int = 256;
pub const RPM_SMD_LEVEL_NOM_PLUS: c_int = 320;
pub const RPM_SMD_LEVEL_TURBO: c_int = 384;
pub const RPM_SMD_LEVEL_TURBO_NO_CPR: c_int = 416;
pub const RPM_SMD_LEVEL_TURBO_HIGH: c_int = 448;
pub const RPM_SMD_LEVEL_BINNING: c_int = 512;

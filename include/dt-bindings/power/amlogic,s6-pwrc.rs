//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/amlogic,s6-pwrc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2025 Amlogic, Inc. All rights reserved
//
pub const PWRC_S6_DSPA_ID: c_int = 0;
pub const PWRC_S6_DOS_HEVC_ID: c_int = 1;
pub const PWRC_S6_DOS_VDEC_ID: c_int = 2;
pub const PWRC_S6_VPU_HDMI_ID: c_int = 3;
pub const PWRC_S6_U2DRD_ID: c_int = 4;
pub const PWRC_S6_U3DRD_ID: c_int = 5;
pub const PWRC_S6_SD_EMMC_C_ID: c_int = 6;
pub const PWRC_S6_GE2D_ID: c_int = 7;
pub const PWRC_S6_AMFC_ID: c_int = 8;
pub const PWRC_S6_VC9000E_ID: c_int = 9;
pub const PWRC_S6_DEWARP_ID: c_int = 10;
pub const PWRC_S6_VICP_ID: c_int = 11;
pub const PWRC_S6_SD_EMMC_A_ID: c_int = 12;
pub const PWRC_S6_SD_EMMC_B_ID: c_int = 13;
pub const PWRC_S6_ETH_ID: c_int = 14;
pub const PWRC_S6_PCIE_ID: c_int = 15;
pub const PWRC_S6_NNA_4T_ID: c_int = 16;
pub const PWRC_S6_AUDIO_ID: c_int = 17;
pub const PWRC_S6_AUCPU_ID: c_int = 18;
pub const PWRC_S6_ADAPT_ID: c_int = 19;

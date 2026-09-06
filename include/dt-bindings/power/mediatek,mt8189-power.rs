//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/mediatek,mt8189-power.h
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
// Copyright (c) 2025 MediaTek Inc.
// Author: Qiqi Wang <qiqi.wang@mediatek.com>
//
// SPM
pub const MT8189_POWER_DOMAIN_CONN: c_int = 0;
pub const MT8189_POWER_DOMAIN_AUDIO: c_int = 1;
pub const MT8189_POWER_DOMAIN_ADSP_TOP_DORMANT: c_int = 2;
pub const MT8189_POWER_DOMAIN_ADSP_INFRA: c_int = 3;
pub const MT8189_POWER_DOMAIN_ADSP_AO: c_int = 4;
pub const MT8189_POWER_DOMAIN_MM_INFRA: c_int = 5;
pub const MT8189_POWER_DOMAIN_ISP_IMG1: c_int = 6;
pub const MT8189_POWER_DOMAIN_ISP_IMG2: c_int = 7;
pub const MT8189_POWER_DOMAIN_ISP_IPE: c_int = 8;
pub const MT8189_POWER_DOMAIN_VDE0: c_int = 9;
pub const MT8189_POWER_DOMAIN_VEN0: c_int = 10;
pub const MT8189_POWER_DOMAIN_CAM_MAIN: c_int = 11;
pub const MT8189_POWER_DOMAIN_CAM_SUBA: c_int = 12;
pub const MT8189_POWER_DOMAIN_CAM_SUBB: c_int = 13;
pub const MT8189_POWER_DOMAIN_MDP0: c_int = 14;
pub const MT8189_POWER_DOMAIN_DISP: c_int = 15;
pub const MT8189_POWER_DOMAIN_DP_TX: c_int = 16;
pub const MT8189_POWER_DOMAIN_CSI_RX: c_int = 17;
pub const MT8189_POWER_DOMAIN_SSUSB: c_int = 18;
pub const MT8189_POWER_DOMAIN_MFG0: c_int = 19;
pub const MT8189_POWER_DOMAIN_MFG1: c_int = 20;
pub const MT8189_POWER_DOMAIN_MFG2: c_int = 21;
pub const MT8189_POWER_DOMAIN_MFG3: c_int = 22;
pub const MT8189_POWER_DOMAIN_EDP_TX_DORMANT: c_int = 23;
pub const MT8189_POWER_DOMAIN_PCIE: c_int = 24;
pub const MT8189_POWER_DOMAIN_PCIE_PHY: c_int = 25;

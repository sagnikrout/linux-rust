//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-ufs-v7.h
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
// Copyright (c) 2026, The Linux Foundation. All rights reserved.
//
// Only for QMP V7 PHY - UFS PCS registers
pub const QPHY_V7_PCS_UFS_PHY_START: c_uint = 0x000;
pub const QPHY_V7_PCS_UFS_POWER_DOWN_CONTROL: c_uint = 0x004;
pub const QPHY_V7_PCS_UFS_SW_RESET: c_uint = 0x008;
pub const QPHY_V7_PCS_UFS_PCS_CTRL1: c_uint = 0x01c;
pub const QPHY_V7_PCS_UFS_PLL_CNTL: c_uint = 0x028;
pub const QPHY_V7_PCS_UFS_TX_LARGE_AMP_DRV_LVL: c_uint = 0x02c;
pub const QPHY_V7_PCS_UFS_TX_HSGEAR_CAPABILITY: c_uint = 0x060;
pub const QPHY_V7_PCS_UFS_RX_HSGEAR_CAPABILITY: c_uint = 0x094;
pub const QPHY_V7_PCS_UFS_LINECFG_DISABLE: c_uint = 0x140;
pub const QPHY_V7_PCS_UFS_RX_SIGDET_CTRL2: c_uint = 0x150;
pub const QPHY_V7_PCS_UFS_READY_STATUS: c_uint = 0x16c;
pub const QPHY_V7_PCS_UFS_TX_MID_TERM_CTRL1: c_uint = 0x1b8;
pub const QPHY_V7_PCS_UFS_MULTI_LANE_CTRL1: c_uint = 0x1c0;

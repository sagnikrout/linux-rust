//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-ufs-v3.h
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
// Copyright (c) 2017, The Linux Foundation. All rights reserved.
//
pub const QPHY_V3_PCS_UFS_PHY_START: c_uint = 0x000;
pub const QPHY_V3_PCS_UFS_POWER_DOWN_CONTROL: c_uint = 0x004;
pub const QPHY_V3_PCS_UFS_TX_LARGE_AMP_DRV_LVL: c_uint = 0x02c;
pub const QPHY_V3_PCS_UFS_TX_SMALL_AMP_DRV_LVL: c_uint = 0x034;
pub const QPHY_V3_PCS_UFS_RX_SYM_RESYNC_CTRL: c_uint = 0x134;
pub const QPHY_V3_PCS_UFS_RX_MIN_HIBERN8_TIME: c_uint = 0x138;
pub const QPHY_V3_PCS_UFS_RX_SIGDET_CTRL1: c_uint = 0x13c;
pub const QPHY_V3_PCS_UFS_RX_SIGDET_CTRL2: c_uint = 0x140;
pub const QPHY_V3_PCS_UFS_READY_STATUS: c_uint = 0x160;
pub const QPHY_V3_PCS_UFS_TX_MID_TERM_CTRL1: c_uint = 0x1bc;
pub const QPHY_V3_PCS_UFS_MULTI_LANE_CTRL1: c_uint = 0x1c4;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-dp-phy-v8.h
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
// Only for QMP V8 PHY - DP PHY registers
pub const QSERDES_V8_DP_PHY_VCO_DIV: c_uint = 0x070;
pub const QSERDES_V8_DP_PHY_AUX_INTERRUPT_STATUS: c_uint = 0x0e0;
pub const QSERDES_V8_DP_PHY_TSYNC_OVRD: c_uint = 0x074;
pub const QSERDES_V8_DP_PHY_TX0_TX1_LANE_CTL: c_uint = 0x078;
pub const QSERDES_V8_DP_PHY_TX2_TX3_LANE_CTL: c_uint = 0x0bc;
pub const QSERDES_V8_DP_PHY_AUXLESS_CFG1: c_uint = 0x0c8;
pub const QSERDES_V8_DP_PHY_LFPS_PERIOD: c_uint = 0x0d0;
pub const QSERDES_V8_DP_PHY_LFPS_CYC: c_uint = 0x0d4;
pub const QSERDES_V8_DP_PHY_AUXLESS_SETUP_CYC: c_uint = 0x0d8;
pub const QSERDES_V8_DP_PHY_AUXLESS_SILENCE_CYC: c_uint = 0x0d8;
pub const QSERDES_V8_DP_PHY_LN0_DRV_LVL: c_uint = 0x0e0;
pub const QSERDES_V8_DP_PHY_LN1_DRV_LVL: c_uint = 0x0e4;
pub const QSERDES_V8_DP_PHY_STATUS: c_uint = 0x114;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-dp-phy.h
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
// QMP PHY - DP PHY registers
pub const QSERDES_DP_PHY_REVISION_ID0: c_uint = 0x000;
pub const QSERDES_DP_PHY_REVISION_ID1: c_uint = 0x004;
pub const QSERDES_DP_PHY_REVISION_ID2: c_uint = 0x008;
pub const QSERDES_DP_PHY_REVISION_ID3: c_uint = 0x00c;
pub const QSERDES_DP_PHY_CFG: c_uint = 0x010;
pub const QSERDES_DP_PHY_CFG_1: c_uint = 0x014;
pub const QSERDES_DP_PHY_PD_CTL: c_uint = 0x018;
pub const QSERDES_DP_PHY_MODE: c_uint = 0x01c;
pub const QSERDES_DP_PHY_AUX_CFG0: c_uint = 0x020;
pub const QSERDES_DP_PHY_AUX_CFG1: c_uint = 0x024;
pub const QSERDES_DP_PHY_AUX_CFG2: c_uint = 0x028;
pub const QSERDES_DP_PHY_AUX_CFG3: c_uint = 0x02c;
pub const QSERDES_DP_PHY_AUX_CFG4: c_uint = 0x030;
pub const QSERDES_DP_PHY_AUX_CFG5: c_uint = 0x034;
pub const QSERDES_DP_PHY_AUX_CFG6: c_uint = 0x038;
pub const QSERDES_DP_PHY_AUX_CFG7: c_uint = 0x03c;
pub const QSERDES_DP_PHY_AUX_CFG8: c_uint = 0x040;
pub const QSERDES_DP_PHY_AUX_CFG9: c_uint = 0x044;
// QSERDES COM_BIAS_EN_CLKBUFLR_EN bits

// QPHY_TX_TX_EMP_POST1_LVL bits

// QPHY_TX_TX_DRV_LVL bits

// QSERDES_DP_PHY_PD_CTL bits

// QPHY_DP_PHY_AUX_INTERRUPT_STATUS bits


//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-pcie-v5.h
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


// Only for QMP V5 PHY - PCS_PCIE registers
// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2017, The Linux Foundation. All rights reserved.
//
// Only for QMP V5 PHY - PCS_PCIE registers
pub const QPHY_V5_PCS_PCIE_POWER_STATE_CONFIG2: c_uint = 0x0c;
pub const QPHY_V5_PCS_PCIE_POWER_STATE_CONFIG4: c_uint = 0x14;
pub const QPHY_V5_PCS_PCIE_ENDPOINT_REFCLK_DRIVE: c_uint = 0x20;
pub const QPHY_V5_PCS_PCIE_L1P1_WAKEUP_DLY_TIME_AUXCLK_L: c_uint = 0x44;
pub const QPHY_V5_PCS_PCIE_L1P1_WAKEUP_DLY_TIME_AUXCLK_H: c_uint = 0x48;
pub const QPHY_V5_PCS_PCIE_L1P2_WAKEUP_DLY_TIME_AUXCLK_L: c_uint = 0x4c;
pub const QPHY_V5_PCS_PCIE_L1P2_WAKEUP_DLY_TIME_AUXCLK_H: c_uint = 0x50;
pub const QPHY_V5_PCS_PCIE_INT_AUX_CLK_CONFIG1: c_uint = 0x54;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_CONFIG1: c_uint = 0x5c;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_CONFIG2: c_uint = 0x60;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_CONFIG4: c_uint = 0x68;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_MODE2_CONFIG2: c_uint = 0x7c;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_MODE2_CONFIG4: c_uint = 0x84;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_MODE2_CONFIG5: c_uint = 0x88;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_MODE2_CONFIG6: c_uint = 0x8c;
pub const QPHY_V5_PCS_PCIE_OSC_DTCT_ACTIONS: c_uint = 0x94;
pub const QPHY_V5_PCS_PCIE_EQ_CONFIG1: c_uint = 0xa4;
pub const QPHY_V5_PCS_PCIE_EQ_CONFIG2: c_uint = 0xa8;
pub const QPHY_V5_PCS_PCIE_PRESET_P10_PRE: c_uint = 0xc0;
pub const QPHY_V5_PCS_PCIE_PRESET_P10_POST: c_uint = 0xe4;

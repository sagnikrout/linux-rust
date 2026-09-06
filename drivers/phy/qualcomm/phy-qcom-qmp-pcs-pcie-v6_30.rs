//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-pcie-v6_30.h
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
// Copyright (c) 2024 Qualcomm Innovation Center. All rights reserved.
//
// Only for QMP V6_30 PHY - PCIE have different offsets than V6
pub const QPHY_PCIE_V6_30_PCS_POWER_STATE_CONFIG2: c_uint = 0x014;
pub const QPHY_PCIE_V6_30_PCS_TX_RX_CONFIG: c_uint = 0x020;
pub const QPHY_PCIE_V6_30_PCS_ENDPOINT_REFCLK_DRIVE: c_uint = 0x024;
pub const QPHY_PCIE_V6_30_PCS_OSC_DTCT_ACTIONS: c_uint = 0x098;
pub const QPHY_PCIE_V6_30_PCS_EQ_CONFIG1: c_uint = 0x0a8;
pub const QPHY_PCIE_V6_30_PCS_G3_RXEQEVAL_TIME: c_uint = 0x0f8;
pub const QPHY_PCIE_V6_30_PCS_G4_RXEQEVAL_TIME: c_uint = 0x0fc;
pub const QPHY_PCIE_V6_30_PCS_G4_EQ_CONFIG5: c_uint = 0x110;
pub const QPHY_PCIE_V6_30_PCS_G4_PRE_GAIN: c_uint = 0x164;
pub const QPHY_PCIE_V6_30_PCS_RX_MARGINING_CONFIG1: c_uint = 0x184;
pub const QPHY_PCIE_V6_30_PCS_RX_MARGINING_CONFIG3: c_uint = 0x18c;
pub const QPHY_PCIE_V6_30_PCS_RX_MARGINING_CONFIG5: c_uint = 0x194;
pub const QPHY_PCIE_V6_30_PCS_G3_FOM_EQ_CONFIG5: c_uint = 0x1b4;
pub const QPHY_PCIE_V6_30_PCS_G4_FOM_EQ_CONFIG5: c_uint = 0x1c8;

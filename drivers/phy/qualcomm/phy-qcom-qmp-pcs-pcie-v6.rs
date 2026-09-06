//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-pcie-v6.h
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
// Copyright (c) 2023, Linaro Limited
//
// Only for QMP V6 PHY - PCIE have different offsets than V5
pub const QPHY_PCIE_V6_PCS_PCIE_EQ_CONFIG1: c_uint = 0xa4;
pub const QPHY_PCIE_V6_PCS_PCIE_RXEQEVAL_TIME: c_uint = 0xf4;
pub const QPHY_PCIE_V6_PCS_PCIE_POWER_STATE_CONFIG2: c_uint = 0x0c;
pub const QPHY_PCIE_V6_PCS_PCIE_POWER_STATE_CONFIG4: c_uint = 0x14;
pub const QPHY_PCIE_V6_PCS_PCIE_ENDPOINT_REFCLK_DRIVE: c_uint = 0x20;
pub const QPHY_PCIE_V6_PCS_PCIE_INT_AUX_CLK_CONFIG1: c_uint = 0x54;
pub const QPHY_PCIE_V6_PCS_PCIE_OSC_DTCT_ACTIONS: c_uint = 0x94;
pub const QPHY_PCIE_V6_PCS_LANE1_INSIG_SW_CTRL2: c_uint = 0x024;
pub const QPHY_PCIE_V6_PCS_LANE1_INSIG_MX_CTRL2: c_uint = 0x028;

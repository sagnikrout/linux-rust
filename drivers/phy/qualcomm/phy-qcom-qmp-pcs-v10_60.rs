//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-v10_60.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// Only for QMP V10_60 PHY - PCIe PCS registers
pub const QPHY_V10_60_PCS_SW_RESET: c_uint = 0x000;
pub const QPHY_V10_60_PCS_PCS_STATUS1: c_uint = 0x014;
pub const QPHY_V10_60_PCS_POWER_DOWN_CONTROL: c_uint = 0x040;
pub const QPHY_V10_60_PCS_START_CONTROL: c_uint = 0x044;
pub const QPHY_V10_60_PCS_G12S1_TXDEEMPH_M6DB: c_uint = 0x170;
pub const QPHY_V10_60_PCS_G3S2_PRE_GAIN: c_uint = 0x178;
pub const QPHY_V10_60_PCS_RX_SIGDET_LVL: c_uint = 0x190;
pub const QPHY_V10_60_PCS_ELECIDLE_DLY_SEL: c_uint = 0x1b8;
pub const QPHY_V10_60_PCS_PCS_TX_RX_CONFIG1: c_uint = 0x1dc;
pub const QPHY_V10_60_PCS_PCS_TX_RX_CONFIG2: c_uint = 0x1e0;
pub const QPHY_V10_60_PCS_EQ_CONFIG4: c_uint = 0x1f8;
pub const QPHY_V10_60_PCS_EQ_CONFIG5: c_uint = 0x1fc;

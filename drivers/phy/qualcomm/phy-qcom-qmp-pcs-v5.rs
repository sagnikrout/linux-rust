//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-v5.h
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
// Only for QMP V5 PHY - USB/PCIe PCS registers
pub const QPHY_V5_PCS_SW_RESET: c_uint = 0x000;
pub const QPHY_V5_PCS_PCS_STATUS1: c_uint = 0x014;
pub const QPHY_V5_PCS_POWER_DOWN_CONTROL: c_uint = 0x040;
pub const QPHY_V5_PCS_START_CONTROL: c_uint = 0x044;
pub const QPHY_V5_PCS_LOCK_DETECT_CONFIG1: c_uint = 0x0c4;
pub const QPHY_V5_PCS_LOCK_DETECT_CONFIG2: c_uint = 0x0c8;
pub const QPHY_V5_PCS_LOCK_DETECT_CONFIG3: c_uint = 0x0cc;
pub const QPHY_V5_PCS_LOCK_DETECT_CONFIG6: c_uint = 0x0d8;
pub const QPHY_V5_PCS_REFGEN_REQ_CONFIG1: c_uint = 0x0dc;
pub const QPHY_V5_PCS_G3S2_PRE_GAIN: c_uint = 0x170;
pub const QPHY_V5_PCS_RX_SIGDET_LVL: c_uint = 0x188;
pub const QPHY_V5_PCS_RCVR_DTCT_DLY_P1U2_L: c_uint = 0x190;
pub const QPHY_V5_PCS_RCVR_DTCT_DLY_P1U2_H: c_uint = 0x194;
pub const QPHY_V5_PCS_RATE_SLEW_CNTRL1: c_uint = 0x198;
pub const QPHY_V5_PCS_CDR_RESET_TIME: c_uint = 0x1b0;
pub const QPHY_V5_PCS_RX_CONFIG: c_uint = 0x1b0;
pub const QPHY_V5_PCS_ALIGN_DETECT_CONFIG1: c_uint = 0x1c0;
pub const QPHY_V5_PCS_ALIGN_DETECT_CONFIG2: c_uint = 0x1c4;
pub const QPHY_V5_PCS_PCS_TX_RX_CONFIG: c_uint = 0x1d0;
pub const QPHY_V5_PCS_EQ_CONFIG1: c_uint = 0x1dc;
pub const QPHY_V5_PCS_EQ_CONFIG2: c_uint = 0x1e0;
pub const QPHY_V5_PCS_EQ_CONFIG3: c_uint = 0x1e4;
pub const QPHY_V5_PCS_EQ_CONFIG5: c_uint = 0x1ec;

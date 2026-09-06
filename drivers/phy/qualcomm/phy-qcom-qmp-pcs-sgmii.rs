//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-sgmii.h
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
pub const QPHY_PCS_PHY_START: c_uint = 0x000;
pub const QPHY_PCS_POWER_DOWN_CONTROL: c_uint = 0x004;
pub const QPHY_PCS_SW_RESET: c_uint = 0x008;
pub const QPHY_PCS_LINE_RESET_TIME: c_uint = 0x00c;
pub const QPHY_PCS_TX_LARGE_AMP_DRV_LVL: c_uint = 0x020;
pub const QPHY_PCS_TX_SMALL_AMP_DRV_LVL: c_uint = 0x028;
pub const QPHY_PCS_PCS_READY_STATUS: c_uint = 0x094;
pub const QPHY_PCS_TX_MID_TERM_CTRL1: c_uint = 0x0d8;
pub const QPHY_PCS_TX_MID_TERM_CTRL2: c_uint = 0x0dc;
pub const QPHY_PCS_SGMII_MISC_CTRL8: c_uint = 0x118;

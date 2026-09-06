//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-usb-v10.h
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
// Copyright (c) 2026, Qualcomm Innovation Center, Inc. All rights reserved.
//
// Only for QMP V10 PHY - USB PCS registers
pub const QPHY_V10_PCS_USB3_POWER_STATE_CONFIG1: c_uint = 0x00;
pub const QPHY_V10_PCS_USB3_AUTONOMOUS_MODE_CTRL: c_uint = 0x08;
pub const QPHY_V10_PCS_USB3_LFPS_RXTERM_IRQ_CLEAR: c_uint = 0x14;
pub const QPHY_V10_PCS_USB3_LFPS_DET_HIGH_COUNT_VAL: c_uint = 0x18;
pub const QPHY_V10_PCS_USB3_RXEQTRAINING_DFE_TIME_S2: c_uint = 0x3c;
pub const QPHY_V10_PCS_USB3_RCVR_DTCT_DLY_U3_L: c_uint = 0x40;
pub const QPHY_V10_PCS_USB3_RCVR_DTCT_DLY_U3_H: c_uint = 0x44;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-usb-v4.h
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
// Only for QMP V4 PHY - USB3 PCS registers
pub const QPHY_V4_PCS_USB3_POWER_STATE_CONFIG1: c_uint = 0x000;
pub const QPHY_V4_PCS_USB3_AUTONOMOUS_MODE_STATUS: c_uint = 0x004;
pub const QPHY_V4_PCS_USB3_AUTONOMOUS_MODE_CTRL: c_uint = 0x008;
pub const QPHY_V4_PCS_USB3_AUTONOMOUS_MODE_CTRL2: c_uint = 0x00c;
pub const QPHY_V4_PCS_USB3_LFPS_RXTERM_IRQ_SOURCE_STATUS: c_uint = 0x010;
pub const QPHY_V4_PCS_USB3_LFPS_RXTERM_IRQ_CLEAR: c_uint = 0x014;
pub const QPHY_V4_PCS_USB3_LFPS_DET_HIGH_COUNT_VAL: c_uint = 0x018;
pub const QPHY_V4_PCS_USB3_LFPS_TX_ECSTART: c_uint = 0x01c;
pub const QPHY_V4_PCS_USB3_LFPS_PER_TIMER_VAL: c_uint = 0x020;
pub const QPHY_V4_PCS_USB3_LFPS_TX_END_CNT_U3_START: c_uint = 0x024;
pub const QPHY_V4_PCS_USB3_RXEQTRAINING_LOCK_TIME: c_uint = 0x028;
pub const QPHY_V4_PCS_USB3_RXEQTRAINING_WAIT_TIME: c_uint = 0x02c;
pub const QPHY_V4_PCS_USB3_RXEQTRAINING_CTLE_TIME: c_uint = 0x030;
pub const QPHY_V4_PCS_USB3_RXEQTRAINING_WAIT_TIME_S2: c_uint = 0x034;
pub const QPHY_V4_PCS_USB3_RXEQTRAINING_DFE_TIME_S2: c_uint = 0x038;
pub const QPHY_V4_PCS_USB3_RCVR_DTCT_DLY_U3_L: c_uint = 0x03c;
pub const QPHY_V4_PCS_USB3_RCVR_DTCT_DLY_U3_H: c_uint = 0x040;
pub const QPHY_V4_PCS_USB3_ARCVR_DTCT_EN_PERIOD: c_uint = 0x044;
pub const QPHY_V4_PCS_USB3_ARCVR_DTCT_CM_DLY: c_uint = 0x048;
pub const QPHY_V4_PCS_USB3_TXONESZEROS_RUN_LENGTH: c_uint = 0x04c;
pub const QPHY_V4_PCS_USB3_ALFPS_DEGLITCH_VAL: c_uint = 0x050;
pub const QPHY_V4_PCS_USB3_SIGDET_STARTUP_TIMER_VAL: c_uint = 0x054;
pub const QPHY_V4_PCS_USB3_TEST_CONTROL: c_uint = 0x058;

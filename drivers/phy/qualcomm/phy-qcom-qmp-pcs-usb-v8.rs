//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-usb-v8.h
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
// Copyright (c) 2025 Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const QPHY_V8_PCS_USB_POWER_STATE_CONFIG1: c_uint = 0x00;
pub const QPHY_V8_PCS_USB_AUTONOMOUS_MODE_STATUS: c_uint = 0x04;
pub const QPHY_V8_PCS_USB_AUTONOMOUS_MODE_CTRL: c_uint = 0x08;
pub const QPHY_V8_PCS_USB_AUTONOMOUS_MODE_CTRL2: c_uint = 0x0c;
pub const QPHY_V8_PCS_USB_LFPS_RXTERM_IRQ_SOURCE_STATUS: c_uint = 0x10;
pub const QPHY_V8_PCS_USB_LFPS_RXTERM_IRQ_CLEAR: c_uint = 0x14;
pub const QPHY_V8_PCS_USB_LFPS_DET_HIGH_COUNT_VAL: c_uint = 0x18;
pub const QPHY_V8_PCS_USB_LFPS_TX_ECSTART: c_uint = 0x1c;
pub const QPHY_V8_PCS_USB_LFPS_PER_TIMER_VAL: c_uint = 0x20;
pub const QPHY_V8_PCS_USB_LFPS_TX_END_CNT_U3_START: c_uint = 0x24;
pub const QPHY_V8_PCS_USB_LFPS_CONFIG1: c_uint = 0x28;
pub const QPHY_V8_PCS_USB_RXEQTRAINING_LOCK_TIME: c_uint = 0x2c;
pub const QPHY_V8_PCS_USB_RXEQTRAINING_WAIT_TIME: c_uint = 0x30;
pub const QPHY_V8_PCS_USB_RXEQTRAINING_CTLE_TIME: c_uint = 0x34;
pub const QPHY_V8_PCS_USB_RXEQTRAINING_WAIT_TIME_S2: c_uint = 0x38;
pub const QPHY_V8_PCS_USB_RXEQTRAINING_DFE_TIME_S2: c_uint = 0x3c;
pub const QPHY_V8_PCS_USB_RCVR_DTCT_DLY_U3_L: c_uint = 0x40;
pub const QPHY_V8_PCS_USB_RCVR_DTCT_DLY_U3_H: c_uint = 0x44;
pub const QPHY_V8_PCS_USB_ARCVR_DTCT_EN_PERIOD: c_uint = 0x48;
pub const QPHY_V8_PCS_USB_ARCVR_DTCT_CM_DLY: c_uint = 0x4c;
pub const QPHY_V8_PCS_USB_TXONESZEROS_RUN_LENGTH: c_uint = 0x50;
pub const QPHY_V8_PCS_USB_ALFPS_DEGLITCH_VAL: c_uint = 0x54;
pub const QPHY_V8_PCS_USB_SIGDET_STARTUP_TIMER_VAL: c_uint = 0x58;
pub const QPHY_V8_PCS_USB_TEST_CONTROL: c_uint = 0x5c;
pub const QPHY_V8_PCS_USB_RXTERMINATION_DLY_SEL: c_uint = 0x60;
pub const QPHY_V8_PCS_USB_POWER_STATE_CONFIG2: c_uint = 0x64;
pub const QPHY_V8_PCS_USB_POWER_STATE_CONFIG3: c_uint = 0x68;
pub const QPHY_V8_PCS_USB_POWER_STATE_CONFIG4: c_uint = 0x6c;

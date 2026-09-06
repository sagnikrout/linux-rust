//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-v2.h
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
// Only for QMP V2 PHY - PCS registers
pub const QPHY_V2_PCS_SW_RESET: c_uint = 0x000;
pub const QPHY_V2_PCS_POWER_DOWN_CONTROL: c_uint = 0x004;
pub const QPHY_V2_PCS_START_CONTROL: c_uint = 0x008;
pub const QPHY_V2_PCS_TXDEEMPH_M6DB_V0: c_uint = 0x024;
pub const QPHY_V2_PCS_TXDEEMPH_M3P5DB_V0: c_uint = 0x028;
pub const QPHY_V2_PCS_ENDPOINT_REFCLK_DRIVE: c_uint = 0x054;
pub const QPHY_V2_PCS_RX_IDLE_DTCT_CNTRL: c_uint = 0x058;
pub const QPHY_V2_PCS_POWER_STATE_CONFIG1: c_uint = 0x060;
pub const QPHY_V2_PCS_POWER_STATE_CONFIG2: c_uint = 0x064;
pub const QPHY_V2_PCS_POWER_STATE_CONFIG4: c_uint = 0x06c;
pub const QPHY_V2_PCS_LOCK_DETECT_CONFIG1: c_uint = 0x080;
pub const QPHY_V2_PCS_LOCK_DETECT_CONFIG2: c_uint = 0x084;
pub const QPHY_V2_PCS_LOCK_DETECT_CONFIG3: c_uint = 0x088;
pub const QPHY_V2_PCS_PWRUP_RESET_DLY_TIME_AUXCLK: c_uint = 0x0a0;
pub const QPHY_V2_PCS_LP_WAKEUP_DLY_TIME_AUXCLK: c_uint = 0x0a4;
pub const QPHY_V2_PCS_PLL_LOCK_CHK_DLY_TIME: c_uint = 0x0a8;
pub const QPHY_V2_PCS_FLL_CNTRL1: c_uint = 0x0c0;
pub const QPHY_V2_PCS_FLL_CNTRL2: c_uint = 0x0c4;
pub const QPHY_V2_PCS_FLL_CNT_VAL_L: c_uint = 0x0c8;
pub const QPHY_V2_PCS_FLL_CNT_VAL_H_TOL: c_uint = 0x0cc;
pub const QPHY_V2_PCS_FLL_MAN_CODE: c_uint = 0x0d0;
pub const QPHY_V2_PCS_AUTONOMOUS_MODE_CTRL: c_uint = 0x0d4;
pub const QPHY_V2_PCS_LFPS_RXTERM_IRQ_CLEAR: c_uint = 0x0d8;
pub const QPHY_V2_PCS_LFPS_RXTERM_IRQ_STATUS: c_uint = 0x178;
pub const QPHY_V2_PCS_USB_PCS_STATUS: c_uint = 0x17c /* USB */;
pub const QPHY_V2_PCS_PLL_LOCK_CHK_DLY_TIME_AUXCLK_LSB: c_uint = 0x1a8;
pub const QPHY_V2_PCS_OSC_DTCT_ACTIONS: c_uint = 0x1ac;
pub const QPHY_V2_PCS_SIGDET_CNTRL: c_uint = 0x1b0;
pub const QPHY_V2_PCS_RX_SIGDET_LVL: c_uint = 0x1d8;
pub const QPHY_V2_PCS_L1SS_WAKEUP_DLY_TIME_AUXCLK_LSB: c_uint = 0x1dc;
pub const QPHY_V2_PCS_L1SS_WAKEUP_DLY_TIME_AUXCLK_MSB: c_uint = 0x1e0;
pub const QPHY_V2_PCS_PCI_PCS_STATUS: c_uint = 0x174 /* PCI */;

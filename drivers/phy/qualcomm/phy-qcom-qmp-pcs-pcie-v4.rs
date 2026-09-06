//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-pcie-v4.h
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
// Only for QMP V4 PHY - PCS_PCIE registers (same as PCS_MISC?)
pub const QPHY_V4_PCS_PCIE_INT_AUX_CLK_STATUS: c_uint = 0x00;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_STATUS: c_uint = 0x04;
pub const QPHY_V4_PCS_PCIE_POWER_STATE_CONFIG1: c_uint = 0x08;
pub const QPHY_V4_PCS_PCIE_POWER_STATE_CONFIG2: c_uint = 0x0c;
pub const QPHY_V4_PCS_PCIE_POWER_STATE_CONFIG3: c_uint = 0x10;
pub const QPHY_V4_PCS_PCIE_POWER_STATE_CONFIG4: c_uint = 0x14;
pub const QPHY_V4_PCS_PCIE_PCS_TX_RX_CONFIG: c_uint = 0x18;
pub const QPHY_V4_PCS_PCIE_ENDPOINT_REFCLK_DRIVE: c_uint = 0x1c;
pub const QPHY_V4_PCS_PCIE_ENDPOINT_REFCLK_CNTRL: c_uint = 0x20;
pub const QPHY_V4_PCS_PCIE_EPCLK_PRE_PLL_LOCK_DLY_AUXCLK: c_uint = 0x24;
pub const QPHY_V4_PCS_PCIE_EPCLK_DLY_COUNT_VAL_L: c_uint = 0x28;
pub const QPHY_V4_PCS_PCIE_EPCLK_DLY_COUNT_VAL_H: c_uint = 0x2c;
pub const QPHY_V4_PCS_PCIE_RX_IDLE_DTCT_CNTRL1: c_uint = 0x30;
pub const QPHY_V4_PCS_PCIE_RX_IDLE_DTCT_CNTRL2: c_uint = 0x34;
pub const QPHY_V4_PCS_PCIE_SIGDET_CNTRL: c_uint = 0x38;
pub const QPHY_V4_PCS_PCIE_SIGDET_LOW_2_IDLE_TIME: c_uint = 0x3c;
pub const QPHY_V4_PCS_PCIE_L1P1_WAKEUP_DLY_TIME_AUXCLK_L: c_uint = 0x40;
pub const QPHY_V4_PCS_PCIE_L1P1_WAKEUP_DLY_TIME_AUXCLK_H: c_uint = 0x44;
pub const QPHY_V4_PCS_PCIE_L1P2_WAKEUP_DLY_TIME_AUXCLK_L: c_uint = 0x48;
pub const QPHY_V4_PCS_PCIE_L1P2_WAKEUP_DLY_TIME_AUXCLK_H: c_uint = 0x4c;
pub const QPHY_V4_PCS_PCIE_INT_AUX_CLK_CONFIG1: c_uint = 0x50;
pub const QPHY_V4_PCS_PCIE_INT_AUX_CLK_CONFIG2: c_uint = 0x54;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_CONFIG1: c_uint = 0x58;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_CONFIG2: c_uint = 0x5c;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_CONFIG3: c_uint = 0x60;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_CONFIG4: c_uint = 0x64;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_CONFIG5: c_uint = 0x68;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_CONFIG6: c_uint = 0x6c;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_CONFIG7: c_uint = 0x70;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_MODE2_CONFIG1: c_uint = 0x74;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_MODE2_CONFIG2: c_uint = 0x78;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_MODE2_CONFIG3: c_uint = 0x7c;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_MODE2_CONFIG4: c_uint = 0x80;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_MODE2_CONFIG5: c_uint = 0x84;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_MODE2_CONFIG6: c_uint = 0x88;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_MODE2_CONFIG7: c_uint = 0x8c;
pub const QPHY_V4_PCS_PCIE_OSC_DTCT_ACTIONS: c_uint = 0x90;
pub const QPHY_V4_PCS_PCIE_LOCAL_FS: c_uint = 0x94;
pub const QPHY_V4_PCS_PCIE_LOCAL_LF: c_uint = 0x98;
pub const QPHY_V4_PCS_PCIE_LOCAL_FS_RS: c_uint = 0x9c;
pub const QPHY_V4_PCS_PCIE_EQ_CONFIG1: c_uint = 0xa0;
pub const QPHY_V4_PCS_PCIE_EQ_CONFIG2: c_uint = 0xa4;
pub const QPHY_V4_PCS_PCIE_PRESET_P0_P1_PRE: c_uint = 0xa8;
pub const QPHY_V4_PCS_PCIE_PRESET_P2_P3_PRE: c_uint = 0xac;
pub const QPHY_V4_PCS_PCIE_PRESET_P4_P5_PRE: c_uint = 0xb0;
pub const QPHY_V4_PCS_PCIE_PRESET_P6_P7_PRE: c_uint = 0xb4;
pub const QPHY_V4_PCS_PCIE_PRESET_P8_P9_PRE: c_uint = 0xb8;
pub const QPHY_V4_PCS_PCIE_PRESET_P10_PRE: c_uint = 0xbc;
pub const QPHY_V4_PCS_PCIE_PRESET_P1_P3_PRE_RS: c_uint = 0xc0;
pub const QPHY_V4_PCS_PCIE_PRESET_P4_P5_PRE_RS: c_uint = 0xc4;
pub const QPHY_V4_PCS_PCIE_PRESET_P6_P9_PRE_RS: c_uint = 0xc8;
pub const QPHY_V4_PCS_PCIE_PRESET_P0_P1_POST: c_uint = 0xcc;
pub const QPHY_V4_PCS_PCIE_PRESET_P2_P3_POST: c_uint = 0xd0;
pub const QPHY_V4_PCS_PCIE_PRESET_P4_P5_POST: c_uint = 0xd4;
pub const QPHY_V4_PCS_PCIE_PRESET_P6_P7_POST: c_uint = 0xd8;
pub const QPHY_V4_PCS_PCIE_PRESET_P8_P9_POST: c_uint = 0xdc;
pub const QPHY_V4_PCS_PCIE_PRESET_P10_POST: c_uint = 0xe0;
pub const QPHY_V4_PCS_PCIE_PRESET_P1_P3_POST_RS: c_uint = 0xe4;
pub const QPHY_V4_PCS_PCIE_PRESET_P4_P5_POST_RS: c_uint = 0xe8;
pub const QPHY_V4_PCS_PCIE_PRESET_P6_P9_POST_RS: c_uint = 0xec;
pub const QPHY_V4_PCS_PCIE_RXEQEVAL_TIME: c_uint = 0xf0;

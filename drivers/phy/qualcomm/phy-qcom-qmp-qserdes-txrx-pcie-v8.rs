//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-pcie-v8.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. All rights reserved.
//
pub const QSERDES_V8_PCIE_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x030;
pub const QSERDES_V8_PCIE_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x034;
pub const QSERDES_V8_PCIE_TX_LANE_MODE_1: c_uint = 0x07c;
pub const QSERDES_V8_PCIE_TX_LANE_MODE_2: c_uint = 0x080;
pub const QSERDES_V8_PCIE_TX_LANE_MODE_3: c_uint = 0x084;
pub const QSERDES_V8_PCIE_TX_TRAN_DRVR_EMP_EN: c_uint = 0x0b4;
pub const QSERDES_V8_PCIE_TX_TX_BAND0: c_uint = 0x0e0;
pub const QSERDES_V8_PCIE_TX_TX_BAND1: c_uint = 0x0e4;
pub const QSERDES_V8_PCIE_TX_SEL_10B_8B: c_uint = 0x0f4;
pub const QSERDES_V8_PCIE_TX_SEL_20B_10B: c_uint = 0x0f8;
pub const QSERDES_V8_PCIE_TX_PARRATE_REC_DETECT_IDLE_EN: c_uint = 0x058;
pub const QSERDES_V8_PCIE_TX_TX_ADAPT_POST_THRESH1: c_uint = 0x118;
pub const QSERDES_V8_PCIE_TX_TX_ADAPT_POST_THRESH2: c_uint = 0x11c;
pub const QSERDES_V8_PCIE_TX_PHPRE_CTRL: c_uint = 0x128;
pub const QSERDES_V8_PCIE_TX_EQ_RCF_CTRL_RATE3: c_uint = 0x148;
pub const QSERDES_V8_PCIE_TX_EQ_RCF_CTRL_RATE4: c_uint = 0x14c;
pub const QSERDES_V8_PCIE_RX_UCDR_FO_GAIN_RATE4: c_uint = 0x0dc;
pub const QSERDES_V8_PCIE_RX_UCDR_SO_GAIN_RATE3: c_uint = 0x0ec;
pub const QSERDES_V8_PCIE_RX_UCDR_SO_GAIN_RATE4: c_uint = 0x0f0;
pub const QSERDES_V8_PCIE_RX_UCDR_PI_CONTROLS: c_uint = 0x0f4;
pub const QSERDES_V8_PCIE_RX_VGA_CAL_CNTRL1: c_uint = 0x170;
pub const QSERDES_V8_PCIE_RX_VGA_CAL_MAN_VAL: c_uint = 0x178;
pub const QSERDES_V8_PCIE_RX_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0x1b4;
pub const QSERDES_V8_PCIE_RX_SIGDET_ENABLES: c_uint = 0x1d8;
pub const QSERDES_V8_PCIE_RX_SIGDET_LVL: c_uint = 0x1e0;
pub const QSERDES_V8_PCIE_RX_RXCLK_DIV2_CTRL: c_uint = 0x0b8;
pub const QSERDES_V8_PCIE_RX_RX_BAND_CTRL0: c_uint = 0x0bc;
pub const QSERDES_V8_PCIE_RX_RX_TERM_BW_CTRL0: c_uint = 0x0c4;
pub const QSERDES_V8_PCIE_RX_RX_TERM_BW_CTRL1: c_uint = 0x0c8;
pub const QSERDES_V8_PCIE_RX_SVS_MODE_CTRL: c_uint = 0x0b4;
pub const QSERDES_V8_PCIE_RX_UCDR_PI_CTRL1: c_uint = 0x058;
pub const QSERDES_V8_PCIE_RX_UCDR_PI_CTRL2: c_uint = 0x05c;
pub const QSERDES_V8_PCIE_RX_UCDR_SB2_THRESH2_RATE3: c_uint = 0x084;
pub const QSERDES_V8_PCIE_RX_UCDR_SB2_GAIN1_RATE3: c_uint = 0x098;
pub const QSERDES_V8_PCIE_RX_UCDR_SB2_GAIN2_RATE3: c_uint = 0x0ac;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE_0_1_B0: c_uint = 0x218;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE_0_1_B1: c_uint = 0x21c;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE_0_1_B2: c_uint = 0x220;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE_0_1_B4: c_uint = 0x228;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE_0_1_B7: c_uint = 0x234;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE3_B0: c_uint = 0x260;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE3_B1: c_uint = 0x264;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE3_B2: c_uint = 0x268;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE3_B3: c_uint = 0x26c;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE3_B4: c_uint = 0x270;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE4_SA_B0: c_uint = 0x284;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE4_SA_B1: c_uint = 0x288;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE4_SA_B2: c_uint = 0x28c;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE4_SA_B3: c_uint = 0x290;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE4_SA_B4: c_uint = 0x294;
pub const QSERDES_V8_PCIE_RX_RX_MODE_RATE4_SA_B5: c_uint = 0x298;
pub const QSERDES_V8_PCIE_RX_Q_PI_INTRINSIC_BIAS_RATE32: c_uint = 0x31c;
pub const QSERDES_V8_PCIE_RX_Q_PI_INTRINSIC_BIAS_RATE4: c_uint = 0x320;
pub const QSERDES_V8_PCIE_RX_EOM_MAX_ERR_LIMIT_LSB: c_uint = 0x11c;
pub const QSERDES_V8_PCIE_RX_EOM_MAX_ERR_LIMIT_MSB: c_uint = 0x120;
pub const QSERDES_V8_PCIE_RX_AUXDATA_BIN_RATE23: c_uint = 0x108;
pub const QSERDES_V8_PCIE_RX_AUXDATA_BIN_RATE4: c_uint = 0x10c;
pub const QSERDES_V8_PCIE_RX_VTHRESH_CAL_MAN_VAL_RATE3: c_uint = 0x198;
pub const QSERDES_V8_PCIE_RX_VTHRESH_CAL_MAN_VAL_RATE4: c_uint = 0x19c;
pub const QSERDES_V8_PCIE_RX_GM_CAL: c_uint = 0x1a0;

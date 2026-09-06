//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v8.h
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
pub const QSERDES_V8_TX_TX_EMP_POST1_LVL: c_uint = 0x00c;
pub const QSERDES_V8_TX_TX_DRV_LVL: c_uint = 0x014;
pub const QSERDES_V8_TX_RES_CODE_LANE_TX: c_uint = 0x034;
pub const QSERDES_V8_TX_RES_CODE_LANE_RX: c_uint = 0x038;
pub const QSERDES_V8_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x03c;
pub const QSERDES_V8_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x040;
pub const QSERDES_V8_TX_TRANSCEIVER_BIAS_EN: c_uint = 0x054;
pub const QSERDES_V8_TX_HIGHZ_DRVR_EN: c_uint = 0x058;
pub const QSERDES_V8_TX_TX_POL_INV: c_uint = 0x05c;
pub const QSERDES_V8_TX_LANE_MODE_1: c_uint = 0x084;
pub const QSERDES_V8_TX_LANE_MODE_2: c_uint = 0x088;
pub const QSERDES_V8_TX_LANE_MODE_3: c_uint = 0x08c;
pub const QSERDES_V8_TX_LANE_MODE_4: c_uint = 0x090;
pub const QSERDES_V8_TX_LANE_MODE_5: c_uint = 0x094;
pub const QSERDES_V8_TX_RCV_DETECT_LVL_2: c_uint = 0x0a4;
pub const QSERDES_V8_TX_PI_QEC_CTRL: c_uint = 0x0e4;
pub const QSERDES_V8_RX_UCDR_FO_GAIN: c_uint = 0x008;
pub const QSERDES_V8_RX_UCDR_SO_GAIN: c_uint = 0x014;
pub const QSERDES_V8_RX_UCDR_SVS_FO_GAIN: c_uint = 0x020;
pub const QSERDES_V8_RX_UCDR_FASTLOCK_FO_GAIN: c_uint = 0x030;
pub const QSERDES_V8_RX_UCDR_SO_SATURATION_AND_ENABLE: c_uint = 0x034;
pub const QSERDES_V8_RX_UCDR_FASTLOCK_COUNT_LOW: c_uint = 0x03c;
pub const QSERDES_V8_RX_UCDR_FASTLOCK_COUNT_HIGH: c_uint = 0x040;
pub const QSERDES_V8_RX_UCDR_PI_CONTROLS: c_uint = 0x044;
pub const QSERDES_V8_RX_UCDR_SB2_THRESH1: c_uint = 0x04c;
pub const QSERDES_V8_RX_UCDR_SB2_THRESH2: c_uint = 0x050;
pub const QSERDES_V8_RX_UCDR_SB2_GAIN1: c_uint = 0x054;
pub const QSERDES_V8_RX_UCDR_SB2_GAIN2: c_uint = 0x058;
pub const QSERDES_V8_RX_AUX_DATA_TCOARSE_TFINE: c_uint = 0x060;
pub const QSERDES_V8_RX_VGA_CAL_CNTRL1: c_uint = 0x0d4;
pub const QSERDES_V8_RX_VGA_CAL_CNTRL2: c_uint = 0x0d8;
pub const QSERDES_V8_RX_GM_CAL: c_uint = 0x0dc;
pub const QSERDES_V8_RX_RX_EQU_ADAPTOR_CNTRL2: c_uint = 0x0ec;
pub const QSERDES_V8_RX_RX_EQU_ADAPTOR_CNTRL3: c_uint = 0x0f0;
pub const QSERDES_V8_RX_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0x0f4;
pub const QSERDES_V8_RX_RX_IDAC_TSETTLE_LOW: c_uint = 0x0f8;
pub const QSERDES_V8_RX_RX_IDAC_TSETTLE_HIGH: c_uint = 0x0fc;
pub const QSERDES_V8_RX_RX_EQ_OFFSET_ADAPTOR_CNTRL1: c_uint = 0x110;
pub const QSERDES_V8_RX_SIGDET_ENABLES: c_uint = 0x118;
pub const QSERDES_V8_RX_SIGDET_CNTRL: c_uint = 0x11c;
pub const QSERDES_V8_RX_SIGDET_DEGLITCH_CNTRL: c_uint = 0x124;
pub const QSERDES_V8_RX_RX_MODE_00_LOW: c_uint = 0x15c;
pub const QSERDES_V8_RX_RX_MODE_00_HIGH: c_uint = 0x160;
pub const QSERDES_V8_RX_RX_MODE_00_HIGH2: c_uint = 0x164;
pub const QSERDES_V8_RX_RX_MODE_00_HIGH3: c_uint = 0x168;
pub const QSERDES_V8_RX_RX_MODE_00_HIGH4: c_uint = 0x16c;
pub const QSERDES_V8_RX_RX_MODE_01_LOW: c_uint = 0x170;
pub const QSERDES_V8_RX_RX_MODE_01_HIGH: c_uint = 0x174;
pub const QSERDES_V8_RX_RX_MODE_01_HIGH2: c_uint = 0x178;
pub const QSERDES_V8_RX_RX_MODE_01_HIGH3: c_uint = 0x17c;
pub const QSERDES_V8_RX_RX_MODE_01_HIGH4: c_uint = 0x180;
pub const QSERDES_V8_RX_DFE_EN_TIMER: c_uint = 0x1a0;
pub const QSERDES_V8_RX_DFE_CTLE_POST_CAL_OFFSET: c_uint = 0x1a4;
pub const QSERDES_V8_RX_DCC_CTRL1: c_uint = 0x1a8;
pub const QSERDES_V8_RX_VTH_CODE: c_uint = 0x1b0;
pub const QSERDES_V8_RX_SIGDET_CAL_CTRL1: c_uint = 0x1e4;
pub const QSERDES_V8_RX_SIGDET_CAL_TRIM: c_uint = 0x1f8;

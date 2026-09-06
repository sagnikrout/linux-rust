//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v6_n4.h
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
pub const QSERDES_V6_N4_TX_CLKBUF_ENABLE: c_uint = 0x08;
pub const QSERDES_V6_N4_TX_TX_EMP_POST1_LVL: c_uint = 0x0c;
pub const QSERDES_V6_N4_TX_TX_DRV_LVL: c_uint = 0x14;
pub const QSERDES_V6_N4_TX_RESET_TSYNC_EN: c_uint = 0x1c;
pub const QSERDES_V6_N4_TX_PRE_STALL_LDO_BOOST_EN: c_uint = 0x20;
pub const QSERDES_V6_N4_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x30;
pub const QSERDES_V6_N4_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x34;
pub const QSERDES_V6_N4_TX_TRANSCEIVER_BIAS_EN: c_uint = 0x48;
pub const QSERDES_V6_N4_TX_HIGHZ_DRVR_EN: c_uint = 0x4c;
pub const QSERDES_V6_N4_TX_TX_POL_INV: c_uint = 0x50;
pub const QSERDES_V6_N4_TX_PARRATE_REC_DETECT_IDLE_EN: c_uint = 0x54;
pub const QSERDES_V6_N4_TX_LANE_MODE_1: c_uint = 0x78;
pub const QSERDES_V6_N4_TX_LANE_MODE_2: c_uint = 0x7c;
pub const QSERDES_V6_N4_TX_LANE_MODE_3: c_uint = 0x80;
pub const QSERDES_V6_N4_TX_TRAN_DRVR_EMP_EN: c_uint = 0xac;
pub const QSERDES_V6_N4_TX_TX_BAND: c_uint = 0xd8;
pub const QSERDES_V6_N4_TX_INTERFACE_SELECT: c_uint = 0xe4;
pub const QSERDES_V6_N4_TX_VMODE_CTRL1: c_uint = 0xb0;
pub const QSERDES_V6_N4_RX_UCDR_FO_GAIN_RATE2: c_uint = 0x8;
pub const QSERDES_V6_N4_RX_UCDR_SO_GAIN_RATE2: c_uint = 0x18;
pub const QSERDES_V6_N4_RX_UCDR_PI_CONTROLS: c_uint = 0x20;
pub const QSERDES_V6_N4_RX_IVCM_CAL_CODE_OVERRIDE: c_uint = 0x94;
pub const QSERDES_V6_N4_RX_RX_IVCM_CAL_CTRL2: c_uint = 0x9c;
pub const QSERDES_V6_N4_RX_RX_IVCM_POSTCAL_OFFSET: c_uint = 0xa0;
pub const QSERDES_V6_N4_RX_DFE_3: c_uint = 0xb4;
pub const QSERDES_V6_N4_RX_VGA_CAL_CNTRL1: c_uint = 0xe0;
pub const QSERDES_V6_N4_RX_VGA_CAL_MAN_VAL: c_uint = 0xe8;
pub const QSERDES_V6_N4_RX_GM_CAL: c_uint = 0x10c;
pub const QSERDES_V6_N4_RX_SIGDET_ENABLES: c_uint = 0x148;
pub const QSERDES_V6_N4_RX_SIGDET_CNTRL: c_uint = 0x14c;
pub const QSERDES_V6_N4_RX_SIGDET_DEGLITCH_CNTRL: c_uint = 0x154;
pub const QSERDES_V6_N4_RX_DFE_CTLE_POST_CAL_OFFSET: c_uint = 0x194;
pub const QSERDES_V6_N4_RX_Q_PI_INTRINSIC_BIAS_RATE32: c_uint = 0x1dc;
pub const QSERDES_V6_N4_RX_UCDR_PI_CTRL1: c_uint = 0x23c;
pub const QSERDES_V6_N4_RX_UCDR_PI_CTRL2: c_uint = 0x240;
pub const QSERDES_V6_N4_RX_UCDR_SB2_GAIN2_RATE2: c_uint = 0x27c;
pub const QSERDES_V6_N4_RX_DFE_DAC_ENABLE1: c_uint = 0x298;
pub const QSERDES_V6_N4_RX_MODE_RATE_0_1_B0: c_uint = 0x2b8;
pub const QSERDES_V6_N4_RX_MODE_RATE_0_1_B1: c_uint = 0x2bc;
pub const QSERDES_V6_N4_RX_MODE_RATE_0_1_B2: c_uint = 0x2c0;
pub const QSERDES_V6_N4_RX_MODE_RATE_0_1_B3: c_uint = 0x2c4;
pub const QSERDES_V6_N4_RX_MODE_RATE_0_1_B4: c_uint = 0x2c8;
pub const QSERDES_V6_N4_RX_MODE_RATE_0_1_B5: c_uint = 0x2cc;
pub const QSERDES_V6_N4_RX_MODE_RATE_0_1_B6: c_uint = 0x2d0;
pub const QSERDES_V6_N4_RX_MODE_RATE2_B0: c_uint = 0x2d4;
pub const QSERDES_V6_N4_RX_MODE_RATE2_B1: c_uint = 0x2d8;
pub const QSERDES_V6_N4_RX_MODE_RATE2_B2: c_uint = 0x2dc;
pub const QSERDES_V6_N4_RX_MODE_RATE2_B3: c_uint = 0x2e0;
pub const QSERDES_V6_N4_RX_MODE_RATE2_B4: c_uint = 0x2e4;
pub const QSERDES_V6_N4_RX_MODE_RATE2_B5: c_uint = 0x2e8;
pub const QSERDES_V6_N4_RX_MODE_RATE2_B6: c_uint = 0x2ec;
pub const QSERDES_V6_N4_RX_RX_SUMMER_CAL_SPD_MODE: c_uint = 0x30c;
pub const QSERDES_V6_N4_RX_RX_BKUP_CTRL1: c_uint = 0x310;

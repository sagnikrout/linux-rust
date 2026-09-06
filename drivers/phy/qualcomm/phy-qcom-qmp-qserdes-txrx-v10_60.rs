//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v10_60.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
pub const QSERDES_V10_60_TXRX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x034;
pub const QSERDES_V10_60_TXRX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x038;
pub const QSERDES_V10_60_TXRX_LANE_MODE_1: c_uint = 0x080;
pub const QSERDES_V10_60_TXRX_LANE_MODE_2: c_uint = 0x084;
pub const QSERDES_V10_60_TXRX_LANE_MODE_3: c_uint = 0x088;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_FO_GAIN_RATE1: c_uint = 0x0c8;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_FO_GAIN_RATE2: c_uint = 0x0cc;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_FO_GAIN_RATE3: c_uint = 0x0d0;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_FO_GAIN_RATE4: c_uint = 0x0d4;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_SO_GAIN_RATE1: c_uint = 0x0e0;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_SO_GAIN_RATE2: c_uint = 0x0e4;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_SO_GAIN_RATE3: c_uint = 0x0e8;
pub const QSERDES_V10_60_TXRX_UCDR_FASTLOCK_SO_GAIN_RATE4: c_uint = 0x0ec;
pub const QSERDES_V10_60_TXRX_UCDR_PI_CTRL1: c_uint = 0x12c;
pub const QSERDES_V10_60_TXRX_UCDR_PI_CTRL2: c_uint = 0x130;
pub const QSERDES_V10_60_TXRX_UCDR_PI_CTRL3: c_uint = 0x134;
pub const QSERDES_V10_60_TXRX_UCDR_PI_CTRL4: c_uint = 0x138;
pub const QSERDES_V10_60_TXRX_SVS_MODE_CTRL: c_uint = 0x19c;
pub const QSERDES_V10_60_TXRX_RXCLK_DIV2_CTRL: c_uint = 0x1a0;
pub const QSERDES_V10_60_TXRX_RX_BAND_CTRL0: c_uint = 0x1a4;
pub const QSERDES_V10_60_TXRX_RX_TERM_BW_CTRL0: c_uint = 0x1ac;
pub const QSERDES_V10_60_TXRX_RX_TERM_BW_CTRL1: c_uint = 0x1b0;
pub const QSERDES_V10_60_TXRX_UCDR_FO_GAIN_RATE1: c_uint = 0x1b8;
pub const QSERDES_V10_60_TXRX_UCDR_FO_GAIN_RATE2: c_uint = 0x1bc;
pub const QSERDES_V10_60_TXRX_UCDR_FO_GAIN_RATE3: c_uint = 0x1c0;
pub const QSERDES_V10_60_TXRX_UCDR_FO_GAIN_RATE4: c_uint = 0x1c4;
pub const QSERDES_V10_60_TXRX_UCDR_SO_GAIN_RATE1: c_uint = 0x1d0;
pub const QSERDES_V10_60_TXRX_UCDR_SO_GAIN_RATE2: c_uint = 0x1d4;
pub const QSERDES_V10_60_TXRX_UCDR_SO_GAIN_RATE3: c_uint = 0x1d8;
pub const QSERDES_V10_60_TXRX_UCDR_SO_GAIN_RATE4: c_uint = 0x1dc;
pub const QSERDES_V10_60_TXRX_UCDR_PI_CONTROLS: c_uint = 0x1e4;
pub const QSERDES_V10_60_TXRX_AUXDATA_BIN_RATE3: c_uint = 0x200;
pub const QSERDES_V10_60_TXRX_AUXDATA_BIN_RATE4: c_uint = 0x204;
pub const QSERDES_V10_60_TXRX_EOM_MAX_ERR_LIMIT_LSB: c_uint = 0x218;
pub const QSERDES_V10_60_TXRX_EOM_MAX_ERR_LIMIT_MSB: c_uint = 0x21c;
pub const QSERDES_V10_60_TXRX_VGA_CAL_CNTRL1: c_uint = 0x280;
pub const QSERDES_V10_60_TXRX_VGA_CAL_MAN_VAL: c_uint = 0x288;
pub const QSERDES_V10_60_TXRX_GM_CAL: c_uint = 0x29c;
pub const QSERDES_V10_60_TXRX_RX_EQU_ADAPTOR_CNTRL6: c_uint = 0x2b8;
pub const QSERDES_V10_60_TXRX_SIGDET_ENABLES: c_uint = 0x2d4;
pub const QSERDES_V10_60_TXRX_SIGDET_CNTRL: c_uint = 0x2d8;
pub const QSERDES_V10_60_TXRX_SIGDET_LVL: c_uint = 0x2dc;
pub const QSERDES_V10_60_TXRX_SIGDET_DEGLITCH_CNTRL: c_uint = 0x2e0;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B0: c_uint = 0x314;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B1: c_uint = 0x318;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B2: c_uint = 0x31c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B3: c_uint = 0x320;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B4: c_uint = 0x324;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B5: c_uint = 0x328;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B6: c_uint = 0x32c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B7: c_uint = 0x330;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B8: c_uint = 0x334;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B9: c_uint = 0x338;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE_0_1_B10: c_uint = 0x33c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B0: c_uint = 0x340;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B1: c_uint = 0x344;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B2: c_uint = 0x348;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B3: c_uint = 0x34c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B4: c_uint = 0x350;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B5: c_uint = 0x354;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B6: c_uint = 0x358;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B7: c_uint = 0x35c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B8: c_uint = 0x360;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B9: c_uint = 0x364;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE2_B10: c_uint = 0x368;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B0: c_uint = 0x36c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B1: c_uint = 0x370;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B2: c_uint = 0x374;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B3: c_uint = 0x378;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B4: c_uint = 0x37c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B5: c_uint = 0x380;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B6: c_uint = 0x384;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B7: c_uint = 0x388;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B8: c_uint = 0x38c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B9: c_uint = 0x390;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE3_B10: c_uint = 0x394;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B0: c_uint = 0x398;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B1: c_uint = 0x39c;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B2: c_uint = 0x3a0;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B3: c_uint = 0x3a4;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B4: c_uint = 0x3a8;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B5: c_uint = 0x3ac;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B6: c_uint = 0x3b0;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B7: c_uint = 0x3b4;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B8: c_uint = 0x3b8;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B9: c_uint = 0x3bc;
pub const QSERDES_V10_60_TXRX_RX_MODE_RATE4_SA_B10: c_uint = 0x3c0;
pub const QSERDES_V10_60_TXRX_Q_PI_INTRINSIC_BIAS_RATE32: c_uint = 0x478;
pub const QSERDES_V10_60_TXRX_Q_PI_INTRINSIC_BIAS_RATE45: c_uint = 0x47c;
pub const QSERDES_V10_60_TXRX_SIGDET_CAL_CTRL1: c_uint = 0x4c8;
pub const QSERDES_V10_60_TXRX_SIGDET_CAL_CTRL2: c_uint = 0x4cc;
pub const QSERDES_V10_60_TXRX_SIGDET_CAL_TRIM: c_uint = 0x4d0;
pub const QSERDES_V10_60_TXRX_TX_BAND0: c_uint = 0x4e8;
pub const QSERDES_V10_60_TXRX_TX_BAND1: c_uint = 0x4ec;
pub const QSERDES_V10_60_TXRX_SEL_10B_8B: c_uint = 0x4f4;
pub const QSERDES_V10_60_TXRX_SEL_20B_10B: c_uint = 0x4f8;
pub const QSERDES_V10_60_TXRX_EQ_RCF_CTRL_RATE3: c_uint = 0x53c;
pub const QSERDES_V10_60_TXRX_EQ_RCF_CTRL_RATE4: c_uint = 0x540;
pub const QSERDES_V10_60_TXRX_PHPRE_CTRL: c_uint = 0x5e8;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v7.h
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
pub const QSERDES_V7_TX_CLKBUF_ENABLE: c_uint = 0x08;
pub const QSERDES_V7_TX_RESET_TSYNC_EN: c_uint = 0x1c;
pub const QSERDES_V7_TX_PRE_STALL_LDO_BOOST_EN: c_uint = 0x20;
pub const QSERDES_V7_TX_TX_BAND: c_uint = 0x24;
pub const QSERDES_V7_TX_INTERFACE_SELECT: c_uint = 0x2c;
pub const QSERDES_V7_TX_RES_CODE_LANE_TX: c_uint = 0x34;
pub const QSERDES_V7_TX_RES_CODE_LANE_RX: c_uint = 0x38;
pub const QSERDES_V7_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x3c;
pub const QSERDES_V7_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x40;
pub const QSERDES_V7_TX_PARRATE_REC_DETECT_IDLE_EN: c_uint = 0x60;
pub const QSERDES_V7_TX_BIST_PATTERN7: c_uint = 0x7c;
pub const QSERDES_V7_TX_LANE_MODE_1: c_uint = 0x84;
pub const QSERDES_V7_TX_LANE_MODE_2: c_uint = 0x88;
pub const QSERDES_V7_TX_LANE_MODE_3: c_uint = 0x8c;
pub const QSERDES_V7_TX_LANE_MODE_4: c_uint = 0x90;
pub const QSERDES_V7_TX_LANE_MODE_5: c_uint = 0x94;
pub const QSERDES_V7_TX_RCV_DETECT_LVL_2: c_uint = 0xa4;
pub const QSERDES_V7_TX_TRAN_DRVR_EMP_EN: c_uint = 0xc0;
pub const QSERDES_V7_TX_TX_INTERFACE_MODE: c_uint = 0xc4;
pub const QSERDES_V7_TX_VMODE_CTRL1: c_uint = 0xc8;
pub const QSERDES_V7_TX_PI_QEC_CTRL: c_uint = 0xe4;
pub const QSERDES_V7_RX_UCDR_FO_GAIN: c_uint = 0x08;
pub const QSERDES_V7_RX_UCDR_SO_GAIN: c_uint = 0x14;
pub const QSERDES_V7_RX_UCDR_FASTLOCK_FO_GAIN: c_uint = 0x30;
pub const QSERDES_V7_RX_UCDR_SO_SATURATION_AND_ENABLE: c_uint = 0x34;
pub const QSERDES_V7_RX_UCDR_FASTLOCK_COUNT_LOW: c_uint = 0x3c;
pub const QSERDES_V7_RX_UCDR_FASTLOCK_COUNT_HIGH: c_uint = 0x40;
pub const QSERDES_V7_RX_UCDR_PI_CONTROLS: c_uint = 0x44;
pub const QSERDES_V7_RX_UCDR_SB2_THRESH1: c_uint = 0x4c;
pub const QSERDES_V7_RX_UCDR_SB2_THRESH2: c_uint = 0x50;
pub const QSERDES_V7_RX_UCDR_SB2_GAIN1: c_uint = 0x54;
pub const QSERDES_V7_RX_UCDR_SB2_GAIN2: c_uint = 0x58;
pub const QSERDES_V7_RX_AUX_DATA_TCOARSE_TFINE: c_uint = 0x60;
pub const QSERDES_V7_RX_TX_ADAPT_PRE_THRESH1: c_uint = 0xc4;
pub const QSERDES_V7_RX_TX_ADAPT_PRE_THRESH2: c_uint = 0xc8;
pub const QSERDES_V7_RX_TX_ADAPT_POST_THRESH: c_uint = 0xcc;
pub const QSERDES_V7_RX_VGA_CAL_CNTRL1: c_uint = 0xd4;
pub const QSERDES_V7_RX_VGA_CAL_CNTRL2: c_uint = 0xd8;
pub const QSERDES_V7_RX_GM_CAL: c_uint = 0xdc;
pub const QSERDES_V7_RX_RX_EQU_ADAPTOR_CNTRL2: c_uint = 0xec;
pub const QSERDES_V7_RX_RX_EQU_ADAPTOR_CNTRL3: c_uint = 0xf0;
pub const QSERDES_V7_RX_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0xf4;
pub const QSERDES_V7_RX_RX_IDAC_TSETTLE_LOW: c_uint = 0xf8;
pub const QSERDES_V7_RX_RX_IDAC_TSETTLE_HIGH: c_uint = 0xfc;
pub const QSERDES_V7_RX_RX_EQ_OFFSET_ADAPTOR_CNTRL1: c_uint = 0x110;
pub const QSERDES_V7_RX_SIGDET_ENABLES: c_uint = 0x118;
pub const QSERDES_V7_RX_SIGDET_CNTRL: c_uint = 0x11c;
pub const QSERDES_V7_RX_SIGDET_DEGLITCH_CNTRL: c_uint = 0x124;
pub const QSERDES_V7_RX_RX_MODE_00_LOW: c_uint = 0x15c;
pub const QSERDES_V7_RX_RX_MODE_00_HIGH: c_uint = 0x160;
pub const QSERDES_V7_RX_RX_MODE_00_HIGH2: c_uint = 0x164;
pub const QSERDES_V7_RX_RX_MODE_00_HIGH3: c_uint = 0x168;
pub const QSERDES_V7_RX_RX_MODE_00_HIGH4: c_uint = 0x16c;
pub const QSERDES_V7_RX_RX_MODE_01_LOW: c_uint = 0x170;
pub const QSERDES_V7_RX_RX_MODE_01_HIGH: c_uint = 0x174;
pub const QSERDES_V7_RX_RX_MODE_01_HIGH2: c_uint = 0x178;
pub const QSERDES_V7_RX_RX_MODE_01_HIGH3: c_uint = 0x17c;
pub const QSERDES_V7_RX_RX_MODE_01_HIGH4: c_uint = 0x180;
pub const QSERDES_V7_RX_RX_MODE_10_LOW: c_uint = 0x184;
pub const QSERDES_V7_RX_RX_MODE_10_HIGH: c_uint = 0x188;
pub const QSERDES_V7_RX_RX_MODE_10_HIGH2: c_uint = 0x18c;
pub const QSERDES_V7_RX_RX_MODE_10_HIGH3: c_uint = 0x190;
pub const QSERDES_V7_RX_RX_MODE_10_HIGH4: c_uint = 0x194;
pub const QSERDES_V7_RX_DFE_EN_TIMER: c_uint = 0x1a0;
pub const QSERDES_V7_RX_DFE_CTLE_POST_CAL_OFFSET: c_uint = 0x1a4;
pub const QSERDES_V7_RX_DCC_CTRL1: c_uint = 0x1a8;
pub const QSERDES_V7_RX_VTH_CODE: c_uint = 0x1b0;
pub const QSERDES_V7_RX_SIGDET_CAL_CTRL1: c_uint = 0x1e4;
pub const QSERDES_V7_RX_SIGDET_CAL_TRIM: c_uint = 0x1f8;

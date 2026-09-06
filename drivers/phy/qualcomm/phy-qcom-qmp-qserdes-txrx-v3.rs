//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v3.h
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
// Only for QMP V3 PHY - TX registers
pub const QSERDES_V3_TX_BIST_MODE_LANENO: c_uint = 0x000;
pub const QSERDES_V3_TX_CLKBUF_ENABLE: c_uint = 0x008;
pub const QSERDES_V3_TX_TX_EMP_POST1_LVL: c_uint = 0x00c;
pub const QSERDES_V3_TX_TX_DRV_LVL: c_uint = 0x01c;
pub const QSERDES_V3_TX_RESET_TSYNC_EN: c_uint = 0x024;
pub const QSERDES_V3_TX_PRE_STALL_LDO_BOOST_EN: c_uint = 0x028;
pub const QSERDES_V3_TX_TX_BAND: c_uint = 0x02c;
pub const QSERDES_V3_TX_SLEW_CNTL: c_uint = 0x030;
pub const QSERDES_V3_TX_INTERFACE_SELECT: c_uint = 0x034;
pub const QSERDES_V3_TX_RES_CODE_LANE_TX: c_uint = 0x03c;
pub const QSERDES_V3_TX_RES_CODE_LANE_RX: c_uint = 0x040;
pub const QSERDES_V3_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x044;
pub const QSERDES_V3_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x048;
pub const QSERDES_V3_TX_DEBUG_BUS_SEL: c_uint = 0x058;
pub const QSERDES_V3_TX_TRANSCEIVER_BIAS_EN: c_uint = 0x05c;
pub const QSERDES_V3_TX_HIGHZ_DRVR_EN: c_uint = 0x060;
pub const QSERDES_V3_TX_TX_POL_INV: c_uint = 0x064;
pub const QSERDES_V3_TX_PARRATE_REC_DETECT_IDLE_EN: c_uint = 0x068;
pub const QSERDES_V3_TX_LANE_MODE_1: c_uint = 0x08c;
pub const QSERDES_V3_TX_LANE_MODE_2: c_uint = 0x090;
pub const QSERDES_V3_TX_LANE_MODE_3: c_uint = 0x094;
pub const QSERDES_V3_TX_RCV_DETECT_LVL_2: c_uint = 0x0a4;
pub const QSERDES_V3_TX_TRAN_DRVR_EMP_EN: c_uint = 0x0c0;
pub const QSERDES_V3_TX_TX_INTERFACE_MODE: c_uint = 0x0c4;
pub const QSERDES_V3_TX_VMODE_CTRL1: c_uint = 0x0f0;
// Only for QMP V3 PHY - RX registers
pub const QSERDES_V3_RX_UCDR_FO_GAIN: c_uint = 0x008;
pub const QSERDES_V3_RX_UCDR_SO_GAIN_HALF: c_uint = 0x00c;
pub const QSERDES_V3_RX_UCDR_SO_GAIN: c_uint = 0x014;
pub const QSERDES_V3_RX_UCDR_SVS_SO_GAIN_HALF: c_uint = 0x024;
pub const QSERDES_V3_RX_UCDR_SVS_SO_GAIN_QUARTER: c_uint = 0x028;
pub const QSERDES_V3_RX_UCDR_SVS_SO_GAIN: c_uint = 0x02c;
pub const QSERDES_V3_RX_UCDR_FASTLOCK_FO_GAIN: c_uint = 0x030;
pub const QSERDES_V3_RX_UCDR_SO_SATURATION_AND_ENABLE: c_uint = 0x034;
pub const QSERDES_V3_RX_UCDR_FASTLOCK_COUNT_LOW: c_uint = 0x03c;
pub const QSERDES_V3_RX_UCDR_FASTLOCK_COUNT_HIGH: c_uint = 0x040;
pub const QSERDES_V3_RX_UCDR_PI_CONTROLS: c_uint = 0x044;
pub const QSERDES_V3_RX_RX_TERM_BW: c_uint = 0x07c;
pub const QSERDES_V3_RX_VGA_CAL_CNTRL1: c_uint = 0x0bc;
pub const QSERDES_V3_RX_VGA_CAL_CNTRL2: c_uint = 0x0c0;
pub const QSERDES_V3_RX_RX_EQ_GAIN2_LSB: c_uint = 0x0c8;
pub const QSERDES_V3_RX_RX_EQ_GAIN2_MSB: c_uint = 0x0cc;
pub const QSERDES_V3_RX_RX_EQU_ADAPTOR_CNTRL1: c_uint = 0x0d0;
pub const QSERDES_V3_RX_RX_EQU_ADAPTOR_CNTRL2: c_uint = 0x0d4;
pub const QSERDES_V3_RX_RX_EQU_ADAPTOR_CNTRL3: c_uint = 0x0d8;
pub const QSERDES_V3_RX_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0x0dc;
pub const QSERDES_V3_RX_RX_EQ_OFFSET_ADAPTOR_CNTRL1: c_uint = 0x0f8;
pub const QSERDES_V3_RX_RX_OFFSET_ADAPTOR_CNTRL2: c_uint = 0x0fc;
pub const QSERDES_V3_RX_SIGDET_ENABLES: c_uint = 0x100;
pub const QSERDES_V3_RX_SIGDET_CNTRL: c_uint = 0x104;
pub const QSERDES_V3_RX_SIGDET_LVL: c_uint = 0x108;
pub const QSERDES_V3_RX_SIGDET_DEGLITCH_CNTRL: c_uint = 0x10c;
pub const QSERDES_V3_RX_RX_BAND: c_uint = 0x110;
pub const QSERDES_V3_RX_RX_INTERFACE_MODE: c_uint = 0x11c;
pub const QSERDES_V3_RX_RX_MODE_00: c_uint = 0x164;
pub const QSERDES_V3_RX_RX_MODE_01: c_uint = 0x168;

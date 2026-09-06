//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v4_20.h
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
// Only for QMP V4_20 PHY - TX registers
pub const QSERDES_V4_20_TX_LANE_MODE_1: c_uint = 0x88;
pub const QSERDES_V4_20_TX_LANE_MODE_2: c_uint = 0x8c;
pub const QSERDES_V4_20_TX_LANE_MODE_3: c_uint = 0x90;
pub const QSERDES_V4_20_TX_VMODE_CTRL1: c_uint = 0xc4;
pub const QSERDES_V4_20_TX_PI_QEC_CTRL: c_uint = 0xe0;
// Only for QMP V4_20 PHY - RX registers
pub const QSERDES_V4_20_RX_FO_GAIN_RATE2: c_uint = 0x008;
pub const QSERDES_V4_20_RX_UCDR_PI_CONTROLS: c_uint = 0x058;
pub const QSERDES_V4_20_RX_AUX_DATA_TCOARSE_TFINE: c_uint = 0x0ac;
pub const QSERDES_V4_20_RX_DFE_3: c_uint = 0x110;
pub const QSERDES_V4_20_RX_DFE_DAC_ENABLE1: c_uint = 0x134;
pub const QSERDES_V4_20_RX_DFE_DAC_ENABLE2: c_uint = 0x138;
pub const QSERDES_V4_20_RX_VGA_CAL_CNTRL2: c_uint = 0x150;
pub const QSERDES_V4_20_RX_RX_EQ_OFFSET_ADAPTOR_CNTRL1: c_uint = 0x178;
pub const QSERDES_V4_20_RX_RX_MODE_RATE_0_1_B1: c_uint = 0x1c8;
pub const QSERDES_V4_20_RX_RX_MODE_RATE_0_1_B2: c_uint = 0x1cc;
pub const QSERDES_V4_20_RX_RX_MODE_RATE_0_1_B3: c_uint = 0x1d0;
pub const QSERDES_V4_20_RX_RX_MODE_RATE_0_1_B4: c_uint = 0x1d4;
pub const QSERDES_V4_20_RX_RX_MODE_RATE2_B0: c_uint = 0x1d8;
pub const QSERDES_V4_20_RX_RX_MODE_RATE2_B1: c_uint = 0x1dc;
pub const QSERDES_V4_20_RX_RX_MODE_RATE2_B2: c_uint = 0x1e0;
pub const QSERDES_V4_20_RX_RX_MODE_RATE2_B3: c_uint = 0x1e4;
pub const QSERDES_V4_20_RX_RX_MODE_RATE2_B4: c_uint = 0x1e8;
pub const QSERDES_V4_20_RX_RX_MODE_RATE3_B0: c_uint = 0x1ec;
pub const QSERDES_V4_20_RX_RX_MODE_RATE3_B1: c_uint = 0x1f0;
pub const QSERDES_V4_20_RX_RX_MODE_RATE3_B2: c_uint = 0x1f4;
pub const QSERDES_V4_20_RX_RX_MODE_RATE3_B3: c_uint = 0x1f8;
pub const QSERDES_V4_20_RX_RX_MODE_RATE3_B4: c_uint = 0x1fc;
pub const QSERDES_V4_20_RX_PHPRE_CTRL: c_uint = 0x200;
pub const QSERDES_V4_20_RX_DFE_CTLE_POST_CAL_OFFSET: c_uint = 0x20c;
pub const QSERDES_V4_20_RX_MARG_COARSE_CTRL2: c_uint = 0x23c;

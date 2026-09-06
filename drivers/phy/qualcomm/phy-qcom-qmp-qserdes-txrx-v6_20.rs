//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v6_20.h
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
pub const QSERDES_V6_20_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x30;
pub const QSERDES_V6_20_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x34;
pub const QSERDES_V6_20_TX_TRAN_DRVR_EMP_EN: c_uint = 0xac;
pub const QSERDES_V6_20_TX_LANE_MODE_1: c_uint = 0x78;
pub const QSERDES_V6_20_TX_LANE_MODE_2: c_uint = 0x7c;
pub const QSERDES_V6_20_TX_LANE_MODE_3: c_uint = 0x80;
pub const QSERDES_V6_20_RX_UCDR_FO_GAIN_RATE_2: c_uint = 0x08;
pub const QSERDES_V6_20_RX_UCDR_FO_GAIN_RATE_3: c_uint = 0x0c;
pub const QSERDES_V6_20_RX_UCDR_SO_GAIN_RATE_2: c_uint = 0x18;
pub const QSERDES_V6_20_RX_UCDR_PI_CONTROLS: c_uint = 0x20;
pub const QSERDES_V6_20_RX_UCDR_SO_ACC_DEFAULT_VAL_RATE3: c_uint = 0x34;
pub const QSERDES_V6_20_RX_IVCM_CAL_CTRL2: c_uint = 0x9c;
pub const QSERDES_V6_20_RX_IVCM_POSTCAL_OFFSET: c_uint = 0xa0;
pub const QSERDES_V6_20_RX_DFE_1: c_uint = 0xac;
pub const QSERDES_V6_20_RX_DFE_2: c_uint = 0xb0;
pub const QSERDES_V6_20_RX_DFE_3: c_uint = 0xb4;
pub const QSERDES_V6_20_RX_TX_ADPT_CTRL: c_uint = 0xd4;
pub const QSERDES_V6_20_VGA_CAL_CNTRL1: c_uint = 0xe0;
pub const QSERDES_V6_20_RX_VGA_CAL_MAN_VAL: c_uint = 0xe8;
pub const QSERDES_V6_20_RX_GM_CAL: c_uint = 0x10c;
pub const QSERDES_V6_20_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0x120;
pub const QSERDES_V6_20_RX_SIGDET_ENABLES: c_uint = 0x148;
pub const QSERDES_V6_20_RX_PHPRE_CTRL: c_uint = 0x188;
pub const QSERDES_V6_20_RX_DFE_CTLE_POST_CAL_OFFSET: c_uint = 0x194;
pub const QSERDES_V6_20_RX_Q_PI_INTRINSIC_BIAS_RATE32: c_uint = 0x1dc;
pub const QSERDES_V6_20_RX_MODE_RATE2_B0: c_uint = 0x1f4;
pub const QSERDES_V6_20_RX_MODE_RATE2_B1: c_uint = 0x1f8;
pub const QSERDES_V6_20_RX_MODE_RATE2_B2: c_uint = 0x1fc;
pub const QSERDES_V6_20_RX_MODE_RATE2_B3: c_uint = 0x200;
pub const QSERDES_V6_20_RX_MODE_RATE2_B4: c_uint = 0x204;
pub const QSERDES_V6_20_RX_MODE_RATE2_B5: c_uint = 0x208;
pub const QSERDES_V6_20_RX_MODE_RATE2_B6: c_uint = 0x20c;
pub const QSERDES_V6_20_RX_MODE_RATE3_B0: c_uint = 0x210;
pub const QSERDES_V6_20_RX_MODE_RATE3_B1: c_uint = 0x214;
pub const QSERDES_V6_20_RX_MODE_RATE3_B2: c_uint = 0x218;
pub const QSERDES_V6_20_RX_MODE_RATE3_B3: c_uint = 0x21c;
pub const QSERDES_V6_20_RX_MODE_RATE3_B4: c_uint = 0x220;
pub const QSERDES_V6_20_RX_MODE_RATE3_B5: c_uint = 0x224;
pub const QSERDES_V6_20_RX_MODE_RATE3_B6: c_uint = 0x228;
pub const QSERDES_V6_20_RX_BKUP_CTRL1: c_uint = 0x22c;

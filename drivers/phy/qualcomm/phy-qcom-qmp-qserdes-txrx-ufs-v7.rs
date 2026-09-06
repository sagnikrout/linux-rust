//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-ufs-v7.h
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
// Copyright (c) 2024, Linaro Limited
//
pub const QSERDES_UFS_V7_TX_RES_CODE_LANE_TX: c_uint = 0x28;
pub const QSERDES_UFS_V7_TX_RES_CODE_LANE_RX: c_uint = 0x2c;
pub const QSERDES_UFS_V7_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x30;
pub const QSERDES_UFS_V7_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x34;
pub const QSERDES_UFS_V7_TX_LANE_MODE_1: c_uint = 0x7c;
pub const QSERDES_UFS_V7_TX_FR_DCC_CTRL: c_uint = 0x108;
pub const QSERDES_UFS_V7_RX_UCDR_FASTLOCK_FO_GAIN_RATE4: c_uint = 0x10;
pub const QSERDES_UFS_V7_RX_UCDR_FASTLOCK_SO_GAIN_RATE4: c_uint = 0x24;
pub const QSERDES_UFS_V7_RX_UCDR_SO_SATURATION: c_uint = 0x28;
pub const QSERDES_UFS_V7_RX_UCDR_FASTLOCK_COUNT_HIGH_RATE4: c_uint = 0x54;
pub const QSERDES_UFS_V7_RX_UCDR_PI_CTRL1: c_uint = 0x58;
pub const QSERDES_UFS_V7_RX_TERM_BW_CTRL0: c_uint = 0xc4;
pub const QSERDES_UFS_V7_RX_UCDR_FO_GAIN_RATE2: c_uint = 0xd4;
pub const QSERDES_UFS_V7_RX_UCDR_FO_GAIN_RATE4: c_uint = 0xdc;
pub const QSERDES_UFS_V7_RX_UCDR_SO_GAIN_RATE4: c_uint = 0xf0;
pub const QSERDES_UFS_V7_RX_UCDR_PI_CONTROLS: c_uint = 0xf4;
pub const QSERDES_UFS_V7_RX_VGA_CAL_MAN_VAL: c_uint = 0x178;
pub const QSERDES_UFS_V7_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0x1b4;
pub const QSERDES_UFS_V7_RX_EQ_OFFSET_ADAPTOR_CNTRL1: c_uint = 0x1cc;
pub const QSERDES_UFS_V7_RX_OFFSET_ADAPTOR_CNTRL3: c_uint = 0x1d4;
pub const QSERDES_UFS_V7_RX_INTERFACE_MODE: c_uint = 0x1f0;
pub const QSERDES_UFS_V7_RX_MODE_RATE_0_1_B0: c_uint = 0x218;
pub const QSERDES_UFS_V7_RX_MODE_RATE_0_1_B1: c_uint = 0x21C;
pub const QSERDES_UFS_V7_RX_MODE_RATE_0_1_B2: c_uint = 0x220;
pub const QSERDES_UFS_V7_RX_MODE_RATE_0_1_B3: c_uint = 0x224;
pub const QSERDES_UFS_V7_RX_MODE_RATE_0_1_B4: c_uint = 0x228;
pub const QSERDES_UFS_V7_RX_MODE_RATE_0_1_B6: c_uint = 0x230;
pub const QSERDES_UFS_V7_RX_MODE_RATE_0_1_B7: c_uint = 0x234;
pub const QSERDES_UFS_V7_RX_MODE_RATE2_B3: c_uint = 0x248;
pub const QSERDES_UFS_V7_RX_MODE_RATE2_B6: c_uint = 0x254;
pub const QSERDES_UFS_V7_RX_MODE_RATE2_B7: c_uint = 0x258;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B0: c_uint = 0x260;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B1: c_uint = 0x264;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B2: c_uint = 0x268;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B3: c_uint = 0x26c;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B4: c_uint = 0x270;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B5: c_uint = 0x274;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B7: c_uint = 0x27c;
pub const QSERDES_UFS_V7_RX_MODE_RATE3_B8: c_uint = 0x280;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B0: c_uint = 0x284;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B1: c_uint = 0x288;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B2: c_uint = 0x28c;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B3: c_uint = 0x290;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B4: c_uint = 0x294;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B5: c_uint = 0x298;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B6: c_uint = 0x29c;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SA_B7: c_uint = 0x2a0;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B0: c_uint = 0x2a8;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B1: c_uint = 0x2ac;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B2: c_uint = 0x2b0;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B3: c_uint = 0x2b4;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B4: c_uint = 0x2b8;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B5: c_uint = 0x2bc;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B6: c_uint = 0x2c0;
pub const QSERDES_UFS_V7_RX_MODE_RATE4_SB_B7: c_uint = 0x2c4;
pub const QSERDES_UFS_V7_RX_DLL0_FTUNE_CTRL: c_uint = 0x348;
pub const QSERDES_UFS_V7_RX_SIGDET_CAL_TRIM: c_uint = 0x380;

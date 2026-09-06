//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-ufs-v6.h
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
pub const QSERDES_UFS_V6_TX_RES_CODE_LANE_TX: c_uint = 0x28;
pub const QSERDES_UFS_V6_TX_RES_CODE_LANE_RX: c_uint = 0x2c;
pub const QSERDES_UFS_V6_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x30;
pub const QSERDES_UFS_V6_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x34;
pub const QSERDES_UFS_V6_TX_LANE_MODE_1: c_uint = 0x7c;
pub const QSERDES_UFS_V6_TX_FR_DCC_CTRL: c_uint = 0x108;
pub const QSERDES_UFS_V6_RX_UCDR_FASTLOCK_FO_GAIN_RATE2: c_uint = 0x08;
pub const QSERDES_UFS_V6_RX_UCDR_FASTLOCK_FO_GAIN_RATE4: c_uint = 0x10;
pub const QSERDES_UFS_V6_RX_UCDR_FASTLOCK_SO_GAIN_RATE4: c_uint = 0x24;
pub const QSERDES_UFS_V6_RX_UCDR_SO_SATURATION: c_uint = 0x28;
pub const QSERDES_UFS_V6_RX_UCDR_FASTLOCK_COUNT_HIGH_RATE4: c_uint = 0x54;
pub const QSERDES_UFS_V6_RX_UCDR_PI_CTRL1: c_uint = 0x58;
pub const QSERDES_UFS_V6_RX_RX_TERM_BW_CTRL0: c_uint = 0xc4;
pub const QSERDES_UFS_V6_RX_UCDR_FO_GAIN_RATE2: c_uint = 0xd4;
pub const QSERDES_UFS_V6_RX_UCDR_FO_GAIN_RATE4: c_uint = 0xdc;
pub const QSERDES_UFS_V6_RX_UCDR_SO_GAIN_RATE4: c_uint = 0xf0;
pub const QSERDES_UFS_V6_RX_UCDR_PI_CONTROLS: c_uint = 0xf4;
pub const QSERDES_UFS_V6_RX_VGA_CAL_MAN_VAL: c_uint = 0x178;
pub const QSERDES_UFS_V6_RX_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0x1ac;
pub const QSERDES_UFS_V6_RX_EQ_OFFSET_ADAPTOR_CNTRL1: c_uint = 0x1bc;
pub const QSERDES_UFS_V6_RX_INTERFACE_MODE: c_uint = 0x1e0;
pub const QSERDES_UFS_V6_RX_OFFSET_ADAPTOR_CNTRL3: c_uint = 0x1c4;
pub const QSERDES_UFS_V6_RX_MODE_RATE_0_1_B0: c_uint = 0x208;
pub const QSERDES_UFS_V6_RX_MODE_RATE_0_1_B1: c_uint = 0x20c;
pub const QSERDES_UFS_V6_RX_MODE_RATE_0_1_B2: c_uint = 0x210;
pub const QSERDES_UFS_V6_RX_MODE_RATE_0_1_B3: c_uint = 0x214;
pub const QSERDES_UFS_V6_RX_MODE_RATE_0_1_B4: c_uint = 0x218;
pub const QSERDES_UFS_V6_RX_MODE_RATE_0_1_B6: c_uint = 0x220;
pub const QSERDES_UFS_V6_RX_MODE_RATE2_B3: c_uint = 0x238;
pub const QSERDES_UFS_V6_RX_MODE_RATE2_B6: c_uint = 0x244;
pub const QSERDES_UFS_V6_RX_MODE_RATE3_B3: c_uint = 0x25c;
pub const QSERDES_UFS_V6_RX_MODE_RATE3_B4: c_uint = 0x260;
pub const QSERDES_UFS_V6_RX_MODE_RATE3_B5: c_uint = 0x264;
pub const QSERDES_UFS_V6_RX_MODE_RATE3_B8: c_uint = 0x270;
pub const QSERDES_UFS_V6_RX_MODE_RATE4_B0: c_uint = 0x274;
pub const QSERDES_UFS_V6_RX_MODE_RATE4_B1: c_uint = 0x278;
pub const QSERDES_UFS_V6_RX_MODE_RATE4_B2: c_uint = 0x27c;
pub const QSERDES_UFS_V6_RX_MODE_RATE4_B3: c_uint = 0x280;
pub const QSERDES_UFS_V6_RX_MODE_RATE4_B4: c_uint = 0x284;
pub const QSERDES_UFS_V6_RX_MODE_RATE4_B6: c_uint = 0x28c;
pub const QSERDES_UFS_V6_RX_DLL0_FTUNE_CTRL: c_uint = 0x2f8;

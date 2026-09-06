//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/mc_pll_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// MC_PLL (Prototype: PLL)
//
pub const mmMC_PLL_NR: c_uint = 0x4A1100;
pub const mmMC_PLL_NF: c_uint = 0x4A1104;
pub const mmMC_PLL_OD: c_uint = 0x4A1108;
pub const mmMC_PLL_NB: c_uint = 0x4A110C;
pub const mmMC_PLL_CFG: c_uint = 0x4A1110;
pub const mmMC_PLL_LOSE_MASK: c_uint = 0x4A1120;
pub const mmMC_PLL_LOCK_INTR: c_uint = 0x4A1128;
pub const mmMC_PLL_LOCK_BYPASS: c_uint = 0x4A112C;
pub const mmMC_PLL_DATA_CHNG: c_uint = 0x4A1130;
pub const mmMC_PLL_RST: c_uint = 0x4A1134;
pub const mmMC_PLL_SLIP_WD_CNTR: c_uint = 0x4A1150;
pub const mmMC_PLL_DIV_FACTOR_0: c_uint = 0x4A1200;
pub const mmMC_PLL_DIV_FACTOR_1: c_uint = 0x4A1204;
pub const mmMC_PLL_DIV_FACTOR_2: c_uint = 0x4A1208;
pub const mmMC_PLL_DIV_FACTOR_3: c_uint = 0x4A120C;
pub const mmMC_PLL_DIV_FACTOR_CMD_0: c_uint = 0x4A1220;
pub const mmMC_PLL_DIV_FACTOR_CMD_1: c_uint = 0x4A1224;
pub const mmMC_PLL_DIV_FACTOR_CMD_2: c_uint = 0x4A1228;
pub const mmMC_PLL_DIV_FACTOR_CMD_3: c_uint = 0x4A122C;
pub const mmMC_PLL_DIV_SEL_0: c_uint = 0x4A1280;
pub const mmMC_PLL_DIV_SEL_1: c_uint = 0x4A1284;
pub const mmMC_PLL_DIV_SEL_2: c_uint = 0x4A1288;
pub const mmMC_PLL_DIV_SEL_3: c_uint = 0x4A128C;
pub const mmMC_PLL_DIV_EN_0: c_uint = 0x4A12A0;
pub const mmMC_PLL_DIV_EN_1: c_uint = 0x4A12A4;
pub const mmMC_PLL_DIV_EN_2: c_uint = 0x4A12A8;
pub const mmMC_PLL_DIV_EN_3: c_uint = 0x4A12AC;
pub const mmMC_PLL_DIV_FACTOR_BUSY_0: c_uint = 0x4A12C0;
pub const mmMC_PLL_DIV_FACTOR_BUSY_1: c_uint = 0x4A12C4;
pub const mmMC_PLL_DIV_FACTOR_BUSY_2: c_uint = 0x4A12C8;
pub const mmMC_PLL_DIV_FACTOR_BUSY_3: c_uint = 0x4A12CC;
pub const mmMC_PLL_CLK_GATER: c_uint = 0x4A1300;
pub const mmMC_PLL_CLK_RLX_0: c_uint = 0x4A1310;
pub const mmMC_PLL_CLK_RLX_1: c_uint = 0x4A1314;
pub const mmMC_PLL_CLK_RLX_2: c_uint = 0x4A1318;
pub const mmMC_PLL_CLK_RLX_3: c_uint = 0x4A131C;
pub const mmMC_PLL_REF_CNTR_PERIOD: c_uint = 0x4A1400;
pub const mmMC_PLL_REF_LOW_THRESHOLD: c_uint = 0x4A1410;
pub const mmMC_PLL_REF_HIGH_THRESHOLD: c_uint = 0x4A1420;
pub const mmMC_PLL_PLL_NOT_STABLE: c_uint = 0x4A1430;
pub const mmMC_PLL_FREQ_CALC_EN: c_uint = 0x4A1440;

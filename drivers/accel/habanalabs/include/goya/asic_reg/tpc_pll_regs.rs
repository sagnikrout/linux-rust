//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/tpc_pll_regs.h
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
// TPC_PLL (Prototype: PLL)
//
pub const mmTPC_PLL_NR: c_uint = 0xE01100;
pub const mmTPC_PLL_NF: c_uint = 0xE01104;
pub const mmTPC_PLL_OD: c_uint = 0xE01108;
pub const mmTPC_PLL_NB: c_uint = 0xE0110C;
pub const mmTPC_PLL_CFG: c_uint = 0xE01110;
pub const mmTPC_PLL_LOSE_MASK: c_uint = 0xE01120;
pub const mmTPC_PLL_LOCK_INTR: c_uint = 0xE01128;
pub const mmTPC_PLL_LOCK_BYPASS: c_uint = 0xE0112C;
pub const mmTPC_PLL_DATA_CHNG: c_uint = 0xE01130;
pub const mmTPC_PLL_RST: c_uint = 0xE01134;
pub const mmTPC_PLL_SLIP_WD_CNTR: c_uint = 0xE01150;
pub const mmTPC_PLL_DIV_FACTOR_0: c_uint = 0xE01200;
pub const mmTPC_PLL_DIV_FACTOR_1: c_uint = 0xE01204;
pub const mmTPC_PLL_DIV_FACTOR_2: c_uint = 0xE01208;
pub const mmTPC_PLL_DIV_FACTOR_3: c_uint = 0xE0120C;
pub const mmTPC_PLL_DIV_FACTOR_CMD_0: c_uint = 0xE01220;
pub const mmTPC_PLL_DIV_FACTOR_CMD_1: c_uint = 0xE01224;
pub const mmTPC_PLL_DIV_FACTOR_CMD_2: c_uint = 0xE01228;
pub const mmTPC_PLL_DIV_FACTOR_CMD_3: c_uint = 0xE0122C;
pub const mmTPC_PLL_DIV_SEL_0: c_uint = 0xE01280;
pub const mmTPC_PLL_DIV_SEL_1: c_uint = 0xE01284;
pub const mmTPC_PLL_DIV_SEL_2: c_uint = 0xE01288;
pub const mmTPC_PLL_DIV_SEL_3: c_uint = 0xE0128C;
pub const mmTPC_PLL_DIV_EN_0: c_uint = 0xE012A0;
pub const mmTPC_PLL_DIV_EN_1: c_uint = 0xE012A4;
pub const mmTPC_PLL_DIV_EN_2: c_uint = 0xE012A8;
pub const mmTPC_PLL_DIV_EN_3: c_uint = 0xE012AC;
pub const mmTPC_PLL_DIV_FACTOR_BUSY_0: c_uint = 0xE012C0;
pub const mmTPC_PLL_DIV_FACTOR_BUSY_1: c_uint = 0xE012C4;
pub const mmTPC_PLL_DIV_FACTOR_BUSY_2: c_uint = 0xE012C8;
pub const mmTPC_PLL_DIV_FACTOR_BUSY_3: c_uint = 0xE012CC;
pub const mmTPC_PLL_CLK_GATER: c_uint = 0xE01300;
pub const mmTPC_PLL_CLK_RLX_0: c_uint = 0xE01310;
pub const mmTPC_PLL_CLK_RLX_1: c_uint = 0xE01314;
pub const mmTPC_PLL_CLK_RLX_2: c_uint = 0xE01318;
pub const mmTPC_PLL_CLK_RLX_3: c_uint = 0xE0131C;
pub const mmTPC_PLL_REF_CNTR_PERIOD: c_uint = 0xE01400;
pub const mmTPC_PLL_REF_LOW_THRESHOLD: c_uint = 0xE01410;
pub const mmTPC_PLL_REF_HIGH_THRESHOLD: c_uint = 0xE01420;
pub const mmTPC_PLL_PLL_NOT_STABLE: c_uint = 0xE01430;
pub const mmTPC_PLL_FREQ_CALC_EN: c_uint = 0xE01440;

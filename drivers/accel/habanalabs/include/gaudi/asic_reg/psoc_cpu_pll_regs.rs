//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi/asic_reg/psoc_cpu_pll_regs.h
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
// PSOC_CPU_PLL (Prototype: PLL)
//
pub const mmPSOC_CPU_PLL_NR: c_uint = 0xC70100;
pub const mmPSOC_CPU_PLL_NF: c_uint = 0xC70104;
pub const mmPSOC_CPU_PLL_OD: c_uint = 0xC70108;
pub const mmPSOC_CPU_PLL_NB: c_uint = 0xC7010C;
pub const mmPSOC_CPU_PLL_CFG: c_uint = 0xC70110;
pub const mmPSOC_CPU_PLL_LOSE_MASK: c_uint = 0xC70120;
pub const mmPSOC_CPU_PLL_LOCK_INTR: c_uint = 0xC70128;
pub const mmPSOC_CPU_PLL_LOCK_BYPASS: c_uint = 0xC7012C;
pub const mmPSOC_CPU_PLL_DATA_CHNG: c_uint = 0xC70130;
pub const mmPSOC_CPU_PLL_RST: c_uint = 0xC70134;
pub const mmPSOC_CPU_PLL_SLIP_WD_CNTR: c_uint = 0xC70150;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_0: c_uint = 0xC70200;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_1: c_uint = 0xC70204;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_2: c_uint = 0xC70208;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_3: c_uint = 0xC7020C;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_CMD_0: c_uint = 0xC70220;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_CMD_1: c_uint = 0xC70224;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_CMD_2: c_uint = 0xC70228;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_CMD_3: c_uint = 0xC7022C;
pub const mmPSOC_CPU_PLL_DIV_SEL_0: c_uint = 0xC70280;
pub const mmPSOC_CPU_PLL_DIV_SEL_1: c_uint = 0xC70284;
pub const mmPSOC_CPU_PLL_DIV_SEL_2: c_uint = 0xC70288;
pub const mmPSOC_CPU_PLL_DIV_SEL_3: c_uint = 0xC7028C;
pub const mmPSOC_CPU_PLL_DIV_EN_0: c_uint = 0xC702A0;
pub const mmPSOC_CPU_PLL_DIV_EN_1: c_uint = 0xC702A4;
pub const mmPSOC_CPU_PLL_DIV_EN_2: c_uint = 0xC702A8;
pub const mmPSOC_CPU_PLL_DIV_EN_3: c_uint = 0xC702AC;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_BUSY_0: c_uint = 0xC702C0;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_BUSY_1: c_uint = 0xC702C4;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_BUSY_2: c_uint = 0xC702C8;
pub const mmPSOC_CPU_PLL_DIV_FACTOR_BUSY_3: c_uint = 0xC702CC;
pub const mmPSOC_CPU_PLL_CLK_GATER: c_uint = 0xC70300;
pub const mmPSOC_CPU_PLL_CLK_RLX_0: c_uint = 0xC70310;
pub const mmPSOC_CPU_PLL_CLK_RLX_1: c_uint = 0xC70314;
pub const mmPSOC_CPU_PLL_CLK_RLX_2: c_uint = 0xC70318;
pub const mmPSOC_CPU_PLL_CLK_RLX_3: c_uint = 0xC7031C;
pub const mmPSOC_CPU_PLL_REF_CNTR_PERIOD: c_uint = 0xC70400;
pub const mmPSOC_CPU_PLL_REF_LOW_THRESHOLD: c_uint = 0xC70410;
pub const mmPSOC_CPU_PLL_REF_HIGH_THRESHOLD: c_uint = 0xC70420;
pub const mmPSOC_CPU_PLL_PLL_NOT_STABLE: c_uint = 0xC70430;
pub const mmPSOC_CPU_PLL_FREQ_CALC_EN: c_uint = 0xC70440;
pub const mmPSOC_CPU_PLL_RLX_BITMAP_CFG: c_uint = 0xC70500;
pub const mmPSOC_CPU_PLL_RLX_BITMAP_0: c_uint = 0xC70510;
pub const mmPSOC_CPU_PLL_RLX_BITMAP_1: c_uint = 0xC70514;
pub const mmPSOC_CPU_PLL_RLX_BITMAP_2: c_uint = 0xC70518;
pub const mmPSOC_CPU_PLL_RLX_BITMAP_3: c_uint = 0xC7051C;

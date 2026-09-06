//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/ic_pll_regs.h
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
// IC_PLL (Prototype: PLL)
//
pub const mmIC_PLL_NR: c_uint = 0x4A3100;
pub const mmIC_PLL_NF: c_uint = 0x4A3104;
pub const mmIC_PLL_OD: c_uint = 0x4A3108;
pub const mmIC_PLL_NB: c_uint = 0x4A310C;
pub const mmIC_PLL_CFG: c_uint = 0x4A3110;
pub const mmIC_PLL_LOSE_MASK: c_uint = 0x4A3120;
pub const mmIC_PLL_LOCK_INTR: c_uint = 0x4A3128;
pub const mmIC_PLL_LOCK_BYPASS: c_uint = 0x4A312C;
pub const mmIC_PLL_DATA_CHNG: c_uint = 0x4A3130;
pub const mmIC_PLL_RST: c_uint = 0x4A3134;
pub const mmIC_PLL_SLIP_WD_CNTR: c_uint = 0x4A3150;
pub const mmIC_PLL_DIV_FACTOR_0: c_uint = 0x4A3200;
pub const mmIC_PLL_DIV_FACTOR_1: c_uint = 0x4A3204;
pub const mmIC_PLL_DIV_FACTOR_2: c_uint = 0x4A3208;
pub const mmIC_PLL_DIV_FACTOR_3: c_uint = 0x4A320C;
pub const mmIC_PLL_DIV_FACTOR_CMD_0: c_uint = 0x4A3220;
pub const mmIC_PLL_DIV_FACTOR_CMD_1: c_uint = 0x4A3224;
pub const mmIC_PLL_DIV_FACTOR_CMD_2: c_uint = 0x4A3228;
pub const mmIC_PLL_DIV_FACTOR_CMD_3: c_uint = 0x4A322C;
pub const mmIC_PLL_DIV_SEL_0: c_uint = 0x4A3280;
pub const mmIC_PLL_DIV_SEL_1: c_uint = 0x4A3284;
pub const mmIC_PLL_DIV_SEL_2: c_uint = 0x4A3288;
pub const mmIC_PLL_DIV_SEL_3: c_uint = 0x4A328C;
pub const mmIC_PLL_DIV_EN_0: c_uint = 0x4A32A0;
pub const mmIC_PLL_DIV_EN_1: c_uint = 0x4A32A4;
pub const mmIC_PLL_DIV_EN_2: c_uint = 0x4A32A8;
pub const mmIC_PLL_DIV_EN_3: c_uint = 0x4A32AC;
pub const mmIC_PLL_DIV_FACTOR_BUSY_0: c_uint = 0x4A32C0;
pub const mmIC_PLL_DIV_FACTOR_BUSY_1: c_uint = 0x4A32C4;
pub const mmIC_PLL_DIV_FACTOR_BUSY_2: c_uint = 0x4A32C8;
pub const mmIC_PLL_DIV_FACTOR_BUSY_3: c_uint = 0x4A32CC;
pub const mmIC_PLL_CLK_GATER: c_uint = 0x4A3300;
pub const mmIC_PLL_CLK_RLX_0: c_uint = 0x4A3310;
pub const mmIC_PLL_CLK_RLX_1: c_uint = 0x4A3314;
pub const mmIC_PLL_CLK_RLX_2: c_uint = 0x4A3318;
pub const mmIC_PLL_CLK_RLX_3: c_uint = 0x4A331C;
pub const mmIC_PLL_REF_CNTR_PERIOD: c_uint = 0x4A3400;
pub const mmIC_PLL_REF_LOW_THRESHOLD: c_uint = 0x4A3410;
pub const mmIC_PLL_REF_HIGH_THRESHOLD: c_uint = 0x4A3420;
pub const mmIC_PLL_PLL_NOT_STABLE: c_uint = 0x4A3430;
pub const mmIC_PLL_FREQ_CALC_EN: c_uint = 0x4A3440;

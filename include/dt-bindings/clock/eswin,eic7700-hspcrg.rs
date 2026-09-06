//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/eswin,eic7700-hspcrg.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd..
// All rights reserved.
//
// Device Tree binding constants for EIC7700 HSP clock controller.
//
// Authors: Xuyang Dong <dongxuyang@eswincomputing.com>
//
pub const EIC7700_HSP_CLK_FAC_CFG_DIV2: c_int = 0;
pub const EIC7700_HSP_CLK_FAC_CFG_DIV4: c_int = 1;
pub const EIC7700_HSP_CLK_FAC_MMC_DIV10: c_int = 2;
pub const EIC7700_HSP_CLK_MUX_EMMC_3MUX1: c_int = 3;
pub const EIC7700_HSP_CLK_MUX_SD0_3MUX1: c_int = 4;
pub const EIC7700_HSP_CLK_MUX_SD1_3MUX1: c_int = 5;
pub const EIC7700_HSP_CLK_MUX_EMMC_CQE_2MUX1: c_int = 6;
pub const EIC7700_HSP_CLK_MUX_SD0_CQE_2MUX1: c_int = 7;
pub const EIC7700_HSP_CLK_MUX_SD1_CQE_2MUX1: c_int = 8;
pub const EIC7700_HSP_CLK_GATE_MSHC0_TMR: c_int = 9;
pub const EIC7700_HSP_CLK_GATE_EMMC: c_int = 10;
pub const EIC7700_HSP_CLK_GATE_MSHC1_TMR: c_int = 11;
pub const EIC7700_HSP_CLK_GATE_SD0: c_int = 12;
pub const EIC7700_HSP_CLK_GATE_MSHC2_TMR: c_int = 13;
pub const EIC7700_HSP_CLK_GATE_SD1: c_int = 14;
pub const EIC7700_HSP_CLK_GATE_USB0: c_int = 15;
pub const EIC7700_HSP_CLK_GATE_USB1: c_int = 16;
pub const EIC7700_HSP_CLK_GATE_SATA: c_int = 17;

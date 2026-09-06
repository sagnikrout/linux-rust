//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-com-v10_60.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// Only for QMP V10_60 PHY - QSERDES COM registers
pub const QSERDES_V10_60_COM_SSC_STEP_SIZE1_MODE1: c_uint = 0x00;
pub const QSERDES_V10_60_COM_SSC_STEP_SIZE2_MODE1: c_uint = 0x04;
pub const QSERDES_V10_60_COM_CP_CTRL_MODE1: c_uint = 0x10;
pub const QSERDES_V10_60_COM_PLL_RCTRL_MODE1: c_uint = 0x14;
pub const QSERDES_V10_60_COM_PLL_CCTRL_MODE1: c_uint = 0x18;
pub const QSERDES_V10_60_COM_CORECLK_DIV_MODE1: c_uint = 0x1c;
pub const QSERDES_V10_60_COM_LOCK_CMP1_MODE1: c_uint = 0x20;
pub const QSERDES_V10_60_COM_LOCK_CMP2_MODE1: c_uint = 0x24;
pub const QSERDES_V10_60_COM_DEC_START_MODE1: c_uint = 0x28;
pub const QSERDES_V10_60_COM_DIV_FRAC_START1_MODE1: c_uint = 0x30;
pub const QSERDES_V10_60_COM_DIV_FRAC_START2_MODE1: c_uint = 0x34;
pub const QSERDES_V10_60_COM_DIV_FRAC_START3_MODE1: c_uint = 0x38;
pub const QSERDES_V10_60_COM_HSCLK_SEL_1: c_uint = 0x3c;
pub const QSERDES_V10_60_COM_SSC_STEP_SIZE1_MODE0: c_uint = 0x60;
pub const QSERDES_V10_60_COM_SSC_STEP_SIZE2_MODE0: c_uint = 0x64;
pub const QSERDES_V10_60_COM_CP_CTRL_MODE0: c_uint = 0x70;
pub const QSERDES_V10_60_COM_PLL_RCTRL_MODE0: c_uint = 0x74;
pub const QSERDES_V10_60_COM_PLL_CCTRL_MODE0: c_uint = 0x78;
pub const QSERDES_V10_60_COM_CORECLK_DIV_MODE0: c_uint = 0x7c;
pub const QSERDES_V10_60_COM_LOCK_CMP1_MODE0: c_uint = 0x80;
pub const QSERDES_V10_60_COM_LOCK_CMP2_MODE0: c_uint = 0x84;
pub const QSERDES_V10_60_COM_DEC_START_MODE0: c_uint = 0x88;
pub const QSERDES_V10_60_COM_DIV_FRAC_START1_MODE0: c_uint = 0x90;
pub const QSERDES_V10_60_COM_DIV_FRAC_START2_MODE0: c_uint = 0x94;
pub const QSERDES_V10_60_COM_DIV_FRAC_START3_MODE0: c_uint = 0x98;
pub const QSERDES_V10_60_COM_HSCLK_HS_SWITCH_SEL_1: c_uint = 0x9c;
pub const QSERDES_V10_60_COM_BG_TIMER: c_uint = 0xbc;
pub const QSERDES_V10_60_COM_SSC_PER1: c_uint = 0xcc;
pub const QSERDES_V10_60_COM_SSC_PER2: c_uint = 0xd0;
pub const QSERDES_V10_60_COM_BIAS_EN_CLKBUFLR_EN: c_uint = 0xdc;
pub const QSERDES_V10_60_COM_CLK_ENABLE1: c_uint = 0xe0;
pub const QSERDES_V10_60_COM_SYS_CLK_CTRL: c_uint = 0xe4;
pub const QSERDES_V10_60_COM_PLL_IVCO: c_uint = 0xf4;
pub const QSERDES_V10_60_COM_SYSCLK_EN_SEL: c_uint = 0x110;
pub const QSERDES_V10_60_COM_LOCK_CMP_EN: c_uint = 0x120;
pub const QSERDES_V10_60_COM_LOCK_CMP_CFG: c_uint = 0x124;
pub const QSERDES_V10_60_COM_VCO_TUNE_MAP: c_uint = 0x140;
pub const QSERDES_V10_60_COM_CLK_SELECT: c_uint = 0x164;
pub const QSERDES_V10_60_COM_CORE_CLK_EN: c_uint = 0x170;
pub const QSERDES_V10_60_COM_CMN_CONFIG_1: c_uint = 0x174;
pub const QSERDES_V10_60_COM_CMN_MISC1: c_uint = 0x184;
pub const QSERDES_V10_60_COM_CMN_MODE: c_uint = 0x188;
pub const QSERDES_V10_60_COM_VCO_DC_LEVEL_CTRL: c_uint = 0x198;
pub const QSERDES_V10_60_COM_PLL_SPARE_FOR_ECO: c_uint = 0x2b4;

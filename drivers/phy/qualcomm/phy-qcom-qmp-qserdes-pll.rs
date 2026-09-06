//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-pll.h
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
// QMP V2 PHY for PCIE gen3 ports - QSERDES PLL registers
pub const QSERDES_PLL_BG_TIMER: c_uint = 0x00c;
pub const QSERDES_PLL_SSC_EN_CENTER: c_uint = 0x010;
pub const QSERDES_PLL_SSC_ADJ_PER1: c_uint = 0x014;
pub const QSERDES_PLL_SSC_ADJ_PER2: c_uint = 0x018;
pub const QSERDES_PLL_SSC_PER1: c_uint = 0x01c;
pub const QSERDES_PLL_SSC_PER2: c_uint = 0x020;
pub const QSERDES_PLL_SSC_STEP_SIZE1_MODE0: c_uint = 0x024;
pub const QSERDES_PLL_SSC_STEP_SIZE2_MODE0: c_uint = 0x028;
pub const QSERDES_PLL_SSC_STEP_SIZE1_MODE1: c_uint = 0x02c;
pub const QSERDES_PLL_SSC_STEP_SIZE2_MODE1: c_uint = 0x030;
pub const QSERDES_PLL_BIAS_EN_CLKBUFLR_EN: c_uint = 0x03c;
pub const QSERDES_PLL_CLK_ENABLE1: c_uint = 0x040;
pub const QSERDES_PLL_SYS_CLK_CTRL: c_uint = 0x044;
pub const QSERDES_PLL_SYSCLK_BUF_ENABLE: c_uint = 0x048;
pub const QSERDES_PLL_PLL_IVCO: c_uint = 0x050;
pub const QSERDES_PLL_LOCK_CMP1_MODE0: c_uint = 0x054;
pub const QSERDES_PLL_LOCK_CMP2_MODE0: c_uint = 0x058;
pub const QSERDES_PLL_LOCK_CMP1_MODE1: c_uint = 0x060;
pub const QSERDES_PLL_LOCK_CMP2_MODE1: c_uint = 0x064;
pub const QSERDES_PLL_BG_TRIM: c_uint = 0x074;
pub const QSERDES_PLL_CLK_EP_DIV_MODE0: c_uint = 0x078;
pub const QSERDES_PLL_CLK_EP_DIV_MODE1: c_uint = 0x07c;
pub const QSERDES_PLL_CP_CTRL_MODE0: c_uint = 0x080;
pub const QSERDES_PLL_CP_CTRL_MODE1: c_uint = 0x084;
pub const QSERDES_PLL_PLL_RCTRL_MODE0: c_uint = 0x088;
pub const QSERDES_PLL_PLL_RCTRL_MODE1: c_uint = 0x08c;
pub const QSERDES_PLL_PLL_CCTRL_MODE0: c_uint = 0x090;
pub const QSERDES_PLL_PLL_CCTRL_MODE1: c_uint = 0x094;
pub const QSERDES_PLL_BIAS_EN_CTRL_BY_PSM: c_uint = 0x0a4;
pub const QSERDES_PLL_SYSCLK_EN_SEL: c_uint = 0x0a8;
pub const QSERDES_PLL_RESETSM_CNTRL: c_uint = 0x0b0;
pub const QSERDES_PLL_LOCK_CMP_EN: c_uint = 0x0c4;
pub const QSERDES_PLL_DEC_START_MODE0: c_uint = 0x0cc;
pub const QSERDES_PLL_DEC_START_MODE1: c_uint = 0x0d0;
pub const QSERDES_PLL_DIV_FRAC_START1_MODE0: c_uint = 0x0d8;
pub const QSERDES_PLL_DIV_FRAC_START2_MODE0: c_uint = 0x0dc;
pub const QSERDES_PLL_DIV_FRAC_START3_MODE0: c_uint = 0x0e0;
pub const QSERDES_PLL_DIV_FRAC_START1_MODE1: c_uint = 0x0e4;
pub const QSERDES_PLL_DIV_FRAC_START2_MODE1: c_uint = 0x0e8;
pub const QSERDES_PLL_DIV_FRAC_START3_MODE1: c_uint = 0x0ec;
pub const QSERDES_PLL_INTEGLOOP_GAIN0_MODE0: c_uint = 0x100;
pub const QSERDES_PLL_INTEGLOOP_GAIN1_MODE0: c_uint = 0x104;
pub const QSERDES_PLL_INTEGLOOP_GAIN0_MODE1: c_uint = 0x108;
pub const QSERDES_PLL_INTEGLOOP_GAIN1_MODE1: c_uint = 0x10c;
pub const QSERDES_PLL_VCO_TUNE_MAP: c_uint = 0x120;
pub const QSERDES_PLL_VCO_TUNE1_MODE0: c_uint = 0x124;
pub const QSERDES_PLL_VCO_TUNE2_MODE0: c_uint = 0x128;
pub const QSERDES_PLL_VCO_TUNE1_MODE1: c_uint = 0x12c;
pub const QSERDES_PLL_VCO_TUNE2_MODE1: c_uint = 0x130;
pub const QSERDES_PLL_VCO_TUNE_TIMER1: c_uint = 0x13c;
pub const QSERDES_PLL_VCO_TUNE_TIMER2: c_uint = 0x140;
pub const QSERDES_PLL_CLK_SELECT: c_uint = 0x16c;
pub const QSERDES_PLL_HSCLK_SEL: c_uint = 0x170;
pub const QSERDES_PLL_CORECLK_DIV: c_uint = 0x17c;
pub const QSERDES_PLL_CORE_CLK_EN: c_uint = 0x184;
pub const QSERDES_PLL_CMN_CONFIG: c_uint = 0x18c;
pub const QSERDES_PLL_SVS_MODE_CLK_SEL: c_uint = 0x194;
pub const QSERDES_PLL_CORECLK_DIV_MODE1: c_uint = 0x1b4;

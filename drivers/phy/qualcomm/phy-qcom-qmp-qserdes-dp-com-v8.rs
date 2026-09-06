//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-dp-com-v8.h
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
// Copyright (C) 2025 Linaro Ltd.
//
// Only for DP QMP V8 PHY - QSERDES COM registers
pub const DP_QSERDES_V8_COM_HSCLK_SEL_1: c_uint = 0x03c;
pub const DP_QSERDES_V8_COM_BIN_VCOCAL_CMP_CODE1_MODE0: c_uint = 0x058;
pub const DP_QSERDES_V8_COM_BIN_VCOCAL_CMP_CODE2_MODE0: c_uint = 0x05c;
pub const DP_QSERDES_V8_COM_SSC_STEP_SIZE1_MODE0: c_uint = 0x060;
pub const DP_QSERDES_V8_COM_SSC_STEP_SIZE2_MODE0: c_uint = 0x064;
pub const DP_QSERDES_V8_COM_CP_CTRL_MODE0: c_uint = 0x070;
pub const DP_QSERDES_V8_COM_PLL_RCTRL_MODE0: c_uint = 0x074;
pub const DP_QSERDES_V8_COM_PLL_CCTRL_MODE0: c_uint = 0x078;
pub const DP_QSERDES_V8_COM_CORECLK_DIV_MODE0: c_uint = 0x07c;
pub const DP_QSERDES_V8_COM_LOCK_CMP1_MODE0: c_uint = 0x080;
pub const DP_QSERDES_V8_COM_LOCK_CMP2_MODE0: c_uint = 0x084;
pub const DP_QSERDES_V8_COM_DEC_START_MODE0: c_uint = 0x088;
pub const DP_QSERDES_V8_COM_DIV_FRAC_START1_MODE0: c_uint = 0x090;
pub const DP_QSERDES_V8_COM_DIV_FRAC_START2_MODE0: c_uint = 0x094;
pub const DP_QSERDES_V8_COM_DIV_FRAC_START3_MODE0: c_uint = 0x098;
pub const DP_QSERDES_V8_COM_INTEGLOOP_GAIN0_MODE0: c_uint = 0x0a0;
pub const DP_QSERDES_V8_COM_VCO_TUNE1_MODE0: c_uint = 0x0a8;
pub const DP_QSERDES_V8_COM_INTEGLOOP_GAIN1_MODE0: c_uint = 0x0a4;
pub const DP_QSERDES_V8_COM_VCO_TUNE2_MODE0: c_uint = 0x0ac;
pub const DP_QSERDES_V8_COM_BG_TIMER: c_uint = 0x0bc;
pub const DP_QSERDES_V8_COM_SSC_EN_CENTER: c_uint = 0x0c0;
pub const DP_QSERDES_V8_COM_SSC_ADJ_PER1: c_uint = 0x0c4;
pub const DP_QSERDES_V8_COM_SSC_PER1: c_uint = 0x0cc;
pub const DP_QSERDES_V8_COM_SSC_PER2: c_uint = 0x0d0;
pub const DP_QSERDES_V8_COM_BIAS_EN_CLKBUFLR_EN: c_uint = 0x0dc;
pub const DP_QSERDES_V8_COM_CLK_ENABLE1: c_uint = 0x0e0;
pub const DP_QSERDES_V8_COM_SYS_CLK_CTRL: c_uint = 0x0e4;
pub const DP_QSERDES_V8_COM_SYSCLK_BUF_ENABLE: c_uint = 0x0e8;
pub const DP_QSERDES_V8_COM_PLL_IVCO: c_uint = 0x0f4;
pub const DP_QSERDES_V8_COM_SYSCLK_EN_SEL: c_uint = 0x110;
pub const DP_QSERDES_V8_COM_RESETSM_CNTRL: c_uint = 0x118;
pub const DP_QSERDES_V8_COM_LOCK_CMP_EN: c_uint = 0x120;
pub const DP_QSERDES_V8_COM_VCO_TUNE_CTRL: c_uint = 0x13c;
pub const DP_QSERDES_V8_COM_VCO_TUNE_MAP: c_uint = 0x140;
pub const DP_QSERDES_V8_COM_CLK_SELECT: c_uint = 0x164;
pub const DP_QSERDES_V8_COM_CORE_CLK_EN: c_uint = 0x170;
pub const DP_QSERDES_V8_COM_CMN_CONFIG_1: c_uint = 0x174;
pub const DP_QSERDES_V8_COM_SVS_MODE_CLK_SEL: c_uint = 0x180;
pub const DP_QSERDES_V8_COM_CLK_FWD_CONFIG_1: c_uint = 0x2f4;
pub const DP_QSERDES_V8_COM_CMN_STATUS: c_uint = 0x314;
pub const DP_QSERDES_V8_COM_C_READY_STATUS: c_uint = 0x33c;

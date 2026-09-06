//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-com-v3.h
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
// Only for QMP V3 PHY - QSERDES COM registers
pub const QSERDES_V3_COM_ATB_SEL1: c_uint = 0x000;
pub const QSERDES_V3_COM_ATB_SEL2: c_uint = 0x004;
pub const QSERDES_V3_COM_FREQ_UPDATE: c_uint = 0x008;
pub const QSERDES_V3_COM_BG_TIMER: c_uint = 0x00c;
pub const QSERDES_V3_COM_SSC_EN_CENTER: c_uint = 0x010;
pub const QSERDES_V3_COM_SSC_ADJ_PER1: c_uint = 0x014;
pub const QSERDES_V3_COM_SSC_ADJ_PER2: c_uint = 0x018;
pub const QSERDES_V3_COM_SSC_PER1: c_uint = 0x01c;
pub const QSERDES_V3_COM_SSC_PER2: c_uint = 0x020;
pub const QSERDES_V3_COM_SSC_STEP_SIZE1: c_uint = 0x024;
pub const QSERDES_V3_COM_SSC_STEP_SIZE2: c_uint = 0x028;
pub const QSERDES_V3_COM_POST_DIV: c_uint = 0x02c;
pub const QSERDES_V3_COM_POST_DIV_MUX: c_uint = 0x030;
pub const QSERDES_V3_COM_BIAS_EN_CLKBUFLR_EN: c_uint = 0x034;
pub const QSERDES_V3_COM_CLK_ENABLE1: c_uint = 0x038;
pub const QSERDES_V3_COM_SYS_CLK_CTRL: c_uint = 0x03c;
pub const QSERDES_V3_COM_SYSCLK_BUF_ENABLE: c_uint = 0x040;
pub const QSERDES_V3_COM_PLL_EN: c_uint = 0x044;
pub const QSERDES_V3_COM_PLL_IVCO: c_uint = 0x048;
pub const QSERDES_V3_COM_CMN_IETRIM: c_uint = 0x04c;
pub const QSERDES_V3_COM_CMN_IPTRIM: c_uint = 0x050;
pub const QSERDES_V3_COM_EP_CLOCK_DETECT_CTR: c_uint = 0x054;
pub const QSERDES_V3_COM_SYSCLK_DET_COMP_STATUS: c_uint = 0x058;
pub const QSERDES_V3_COM_CLK_EP_DIV: c_uint = 0x05c;
pub const QSERDES_V3_COM_CP_CTRL_MODE0: c_uint = 0x060;
pub const QSERDES_V3_COM_CP_CTRL_MODE1: c_uint = 0x064;
pub const QSERDES_V3_COM_PLL_RCTRL_MODE0: c_uint = 0x068;
pub const QSERDES_V3_COM_PLL_RCTRL_MODE1: c_uint = 0x06c;
pub const QSERDES_V3_COM_PLL_CCTRL_MODE0: c_uint = 0x070;
pub const QSERDES_V3_COM_PLL_CCTRL_MODE1: c_uint = 0x074;
pub const QSERDES_V3_COM_PLL_CNTRL: c_uint = 0x078;
pub const QSERDES_V3_COM_BIAS_EN_CTRL_BY_PSM: c_uint = 0x07c;
pub const QSERDES_V3_COM_SYSCLK_EN_SEL: c_uint = 0x080;
pub const QSERDES_V3_COM_CML_SYSCLK_SEL: c_uint = 0x084;
pub const QSERDES_V3_COM_RESETSM_CNTRL: c_uint = 0x088;
pub const QSERDES_V3_COM_RESETSM_CNTRL2: c_uint = 0x08c;
pub const QSERDES_V3_COM_LOCK_CMP_EN: c_uint = 0x090;
pub const QSERDES_V3_COM_LOCK_CMP_CFG: c_uint = 0x094;
pub const QSERDES_V3_COM_LOCK_CMP1_MODE0: c_uint = 0x098;
pub const QSERDES_V3_COM_LOCK_CMP2_MODE0: c_uint = 0x09c;
pub const QSERDES_V3_COM_LOCK_CMP3_MODE0: c_uint = 0x0a0;
pub const QSERDES_V3_COM_LOCK_CMP1_MODE1: c_uint = 0x0a4;
pub const QSERDES_V3_COM_LOCK_CMP2_MODE1: c_uint = 0x0a8;
pub const QSERDES_V3_COM_LOCK_CMP3_MODE1: c_uint = 0x0ac;
pub const QSERDES_V3_COM_DEC_START_MODE0: c_uint = 0x0b0;
pub const QSERDES_V3_COM_DEC_START_MODE1: c_uint = 0x0b4;
pub const QSERDES_V3_COM_DIV_FRAC_START1_MODE0: c_uint = 0x0b8;
pub const QSERDES_V3_COM_DIV_FRAC_START2_MODE0: c_uint = 0x0bc;
pub const QSERDES_V3_COM_DIV_FRAC_START3_MODE0: c_uint = 0x0c0;
pub const QSERDES_V3_COM_DIV_FRAC_START1_MODE1: c_uint = 0x0c4;
pub const QSERDES_V3_COM_DIV_FRAC_START2_MODE1: c_uint = 0x0c8;
pub const QSERDES_V3_COM_DIV_FRAC_START3_MODE1: c_uint = 0x0cc;
pub const QSERDES_V3_COM_INTEGLOOP_INITVAL: c_uint = 0x0d0;
pub const QSERDES_V3_COM_INTEGLOOP_EN: c_uint = 0x0d4;
pub const QSERDES_V3_COM_INTEGLOOP_GAIN0_MODE0: c_uint = 0x0d8;
pub const QSERDES_V3_COM_INTEGLOOP_GAIN1_MODE0: c_uint = 0x0dc;
pub const QSERDES_V3_COM_INTEGLOOP_GAIN0_MODE1: c_uint = 0x0e0;
pub const QSERDES_V3_COM_INTEGLOOP_GAIN1_MODE1: c_uint = 0x0e4;
pub const QSERDES_V3_COM_VCOCAL_DEADMAN_CTRL: c_uint = 0x0e8;
pub const QSERDES_V3_COM_VCO_TUNE_CTRL: c_uint = 0x0ec;
pub const QSERDES_V3_COM_VCO_TUNE_MAP: c_uint = 0x0f0;
pub const QSERDES_V3_COM_VCO_TUNE1_MODE0: c_uint = 0x0f4;
pub const QSERDES_V3_COM_VCO_TUNE2_MODE0: c_uint = 0x0f8;
pub const QSERDES_V3_COM_VCO_TUNE1_MODE1: c_uint = 0x0fc;
pub const QSERDES_V3_COM_VCO_TUNE2_MODE1: c_uint = 0x100;
pub const QSERDES_V3_COM_VCO_TUNE_INITVAL1: c_uint = 0x104;
pub const QSERDES_V3_COM_VCO_TUNE_INITVAL2: c_uint = 0x108;
pub const QSERDES_V3_COM_VCO_TUNE_MINVAL1: c_uint = 0x10c;
pub const QSERDES_V3_COM_VCO_TUNE_MINVAL2: c_uint = 0x110;
pub const QSERDES_V3_COM_VCO_TUNE_MAXVAL1: c_uint = 0x114;
pub const QSERDES_V3_COM_VCO_TUNE_MAXVAL2: c_uint = 0x118;
pub const QSERDES_V3_COM_VCO_TUNE_TIMER1: c_uint = 0x11c;
pub const QSERDES_V3_COM_VCO_TUNE_TIMER2: c_uint = 0x120;
pub const QSERDES_V3_COM_CMN_STATUS: c_uint = 0x124;
pub const QSERDES_V3_COM_RESET_SM_STATUS: c_uint = 0x128;
pub const QSERDES_V3_COM_RESTRIM_CODE_STATUS: c_uint = 0x12c;
pub const QSERDES_V3_COM_PLLCAL_CODE1_STATUS: c_uint = 0x130;
pub const QSERDES_V3_COM_PLLCAL_CODE2_STATUS: c_uint = 0x134;
pub const QSERDES_V3_COM_CLK_SELECT: c_uint = 0x138;
pub const QSERDES_V3_COM_HSCLK_SEL: c_uint = 0x13c;
pub const QSERDES_V3_COM_INTEGLOOP_BINCODE_STATUS: c_uint = 0x140;
pub const QSERDES_V3_COM_PLL_ANALOG: c_uint = 0x144;
pub const QSERDES_V3_COM_CORECLK_DIV_MODE0: c_uint = 0x148;
pub const QSERDES_V3_COM_CORECLK_DIV_MODE1: c_uint = 0x14c;
pub const QSERDES_V3_COM_SW_RESET: c_uint = 0x150;
pub const QSERDES_V3_COM_CORE_CLK_EN: c_uint = 0x154;
pub const QSERDES_V3_COM_C_READY_STATUS: c_uint = 0x158;
pub const QSERDES_V3_COM_CMN_CONFIG: c_uint = 0x15c;
pub const QSERDES_V3_COM_CMN_RATE_OVERRIDE: c_uint = 0x160;
pub const QSERDES_V3_COM_SVS_MODE_CLK_SEL: c_uint = 0x164;
pub const QSERDES_V3_COM_DEBUG_BUS0: c_uint = 0x168;
pub const QSERDES_V3_COM_DEBUG_BUS1: c_uint = 0x16c;
pub const QSERDES_V3_COM_DEBUG_BUS2: c_uint = 0x170;
pub const QSERDES_V3_COM_DEBUG_BUS3: c_uint = 0x174;
pub const QSERDES_V3_COM_DEBUG_BUS_SEL: c_uint = 0x178;
pub const QSERDES_V3_COM_CMN_MISC1: c_uint = 0x17c;
pub const QSERDES_V3_COM_CMN_MISC2: c_uint = 0x180;
pub const QSERDES_V3_COM_CMN_MODE: c_uint = 0x184;
pub const QSERDES_V3_COM_CMN_VREG_SEL: c_uint = 0x188;

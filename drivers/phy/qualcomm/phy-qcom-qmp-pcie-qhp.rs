//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcie-qhp.h
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
// PCIE GEN3 COM registers
pub const PCIE_GEN3_QHP_COM_SSC_EN_CENTER: c_uint = 0x14;
pub const PCIE_GEN3_QHP_COM_SSC_PER1: c_uint = 0x20;
pub const PCIE_GEN3_QHP_COM_SSC_PER2: c_uint = 0x24;
pub const PCIE_GEN3_QHP_COM_SSC_STEP_SIZE1: c_uint = 0x28;
pub const PCIE_GEN3_QHP_COM_SSC_STEP_SIZE2: c_uint = 0x2c;
pub const PCIE_GEN3_QHP_COM_SSC_STEP_SIZE1_MODE1: c_uint = 0x34;
pub const PCIE_GEN3_QHP_COM_SSC_STEP_SIZE2_MODE1: c_uint = 0x38;
pub const PCIE_GEN3_QHP_COM_BIAS_EN_CKBUFLR_EN: c_uint = 0x54;
pub const PCIE_GEN3_QHP_COM_CLK_ENABLE1: c_uint = 0x58;
pub const PCIE_GEN3_QHP_COM_LOCK_CMP1_MODE0: c_uint = 0x6c;
pub const PCIE_GEN3_QHP_COM_LOCK_CMP2_MODE0: c_uint = 0x70;
pub const PCIE_GEN3_QHP_COM_LOCK_CMP1_MODE1: c_uint = 0x78;
pub const PCIE_GEN3_QHP_COM_LOCK_CMP2_MODE1: c_uint = 0x7c;
pub const PCIE_GEN3_QHP_COM_BGV_TRIM: c_uint = 0x98;
pub const PCIE_GEN3_QHP_COM_CP_CTRL_MODE0: c_uint = 0xb4;
pub const PCIE_GEN3_QHP_COM_CP_CTRL_MODE1: c_uint = 0xb8;
pub const PCIE_GEN3_QHP_COM_PLL_RCTRL_MODE0: c_uint = 0xc0;
pub const PCIE_GEN3_QHP_COM_PLL_RCTRL_MODE1: c_uint = 0xc4;
pub const PCIE_GEN3_QHP_COM_PLL_CCTRL_MODE0: c_uint = 0xcc;
pub const PCIE_GEN3_QHP_COM_PLL_CCTRL_MODE1: c_uint = 0xd0;
pub const PCIE_GEN3_QHP_COM_SYSCLK_EN_SEL: c_uint = 0xdc;
pub const PCIE_GEN3_QHP_COM_RESTRIM_CTRL2: c_uint = 0xf0;
pub const PCIE_GEN3_QHP_COM_LOCK_CMP_EN: c_uint = 0xf8;
pub const PCIE_GEN3_QHP_COM_DEC_START_MODE0: c_uint = 0x100;
pub const PCIE_GEN3_QHP_COM_DEC_START_MODE1: c_uint = 0x108;
pub const PCIE_GEN3_QHP_COM_DIV_FRAC_START1_MODE0: c_uint = 0x11c;
pub const PCIE_GEN3_QHP_COM_DIV_FRAC_START2_MODE0: c_uint = 0x120;
pub const PCIE_GEN3_QHP_COM_DIV_FRAC_START3_MODE0: c_uint = 0x124;
pub const PCIE_GEN3_QHP_COM_DIV_FRAC_START1_MODE1: c_uint = 0x128;
pub const PCIE_GEN3_QHP_COM_DIV_FRAC_START2_MODE1: c_uint = 0x12c;
pub const PCIE_GEN3_QHP_COM_DIV_FRAC_START3_MODE1: c_uint = 0x130;
pub const PCIE_GEN3_QHP_COM_INTEGLOOP_GAIN0_MODE0: c_uint = 0x150;
pub const PCIE_GEN3_QHP_COM_INTEGLOOP_GAIN0_MODE1: c_uint = 0x158;
pub const PCIE_GEN3_QHP_COM_VCO_TUNE_MAP: c_uint = 0x178;
pub const PCIE_GEN3_QHP_COM_BG_CTRL: c_uint = 0x1c8;
pub const PCIE_GEN3_QHP_COM_CLK_SELECT: c_uint = 0x1cc;
pub const PCIE_GEN3_QHP_COM_HSCLK_SEL1: c_uint = 0x1d0;
pub const PCIE_GEN3_QHP_COM_CORECLK_DIV: c_uint = 0x1e0;
pub const PCIE_GEN3_QHP_COM_CORE_CLK_EN: c_uint = 0x1e8;
pub const PCIE_GEN3_QHP_COM_CMN_CONFIG: c_uint = 0x1f0;
pub const PCIE_GEN3_QHP_COM_SVS_MODE_CLK_SEL: c_uint = 0x1fc;
pub const PCIE_GEN3_QHP_COM_CORECLK_DIV_MODE1: c_uint = 0x21c;
pub const PCIE_GEN3_QHP_COM_CMN_MODE: c_uint = 0x224;
pub const PCIE_GEN3_QHP_COM_VREGCLK_DIV1: c_uint = 0x228;
pub const PCIE_GEN3_QHP_COM_VREGCLK_DIV2: c_uint = 0x22c;
// PCIE GEN3 QHP Lane registers
pub const PCIE_GEN3_QHP_L0_DRVR_CTRL0: c_uint = 0xc;
pub const PCIE_GEN3_QHP_L0_DRVR_CTRL1: c_uint = 0x10;
pub const PCIE_GEN3_QHP_L0_DRVR_CTRL2: c_uint = 0x14;
pub const PCIE_GEN3_QHP_L0_DRVR_TAP_EN: c_uint = 0x18;
pub const PCIE_GEN3_QHP_L0_TX_BAND_MODE: c_uint = 0x60;
pub const PCIE_GEN3_QHP_L0_LANE_MODE: c_uint = 0x64;
pub const PCIE_GEN3_QHP_L0_PARALLEL_RATE: c_uint = 0x7c;
pub const PCIE_GEN3_QHP_L0_CML_CTRL_MODE0: c_uint = 0xc0;
pub const PCIE_GEN3_QHP_L0_CML_CTRL_MODE1: c_uint = 0xc4;
pub const PCIE_GEN3_QHP_L0_CML_CTRL_MODE2: c_uint = 0xc8;
pub const PCIE_GEN3_QHP_L0_PREAMP_CTRL_MODE1: c_uint = 0xd0;
pub const PCIE_GEN3_QHP_L0_PREAMP_CTRL_MODE2: c_uint = 0xd4;
pub const PCIE_GEN3_QHP_L0_MIXER_CTRL_MODE0: c_uint = 0xd8;
pub const PCIE_GEN3_QHP_L0_MIXER_CTRL_MODE1: c_uint = 0xdc;
pub const PCIE_GEN3_QHP_L0_MIXER_CTRL_MODE2: c_uint = 0xe0;
pub const PCIE_GEN3_QHP_L0_CTLE_THRESH_DFE: c_uint = 0xfc;
pub const PCIE_GEN3_QHP_L0_CGA_THRESH_DFE: c_uint = 0x100;
pub const PCIE_GEN3_QHP_L0_RXENGINE_EN0: c_uint = 0x108;
pub const PCIE_GEN3_QHP_L0_CTLE_TRAIN_TIME: c_uint = 0x114;
pub const PCIE_GEN3_QHP_L0_CTLE_DFE_OVRLP_TIME: c_uint = 0x118;
pub const PCIE_GEN3_QHP_L0_DFE_REFRESH_TIME: c_uint = 0x11c;
pub const PCIE_GEN3_QHP_L0_DFE_ENABLE_TIME: c_uint = 0x120;
pub const PCIE_GEN3_QHP_L0_VGA_GAIN: c_uint = 0x124;
pub const PCIE_GEN3_QHP_L0_DFE_GAIN: c_uint = 0x128;
pub const PCIE_GEN3_QHP_L0_EQ_GAIN: c_uint = 0x130;
pub const PCIE_GEN3_QHP_L0_OFFSET_GAIN: c_uint = 0x134;
pub const PCIE_GEN3_QHP_L0_PRE_GAIN: c_uint = 0x138;
pub const PCIE_GEN3_QHP_L0_VGA_INITVAL: c_uint = 0x13c;
pub const PCIE_GEN3_QHP_L0_EQ_INTVAL: c_uint = 0x154;
pub const PCIE_GEN3_QHP_L0_EDAC_INITVAL: c_uint = 0x160;
pub const PCIE_GEN3_QHP_L0_RXEQ_INITB0: c_uint = 0x168;
pub const PCIE_GEN3_QHP_L0_RXEQ_INITB1: c_uint = 0x16c;
pub const PCIE_GEN3_QHP_L0_RCVRDONE_THRESH1: c_uint = 0x178;
pub const PCIE_GEN3_QHP_L0_RXEQ_CTRL: c_uint = 0x180;
pub const PCIE_GEN3_QHP_L0_UCDR_FO_GAIN_MODE0: c_uint = 0x184;
pub const PCIE_GEN3_QHP_L0_UCDR_FO_GAIN_MODE1: c_uint = 0x188;
pub const PCIE_GEN3_QHP_L0_UCDR_FO_GAIN_MODE2: c_uint = 0x18c;
pub const PCIE_GEN3_QHP_L0_UCDR_SO_GAIN_MODE0: c_uint = 0x190;
pub const PCIE_GEN3_QHP_L0_UCDR_SO_GAIN_MODE1: c_uint = 0x194;
pub const PCIE_GEN3_QHP_L0_UCDR_SO_GAIN_MODE2: c_uint = 0x198;
pub const PCIE_GEN3_QHP_L0_UCDR_SO_CONFIG: c_uint = 0x19c;
pub const PCIE_GEN3_QHP_L0_RX_BAND: c_uint = 0x1a4;
pub const PCIE_GEN3_QHP_L0_RX_RCVR_PATH1_MODE0: c_uint = 0x1c0;
pub const PCIE_GEN3_QHP_L0_RX_RCVR_PATH1_MODE1: c_uint = 0x1c4;
pub const PCIE_GEN3_QHP_L0_RX_RCVR_PATH1_MODE2: c_uint = 0x1c8;
pub const PCIE_GEN3_QHP_L0_SIGDET_ENABLES: c_uint = 0x230;
pub const PCIE_GEN3_QHP_L0_SIGDET_CNTRL: c_uint = 0x234;
pub const PCIE_GEN3_QHP_L0_SIGDET_DEGLITCH_CNTRL: c_uint = 0x238;
pub const PCIE_GEN3_QHP_L0_DCC_GAIN: c_uint = 0x2a4;
pub const PCIE_GEN3_QHP_L0_RSM_START: c_uint = 0x2a8;
pub const PCIE_GEN3_QHP_L0_RX_EN_SIGNAL: c_uint = 0x2ac;
pub const PCIE_GEN3_QHP_L0_PSM_RX_EN_CAL: c_uint = 0x2b0;
pub const PCIE_GEN3_QHP_L0_RX_MISC_CNTRL0: c_uint = 0x2b8;
pub const PCIE_GEN3_QHP_L0_TS0_TIMER: c_uint = 0x2c0;
pub const PCIE_GEN3_QHP_L0_DLL_HIGHDATARATE: c_uint = 0x2c4;
pub const PCIE_GEN3_QHP_L0_RX_RESETCODE_OFFSET: c_uint = 0x2cc;
// PCIE GEN3 PCS registers
pub const PCIE_GEN3_QHP_PHY_TXMGN_MAIN_V0_M3P5DB: c_uint = 0x2c;
pub const PCIE_GEN3_QHP_PHY_TXMGN_POST_V0_M3P5DB: c_uint = 0x40;
pub const PCIE_GEN3_QHP_PHY_TXMGN_MAIN_V0_M6DB: c_uint = 0x54;
pub const PCIE_GEN3_QHP_PHY_TXMGN_POST_V0_M6DB: c_uint = 0x68;
pub const PCIE_GEN3_QHP_PHY_POWER_STATE_CONFIG: c_uint = 0x15c;
pub const PCIE_GEN3_QHP_PHY_POWER_STATE_CONFIG5: c_uint = 0x16c;
pub const PCIE_GEN3_QHP_PHY_PCS_TX_RX_CONFIG: c_uint = 0x174;

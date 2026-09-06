//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-txrx-v5_20.h
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
// Only for QMP V5_20 PHY - TX registers
pub const QSERDES_V5_20_TX_RES_CODE_LANE_OFFSET_TX: c_uint = 0x30;
pub const QSERDES_V5_20_TX_RES_CODE_LANE_OFFSET_RX: c_uint = 0x34;
pub const QSERDES_V5_20_TX_LANE_MODE_1: c_uint = 0x78;
pub const QSERDES_V5_20_TX_LANE_MODE_2: c_uint = 0x7c;
pub const QSERDES_V5_20_TX_LANE_MODE_3: c_uint = 0x80;
pub const QSERDES_V5_20_TX_RCV_DETECT_LVL_2: c_uint = 0x90;
pub const QSERDES_V5_20_TX_VMODE_CTRL1: c_uint = 0xb0;
pub const QSERDES_V5_20_TX_PI_QEC_CTRL: c_uint = 0xcc;
// Only for QMP V5_20 PHY - RX registers
pub const QSERDES_V5_20_RX_UCDR_FO_GAIN_RATE2: c_uint = 0x008;
pub const QSERDES_V5_20_RX_UCDR_FO_GAIN_RATE3: c_uint = 0x00c;
pub const QSERDES_V5_20_RX_UCDR_SO_GAIN_RATE3: c_uint = 0x01c;
pub const QSERDES_V5_20_RX_UCDR_PI_CONTROLS: c_uint = 0x020;
pub const QSERDES_V5_20_RX_AUX_DATA_THRESH_BIN_RATE_0_1: c_uint = 0x02c;
pub const QSERDES_V5_20_RX_AUX_DATA_THRESH_BIN_RATE_2_3: c_uint = 0x030;
pub const QSERDES_V5_20_RX_RX_IDAC_SAOFFSET: c_uint = 0x07c;
pub const QSERDES_V5_20_RX_DFE_1: c_uint = 0x088;
pub const QSERDES_V5_20_RX_DFE_2: c_uint = 0x08c;
pub const QSERDES_V5_20_RX_DFE_3: c_uint = 0x090;
pub const QSERDES_V5_20_RX_DFE_DAC_ENABLE1: c_uint = 0x0b4;
pub const QSERDES_V5_20_RX_TX_ADAPT_PRE_THRESH1: c_uint = 0x0bc;
pub const QSERDES_V5_20_RX_TX_ADAPT_PRE_THRESH2: c_uint = 0x0c0;
pub const QSERDES_V5_20_RX_TX_ADAPT_POST_THRESH1: c_uint = 0x0c4;
pub const QSERDES_V5_20_RX_TX_ADAPT_POST_THRESH2: c_uint = 0x0c8;
pub const QSERDES_V5_20_RX_TX_ADAPT_MAIN_THRESH1: c_uint = 0x0cc;
pub const QSERDES_V5_20_RX_TX_ADAPT_MAIN_THRESH2: c_uint = 0x0d0;
pub const QSERDES_V5_20_RX_VGA_CAL_CNTRL1: c_uint = 0x0d4;
pub const QSERDES_V5_20_RX_VGA_CAL_CNTRL2: c_uint = 0x0d8;
pub const QSERDES_V5_20_RX_VGA_CAL_MAN_VAL: c_uint = 0x0dc;
pub const QSERDES_V5_20_RX_GM_CAL: c_uint = 0x0ec;
pub const QSERDES_V5_20_RX_RX_EQU_ADAPTOR_CNTRL2: c_uint = 0x100;
pub const QSERDES_V5_20_RX_RX_EQU_ADAPTOR_CNTRL3: c_uint = 0x104;
pub const QSERDES_V5_20_RX_RX_EQU_ADAPTOR_CNTRL4: c_uint = 0x108;
pub const QSERDES_V5_20_RX_RX_EQ_OFFSET_ADAPTOR_CNTRL1: c_uint = 0x118;
pub const QSERDES_V5_20_RX_RX_OFFSET_ADAPTOR_CNTRL2: c_uint = 0x11c;
pub const QSERDES_V5_20_RX_SIGDET_ENABLES: c_uint = 0x120;
pub const QSERDES_V5_20_RX_SIGDET_CNTRL: c_uint = 0x124;
pub const QSERDES_V5_20_RX_SIGDET_DEGLITCH_CNTRL: c_uint = 0x12c;
pub const QSERDES_V5_20_RX_RX_MODE_RATE_0_1_B0: c_uint = 0x160;
pub const QSERDES_V5_20_RX_RX_MODE_RATE_0_1_B1: c_uint = 0x164;
pub const QSERDES_V5_20_RX_RX_MODE_RATE_0_1_B2: c_uint = 0x168;
pub const QSERDES_V5_20_RX_RX_MODE_RATE_0_1_B3: c_uint = 0x16c;
pub const QSERDES_V5_20_RX_RX_MODE_RATE_0_1_B4: c_uint = 0x170;
pub const QSERDES_V5_20_RX_RX_MODE_RATE_0_1_B5: c_uint = 0x174;
pub const QSERDES_V5_20_RX_RX_MODE_RATE_0_1_B6: c_uint = 0x178;
pub const QSERDES_V5_20_RX_RX_MODE_RATE2_B0: c_uint = 0x17c;
pub const QSERDES_V5_20_RX_RX_MODE_RATE2_B1: c_uint = 0x180;
pub const QSERDES_V5_20_RX_RX_MODE_RATE2_B2: c_uint = 0x184;
pub const QSERDES_V5_20_RX_RX_MODE_RATE2_B3: c_uint = 0x188;
pub const QSERDES_V5_20_RX_RX_MODE_RATE2_B4: c_uint = 0x18c;
pub const QSERDES_V5_20_RX_RX_MODE_RATE2_B5: c_uint = 0x190;
pub const QSERDES_V5_20_RX_RX_MODE_RATE2_B6: c_uint = 0x194;
pub const QSERDES_V5_20_RX_RX_MODE_RATE3_B0: c_uint = 0x198;
pub const QSERDES_V5_20_RX_RX_MODE_RATE3_B1: c_uint = 0x19c;
pub const QSERDES_V5_20_RX_RX_MODE_RATE3_B2: c_uint = 0x1a0;
pub const QSERDES_V5_20_RX_RX_MODE_RATE3_B3: c_uint = 0x1a4;
pub const QSERDES_V5_20_RX_RX_MODE_RATE3_B4: c_uint = 0x1a8;
pub const QSERDES_V5_20_RX_RX_MODE_RATE3_B5: c_uint = 0x1ac;
pub const QSERDES_V5_20_RX_RX_MODE_RATE3_B6: c_uint = 0x1b0;
pub const QSERDES_V5_20_RX_PHPRE_CTRL: c_uint = 0x1b4;
pub const QSERDES_V5_20_RX_DFE_DAC_ENABLE2: c_uint = 0x1b8;
pub const QSERDES_V5_20_RX_DFE_EN_TIMER: c_uint = 0x1bc;
pub const QSERDES_V5_20_RX_DFE_CTLE_POST_CAL_OFFSET: c_uint = 0x1c0;
pub const QSERDES_V5_20_RX_DCC_CTRL1: c_uint = 0x1c4;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH1_RATE210: c_uint = 0x1f4;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH1_RATE3: c_uint = 0x1f8;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH2_RATE210: c_uint = 0x1fc;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH2_RATE3: c_uint = 0x200;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH3_RATE210: c_uint = 0x204;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH3_RATE3: c_uint = 0x208;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH4_RATE3: c_uint = 0x210;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH5_RATE3: c_uint = 0x218;
pub const QSERDES_V5_20_RX_RX_MARG_COARSE_THRESH6_RATE3: c_uint = 0x220;
pub const QSERDES_V5_20_RX_Q_PI_INTRINSIC_BIAS_RATE32: c_uint = 0x238;

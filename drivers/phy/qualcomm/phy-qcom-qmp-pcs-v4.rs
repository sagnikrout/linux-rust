//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-v4.h
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
// Only for QMP V4 PHY - USB/PCIe PCS registers
pub const QPHY_V4_PCS_SW_RESET: c_uint = 0x000;
pub const QPHY_V4_PCS_REVISION_ID0: c_uint = 0x004;
pub const QPHY_V4_PCS_REVISION_ID1: c_uint = 0x008;
pub const QPHY_V4_PCS_REVISION_ID2: c_uint = 0x00c;
pub const QPHY_V4_PCS_REVISION_ID3: c_uint = 0x010;
pub const QPHY_V4_PCS_PCS_STATUS1: c_uint = 0x014;
pub const QPHY_V4_PCS_PCS_STATUS2: c_uint = 0x018;
pub const QPHY_V4_PCS_PCS_STATUS3: c_uint = 0x01c;
pub const QPHY_V4_PCS_PCS_STATUS4: c_uint = 0x020;
pub const QPHY_V4_PCS_PCS_STATUS5: c_uint = 0x024;
pub const QPHY_V4_PCS_PCS_STATUS6: c_uint = 0x028;
pub const QPHY_V4_PCS_PCS_STATUS7: c_uint = 0x02c;
pub const QPHY_V4_PCS_DEBUG_BUS_0_STATUS: c_uint = 0x030;
pub const QPHY_V4_PCS_DEBUG_BUS_1_STATUS: c_uint = 0x034;
pub const QPHY_V4_PCS_DEBUG_BUS_2_STATUS: c_uint = 0x038;
pub const QPHY_V4_PCS_DEBUG_BUS_3_STATUS: c_uint = 0x03c;
pub const QPHY_V4_PCS_POWER_DOWN_CONTROL: c_uint = 0x040;
pub const QPHY_V4_PCS_START_CONTROL: c_uint = 0x044;
pub const QPHY_V4_PCS_INSIG_SW_CTRL1: c_uint = 0x048;
pub const QPHY_V4_PCS_INSIG_SW_CTRL2: c_uint = 0x04c;
pub const QPHY_V4_PCS_INSIG_SW_CTRL3: c_uint = 0x050;
pub const QPHY_V4_PCS_INSIG_SW_CTRL4: c_uint = 0x054;
pub const QPHY_V4_PCS_INSIG_SW_CTRL5: c_uint = 0x058;
pub const QPHY_V4_PCS_INSIG_SW_CTRL6: c_uint = 0x05c;
pub const QPHY_V4_PCS_INSIG_SW_CTRL7: c_uint = 0x060;
pub const QPHY_V4_PCS_INSIG_SW_CTRL8: c_uint = 0x064;
pub const QPHY_V4_PCS_INSIG_MX_CTRL1: c_uint = 0x068;
pub const QPHY_V4_PCS_INSIG_MX_CTRL2: c_uint = 0x06c;
pub const QPHY_V4_PCS_INSIG_MX_CTRL3: c_uint = 0x070;
pub const QPHY_V4_PCS_INSIG_MX_CTRL4: c_uint = 0x074;
pub const QPHY_V4_PCS_INSIG_MX_CTRL5: c_uint = 0x078;
pub const QPHY_V4_PCS_INSIG_MX_CTRL7: c_uint = 0x07c;
pub const QPHY_V4_PCS_INSIG_MX_CTRL8: c_uint = 0x080;
pub const QPHY_V4_PCS_OUTSIG_SW_CTRL1: c_uint = 0x084;
pub const QPHY_V4_PCS_OUTSIG_MX_CTRL1: c_uint = 0x088;
pub const QPHY_V4_PCS_CLAMP_ENABLE: c_uint = 0x08c;
pub const QPHY_V4_PCS_POWER_STATE_CONFIG1: c_uint = 0x090;
pub const QPHY_V4_PCS_POWER_STATE_CONFIG2: c_uint = 0x094;
pub const QPHY_V4_PCS_FLL_CNTRL1: c_uint = 0x098;
pub const QPHY_V4_PCS_FLL_CNTRL2: c_uint = 0x09c;
pub const QPHY_V4_PCS_FLL_CNT_VAL_L: c_uint = 0x0a0;
pub const QPHY_V4_PCS_FLL_CNT_VAL_H_TOL: c_uint = 0x0a4;
pub const QPHY_V4_PCS_FLL_MAN_CODE: c_uint = 0x0a8;
pub const QPHY_V4_PCS_TEST_CONTROL1: c_uint = 0x0ac;
pub const QPHY_V4_PCS_TEST_CONTROL2: c_uint = 0x0b0;
pub const QPHY_V4_PCS_TEST_CONTROL3: c_uint = 0x0b4;
pub const QPHY_V4_PCS_TEST_CONTROL4: c_uint = 0x0b8;
pub const QPHY_V4_PCS_TEST_CONTROL5: c_uint = 0x0bc;
pub const QPHY_V4_PCS_TEST_CONTROL6: c_uint = 0x0c0;
pub const QPHY_V4_PCS_LOCK_DETECT_CONFIG1: c_uint = 0x0c4;
pub const QPHY_V4_PCS_LOCK_DETECT_CONFIG2: c_uint = 0x0c8;
pub const QPHY_V4_PCS_LOCK_DETECT_CONFIG3: c_uint = 0x0cc;
pub const QPHY_V4_PCS_LOCK_DETECT_CONFIG4: c_uint = 0x0d0;
pub const QPHY_V4_PCS_LOCK_DETECT_CONFIG5: c_uint = 0x0d4;
pub const QPHY_V4_PCS_LOCK_DETECT_CONFIG6: c_uint = 0x0d8;
pub const QPHY_V4_PCS_REFGEN_REQ_CONFIG1: c_uint = 0x0dc;
pub const QPHY_V4_PCS_REFGEN_REQ_CONFIG2: c_uint = 0x0e0;
pub const QPHY_V4_PCS_REFGEN_REQ_CONFIG3: c_uint = 0x0e4;
pub const QPHY_V4_PCS_BIST_CTRL: c_uint = 0x0e8;
pub const QPHY_V4_PCS_PRBS_POLY0: c_uint = 0x0ec;
pub const QPHY_V4_PCS_PRBS_POLY1: c_uint = 0x0f0;
pub const QPHY_V4_PCS_FIXED_PAT0: c_uint = 0x0f4;
pub const QPHY_V4_PCS_FIXED_PAT1: c_uint = 0x0f8;
pub const QPHY_V4_PCS_FIXED_PAT2: c_uint = 0x0fc;
pub const QPHY_V4_PCS_FIXED_PAT3: c_uint = 0x100;
pub const QPHY_V4_PCS_FIXED_PAT4: c_uint = 0x104;
pub const QPHY_V4_PCS_FIXED_PAT5: c_uint = 0x108;
pub const QPHY_V4_PCS_FIXED_PAT6: c_uint = 0x10c;
pub const QPHY_V4_PCS_FIXED_PAT7: c_uint = 0x110;
pub const QPHY_V4_PCS_FIXED_PAT8: c_uint = 0x114;
pub const QPHY_V4_PCS_FIXED_PAT9: c_uint = 0x118;
pub const QPHY_V4_PCS_FIXED_PAT10: c_uint = 0x11c;
pub const QPHY_V4_PCS_FIXED_PAT11: c_uint = 0x120;
pub const QPHY_V4_PCS_FIXED_PAT12: c_uint = 0x124;
pub const QPHY_V4_PCS_FIXED_PAT13: c_uint = 0x128;
pub const QPHY_V4_PCS_FIXED_PAT14: c_uint = 0x12c;
pub const QPHY_V4_PCS_FIXED_PAT15: c_uint = 0x130;
pub const QPHY_V4_PCS_TXMGN_CONFIG: c_uint = 0x134;
pub const QPHY_V4_PCS_G12S1_TXMGN_V0: c_uint = 0x138;
pub const QPHY_V4_PCS_G12S1_TXMGN_V1: c_uint = 0x13c;
pub const QPHY_V4_PCS_G12S1_TXMGN_V2: c_uint = 0x140;
pub const QPHY_V4_PCS_G12S1_TXMGN_V3: c_uint = 0x144;
pub const QPHY_V4_PCS_G12S1_TXMGN_V4: c_uint = 0x148;
pub const QPHY_V4_PCS_G12S1_TXMGN_V0_RS: c_uint = 0x14c;
pub const QPHY_V4_PCS_G12S1_TXMGN_V1_RS: c_uint = 0x150;
pub const QPHY_V4_PCS_G12S1_TXMGN_V2_RS: c_uint = 0x154;
pub const QPHY_V4_PCS_G12S1_TXMGN_V3_RS: c_uint = 0x158;
pub const QPHY_V4_PCS_G12S1_TXMGN_V4_RS: c_uint = 0x15c;
pub const QPHY_V4_PCS_G3S2_TXMGN_MAIN: c_uint = 0x160;
pub const QPHY_V4_PCS_G3S2_TXMGN_MAIN_RS: c_uint = 0x164;
pub const QPHY_V4_PCS_G12S1_TXDEEMPH_M6DB: c_uint = 0x168;
pub const QPHY_V4_PCS_G12S1_TXDEEMPH_M3P5DB: c_uint = 0x16c;
pub const QPHY_V4_PCS_G3S2_PRE_GAIN: c_uint = 0x170;
pub const QPHY_V4_PCS_G3S2_POST_GAIN: c_uint = 0x174;
pub const QPHY_V4_PCS_G3S2_PRE_POST_OFFSET: c_uint = 0x178;
pub const QPHY_V4_PCS_G3S2_PRE_GAIN_RS: c_uint = 0x17c;
pub const QPHY_V4_PCS_G3S2_POST_GAIN_RS: c_uint = 0x180;
pub const QPHY_V4_PCS_G3S2_PRE_POST_OFFSET_RS: c_uint = 0x184;
pub const QPHY_V4_PCS_RX_SIGDET_LVL: c_uint = 0x188;
pub const QPHY_V4_PCS_RX_SIGDET_DTCT_CNTRL: c_uint = 0x18c;
pub const QPHY_V4_PCS_RCVR_DTCT_DLY_P1U2_L: c_uint = 0x190;
pub const QPHY_V4_PCS_RCVR_DTCT_DLY_P1U2_H: c_uint = 0x194;
pub const QPHY_V4_PCS_RATE_SLEW_CNTRL1: c_uint = 0x198;
pub const QPHY_V4_PCS_RATE_SLEW_CNTRL2: c_uint = 0x19c;
pub const QPHY_V4_PCS_PWRUP_RESET_DLY_TIME_AUXCLK: c_uint = 0x1a0;
pub const QPHY_V4_PCS_P2U3_WAKEUP_DLY_TIME_AUXCLK_L: c_uint = 0x1a4;
pub const QPHY_V4_PCS_P2U3_WAKEUP_DLY_TIME_AUXCLK_H: c_uint = 0x1a8;
pub const QPHY_V4_PCS_TSYNC_RSYNC_TIME: c_uint = 0x1ac;
pub const QPHY_V4_PCS_CDR_RESET_TIME: c_uint = 0x1b0;
pub const QPHY_V4_PCS_TSYNC_DLY_TIME: c_uint = 0x1b4;
pub const QPHY_V4_PCS_ELECIDLE_DLY_SEL: c_uint = 0x1b8;
pub const QPHY_V4_PCS_CMN_ACK_OUT_SEL: c_uint = 0x1bc;
pub const QPHY_V4_PCS_ALIGN_DETECT_CONFIG1: c_uint = 0x1c0;
pub const QPHY_V4_PCS_ALIGN_DETECT_CONFIG2: c_uint = 0x1c4;
pub const QPHY_V4_PCS_ALIGN_DETECT_CONFIG3: c_uint = 0x1c8;
pub const QPHY_V4_PCS_ALIGN_DETECT_CONFIG4: c_uint = 0x1cc;
pub const QPHY_V4_PCS_PCS_TX_RX_CONFIG: c_uint = 0x1d0;
pub const QPHY_V4_PCS_RX_IDLE_DTCT_CNTRL: c_uint = 0x1d4;
pub const QPHY_V4_PCS_RX_DCC_CAL_CONFIG: c_uint = 0x1d8;
pub const QPHY_V4_PCS_EQ_CONFIG1: c_uint = 0x1dc;
pub const QPHY_V4_PCS_EQ_CONFIG2: c_uint = 0x1e0;
pub const QPHY_V4_PCS_EQ_CONFIG3: c_uint = 0x1e4;
pub const QPHY_V4_PCS_EQ_CONFIG4: c_uint = 0x1e8;
pub const QPHY_V4_PCS_EQ_CONFIG5: c_uint = 0x1ec;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/analogix/analogix_dp_reg.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Register definition file for Analogix DP core driver
//
// Copyright (C) 2012 Samsung Electronics Co., Ltd.
// Author: Jingoo Han <jg1.han@samsung.com>
//
pub const ANALOGIX_DP_TX_SW_RESET: c_uint = 0x14;
pub const ANALOGIX_DP_FUNC_EN_1: c_uint = 0x18;
pub const ANALOGIX_DP_FUNC_EN_2: c_uint = 0x1C;
pub const ANALOGIX_DP_VIDEO_CTL_1: c_uint = 0x20;
pub const ANALOGIX_DP_VIDEO_CTL_2: c_uint = 0x24;
pub const ANALOGIX_DP_VIDEO_CTL_3: c_uint = 0x28;
pub const ANALOGIX_DP_VIDEO_CTL_8: c_uint = 0x3C;
pub const ANALOGIX_DP_VIDEO_CTL_10: c_uint = 0x44;
pub const ANALOGIX_DP_SPDIF_AUDIO_CTL_0: c_uint = 0xD8;
pub const ANALOGIX_DP_PLL_REG_1: c_uint = 0xfc;
pub const ANALOGIX_DP_PLL_REG_2: c_uint = 0x9e4;
pub const ANALOGIX_DP_PLL_REG_3: c_uint = 0x9e8;
pub const ANALOGIX_DP_PLL_REG_4: c_uint = 0x9ec;
pub const ANALOGIX_DP_PLL_REG_5: c_uint = 0xa00;
pub const ANALOGIX_DP_PD: c_uint = 0x12c;
pub const ANALOGIX_DP_IF_TYPE: c_uint = 0x244;
pub const ANALOGIX_DP_IF_PKT_DB1: c_uint = 0x254;
pub const ANALOGIX_DP_IF_PKT_DB2: c_uint = 0x258;
pub const ANALOGIX_DP_SPD_HB0: c_uint = 0x2F8;
pub const ANALOGIX_DP_SPD_HB1: c_uint = 0x2FC;
pub const ANALOGIX_DP_SPD_HB2: c_uint = 0x300;
pub const ANALOGIX_DP_SPD_HB3: c_uint = 0x304;
pub const ANALOGIX_DP_SPD_PB0: c_uint = 0x308;
pub const ANALOGIX_DP_SPD_PB1: c_uint = 0x30C;
pub const ANALOGIX_DP_SPD_PB2: c_uint = 0x310;
pub const ANALOGIX_DP_SPD_PB3: c_uint = 0x314;
pub const ANALOGIX_DP_PSR_FRAME_UPDATE_CTRL: c_uint = 0x318;
pub const ANALOGIX_DP_VSC_SHADOW_DB0: c_uint = 0x31C;
pub const ANALOGIX_DP_VSC_SHADOW_DB1: c_uint = 0x320;
pub const ANALOGIX_DP_LANE_MAP: c_uint = 0x35C;
pub const ANALOGIX_DP_ANALOG_CTL_1: c_uint = 0x370;
pub const ANALOGIX_DP_ANALOG_CTL_2: c_uint = 0x374;
pub const ANALOGIX_DP_ANALOG_CTL_3: c_uint = 0x378;
pub const ANALOGIX_DP_PLL_FILTER_CTL_1: c_uint = 0x37C;
pub const ANALOGIX_DP_TX_AMP_TUNING_CTL: c_uint = 0x380;
pub const ANALOGIX_DP_AUX_HW_RETRY_CTL: c_uint = 0x390;
pub const ANALOGIX_DP_COMMON_INT_STA_1: c_uint = 0x3C4;
pub const ANALOGIX_DP_COMMON_INT_STA_2: c_uint = 0x3C8;
pub const ANALOGIX_DP_COMMON_INT_STA_3: c_uint = 0x3CC;
pub const ANALOGIX_DP_COMMON_INT_STA_4: c_uint = 0x3D0;
pub const ANALOGIX_DP_INT_STA: c_uint = 0x3DC;
pub const ANALOGIX_DP_COMMON_INT_MASK_1: c_uint = 0x3E0;
pub const ANALOGIX_DP_COMMON_INT_MASK_2: c_uint = 0x3E4;
pub const ANALOGIX_DP_COMMON_INT_MASK_3: c_uint = 0x3E8;
pub const ANALOGIX_DP_COMMON_INT_MASK_4: c_uint = 0x3EC;
pub const ANALOGIX_DP_INT_STA_MASK: c_uint = 0x3F8;
pub const ANALOGIX_DP_INT_CTL: c_uint = 0x3FC;
pub const ANALOGIX_DP_SYS_CTL_1: c_uint = 0x600;
pub const ANALOGIX_DP_SYS_CTL_2: c_uint = 0x604;
pub const ANALOGIX_DP_SYS_CTL_3: c_uint = 0x608;
pub const ANALOGIX_DP_SYS_CTL_4: c_uint = 0x60C;
pub const ANALOGIX_DP_PKT_SEND_CTL: c_uint = 0x640;
pub const ANALOGIX_DP_HDCP_CTL: c_uint = 0x648;
pub const ANALOGIX_DP_LINK_BW_SET: c_uint = 0x680;
pub const ANALOGIX_DP_LANE_COUNT_SET: c_uint = 0x684;
pub const ANALOGIX_DP_TRAINING_PTN_SET: c_uint = 0x688;
pub const ANALOGIX_DP_LN0_LINK_TRAINING_CTL: c_uint = 0x68C;
pub const ANALOGIX_DP_LN1_LINK_TRAINING_CTL: c_uint = 0x690;
pub const ANALOGIX_DP_LN2_LINK_TRAINING_CTL: c_uint = 0x694;
pub const ANALOGIX_DP_LN3_LINK_TRAINING_CTL: c_uint = 0x698;
pub const ANALOGIX_DP_DEBUG_CTL: c_uint = 0x6C0;
pub const ANALOGIX_DP_HPD_DEGLITCH_L: c_uint = 0x6C4;
pub const ANALOGIX_DP_HPD_DEGLITCH_H: c_uint = 0x6C8;
pub const ANALOGIX_DP_LINK_DEBUG_CTL: c_uint = 0x6E0;
pub const ANALOGIX_DP_M_VID_0: c_uint = 0x700;
pub const ANALOGIX_DP_M_VID_1: c_uint = 0x704;
pub const ANALOGIX_DP_M_VID_2: c_uint = 0x708;
pub const ANALOGIX_DP_N_VID_0: c_uint = 0x70C;
pub const ANALOGIX_DP_N_VID_1: c_uint = 0x710;
pub const ANALOGIX_DP_N_VID_2: c_uint = 0x714;
pub const ANALOGIX_DP_PLL_CTL: c_uint = 0x71C;
pub const ANALOGIX_DP_PHY_PD: c_uint = 0x720;
pub const ANALOGIX_DP_PHY_TEST: c_uint = 0x724;
pub const ANALOGIX_DP_VIDEO_FIFO_THRD: c_uint = 0x730;
pub const ANALOGIX_DP_AUDIO_MARGIN: c_uint = 0x73C;
pub const ANALOGIX_DP_M_VID_GEN_FILTER_TH: c_uint = 0x764;
pub const ANALOGIX_DP_M_AUD_GEN_FILTER_TH: c_uint = 0x778;
pub const ANALOGIX_DP_AUX_CH_STA: c_uint = 0x780;
pub const ANALOGIX_DP_AUX_CH_DEFER_CTL: c_uint = 0x788;
pub const ANALOGIX_DP_AUX_RX_COMM: c_uint = 0x78C;
pub const ANALOGIX_DP_BUFFER_DATA_CTL: c_uint = 0x790;
pub const ANALOGIX_DP_AUX_CH_CTL_1: c_uint = 0x794;
pub const ANALOGIX_DP_AUX_ADDR_7_0: c_uint = 0x798;
pub const ANALOGIX_DP_AUX_ADDR_15_8: c_uint = 0x79C;
pub const ANALOGIX_DP_AUX_ADDR_19_16: c_uint = 0x7A0;
pub const ANALOGIX_DP_AUX_CH_CTL_2: c_uint = 0x7A4;
pub const ANALOGIX_DP_BUF_DATA_0: c_uint = 0x7C0;
pub const ANALOGIX_DP_SOC_GENERAL_CTL: c_uint = 0x800;
pub const ANALOGIX_DP_CRC_CON: c_uint = 0x890;
// ANALOGIX_DP_TX_SW_RESET

// ANALOGIX_DP_FUNC_EN_1

// ANALOGIX_DP_FUNC_EN_2

// ANALOGIX_DP_VIDEO_CTL_1

// ANALOGIX_DP_VIDEO_CTL_1

// ANALOGIX_DP_VIDEO_CTL_3

// ANALOGIX_DP_VIDEO_CTL_8

// ANALOGIX_DP_VIDEO_CTL_10

// ANALOGIX_DP_PLL_REG_1

// ANALOGIX_DP_PSR_FRAME_UPDATE_CTRL

// ANALOGIX_DP_LANE_MAP

// ANALOGIX_DP_ANALOG_CTL_1

// ANALOGIX_DP_ANALOG_CTL_2

// ANALOGIX_DP_ANALOG_CTL_3

// ANALOGIX_DP_PLL_FILTER_CTL_1

// ANALOGIX_DP_TX_AMP_TUNING_CTL

// ANALOGIX_DP_AUX_HW_RETRY_CTL

// ANALOGIX_DP_COMMON_INT_STA_1

// ANALOGIX_DP_COMMON_INT_STA_2

// ANALOGIX_DP_COMMON_INT_STA_3

// ANALOGIX_DP_COMMON_INT_STA_4

// ANALOGIX_DP_INT_STA

// ANALOGIX_DP_INT_CTL

// ANALOGIX_DP_SYS_CTL_1

// ANALOGIX_DP_SYS_CTL_2

// ANALOGIX_DP_SYS_CTL_3

// ANALOGIX_DP_SYS_CTL_4

// ANALOGIX_DP_TRAINING_PTN_SET

// ANALOGIX_DP_LN0_LINK_TRAINING_CTL

// ANALOGIX_DP_DEBUG_CTL

// ANALOGIX_DP_PLL_CTL

// ANALOGIX_DP_PHY_PD

// ANALOGIX_DP_PHY_TEST

// ANALOGIX_DP_AUX_CH_STA

// ANALOGIX_DP_AUX_CH_DEFER_CTL

// ANALOGIX_DP_AUX_RX_COMM

// ANALOGIX_DP_BUFFER_DATA_CTL

// ANALOGIX_DP_AUX_CH_CTL_1

// ANALOGIX_DP_AUX_ADDR_7_0

// ANALOGIX_DP_AUX_ADDR_15_8

// ANALOGIX_DP_AUX_ADDR_19_16

// ANALOGIX_DP_AUX_CH_CTL_2

// ANALOGIX_DP_SOC_GENERAL_CTL

// ANALOGIX_DP_PKT_SEND_CTL

// ANALOGIX_DP_CRC_CON


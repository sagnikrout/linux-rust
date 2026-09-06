//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/hibmc/dp/dp_reg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Hisilicon Limited.
pub const HIBMC_DP_AUX_CMD_ADDR: c_uint = 0x50;
pub const HIBMC_DP_AUX_WR_DATA0: c_uint = 0x54;
pub const HIBMC_DP_AUX_WR_DATA1: c_uint = 0x58;
pub const HIBMC_DP_AUX_WR_DATA2: c_uint = 0x5c;
pub const HIBMC_DP_AUX_WR_DATA3: c_uint = 0x60;
pub const HIBMC_DP_AUX_RD_DATA0: c_uint = 0x64;
pub const HIBMC_DP_AUX_REQ: c_uint = 0x74;

pub const HIBMC_DP_AUX_STATUS: c_uint = 0x78;

pub const HIBMC_DP_HPD_STATUS: c_uint = 0x98;

pub const HIBMC_DP_PHYIF_CTRL0: c_uint = 0xa0;

pub const HIBMC_DP_VIDEO_CTRL: c_uint = 0x100;

pub const HIBMC_DP_VIDEO_CONFIG0: c_uint = 0x104;

pub const HIBMC_DP_VIDEO_CONFIG1: c_uint = 0x108;

pub const HIBMC_DP_VIDEO_CONFIG2: c_uint = 0x10c;

pub const HIBMC_DP_VIDEO_CONFIG3: c_uint = 0x110;

pub const HIBMC_DP_VIDEO_PACKET: c_uint = 0x114;

pub const HIBMC_DP_VIDEO_MSA0: c_uint = 0x118;

pub const HIBMC_DP_VIDEO_MSA1: c_uint = 0x11c;
pub const HIBMC_DP_VIDEO_MSA2: c_uint = 0x120;

pub const HIBMC_DP_COLOR_BAR_CTRL: c_uint = 0x260;
pub const HIBMC_DP_COLOR_BAR_CTRL1: c_uint = 0x264;
pub const HIBMC_DP_TIMING_GEN_CONFIG0: c_uint = 0x26c;

pub const HIBMC_DP_TIMING_GEN_CONFIG2: c_uint = 0x274;

pub const HIBMC_DP_TIMING_GEN_CONFIG3: c_uint = 0x278;

pub const HIBMC_DP_HDCP_CFG: c_uint = 0x600;
pub const HIBMC_DP_DPTX_RST_CTRL: c_uint = 0x700;

pub const HIBMC_DP_DPTX_CLK_CTRL: c_uint = 0x704;
pub const HIBMC_DP_DPTX_GCTL0: c_uint = 0x708;

pub const HIBMC_DP_INTR_ENABLE: c_uint = 0x720;
pub const HIBMC_DP_INTR_ORIGINAL_STATUS: c_uint = 0x728;
pub const HIBMC_DP_TIMING_MODEL_CTRL: c_uint = 0x884;

pub const HIBMC_DP_TIMING_SYNC_CTRL: c_uint = 0xFF0;
pub const HIBMC_DP_INTSTAT: c_uint = 0x1e0724;
pub const HIBMC_DP_INTCLR: c_uint = 0x1e0728;
// dp serdes reg
pub const HIBMC_DP_HOST_OFFSET: c_uint = 0x10000;
pub const HIBMC_DP_LANE0_RATE_OFFSET: c_uint = 0x4;
pub const HIBMC_DP_LANE1_RATE_OFFSET: c_uint = 0xc;
pub const HIBMC_DP_LANE_STATUS_OFFSET: c_uint = 0x10;
pub const HIBMC_DP_PMA_LANE0_OFFSET: c_uint = 0x18;
pub const HIBMC_DP_PMA_LANE1_OFFSET: c_uint = 0x1c;
pub const HIBMC_DP_HOST_SERDES_CTRL: c_uint = 0x1f001c;

pub const DP_SERDES_DONE: c_uint = 0x3;
// dp serdes TX-Deempth Configuration
pub const DP_SERDES_VOL0_PRE0: c_uint = 0x280;
pub const DP_SERDES_VOL0_PRE1: c_uint = 0x2300;
pub const DP_SERDES_VOL0_PRE2: c_uint = 0x53c0;
pub const DP_SERDES_VOL0_PRE3: c_uint = 0x8400;
pub const DP_SERDES_VOL1_PRE0: c_uint = 0x380;
pub const DP_SERDES_VOL1_PRE1: c_uint = 0x3440;
pub const DP_SERDES_VOL1_PRE2: c_uint = 0x6480;
pub const DP_SERDES_VOL2_PRE0: c_uint = 0x4c1;
pub const DP_SERDES_VOL2_PRE1: c_uint = 0x4500;
pub const DP_SERDES_VOL3_PRE0: c_uint = 0x600;
pub const DP_SERDES_BW_8_1: c_uint = 0x3;
pub const DP_SERDES_BW_5_4: c_uint = 0x2;
pub const DP_SERDES_BW_2_7: c_uint = 0x1;
pub const DP_SERDES_BW_1_62: c_uint = 0x0;

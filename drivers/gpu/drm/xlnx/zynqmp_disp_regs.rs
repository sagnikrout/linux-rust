//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xlnx/zynqmp_disp_regs.h
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
// ZynqMP Display Controller Driver - Register Definitions
//
// Copyright (C) 2017 - 2020 Xilinx, Inc.
//
// Authors:
// - Hyun Woo Kwon <hyun.kwon@xilinx.com>
// - Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

// Blender registers
pub const ZYNQMP_DISP_V_BLEND_BG_CLR_0: c_uint = 0x0;
pub const ZYNQMP_DISP_V_BLEND_BG_CLR_1: c_uint = 0x4;
pub const ZYNQMP_DISP_V_BLEND_BG_CLR_2: c_uint = 0x8;
pub const ZYNQMP_DISP_V_BLEND_BG_MAX: c_uint = 0xfff;
pub const ZYNQMP_DISP_V_BLEND_SET_GLOBAL_ALPHA: c_uint = 0xc;

pub const ZYNQMP_DISP_V_BLEND_OUTPUT_VID_FMT: c_uint = 0x14;
pub const ZYNQMP_DISP_V_BLEND_OUTPUT_VID_FMT_RGB: c_uint = 0x0;
pub const ZYNQMP_DISP_V_BLEND_OUTPUT_VID_FMT_YCBCR444: c_uint = 0x1;
pub const ZYNQMP_DISP_V_BLEND_OUTPUT_VID_FMT_YCBCR422: c_uint = 0x2;
pub const ZYNQMP_DISP_V_BLEND_OUTPUT_VID_FMT_YONLY: c_uint = 0x3;
pub const ZYNQMP_DISP_V_BLEND_OUTPUT_VID_FMT_XVYCC: c_uint = 0x4;

pub const ZYNQMP_DISP_V_BLEND_NUM_COEFF: c_int = 9;
pub const ZYNQMP_DISP_V_BLEND_NUM_OFFSET: c_int = 3;

pub const ZYNQMP_DISP_V_BLEND_CHROMA_KEY_ENABLE: c_uint = 0x1d0;
pub const ZYNQMP_DISP_V_BLEND_CHROMA_KEY_COMP1: c_uint = 0x1d4;
pub const ZYNQMP_DISP_V_BLEND_CHROMA_KEY_COMP2: c_uint = 0x1d8;
pub const ZYNQMP_DISP_V_BLEND_CHROMA_KEY_COMP3: c_uint = 0x1dc;
// AV buffer manager registers
pub const ZYNQMP_DISP_AV_BUF_FMT: c_uint = 0x0;
pub const ZYNQMP_DISP_AV_BUF_FMT_NL_VID_SHIFT: c_int = 0;

pub const ZYNQMP_DISP_AV_BUF_FMT_NL_GFX_SHIFT: c_int = 8;

pub const ZYNQMP_DISP_AV_BUF_NON_LIVE_LATENCY: c_uint = 0x8;

pub const ZYNQMP_DISP_AV_BUF_CHBUF_BURST_LEN_SHIFT: c_int = 2;

pub const ZYNQMP_DISP_AV_BUF_CHBUF_BURST_LEN_MAX: c_uint = 0xf;
pub const ZYNQMP_DISP_AV_BUF_CHBUF_BURST_LEN_AUD_MAX: c_uint = 0x3;
pub const ZYNQMP_DISP_AV_BUF_STATUS: c_uint = 0x28;
pub const ZYNQMP_DISP_AV_BUF_STC_CTRL: c_uint = 0x2c;

pub const ZYNQMP_DISP_AV_BUF_STC_CTRL_EVENT_SHIFT: c_int = 1;
pub const ZYNQMP_DISP_AV_BUF_STC_CTRL_EVENT_EX_VSYNC: c_int = 0;
pub const ZYNQMP_DISP_AV_BUF_STC_CTRL_EVENT_EX_VID: c_int = 1;
pub const ZYNQMP_DISP_AV_BUF_STC_CTRL_EVENT_EX_AUD: c_int = 2;
pub const ZYNQMP_DISP_AV_BUF_STC_CTRL_EVENT_INT_VSYNC: c_int = 3;
pub const ZYNQMP_DISP_AV_BUF_STC_INIT_VALUE0: c_uint = 0x30;
pub const ZYNQMP_DISP_AV_BUF_STC_INIT_VALUE1: c_uint = 0x34;
pub const ZYNQMP_DISP_AV_BUF_STC_ADJ: c_uint = 0x38;
pub const ZYNQMP_DISP_AV_BUF_STC_VID_VSYNC_TS0: c_uint = 0x3c;
pub const ZYNQMP_DISP_AV_BUF_STC_VID_VSYNC_TS1: c_uint = 0x40;
pub const ZYNQMP_DISP_AV_BUF_STC_EXT_VSYNC_TS0: c_uint = 0x44;
pub const ZYNQMP_DISP_AV_BUF_STC_EXT_VSYNC_TS1: c_uint = 0x48;
pub const ZYNQMP_DISP_AV_BUF_STC_CUSTOM_EVENT_TS0: c_uint = 0x4c;
pub const ZYNQMP_DISP_AV_BUF_STC_CUSTOM_EVENT_TS1: c_uint = 0x50;
pub const ZYNQMP_DISP_AV_BUF_STC_CUSTOM_EVENT2_TS0: c_uint = 0x54;
pub const ZYNQMP_DISP_AV_BUF_STC_CUSTOM_EVENT2_TS1: c_uint = 0x58;
pub const ZYNQMP_DISP_AV_BUF_STC_SNAPSHOT0: c_uint = 0x60;
pub const ZYNQMP_DISP_AV_BUF_STC_SNAPSHOT1: c_uint = 0x64;
pub const ZYNQMP_DISP_AV_BUF_OUTPUT: c_uint = 0x70;
pub const ZYNQMP_DISP_AV_BUF_OUTPUT_VID1_SHIFT: c_int = 0;

pub const ZYNQMP_DISP_AV_BUF_OUTPUT_VID2_SHIFT: c_int = 2;

pub const ZYNQMP_DISP_AV_BUF_OUTPUT_AUD1_SHIFT: c_int = 4;

pub const ZYNQMP_DISP_AV_BUF_HCOUNT_VCOUNT_INT0: c_uint = 0x74;
pub const ZYNQMP_DISP_AV_BUF_HCOUNT_VCOUNT_INT1: c_uint = 0x78;
pub const ZYNQMP_DISP_AV_BUF_PATTERN_GEN_SELECT: c_uint = 0x100;
pub const ZYNQMP_DISP_AV_BUF_CLK_SRC: c_uint = 0x120;

pub const ZYNQMP_DISP_AV_BUF_SRST_REG: c_uint = 0x124;

pub const ZYNQMP_DISP_AV_BUF_AUDIO_CH_CONFIG: c_uint = 0x12c;

pub const ZYNQMP_DISP_AV_BUF_LIVE_VID_CONFIG: c_uint = 0x224;

pub const ZYNQMP_DISP_AV_BUF_LIVE_GFX_CONFIG: c_uint = 0x234;
pub const ZYNQMP_DISP_AV_BUF_4BIT_SF: c_uint = 0x11111;
pub const ZYNQMP_DISP_AV_BUF_5BIT_SF: c_uint = 0x10842;
pub const ZYNQMP_DISP_AV_BUF_6BIT_SF: c_uint = 0x10410;
pub const ZYNQMP_DISP_AV_BUF_8BIT_SF: c_uint = 0x10101;
pub const ZYNQMP_DISP_AV_BUF_10BIT_SF: c_uint = 0x10040;
pub const ZYNQMP_DISP_AV_BUF_NULL_SF: c_int = 0;
pub const ZYNQMP_DISP_AV_BUF_NUM_SF: c_int = 3;
pub const ZYNQMP_DISP_AV_BUF_LIVE_CONFIG_BPC_6: c_uint = 0x0;
pub const ZYNQMP_DISP_AV_BUF_LIVE_CONFIG_BPC_8: c_uint = 0x1;
pub const ZYNQMP_DISP_AV_BUF_LIVE_CONFIG_BPC_10: c_uint = 0x2;
pub const ZYNQMP_DISP_AV_BUF_LIVE_CONFIG_BPC_12: c_uint = 0x3;

pub const ZYNQMP_DISP_AV_BUF_PALETTE_MEMORY: c_uint = 0x400;
// Audio registers
pub const ZYNQMP_DISP_AUD_MIXER_VOLUME: c_uint = 0x0;
pub const ZYNQMP_DISP_AUD_MIXER_VOLUME_NO_SCALE: c_uint = 0x20002000;
pub const ZYNQMP_DISP_AUD_MIXER_META_DATA: c_uint = 0x4;

pub const ZYNQMP_DISP_AUD_CH_A_DATA0: c_uint = 0x20;
pub const ZYNQMP_DISP_AUD_CH_A_DATA1: c_uint = 0x24;
pub const ZYNQMP_DISP_AUD_CH_A_DATA2: c_uint = 0x28;
pub const ZYNQMP_DISP_AUD_CH_A_DATA3: c_uint = 0x2c;
pub const ZYNQMP_DISP_AUD_CH_A_DATA4: c_uint = 0x30;
pub const ZYNQMP_DISP_AUD_CH_A_DATA5: c_uint = 0x34;
pub const ZYNQMP_DISP_AUD_CH_B_DATA0: c_uint = 0x38;
pub const ZYNQMP_DISP_AUD_CH_B_DATA1: c_uint = 0x3c;
pub const ZYNQMP_DISP_AUD_CH_B_DATA2: c_uint = 0x40;
pub const ZYNQMP_DISP_AUD_CH_B_DATA3: c_uint = 0x44;
pub const ZYNQMP_DISP_AUD_CH_B_DATA4: c_uint = 0x48;
pub const ZYNQMP_DISP_AUD_CH_B_DATA5: c_uint = 0x4c;
pub const ZYNQMP_DISP_AUD_SOFT_RESET: c_uint = 0xc00;


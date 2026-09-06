//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/mediatek/mt8188-mmsys.h
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

pub const MT8188_VDO0_SW0_RST_B: c_uint = 0x190;
pub const MT8188_VDO0_OVL_MOUT_EN: c_uint = 0xf14;

pub const MT8188_VDO0_SEL_IN: c_uint = 0xf34;
pub const MT8188_VDO0_SEL_OUT: c_uint = 0xf38;
pub const MT8188_VDO0_DISP_RDMA_SEL: c_uint = 0xf40;

pub const MT8188_VDO0_DSI0_SEL_IN: c_uint = 0xf44;

pub const MT8188_VDO0_DP_INTF0_SEL_IN: c_uint = 0xf4C;

pub const MT8188_VDO0_DISP_DITHER0_SEL_OUT: c_uint = 0xf58;

pub const MT8188_VDO0_VPP_MERGE_SEL: c_uint = 0xf60;

pub const MT8188_VDO0_DSC_WARP_SEL: c_uint = 0xf64;

pub const MT8188_VDO1_SW0_RST_B: c_uint = 0x1d0;
pub const MT8188_VDO1_HDR_TOP_CFG: c_uint = 0xd00;
pub const MT8188_VDO1_MIXER_IN1_ALPHA: c_uint = 0xd30;
pub const MT8188_VDO1_MIXER_IN1_PAD: c_uint = 0xd40;
pub const MT8188_VDO1_MIXER_VSYNC_LEN: c_uint = 0xd5c;
pub const MT8188_VDO1_MERGE0_ASYNC_CFG_WD: c_uint = 0xe30;
pub const MT8188_VDO1_HDRBE_ASYNC_CFG_WD: c_uint = 0xe70;
pub const MT8188_VDO1_VPP_MERGE0_P0_SEL_IN: c_uint = 0xf04;
pub const MT8188_VPP_MERGE0_P0_SEL_IN_FROM_MDP_RDMA0: c_int = 1;
pub const MT8188_VDO1_VPP_MERGE0_P1_SEL_IN: c_uint = 0xf08;
pub const MT8188_VPP_MERGE0_P1_SEL_IN_FROM_MDP_RDMA1: c_int = 1;
pub const MT8188_VDO1_DISP_DPI1_SEL_IN: c_uint = 0xf10;
pub const MT8188_DISP_DPI1_SEL_IN_FROM_VPP_MERGE4_MOUT: c_int = 0;
pub const MT8188_VDO1_DISP_DP_INTF0_SEL_IN: c_uint = 0xf14;
pub const MT8188_DISP_DP_INTF0_SEL_IN_FROM_VPP_MERGE4_MOUT: c_int = 0;
pub const MT8188_VDO1_MERGE4_SOUT_SEL: c_uint = 0xf18;

pub const MT8188_VDO1_MIXER_IN1_SEL_IN: c_uint = 0xf24;
pub const MT8188_MIXER_IN1_SEL_IN_FROM_MERGE0_ASYNC_SOUT: c_int = 1;
pub const MT8188_VDO1_MIXER_IN2_SEL_IN: c_uint = 0xf28;
pub const MT8188_MIXER_IN2_SEL_IN_FROM_MERGE1_ASYNC_SOUT: c_int = 1;
pub const MT8188_VDO1_MIXER_IN3_SEL_IN: c_uint = 0xf2c;
pub const MT8188_MIXER_IN3_SEL_IN_FROM_MERGE2_ASYNC_SOUT: c_int = 1;
pub const MT8188_VDO1_MIXER_IN4_SEL_IN: c_uint = 0xf30;
pub const MT8188_MIXER_IN4_SEL_IN_FROM_MERGE3_ASYNC_SOUT: c_int = 1;
pub const MT8188_VDO1_MIXER_OUT_SOUT_SEL: c_uint = 0xf34;
pub const MT8188_MIXER_SOUT_TO_MERGE4_ASYNC_SEL: c_int = 1;
pub const MT8188_VDO1_VPP_MERGE1_P0_SEL_IN: c_uint = 0xf3c;
pub const MT8188_VPP_MERGE1_P0_SEL_IN_FROM_MDP_RDMA2: c_int = 1;
pub const MT8188_VDO1_MERGE0_ASYNC_SOUT_SEL: c_uint = 0xf40;
pub const MT8188_SOUT_TO_MIXER_IN1_SEL: c_int = 1;
pub const MT8188_VDO1_MERGE1_ASYNC_SOUT_SEL: c_uint = 0xf44;
pub const MT8188_SOUT_TO_MIXER_IN2_SEL: c_int = 1;
pub const MT8188_VDO1_MERGE2_ASYNC_SOUT_SEL: c_uint = 0xf48;
pub const MT8188_SOUT_TO_MIXER_IN3_SEL: c_int = 1;
pub const MT8188_VDO1_MERGE3_ASYNC_SOUT_SEL: c_uint = 0xf4c;
pub const MT8188_SOUT_TO_MIXER_IN4_SEL: c_int = 1;
pub const MT8188_VDO1_MERGE4_ASYNC_SEL_IN: c_uint = 0xf50;
pub const MT8188_MERGE4_ASYNC_SEL_IN_FROM_MIXER_OUT_SOUT: c_int = 1;
pub const MT8188_VDO1_MIXER_IN1_SOUT_SEL: c_uint = 0xf58;
pub const MT8188_MIXER_IN1_SOUT_TO_DISP_MIXER: c_int = 0;
pub const MT8188_VDO1_MIXER_IN2_SOUT_SEL: c_uint = 0xf5c;
pub const MT8188_MIXER_IN2_SOUT_TO_DISP_MIXER: c_int = 0;
pub const MT8188_VDO1_MIXER_IN3_SOUT_SEL: c_uint = 0xf60;
pub const MT8188_MIXER_IN3_SOUT_TO_DISP_MIXER: c_int = 0;
pub const MT8188_VDO1_MIXER_IN4_SOUT_SEL: c_uint = 0xf64;
pub const MT8188_MIXER_IN4_SOUT_TO_DISP_MIXER: c_int = 0;
pub const MT8188_VDO1_MIXER_SOUT_SEL_IN: c_uint = 0xf68;
pub const MT8188_MIXER_SOUT_SEL_IN_FROM_DISP_MIXER: c_int = 0;

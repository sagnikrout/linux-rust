//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun6i-csi/sun6i_csi_reg.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2011-2018 Magewell Electronics Co., Ltd. (Nanjing)
// Author: Yong Deng <yong.deng@magewell.com>
// Copyright 2021-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

pub const SUN6I_CSI_EN_REG: c_uint = 0x0;

// Note that Allwinner manuals and code invert positive/negative definitions.
pub const SUN6I_CSI_IF_CFG_REG: c_uint = 0x4;

pub const SUN6I_CSI_CAP_REG: c_uint = 0x8;

pub const SUN6I_CSI_SYNC_CNT_REG: c_uint = 0xc;
pub const SUN6I_CSI_FIFO_THRS_REG: c_uint = 0x10;
pub const SUN6I_CSI_BT656_HEAD_CFG_REG: c_uint = 0x14;
pub const SUN6I_CSI_PTN_LEN_REG: c_uint = 0x30;
pub const SUN6I_CSI_PTN_ADDR_REG: c_uint = 0x34;
pub const SUN6I_CSI_VER_REG: c_uint = 0x3c;
pub const SUN6I_CSI_CH_CFG_REG: c_uint = 0x44;

pub const SUN6I_CSI_INPUT_FMT_RAW: c_int = 0;
pub const SUN6I_CSI_INPUT_FMT_YUV422: c_int = 3;
pub const SUN6I_CSI_INPUT_FMT_YUV420: c_int = 4;
// Note that Allwinner manuals and code invert frame/field definitions.
// RAW
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_RAW_8: c_int = 0;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_RAW_10: c_int = 1;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_RAW_12: c_int = 2;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_RGB565: c_int = 4;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_RGB888: c_int = 5;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_PRGB888: c_int = 6;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_RAW_8: c_int = 8;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_RAW_10: c_int = 9;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_RAW_12: c_int = 10;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_RGB565: c_int = 12;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_RGB888: c_int = 13;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_PRGB888: c_int = 14;
// YUV
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV422P: c_int = 0;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV420P: c_int = 1;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_YUV420P: c_int = 2;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_YUV422P: c_int = 3;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV422SP: c_int = 4;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV420SP: c_int = 5;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_YUV420SP: c_int = 6;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_YUV422SP: c_int = 7;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV422MB: c_int = 8;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV420MB: c_int = 9;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_YUV420MB: c_int = 10;
pub const SUN6I_CSI_OUTPUT_FMT_FIELD_YUV422MB: c_int = 11;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV422SP_10: c_int = 12;
pub const SUN6I_CSI_OUTPUT_FMT_FRAME_YUV420SP_10: c_int = 13;
// YUV Planar
pub const SUN6I_CSI_INPUT_YUV_SEQ_YUYV: c_int = 0;
pub const SUN6I_CSI_INPUT_YUV_SEQ_YVYU: c_int = 1;
pub const SUN6I_CSI_INPUT_YUV_SEQ_UYVY: c_int = 2;
pub const SUN6I_CSI_INPUT_YUV_SEQ_VYUY: c_int = 3;
// YUV Semi-planar
pub const SUN6I_CSI_INPUT_YUV_SEQ_UV: c_int = 0;
pub const SUN6I_CSI_INPUT_YUV_SEQ_VU: c_int = 1;
pub const SUN6I_CSI_CH_SCALE_REG: c_uint = 0x4c;

pub const SUN6I_CSI_CH_FIFO0_ADDR_REG: c_uint = 0x50;
pub const SUN6I_CSI_CH_FIFO1_ADDR_REG: c_uint = 0x58;
pub const SUN6I_CSI_CH_FIFO2_ADDR_REG: c_uint = 0x60;
pub const SUN6I_CSI_CH_STA_REG: c_uint = 0x6c;

pub const SUN6I_CSI_CH_INT_EN_REG: c_uint = 0x70;

pub const SUN6I_CSI_CH_INT_STA_REG: c_uint = 0x74;
pub const SUN6I_CSI_CH_INT_STA_CLEAR: c_uint = 0xff;

pub const SUN6I_CSI_CH_FLD1_VSIZE_REG: c_uint = 0x78;

pub const SUN6I_CSI_CH_HSIZE_REG: c_uint = 0x80;

pub const SUN6I_CSI_CH_VSIZE_REG: c_uint = 0x84;

pub const SUN6I_CSI_CH_BUF_LEN_REG: c_uint = 0x88;

pub const SUN6I_CSI_CH_FLIP_SIZE_REG: c_uint = 0x8c;

pub const SUN6I_CSI_CH_FRM_CLK_CNT_REG: c_uint = 0x90;
pub const SUN6I_CSI_CH_ACC_ITNL_CLK_CNT_REG: c_uint = 0x94;
pub const SUN6I_CSI_CH_FIFO_STAT_REG: c_uint = 0x98;
pub const SUN6I_CSI_CH_PCLK_STAT_REG: c_uint = 0x9c;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-scaler.h
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
// drivers/gpu/drm/exynos/regs-scaler.h
//
// Copyright (c) 2017 Samsung Electronics Co., Ltd.
// http://www.samsung.com
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//
// Register definition file for Samsung scaler driver
//
// Register part
// Global setting
pub const SCALER_STATUS: c_uint = 0x0	/* no shadow */;
pub const SCALER_CFG: c_uint = 0x4;
// Interrupt
pub const SCALER_INT_EN: c_uint = 0x8	/* no shadow */;
pub const SCALER_INT_STATUS: c_uint = 0xc	/* no shadow */;
// SRC
pub const SCALER_SRC_CFG: c_uint = 0x10;
pub const SCALER_SRC_Y_BASE: c_uint = 0x14;
pub const SCALER_SRC_CB_BASE: c_uint = 0x18;
pub const SCALER_SRC_CR_BASE: c_uint = 0x294;
pub const SCALER_SRC_SPAN: c_uint = 0x1c;
pub const SCALER_SRC_Y_POS: c_uint = 0x20;
pub const SCALER_SRC_WH: c_uint = 0x24;
pub const SCALER_SRC_C_POS: c_uint = 0x28;
// DST
pub const SCALER_DST_CFG: c_uint = 0x30;
pub const SCALER_DST_Y_BASE: c_uint = 0x34;
pub const SCALER_DST_CB_BASE: c_uint = 0x38;
pub const SCALER_DST_CR_BASE: c_uint = 0x298;
pub const SCALER_DST_SPAN: c_uint = 0x3c;
pub const SCALER_DST_WH: c_uint = 0x40;
pub const SCALER_DST_POS: c_uint = 0x44;
// Ratio
pub const SCALER_H_RATIO: c_uint = 0x50;
pub const SCALER_V_RATIO: c_uint = 0x54;
// Rotation
pub const SCALER_ROT_CFG: c_uint = 0x58;
// Coefficient
//
// YHCOEF_{x}{A|B|C|D}			CHCOEF_{x}{A|B|C|D}
//
// A	B	C	D	A	B	C	D
// 0	60	64	68	6c	140	144	148	14c
// 1	70	74	78	7c	150	154	158	15c
// 2	80	84	88	8c	160	164	168	16c
// 3	90	94	98	9c	170	174	178	17c
// 4	a0	a4	a8	ac	180	184	188	18c
// 5	b0	b4	b8	bc	190	194	198	19c
// 6	c0	c4	c8	cc	1a0	1a4	1a8	1ac
// 7	d0	d4	d8	dc	1b0	1b4	1b8	1bc
// 8	e0	e4	e8	ec	1c0	1c4	1c8	1cc
//
// YVCOEF_{x}{A|B}			CVCOEF_{x}{A|B}
//
// A	B			A	B
// 0	f0	f4			1d0	1d4
// 1	f8	fc			1d8	1dc
// 2	100	104			1e0	1e4
// 3	108	10c			1e8	1ec
// 4	110	114			1f0	1f4
// 5	118	11c			1f8	1fc
// 6	120	124			200	204
// 7	128	12c			208	20c
// 8	130	134			210	214
//

// Color Space Conversion

// Dithering
pub const SCALER_DITH_CFG: c_uint = 0x250;
// Version Number
pub const SCALER_VER: c_uint = 0x260	/* no shadow */;
// Cycle count and Timeout
pub const SCALER_CYCLE_COUNT: c_uint = 0x278	/* no shadow */;
pub const SCALER_TIMEOUT_CTRL: c_uint = 0x2c0	/* no shadow */;
pub const SCALER_TIMEOUT_CNT: c_uint = 0x2c4	/* no shadow */;
// Blending
pub const SCALER_SRC_BLEND_COLOR: c_uint = 0x280;
pub const SCALER_SRC_BLEND_ALPHA: c_uint = 0x284;
pub const SCALER_DST_BLEND_COLOR: c_uint = 0x288;
pub const SCALER_DST_BLEND_ALPHA: c_uint = 0x28c;
// Color Fill
pub const SCALER_FILL_COLOR: c_uint = 0x290;
// Multiple Command Queue
pub const SCALER_ADDR_Q_CONFIG: c_uint = 0x2a0	/* no shadow */;
pub const SCALER_SRC_ADDR_Q_STATUS: c_uint = 0x2a4	/* no shadow */;
pub const SCALER_SRC_ADDR_Q: c_uint = 0x2a8	/* no shadow */;
// CRC
pub const SCALER_CRC_COLOR00_10: c_uint = 0x2b0	/* no shadow */;
pub const SCALER_CRC_COLOR20_30: c_uint = 0x2b4	/* no shadow */;
pub const SCALER_CRC_COLOR01_11: c_uint = 0x2b8	/* no shadow */;
pub const SCALER_CRC_COLOR21_31: c_uint = 0x2bc	/* no shadow */;
// Shadow Registers
pub const SCALER_SHADOW_OFFSET: c_uint = 0x1000;
// Bit definition part

// SCALER_STATUS

// SCALER_CFG

// SCALER_INT_EN

// SCALER_INT_STATUS

// SCALER_SRC_CFG

pub const SCALER_YUV420_2P_UV: c_int = 0;
pub const SCALER_YUV422_2P_UV: c_int = 2;
pub const SCALER_YUV444_2P_UV: c_int = 3;
pub const SCALER_RGB_565: c_int = 4;
pub const SCALER_ARGB1555: c_int = 5;
pub const SCALER_ARGB8888: c_int = 6;
pub const SCALER_ARGB8888_PRE: c_int = 7;
pub const SCALER_YUV422_1P_YVYU: c_int = 9;
pub const SCALER_YUV422_1P_YUYV: c_int = 10;
pub const SCALER_YUV422_1P_UYVY: c_int = 11;
pub const SCALER_ARGB4444: c_int = 12;
pub const SCALER_L8A8: c_int = 13;
pub const SCALER_RGBA8888: c_int = 14;
pub const SCALER_L8: c_int = 15;
pub const SCALER_YUV420_2P_VU: c_int = 16;
pub const SCALER_YUV422_2P_VU: c_int = 18;
pub const SCALER_YUV444_2P_VU: c_int = 19;
pub const SCALER_YUV420_3P: c_int = 20;
pub const SCALER_YUV422_3P: c_int = 22;
pub const SCALER_YUV444_3P: c_int = 23;
// SCALER_SRC_SPAN

// SCALER_SRC_Y_POS

// SCALER_SRC_WH

// SCALER_SRC_C_POS

// SCALER_DST_CFG

// SCALER_DST_SPAN

// SCALER_DST_WH

// SCALER_DST_POS

// SCALER_H_RATIO

// SCALER_V_RATIO

// SCALER_ROT_CFG

pub const SCALER_ROT_MODE_90: c_int = 1;
pub const SCALER_ROT_MODE_180: c_int = 2;
pub const SCALER_ROT_MODE_270: c_int = 3;
// SCALER_HCOEF, SCALER_VCOEF

// SCALER_CSC_COEFxy

// SCALER_DITH_CFG

// SCALER_TIMEOUT_CTRL

// SCALER_TIMEOUT_CNT

// SCALER_SRC_BLEND_COLOR

// SCALER_SRC_BLEND_ALPHA

// SCALER_DST_BLEND_COLOR

// SCALER_DST_BLEND_ALPHA

// SCALER_FILL_COLOR

// SCALER_ADDR_Q_CONFIG

// SCALER_SRC_ADDR_Q_STATUS

// SCALER_DST_ADDR_Q_STATUS

// SCALER_CRC_COLOR00_10

// SCALER_CRC_COLOR20_30

// SCALER_CRC_COLOR01_11

// SCALER_CRC_COLOR21_31


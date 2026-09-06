//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rga/rga3-hw.h
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
// Copyright (C) Pengutronix e.K.
// Author: Sven Püschel <s.pueschel@pengutronix.de>
//

pub const RGA3_CMDBUF_SIZE: c_uint = 0xb8;
pub const RGA3_MIN_WIDTH: c_int = 128;
pub const RGA3_MIN_HEIGHT: c_int = 128;

pub const RGA3_MAX_SCALING_FACTOR: c_int = 8;
pub const RGA3_RESET_TIMEOUT: c_int = 1000;
// Registers address
// sys reg
pub const RGA3_SYS_CTRL: c_uint = 0x000;
pub const RGA3_CMD_CTRL: c_uint = 0x004;
pub const RGA3_CMD_ADDR: c_uint = 0x008;
pub const RGA3_MI_GROUP_CTRL: c_uint = 0x00c;
pub const RGA3_ARQOS_CTRL: c_uint = 0x010;
pub const RGA3_VERSION_NUM: c_uint = 0x018;
pub const RGA3_VERSION_TIM: c_uint = 0x01c;
pub const RGA3_INT_EN: c_uint = 0x020;
pub const RGA3_INT_RAW: c_uint = 0x024;
pub const RGA3_INT_MSK: c_uint = 0x028;
pub const RGA3_INT_CLR: c_uint = 0x02c;
pub const RGA3_RO_SRST: c_uint = 0x030;
pub const RGA3_STATUS0: c_uint = 0x034;
pub const RGA3_SCAN_CNT: c_uint = 0x038;
pub const RGA3_CMD_STATE: c_uint = 0x040;
// cmd reg
pub const RGA3_WIN0_RD_CTRL: c_uint = 0x100;

pub const RGA3_WIN0_Y_BASE: c_uint = 0x110;
pub const RGA3_WIN0_U_BASE: c_uint = 0x114;
pub const RGA3_WIN0_V_BASE: c_uint = 0x118;
pub const RGA3_WIN0_VIR_STRIDE: c_uint = 0x11c;
pub const RGA3_WIN0_FBC_OFF: c_uint = 0x120;
pub const RGA3_WIN0_SRC_SIZE: c_uint = 0x124;
pub const RGA3_WIN0_ACT_OFF: c_uint = 0x128;
pub const RGA3_WIN0_ACT_SIZE: c_uint = 0x12c;
pub const RGA3_WIN0_DST_SIZE: c_uint = 0x130;
pub const RGA3_WIN0_SCL_FAC: c_uint = 0x134;
pub const RGA3_WIN0_UV_VIR_STRIDE: c_uint = 0x138;
pub const RGA3_WIN1_RD_CTRL: c_uint = 0x140;
pub const RGA3_WIN1_Y_BASE: c_uint = 0x150;
pub const RGA3_WIN1_U_BASE: c_uint = 0x154;
pub const RGA3_WIN1_V_BASE: c_uint = 0x158;
pub const RGA3_WIN1_VIR_STRIDE: c_uint = 0x15c;
pub const RGA3_WIN1_FBC_OFF: c_uint = 0x160;
pub const RGA3_WIN1_SRC_SIZE: c_uint = 0x164;
pub const RGA3_WIN1_ACT_OFF: c_uint = 0x168;
pub const RGA3_WIN1_ACT_SIZE: c_uint = 0x16c;
pub const RGA3_WIN1_DST_SIZE: c_uint = 0x170;
pub const RGA3_WIN1_SCL_FAC: c_uint = 0x174;
pub const RGA3_WIN1_UV_VIR_STRIDE: c_uint = 0x178;
pub const RGA3_OVLP_CTRL: c_uint = 0x180;
pub const RGA3_OVLP_OFF: c_uint = 0x184;
pub const RGA3_OVLP_TOP_KEY_MIN: c_uint = 0x188;
pub const RGA3_OVLP_TOP_KEY_MAX: c_uint = 0x18c;
pub const RGA3_OVLP_TOP_CTRL: c_uint = 0x190;
pub const RGA3_OVLP_BOT_CTRL: c_uint = 0x194;
pub const RGA3_OVLP_TOP_ALPHA: c_uint = 0x198;
pub const RGA3_OVLP_BOT_ALPHA: c_uint = 0x19c;
pub const RGA3_WR_CTRL: c_uint = 0x1a0;
pub const RGA3_WR_FBCE_CTRL: c_uint = 0x1a4;
pub const RGA3_WR_VIR_STRIDE: c_uint = 0x1a8;
pub const RGA3_WR_PL_VIR_STRIDE: c_uint = 0x1ac;
pub const RGA3_WR_Y_BASE: c_uint = 0x1b0;
pub const RGA3_WR_U_BASE: c_uint = 0x1b4;
pub const RGA3_WR_V_BASE: c_uint = 0x1b8;
// Registers value
pub const RGA3_COLOR_FMT_YUV420: c_uint = 0x0;
pub const RGA3_COLOR_FMT_YUV422: c_uint = 0x1;
pub const RGA3_COLOR_FMT_YUV420_10B: c_uint = 0x2;
pub const RGA3_COLOR_FMT_YUV422_10B: c_uint = 0x3;
//
// Use memory ordering names
// instead of the datasheet naming RGB formats in big endian order
//
pub const RGA3_COLOR_FMT_BGR565: c_uint = 0x4;
pub const RGA3_COLOR_FMT_BGR888: c_uint = 0x5;

pub const RGA3_COLOR_FMT_BGRA8888: c_uint = 0x6;

// the following are only supported as inputs
pub const RGA3_COLOR_FMT_ABGR8888: c_uint = 0x7;
//
// the following seem to be unnecessary,
// as they can be achieved with RB swaps
//
pub const RGA3_COLOR_FMT_RGBA8888: c_uint = 0x8;
pub const RGA3_COLOR_FMT_ARGB8888: c_uint = 0x9;
pub const RGA3_RDWR_FORMAT_SEMI_PLANAR: c_uint = 0x1;
pub const RGA3_RDWR_FORMAT_INTERLEAVED: c_uint = 0x2;
pub const RGA3_CMD_MODE_MASTER: c_uint = 0x1;
pub const RGA3_WIN_CSC_MODE_BT601_L: c_uint = 0x0;
pub const RGA3_WIN_CSC_MODE_BT709_L: c_uint = 0x1;
pub const RGA3_WIN_CSC_MODE_BT601_F: c_uint = 0x2;
pub const RGA3_WIN_CSC_MODE_BT2020_L: c_uint = 0x3;
// RGA masks
// SYS_CTRL

// CMD_CTRL

// VERSION_NUM

// INT_*

// RO_SRST

// *_SIZE

// SCL_FAC

// WINx_CTRL

// COLOR_CTRL

// ALPHA_CTRL

// WR_CTRL

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rga3_fmt {
    pub fourcc: u32,
    pub hw_format: u8,
    pub rbuv_swap: bool,
    pub yc_swap: bool,
}

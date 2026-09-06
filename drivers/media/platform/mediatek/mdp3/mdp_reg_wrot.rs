//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mdp_reg_wrot.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//
pub const VIDO_CTRL: c_uint = 0x000;
pub const VIDO_MAIN_BUF_SIZE: c_uint = 0x008;
pub const VIDO_SOFT_RST: c_uint = 0x010;
pub const VIDO_SOFT_RST_STAT: c_uint = 0x014;
pub const VIDO_CROP_OFST: c_uint = 0x020;
pub const VIDO_TAR_SIZE: c_uint = 0x024;
pub const VIDO_OFST_ADDR: c_uint = 0x02c;
pub const VIDO_STRIDE: c_uint = 0x030;
pub const VIDO_OFST_ADDR_C: c_uint = 0x038;
pub const VIDO_STRIDE_C: c_uint = 0x03c;
pub const VIDO_CTRL_2: c_uint = 0x048;
pub const VIDO_DITHER: c_uint = 0x054;
pub const VIDO_STRIDE_V: c_uint = 0x06c;
pub const VIDO_OFST_ADDR_V: c_uint = 0x068;
pub const VIDO_RSV_1: c_uint = 0x070;
pub const VIDO_DMA_PREULTRA: c_uint = 0x074;
pub const VIDO_IN_SIZE: c_uint = 0x078;
pub const VIDO_ROT_EN: c_uint = 0x07c;
pub const VIDO_FIFO_TEST: c_uint = 0x080;
pub const VIDO_MAT_CTRL: c_uint = 0x084;
pub const VIDO_SCAN_10BIT: c_uint = 0x0dc;
pub const VIDO_PENDING_ZERO: c_uint = 0x0e0;
pub const VIDO_BASE_ADDR: c_uint = 0xf00;
pub const VIDO_BASE_ADDR_C: c_uint = 0xf04;
pub const VIDO_BASE_ADDR_V: c_uint = 0xf08;
// MASK
pub const VIDO_CTRL_MASK: c_uint = 0xf530711f;
pub const VIDO_MAIN_BUF_SIZE_MASK: c_uint = 0x1fff7f77;
pub const VIDO_SOFT_RST_MASK: c_uint = 0x00000001;
pub const VIDO_SOFT_RST_STAT_MASK: c_uint = 0x00000001;
pub const VIDO_TAR_SIZE_MASK: c_uint = 0x1fff1fff;
pub const VIDO_CROP_OFST_MASK: c_uint = 0x1fff1fff;
pub const VIDO_OFST_ADDR_MASK: c_uint = 0x0fffffff;
pub const VIDO_STRIDE_MASK: c_uint = 0x0000ffff;
pub const VIDO_OFST_ADDR_C_MASK: c_uint = 0x0fffffff;
pub const VIDO_STRIDE_C_MASK: c_uint = 0x0000ffff;
pub const VIDO_CTRL_2_MASK: c_uint = 0x0000000f;
pub const VIDO_DITHER_MASK: c_uint = 0xff000001;
pub const VIDO_STRIDE_V_MASK: c_uint = 0x0000ffff;
pub const VIDO_OFST_ADDR_V_MASK: c_uint = 0x0fffffff;
pub const VIDO_RSV_1_MASK: c_uint = 0xffffffff;
pub const VIDO_DMA_PREULTRA_MASK: c_uint = 0x00ffffff;
pub const VIDO_IN_SIZE_MASK: c_uint = 0x1fff1fff;
pub const VIDO_ROT_EN_MASK: c_uint = 0x00000001;
pub const VIDO_FIFO_TEST_MASK: c_uint = 0x00000fff;
pub const VIDO_MAT_CTRL_MASK: c_uint = 0x000000f3;
pub const VIDO_SCAN_10BIT_MASK: c_uint = 0x0000000f;
pub const VIDO_PENDING_ZERO_MASK: c_uint = 0x07ffffff;
pub const VIDO_BASE_ADDR_MASK: c_uint = 0xffffffff;
pub const VIDO_BASE_ADDR_C_MASK: c_uint = 0xffffffff;
pub const VIDO_BASE_ADDR_V_MASK: c_uint = 0xffffffff;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mdp_reg_wdma.h
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
pub const WDMA_EN: c_uint = 0x008;
pub const WDMA_RST: c_uint = 0x00c;
pub const WDMA_CFG: c_uint = 0x014;
pub const WDMA_SRC_SIZE: c_uint = 0x018;
pub const WDMA_CLIP_SIZE: c_uint = 0x01c;
pub const WDMA_CLIP_COORD: c_uint = 0x020;
pub const WDMA_DST_W_IN_BYTE: c_uint = 0x028;
pub const WDMA_ALPHA: c_uint = 0x02c;
pub const WDMA_BUF_CON2: c_uint = 0x03c;
pub const WDMA_DST_UV_PITCH: c_uint = 0x078;
pub const WDMA_DST_ADDR_OFFSET: c_uint = 0x080;
pub const WDMA_DST_U_ADDR_OFFSET: c_uint = 0x084;
pub const WDMA_DST_V_ADDR_OFFSET: c_uint = 0x088;
pub const WDMA_FLOW_CTRL_DBG: c_uint = 0x0a0;
pub const WDMA_DST_ADDR: c_uint = 0xf00;
pub const WDMA_DST_U_ADDR: c_uint = 0xf04;
pub const WDMA_DST_V_ADDR: c_uint = 0xf08;
// MASK
pub const WDMA_EN_MASK: c_uint = 0x00000001;
pub const WDMA_RST_MASK: c_uint = 0x00000001;
pub const WDMA_CFG_MASK: c_uint = 0xff03bff0;
pub const WDMA_SRC_SIZE_MASK: c_uint = 0x3fff3fff;
pub const WDMA_CLIP_SIZE_MASK: c_uint = 0x3fff3fff;
pub const WDMA_CLIP_COORD_MASK: c_uint = 0x3fff3fff;
pub const WDMA_DST_W_IN_BYTE_MASK: c_uint = 0x0000ffff;
pub const WDMA_ALPHA_MASK: c_uint = 0x800000ff;
pub const WDMA_BUF_CON2_MASK: c_uint = 0xffffffff;
pub const WDMA_DST_UV_PITCH_MASK: c_uint = 0x0000ffff;
pub const WDMA_DST_ADDR_OFFSET_MASK: c_uint = 0x0fffffff;
pub const WDMA_DST_U_ADDR_OFFSET_MASK: c_uint = 0x0fffffff;
pub const WDMA_DST_V_ADDR_OFFSET_MASK: c_uint = 0x0fffffff;
pub const WDMA_FLOW_CTRL_DBG_MASK: c_uint = 0x0000f3ff;
pub const WDMA_DST_ADDR_MASK: c_uint = 0xffffffff;
pub const WDMA_DST_U_ADDR_MASK: c_uint = 0xffffffff;
pub const WDMA_DST_V_ADDR_MASK: c_uint = 0xffffffff;

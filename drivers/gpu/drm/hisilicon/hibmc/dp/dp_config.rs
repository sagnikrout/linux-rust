//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/hibmc/dp/dp_config.h
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
pub const HIBMC_DP_BPP: c_int = 24;
pub const HIBMC_DP_SYMBOL_PER_FCLK: c_int = 4;
pub const HIBMC_DP_MSA1: c_uint = 0x20;
pub const HIBMC_DP_MSA2: c_uint = 0x845c00;
pub const HIBMC_DP_OFFSET: c_uint = 0x1e0000;
pub const HIBMC_DP_HDCP: c_uint = 0x2;
pub const HIBMC_DP_INT_RST: c_uint = 0xffff;
pub const HIBMC_DP_DPTX_RST: c_uint = 0x3ff;
pub const HIBMC_DP_CLK_EN: c_uint = 0x7;
pub const HIBMC_DP_SYNC_EN_MASK: c_uint = 0x3;
pub const HIBMC_DP_LINK_RATE_CAL: c_int = 27;

pub const HIBMC_DP_INT_ENABLE: c_uint = 0xc;
// HIBMC_DP_LINK_RATE_CAL * 10000 * 80% = 216000
pub const DP_MODE_VALI_CAL: c_int = 216000;

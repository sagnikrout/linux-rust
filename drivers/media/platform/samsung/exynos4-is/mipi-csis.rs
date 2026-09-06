//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/mipi-csis.h
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
// Samsung S5P/EXYNOS4 SoC series MIPI-CSI receiver driver
//
// Copyright (C) 2011 Samsung Electronics Co., Ltd.
//

pub const CSIS_MAX_ENTITIES: c_int = 2;
pub const CSIS0_MAX_LANES: c_int = 4;
pub const CSIS1_MAX_LANES: c_int = 2;
pub const CSIS_PAD_SINK: c_int = 0;
pub const CSIS_PAD_SOURCE: c_int = 1;
pub const CSIS_PADS_NUM: c_int = 2;
pub const S5PCSIS_DEF_PIX_WIDTH: c_int = 640;
pub const S5PCSIS_DEF_PIX_HEIGHT: c_int = 480;

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/hantro_v4l2.h
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
// Hantro VPU codec driver
//
// Copyright (C) 2018 Rockchip Electronics Co., Ltd.
// Alpha Lin <Alpha.Lin@rock-chips.com>
// Jeffy Chen <jeffy.chen@rock-chips.com>
//
// Copyright 2018 Google LLC.
// Tomasz Figa <tfiga@chromium.org>
//
// Based on s5p-mfc driver by Samsung Electronics Co., Ltd.
// Copyright (C) 2011 Samsung Electronics Co., Ltd.
//

extern "C" {
    pub fn hantro_reset_raw_fmt(ctx: *mut hantro_ctx, bit_depth: c_int, need_postproc: bool) -> c_int;
}
extern "C" {
    pub fn hantro_reset_fmts(ctx: *mut hantro_ctx);
}
extern "C" {
    pub fn hantro_get_format_depth(fourcc: u32) -> c_int;
}

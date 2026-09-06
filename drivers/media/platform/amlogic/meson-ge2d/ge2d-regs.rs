//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amlogic/meson-ge2d/ge2d-regs.h
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
//
// Copyright (C) 2020 BayLibre, SAS
// Author: Neil Armstrong <narmstrong@baylibre.com>
// Copyright (C) 2017 Amlogic, Inc. All rights reserved.
//
// Registers starts at (GE2D_REG(0x8a0 * 4)

pub const GE2D_FORMAT_8BIT: c_int = 0;
pub const GE2D_FORMAT_16BIT: c_int = 1;
pub const GE2D_FORMAT_24BIT: c_int = 2;
pub const GE2D_FORMAT_32BIT: c_int = 3;
// 16 bit
pub const GE2D_COLOR_MAP_YUV422: c_int = 0;
pub const GE2D_COLOR_MAP_RGB655: c_int = 1;
pub const GE2D_COLOR_MAP_YUV655: c_int = 1;
pub const GE2D_COLOR_MAP_RGB844: c_int = 2;
pub const GE2D_COLOR_MAP_YUV844: c_int = 2;
pub const GE2D_COLOR_MAP_RGBA6442: c_int = 3;
pub const GE2D_COLOR_MAP_YUVA6442: c_int = 3;
pub const GE2D_COLOR_MAP_RGBA4444: c_int = 4;
pub const GE2D_COLOR_MAP_YUVA4444: c_int = 4;
pub const GE2D_COLOR_MAP_RGB565: c_int = 5;
pub const GE2D_COLOR_MAP_YUV565: c_int = 5;
pub const GE2D_COLOR_MAP_ARGB4444: c_int = 6;
pub const GE2D_COLOR_MAP_AYUV4444: c_int = 6;
pub const GE2D_COLOR_MAP_ARGB1555: c_int = 7;
pub const GE2D_COLOR_MAP_AYUV1555: c_int = 7;
pub const GE2D_COLOR_MAP_RGBA4642: c_int = 8;
pub const GE2D_COLOR_MAP_YUVA4642: c_int = 8;
// 24 bit
pub const GE2D_COLOR_MAP_RGB888: c_int = 0;
pub const GE2D_COLOR_MAP_YUV444: c_int = 0;
pub const GE2D_COLOR_MAP_RGBA5658: c_int = 1;
pub const GE2D_COLOR_MAP_YUVA5658: c_int = 1;
pub const GE2D_COLOR_MAP_ARGB8565: c_int = 2;
pub const GE2D_COLOR_MAP_AYUV8565: c_int = 2;
pub const GE2D_COLOR_MAP_RGBA6666: c_int = 3;
pub const GE2D_COLOR_MAP_YUVA6666: c_int = 3;
pub const GE2D_COLOR_MAP_ARGB6666: c_int = 4;
pub const GE2D_COLOR_MAP_AYUV6666: c_int = 4;
pub const GE2D_COLOR_MAP_BGR888: c_int = 5;
pub const GE2D_COLOR_MAP_VUY888: c_int = 5;
// 32 bit
pub const GE2D_COLOR_MAP_RGBA8888: c_int = 0;
pub const GE2D_COLOR_MAP_YUVA8888: c_int = 0;
pub const GE2D_COLOR_MAP_ARGB8888: c_int = 1;
pub const GE2D_COLOR_MAP_AYUV8888: c_int = 1;
pub const GE2D_COLOR_MAP_ABGR8888: c_int = 2;
pub const GE2D_COLOR_MAP_AVUY8888: c_int = 2;
pub const GE2D_COLOR_MAP_BGRA8888: c_int = 3;
pub const GE2D_COLOR_MAP_VUYA8888: c_int = 3;

pub const OPERATION_LOGIC: c_int = 5;

pub const COLOR_FACTOR_ZERO: c_int = 0;
pub const COLOR_FACTOR_ONE: c_int = 1;
pub const COLOR_FACTOR_SRC_COLOR: c_int = 2;
pub const COLOR_FACTOR_ONE_MINUS_SRC_COLOR: c_int = 3;
pub const COLOR_FACTOR_DST_COLOR: c_int = 4;
pub const COLOR_FACTOR_ONE_MINUS_DST_COLOR: c_int = 5;
pub const COLOR_FACTOR_SRC_ALPHA: c_int = 6;
pub const COLOR_FACTOR_ONE_MINUS_SRC_ALPHA: c_int = 7;
pub const COLOR_FACTOR_DST_ALPHA: c_int = 8;
pub const COLOR_FACTOR_ONE_MINUS_DST_ALPHA: c_int = 9;
pub const COLOR_FACTOR_CONST_COLOR: c_int = 10;
pub const COLOR_FACTOR_ONE_MINUS_CONST_COLOR: c_int = 11;
pub const COLOR_FACTOR_CONST_ALPHA: c_int = 12;
pub const COLOR_FACTOR_ONE_MINUS_CONST_ALPHA: c_int = 13;
pub const COLOR_FACTOR_SRC_ALPHA_SATURATE: c_int = 14;

pub const LOGIC_OPERATION_CLEAR: c_int = 0;
pub const LOGIC_OPERATION_COPY: c_int = 1;
pub const LOGIC_OPERATION_NOOP: c_int = 2;
pub const LOGIC_OPERATION_SET: c_int = 3;
pub const LOGIC_OPERATION_COPY_INVERT: c_int = 4;
pub const LOGIC_OPERATION_INVERT: c_int = 5;
pub const LOGIC_OPERATION_AND_REVERSE: c_int = 6;
pub const LOGIC_OPERATION_OR_REVERSE: c_int = 7;
pub const LOGIC_OPERATION_AND: c_int = 8;
pub const LOGIC_OPERATION_OR: c_int = 9;
pub const LOGIC_OPERATION_NAND: c_int = 10;
pub const LOGIC_OPERATION_NOR: c_int = 11;
pub const LOGIC_OPERATION_XOR: c_int = 12;
pub const LOGIC_OPERATION_EQUIV: c_int = 13;
pub const LOGIC_OPERATION_AND_INVERT: c_int = 14;
pub const LOGIC_OPERATION_OR_INVERT: c_int = 15;

pub const ALPHA_FACTOR_ZERO: c_int = 0;
pub const ALPHA_FACTOR_ONE: c_int = 1;
pub const ALPHA_FACTOR_SRC_ALPHA: c_int = 2;
pub const ALPHA_FACTOR_ONE_MINUS_SRC_ALPHA: c_int = 3;
pub const ALPHA_FACTOR_DST_ALPHA: c_int = 4;
pub const ALPHA_FACTOR_ONE_MINUS_DST_ALPHA: c_int = 5;
pub const ALPHA_FACTOR_CONST_ALPHA: c_int = 6;
pub const ALPHA_FACTOR_ONE_MINUS_CONST_ALPHA: c_int = 7;


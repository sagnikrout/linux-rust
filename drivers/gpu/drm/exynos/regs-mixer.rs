//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-mixer.h
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
// Cloned from drivers/media/video/s5p-tv/regs-mixer.h
//
// Copyright (c) 2010-2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Mixer register header file for Samsung Mixer driver
//
// Register part
//
pub const MXR_STATUS: c_uint = 0x0000;
pub const MXR_CFG: c_uint = 0x0004;
pub const MXR_INT_EN: c_uint = 0x0008;
pub const MXR_INT_STATUS: c_uint = 0x000C;
pub const MXR_LAYER_CFG: c_uint = 0x0010;
pub const MXR_VIDEO_CFG: c_uint = 0x0014;
pub const MXR_GRAPHIC0_CFG: c_uint = 0x0020;
pub const MXR_GRAPHIC0_BASE: c_uint = 0x0024;
pub const MXR_GRAPHIC0_SPAN: c_uint = 0x0028;
pub const MXR_GRAPHIC0_SXY: c_uint = 0x002C;
pub const MXR_GRAPHIC0_WH: c_uint = 0x0030;
pub const MXR_GRAPHIC0_DXY: c_uint = 0x0034;
pub const MXR_GRAPHIC0_BLANK: c_uint = 0x0038;
pub const MXR_GRAPHIC1_CFG: c_uint = 0x0040;
pub const MXR_GRAPHIC1_BASE: c_uint = 0x0044;
pub const MXR_GRAPHIC1_SPAN: c_uint = 0x0048;
pub const MXR_GRAPHIC1_SXY: c_uint = 0x004C;
pub const MXR_GRAPHIC1_WH: c_uint = 0x0050;
pub const MXR_GRAPHIC1_DXY: c_uint = 0x0054;
pub const MXR_GRAPHIC1_BLANK: c_uint = 0x0058;
pub const MXR_BG_CFG: c_uint = 0x0060;
pub const MXR_BG_COLOR0: c_uint = 0x0064;
pub const MXR_BG_COLOR1: c_uint = 0x0068;
pub const MXR_BG_COLOR2: c_uint = 0x006C;
pub const MXR_CM_COEFF_Y: c_uint = 0x0080;
pub const MXR_CM_COEFF_CB: c_uint = 0x0084;
pub const MXR_CM_COEFF_CR: c_uint = 0x0088;
pub const MXR_MO: c_uint = 0x0304;
pub const MXR_RESOLUTION: c_uint = 0x0310;
pub const MXR_CFG_S: c_uint = 0x2004;
pub const MXR_GRAPHIC0_BASE_S: c_uint = 0x2024;
pub const MXR_GRAPHIC1_BASE_S: c_uint = 0x2044;
// for parametrized access to layer registers

//
// Bit definition part
//
// generates mask for range of bits

// bits for MXR_STATUS

// bits for MXR_CFG

pub const MXR_CFG_RGB_FMT_MASK: c_uint = 0x600;

pub const MXR_CFG_SCAN_MASK: c_uint = 0x47;
// bits for MXR_VIDEO_CFG

// bits for MXR_GRAPHICn_CFG

// bits for MXR_GRAPHICn_WH

// bits for MXR_RESOLUTION

// bits for MXR_GRAPHICn_SXY

// bits for MXR_GRAPHICn_DXY

// bits for MXR_INT_EN

// bits for MXR_INT_STATUS

// bits for MXR_LAYER_CFG

// bits for MXR_CM_COEFF_Y


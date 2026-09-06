//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-vp.h
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
// Cloned from drivers/media/video/s5p-tv/regs-vp.h
//
// Copyright (c) 2010-2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Video processor register header file for Samsung Mixer driver
//
// Register part
//
pub const VP_ENABLE: c_uint = 0x0000;
pub const VP_SRESET: c_uint = 0x0004;
pub const VP_SHADOW_UPDATE: c_uint = 0x0008;
pub const VP_FIELD_ID: c_uint = 0x000C;
pub const VP_MODE: c_uint = 0x0010;
pub const VP_IMG_SIZE_Y: c_uint = 0x0014;
pub const VP_IMG_SIZE_C: c_uint = 0x0018;
pub const VP_PER_RATE_CTRL: c_uint = 0x001C;
pub const VP_TOP_Y_PTR: c_uint = 0x0028;
pub const VP_BOT_Y_PTR: c_uint = 0x002C;
pub const VP_TOP_C_PTR: c_uint = 0x0030;
pub const VP_BOT_C_PTR: c_uint = 0x0034;
pub const VP_ENDIAN_MODE: c_uint = 0x03CC;
pub const VP_SRC_H_POSITION: c_uint = 0x0044;
pub const VP_SRC_V_POSITION: c_uint = 0x0048;
pub const VP_SRC_WIDTH: c_uint = 0x004C;
pub const VP_SRC_HEIGHT: c_uint = 0x0050;
pub const VP_DST_H_POSITION: c_uint = 0x0054;
pub const VP_DST_V_POSITION: c_uint = 0x0058;
pub const VP_DST_WIDTH: c_uint = 0x005C;
pub const VP_DST_HEIGHT: c_uint = 0x0060;
pub const VP_H_RATIO: c_uint = 0x0064;
pub const VP_V_RATIO: c_uint = 0x0068;
pub const VP_POLY8_Y0_LL: c_uint = 0x006C;
pub const VP_POLY4_Y0_LL: c_uint = 0x00EC;
pub const VP_POLY4_C0_LL: c_uint = 0x012C;
//
// Bit definition part
//
// generates mask for range of bits

// VP_ENABLE

// VP_SRESET

// VP_SHADOW_UPDATE

// VP_MODE

// VP_IMG_SIZE_Y
// VP_IMG_SIZE_C

// VP_SRC_H_POSITION

// VP_ENDIAN_MODE


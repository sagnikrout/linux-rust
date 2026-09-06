//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/regs-rotator.h
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
// drivers/gpu/drm/exynos/regs-rotator.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Register definition file for Samsung Rotator Interface (Rotator) driver
//
// Configuration
pub const ROT_CONFIG: c_uint = 0x00;

// Image Control
pub const ROT_CONTROL: c_uint = 0x10;

// Status
pub const ROT_STATUS: c_uint = 0x20;

pub const ROT_STATUS_IRQ_VAL_COMPLETE: c_int = 1;
pub const ROT_STATUS_IRQ_VAL_ILLEGAL: c_int = 2;
// Buffer Address

// Buffer Size
pub const ROT_SRC_BUF_SIZE: c_uint = 0x3c;
pub const ROT_DST_BUF_SIZE: c_uint = 0x5c;

// Crop Position
pub const ROT_SRC_CROP_POS: c_uint = 0x40;
pub const ROT_DST_CROP_POS: c_uint = 0x60;

// Source Crop Size
pub const ROT_SRC_CROP_SIZE: c_uint = 0x44;

// Round to nearest aligned value

// Minimum limit value

// Maximum limit value


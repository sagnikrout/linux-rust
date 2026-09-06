//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/gr2d.h
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
// Copyright (C) 2013 NVIDIA Corporation
//
pub const GR2D_UA_BASE_ADDR: c_uint = 0x1a;
pub const GR2D_VA_BASE_ADDR: c_uint = 0x1b;
pub const GR2D_PAT_BASE_ADDR: c_uint = 0x26;
pub const GR2D_DSTA_BASE_ADDR: c_uint = 0x2b;
pub const GR2D_DSTB_BASE_ADDR: c_uint = 0x2c;
pub const GR2D_DSTC_BASE_ADDR: c_uint = 0x2d;
pub const GR2D_SRCA_BASE_ADDR: c_uint = 0x31;
pub const GR2D_SRCB_BASE_ADDR: c_uint = 0x32;
pub const GR2D_PATBASE_ADDR: c_uint = 0x47;
pub const GR2D_SRC_BASE_ADDR_SB: c_uint = 0x48;
pub const GR2D_DSTA_BASE_ADDR_SB: c_uint = 0x49;
pub const GR2D_DSTB_BASE_ADDR_SB: c_uint = 0x4a;
pub const GR2D_UA_BASE_ADDR_SB: c_uint = 0x4b;
pub const GR2D_VA_BASE_ADDR_SB: c_uint = 0x4c;
pub const GR2D_NUM_REGS: c_uint = 0x4d;

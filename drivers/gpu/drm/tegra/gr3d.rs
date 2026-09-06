//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/gr3d.h
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

pub const GR3D_IDX_INDEX_BASE: c_uint = 0x121;
pub const GR3D_QR_ZTAG_ADDR: c_uint = 0x415;
pub const GR3D_QR_CTAG_ADDR: c_uint = 0x417;
pub const GR3D_QR_CZ_ADDR: c_uint = 0x419;

pub const GR3D_DW_MEMORY_OUTPUT_ADDRESS: c_uint = 0x904;

pub const GR3D_GLOBAL_SPILLSURFADDR: c_uint = 0xe2a;

pub const GR3D_NUM_REGS: c_uint = 0xe88;

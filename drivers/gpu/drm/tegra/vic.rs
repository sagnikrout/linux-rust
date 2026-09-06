//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/vic.h
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
// Copyright (c) 2015, NVIDIA Corporation.
//
// VIC methods
pub const VIC_SET_FCE_UCODE_SIZE: c_uint = 0x0000071C;
pub const VIC_SET_FCE_UCODE_OFFSET: c_uint = 0x0000072C;
// VIC registers
pub const VIC_THI_STREAMID0: c_uint = 0x00000030;
pub const VIC_THI_STREAMID1: c_uint = 0x00000034;
pub const NV_PVIC_MISC_PRI_VIC_CG: c_uint = 0x000016d0;

pub const TRANSCFG_SID_HW: c_int = 0;
pub const TRANSCFG_SID_PHY: c_int = 1;
pub const TRANSCFG_SID_FALCON: c_int = 2;
// Firmware offsets


//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_dsc_helper.h
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
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved
//
// Helper methods for MSM-specific DSC calculations that are common between timing engine,
// DSI, and DP.
//

//
// msm_dsc_get_bytes_per_line() - calculate bytes per line
// @dsc: Pointer to drm dsc config struct
// Returns: Integer value representing bytes per line. DSI and DP need
// to perform further calculations to turn this into pclk_per_intf,
// such as dividing by different values depending on if widebus is enabled.
//

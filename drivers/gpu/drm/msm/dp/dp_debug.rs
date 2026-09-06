//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_debug.h
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
// Copyright (c) 2017-2020, The Linux Foundation. All rights reserved.
//

//
// msm_dp_debug_init() - configure and get the DisplayPlot debug module data
//
// @dev: device instance of the caller
// @panel: instance of panel module
// @link: instance of link module
// @connector: double pointer to display connector
// @root: connector's debugfs root
// @is_edp: set for eDP connectors / panels
// return: pointer to allocated debug module data
//
// This function sets up the debug module and provides a way
// for debugfs input to be communicated with existing modules
//


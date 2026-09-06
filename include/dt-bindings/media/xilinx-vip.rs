//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/media/xilinx-vip.h
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
// Xilinx Video IP Core
//
// Copyright (C) 2013-2015 Ideas on Board
// Copyright (C) 2013-2015 Xilinx, Inc.
//
// Contacts: Hyun Kwon <hyun.kwon@xilinx.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
// Video format codes as defined in "AXI4-Stream Video IP and System Design
// Guide".
//
pub const XVIP_VF_YUV_422: c_int = 0;
pub const XVIP_VF_YUV_444: c_int = 1;
pub const XVIP_VF_RBG: c_int = 2;
pub const XVIP_VF_YUV_420: c_int = 3;
pub const XVIP_VF_YUVA_422: c_int = 4;
pub const XVIP_VF_YUVA_444: c_int = 5;
pub const XVIP_VF_RGBA: c_int = 6;
pub const XVIP_VF_YUVA_420: c_int = 7;
pub const XVIP_VF_YUVD_422: c_int = 8;
pub const XVIP_VF_YUVD_444: c_int = 9;
pub const XVIP_VF_RGBD: c_int = 10;
pub const XVIP_VF_YUVD_420: c_int = 11;
pub const XVIP_VF_MONO_SENSOR: c_int = 12;
pub const XVIP_VF_CUSTOM2: c_int = 13;
pub const XVIP_VF_CUSTOM3: c_int = 14;
pub const XVIP_VF_CUSTOM4: c_int = 15;

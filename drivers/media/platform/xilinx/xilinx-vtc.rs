//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/xilinx/xilinx-vtc.h
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
// Xilinx Video Timing Controller
//
// Copyright (C) 2013-2015 Ideas on Board
// Copyright (C) 2013-2015 Xilinx, Inc.
//
// Contacts: Hyun Kwon <hyun.kwon@xilinx.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
pub const XVTC_MAX_HSIZE: c_int = 8191;
pub const XVTC_MAX_VSIZE: c_int = 8191;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xvtc_config {
    pub hblank_start: c_uint,
    pub hsync_start: c_uint,
    pub hsync_end: c_uint,
    pub hsize: c_uint,
    pub vblank_start: c_uint,
    pub vsync_start: c_uint,
    pub vsync_end: c_uint,
    pub vsize: c_uint,
}

extern "C" {
    pub fn xvtc_put(xvtc: *mut xvtc_device);
}
extern "C" {
    pub fn xvtc_generator_stop(xvtc: *mut xvtc_device) -> c_int;
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-isys-subdev.h
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
// Copyright (C) 2013--2024 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_subdev {
    pub sd: v4l2_subdev,
    pub isys: *mut ipu6_isys,
    pub supported_codes: *const u32,
    pub pad: *mut media_pad,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub sd): *mut *mut void (ctrl_init)(struct v4l2_subdev,
    pub /: *mut *mut int source; / SSI stream source; -1 if unset,
}

extern "C" {
    pub fn ipu6_isys_mbus_code_to_bpp(code: u32) -> c_uint;
}
extern "C" {
    pub fn ipu6_isys_mbus_code_to_mipi(code: u32) -> c_uint;
}
extern "C" {
    pub fn ipu6_isys_is_bayer_format(code: u32) -> bool;
}
extern "C" {
    pub fn ipu6_isys_convert_bayer_order(code: u32, x: c_int, y: c_int) -> u32;
}
// code);
extern "C" {
    pub fn ipu6_isys_get_src_stream_by_src_pad(sd: *mut v4l2_subdev, pad: u32) -> u32;
}
extern "C" {
    pub fn ipu6_isys_subdev_cleanup(asd: *mut ipu6_isys_subdev);
}

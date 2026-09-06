//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/v4l2-common.h
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


// SPDX-License-Identifier: ((GPL-2.0+ WITH Linux-syscall-note) OR BSD-3-Clause)
//
// include/linux/v4l2-common.h
//
// Common V4L2 and V4L2 subdev definitions.
//
// Users are advised to #include this file either through videodev2.h
// (V4L2) or through v4l2-subdev.h (V4L2 subdev) rather than to refer
// to this file directly.
//
// Copyright (C) 2012 Nokia Corporation
// Contact: Sakari Ailus <sakari.ailus@iki.fi>
//

//
// Selection interface definitions
//
// Current cropping area
pub const V4L2_SEL_TGT_CROP: c_uint = 0x0000;
// Default cropping area
pub const V4L2_SEL_TGT_CROP_DEFAULT: c_uint = 0x0001;
// Cropping bounds
pub const V4L2_SEL_TGT_CROP_BOUNDS: c_uint = 0x0002;
// Native frame size
pub const V4L2_SEL_TGT_NATIVE_SIZE: c_uint = 0x0003;
// Current composing area
pub const V4L2_SEL_TGT_COMPOSE: c_uint = 0x0100;
// Default composing area
pub const V4L2_SEL_TGT_COMPOSE_DEFAULT: c_uint = 0x0101;
// Composing bounds
pub const V4L2_SEL_TGT_COMPOSE_BOUNDS: c_uint = 0x0102;
// Current composing area plus all padding pixels
pub const V4L2_SEL_TGT_COMPOSE_PADDED: c_uint = 0x0103;
// Selection flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_edid {
    pub pad: __u32,
    pub start_block: __u32,
    pub blocks: __u32,
    pub reserved: [__u32; 5],
    pub edid: *mut __u8,
}

// Backward compatibility target definitions --- to be removed.

// Backward compatibility flag definitions --- to be removed.


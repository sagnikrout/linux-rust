//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-image-sizes.h
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
// Standard image size definitions
//
// Copyright (C) 2013, Sylwester Nawrocki <sylvester.nawrocki@gmail.com>
//
pub const CIF_WIDTH: c_int = 352;
pub const CIF_HEIGHT: c_int = 288;
pub const HD_720_WIDTH: c_int = 1280;
pub const HD_720_HEIGHT: c_int = 720;
pub const HD_1080_WIDTH: c_int = 1920;
pub const HD_1080_HEIGHT: c_int = 1080;
pub const QCIF_WIDTH: c_int = 176;
pub const QCIF_HEIGHT: c_int = 144;
pub const QQCIF_WIDTH: c_int = 88;
pub const QQCIF_HEIGHT: c_int = 72;
pub const QQVGA_WIDTH: c_int = 160;
pub const QQVGA_HEIGHT: c_int = 120;
pub const QVGA_WIDTH: c_int = 320;
pub const QVGA_HEIGHT: c_int = 240;
pub const SVGA_WIDTH: c_int = 800;
pub const SVGA_HEIGHT: c_int = 600;
pub const SXGA_WIDTH: c_int = 1280;
pub const SXGA_HEIGHT: c_int = 1024;
pub const VGA_WIDTH: c_int = 640;
pub const VGA_HEIGHT: c_int = 480;
pub const UXGA_WIDTH: c_int = 1600;
pub const UXGA_HEIGHT: c_int = 1200;
pub const XGA_WIDTH: c_int = 1024;
pub const XGA_HEIGHT: c_int = 768;

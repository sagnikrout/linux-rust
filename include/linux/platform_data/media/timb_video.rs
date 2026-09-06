//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/media/timb_video.h
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
// timb_video.h Platform struct for the Timberdale video driver
// Copyright (c) 2009-2010 Intel Corporation
//
pub const _TIMB_VIDEO_: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timb_video_platform_data {
    pub dma_channel: c_int,
    pub /: *mut *mut int i2c_adapter; / The I2C adapter where the encoder is attached,
    pub module_name: *const c_char,
    pub info: *mut i2c_board_info,
    pub encoder: },
}

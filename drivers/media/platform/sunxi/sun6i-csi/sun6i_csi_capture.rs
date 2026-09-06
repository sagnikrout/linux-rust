//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun6i-csi/sun6i_csi_capture.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2011-2018 Magewell Electronics Co., Ltd. (Nanjing)
// Author: Yong Deng <yong.deng@magewell.com>
// Copyright 2021-2022 Bootlin
// Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
//

pub const SUN6I_CSI_CAPTURE_WIDTH_MIN: c_int = 32;
pub const SUN6I_CSI_CAPTURE_WIDTH_MAX: c_int = 4800;
pub const SUN6I_CSI_CAPTURE_HEIGHT_MIN: c_int = 32;
pub const SUN6I_CSI_CAPTURE_HEIGHT_MAX: c_int = 4800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_capture_format {
    pub pixelformat: u32,
    pub output_format_field: u8,
    pub output_format_frame: u8,
    pub input_yuv_seq_invert: bool,
    pub input_format_raw: bool,
    pub hsize_len_factor: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_capture_format_match {
    pub pixelformat: u32,
    pub mbus_code: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_capture_state {
    pub queue: list_head,
    pub /: *mut *mut spinlock_t lock; / Queue and buffers lock.,
    pub pending: *mut sun6i_csi_buffer,
    pub current: *mut sun6i_csi_buffer,
    pub complete: *mut sun6i_csi_buffer,
    pub sequence: c_uint,
    pub streaming: bool,
    pub setup: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun6i_csi_capture {
    pub state: sun6i_csi_capture_state,
    pub video_dev: video_device,
    pub queue: vb2_queue,
    pub /: *mut *mut mutex lock; / Queue lock.,
    pub pad: media_pad,
    pub format: v4l2_format,
}

// Helpers
// Format
// Capture
extern "C" {
    pub fn sun6i_csi_capture_configure(csi_dev: *mut sun6i_csi_device);
}
extern "C" {
    pub fn sun6i_csi_capture_state_update(csi_dev: *mut sun6i_csi_device);
}
// State
extern "C" {
    pub fn sun6i_csi_capture_sync(csi_dev: *mut sun6i_csi_device);
}
extern "C" {
    pub fn sun6i_csi_capture_frame_done(csi_dev: *mut sun6i_csi_device);
}
// Capture
extern "C" {
    pub fn sun6i_csi_capture_setup(csi_dev: *mut sun6i_csi_device) -> c_int;
}
extern "C" {
    pub fn sun6i_csi_capture_cleanup(csi_dev: *mut sun6i_csi_device);
}

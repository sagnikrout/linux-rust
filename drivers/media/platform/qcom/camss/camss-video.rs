//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-video.h
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
// camss-video.h
//
// Qualcomm MSM Camera Subsystem - V4L2 device node
//
// Copyright (c) 2013-2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2018 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_buffer {
    pub vb: vb2_v4l2_buffer,
    pub addr: [dma_addr_t; 3],
    pub queue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_video_ops {
    pub buf): *mut *mut *mut int (queue_buffer)(struct camss_video vid, struct camss_buffer,
    pub state): vb2_buffer_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_video {
    pub camss: *mut camss,
    pub vb2_q: vb2_queue,
    pub vdev: video_device,
    pub pad: media_pad,
    pub active_fmt: v4l2_format,
    pub type: v4l2_buf_type,
    pub pipe: media_pipeline,
    pub ops: *const camss_video_ops,
    pub lock: mutex,
    pub q_lock: mutex,
    pub bpl_alignment: c_uint,
    pub line_based: c_uint,
    pub formats: *const camss_format_info,
    pub nformats: c_uint,
}

extern "C" {
    pub fn msm_video_unregister(video: *mut camss_video);
}

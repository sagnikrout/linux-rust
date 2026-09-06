//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amd/isp4/isp4_video.h
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
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4vid_capture_buffer {
//
// struct vb2_v4l2_buffer must be the first element
// the videobuf2 framework will allocate this struct based on
// buf_struct_size and use the first sizeof(struct vb2_buffer) bytes of
// memory as a vb2_buffer
//
    pub vb2: vb2_v4l2_buffer,
    pub img_buf: isp4if_img_buf_info,
    pub list: list_head,
    pub dbuf: *mut dma_buf,
    pub bo: *mut c_void,
    pub gpu_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp4vid_dev {
    pub vdev: video_device,
    pub vdev_pad: media_pad,
    pub format: v4l2_pix_format,
// mutex that protects vbq
    pub vbq_lock: mutex,
    pub vbq: vb2_queue,
// mutex that protects buf_list
    pub buf_list_lock: mutex,
    pub buf_list: list_head,
    pub sequence: u32,
    pub stream_started: bool,
    pub dev: *mut device,
    pub isp_sdev: *mut v4l2_subdev,
    pub timeperframe: v4l2_fract,
}

extern "C" {
    pub fn isp4vid_dev_init(isp_vdev: *mut isp4vid_dev, isp_sd: *mut v4l2_subdev) -> c_int;
}
extern "C" {
    pub fn isp4vid_dev_deinit(isp_vdev: *mut isp4vid_dev);
}

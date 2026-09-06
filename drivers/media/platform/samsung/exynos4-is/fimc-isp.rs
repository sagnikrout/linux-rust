//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/exynos4-is/fimc-isp.h
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
// Samsung EXYNOS4x12 FIMC-IS (Imaging Subsystem) driver
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//
// Authors: Sylwester Nawrocki <s.nawrocki@samsung.com>
// Younghwan Joo <yhwan.joo@samsung.com>
//

// FIXME: revisit these constraints

pub const FIMC_ISP_SOURCE_WIDTH_MIN: c_int = 8;
pub const FIMC_ISP_SOURCE_HEIGHT_MIN: c_int = 8;
pub const FIMC_ISP_CAC_MARGIN_WIDTH: c_int = 16;
pub const FIMC_ISP_CAC_MARGIN_HEIGHT: c_int = 12;

pub const FIMC_ISP_SOURCE_WIDTH_MAX: c_int = 4000;
pub const FIMC_ISP_SOURCE_HEIGHT_MAX: c_int = 4000;
pub const FIMC_ISP_NUM_FORMATS: c_int = 3;
pub const FIMC_ISP_REQ_BUFS_MIN: c_int = 2;
pub const FIMC_ISP_REQ_BUFS_MAX: c_int = 32;
pub const FIMC_ISP_SD_PAD_SINK: c_int = 0;
pub const FIMC_ISP_SD_PAD_SRC_FIFO: c_int = 1;
pub const FIMC_ISP_SD_PAD_SRC_DMA: c_int = 2;
pub const FIMC_ISP_SD_PADS_NUM: c_int = 3;
pub const FIMC_ISP_MAX_PLANES: c_int = 1;
//
// struct fimc_isp_frame - source/target frame properties
// @width: full image width
// @height: full image height
// @rect: crop/composition rectangle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_isp_frame {
    pub width: u16,
    pub height: u16,
    pub rect: v4l2_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_isp_ctrls {
    pub handler: v4l2_ctrl_handler,
// Auto white balance
    pub auto_wb: *mut v4l2_ctrl,
// Auto ISO control cluster
    pub auto_iso: *mut v4l2_ctrl,
    pub iso: *mut v4l2_ctrl,
}

// Adjust - contrast
// Adjust - saturation
// Adjust - sharpness
// Adjust - brightness
// Adjust - hue
// Auto/manual exposure
// Manual exposure value
// AE/AWB lock/unlock
// Exposure metering mode
// AFC
// ISP image effect
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_video_buf {
    pub vb: vb2_v4l2_buffer,
    pub dma_addr: [dma_addr_t; FIMC_ISP_MAX_PLANES],
    pub index: c_uint,
}

pub const FIMC_ISP_MAX_BUFS: c_int = 4;
//
// struct fimc_is_video - fimc-is video device structure
// @ve: video_device structure and media pipeline
// @type: video device type (CAPTURE/OUTPUT)
// @pad: video device media (sink) pad
// @pending_buf_q: pending buffers queue head
// @active_buf_q: a queue head of buffers scheduled in hardware
// @vb_queue: vb2 buffer queue
// @reqbufs_count: the number of buffers requested in REQBUFS ioctl
// @buf_count: number of video buffers scheduled in hardware
// @buf_mask: bitmask of the queued video buffer indices
// @frame_count: counter of frames dequeued to user space
// @streaming: is streaming in progress?
// @buffers: buffer info
// @format: current fimc pixel format
// @pixfmt: current pixel format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_is_video {
    pub ve: exynos_video_entity,
    pub type: v4l2_buf_type,
    pub pad: media_pad,
    pub pending_buf_q: list_head,
    pub active_buf_q: list_head,
    pub vb_queue: vb2_queue,
    pub reqbufs_count: c_uint,
    pub buf_count: c_uint,
    pub buf_mask: c_uint,
    pub frame_count: c_uint,
    pub streaming: c_int,
    pub buffers: [*mut isp_video_buf; FIMC_ISP_MAX_BUFS],
    pub format: *const fimc_fmt,
    pub pixfmt: v4l2_pix_format_mplane,
}

// struct fimc_isp:state bit definitions
pub const ST_ISP_VID_CAP_BUF_PREP: c_int = 0;
pub const ST_ISP_VID_CAP_STREAMING: c_int = 1;
//
// struct fimc_isp - FIMC-IS ISP data structure
// @pdev: pointer to FIMC-IS platform device
// @subdev: ISP v4l2_subdev
// @subdev_pads: the ISP subdev media pads
// @src_fmt: source mediabus format
// @sink_fmt: sink mediabus format
// @test_pattern: test pattern controls
// @ctrls: v4l2 controls structure
// @video_lock: mutex serializing video device operations
// @subdev_lock: mutex serializing subdev operations
// @cac_margin_x: horizontal CAC margin in pixels
// @cac_margin_y: vertical CAC margin in pixels
// @state: driver state flags
// @video_capture: the ISP block video capture device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fimc_isp {
    pub pdev: *mut platform_device,
    pub subdev: v4l2_subdev,
    pub subdev_pads: [media_pad; FIMC_ISP_SD_PADS_NUM],
    pub src_fmt: v4l2_mbus_framefmt,
    pub sink_fmt: v4l2_mbus_framefmt,
    pub test_pattern: *mut v4l2_ctrl,
    pub ctrls: fimc_isp_ctrls,
    pub video_lock: mutex,
    pub subdev_lock: mutex,
    pub cac_margin_x: c_uint,
    pub cac_margin_y: c_uint,
    pub state: c_ulong,
    pub video_capture: fimc_is_video,
}

extern "C" {
    pub fn fimc_isp_subdev_create(isp: *mut fimc_isp) -> c_int;
}
extern "C" {
    pub fn fimc_isp_subdev_destroy(isp: *mut fimc_isp);
}
extern "C" {
    pub fn fimc_isp_irq_handler(is: *mut fimc_is);
}
extern "C" {
    pub fn fimc_is_create_controls(isp: *mut fimc_isp) -> c_int;
}
extern "C" {
    pub fn fimc_is_delete_controls(isp: *mut fimc_isp) -> c_int;
}

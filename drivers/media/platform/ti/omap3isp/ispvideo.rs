//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispvideo.h
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
// ispvideo.h
//
// TI OMAP3 ISP - Generic video node
//
// Copyright (C) 2009-2010 Nokia Corporation
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

//
// struct isp_format_info - ISP media bus format information
// @code: V4L2 media bus format code
// @truncated: V4L2 media bus format code for the same format truncated to 10
// bits. Identical to @code if the format is 10 bits wide or less.
// @uncompressed: V4L2 media bus format code for the corresponding uncompressed
// format. Identical to @code if the format is not DPCM compressed.
// @flavor: V4L2 media bus format code for the same pixel layout but
// shifted to be 8 bits per pixel. =0 if format is not shiftable.
// @pixelformat: V4L2 pixel format FCC identifier
// @width: Bits per pixel (when transferred over a bus)
// @bpp: Bytes per pixel (when stored in memory)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_format_info {
    pub code: u32,
    pub truncated: u32,
    pub uncompressed: u32,
    pub flavor: u32,
    pub pixelformat: u32,
    pub width: c_uint,
    pub bpp: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_pipeline_stream_state {
    ISP_PIPELINE_STREAM_STOPPED = 0,
    ISP_PIPELINE_STREAM_CONTINUOUS = 1,
    ISP_PIPELINE_STREAM_SINGLESHOT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_pipeline_state {
// The stream has been started on the input video node.
    ISP_PIPELINE_STREAM_INPUT = 1,
// The stream has been started on the output video node.
    ISP_PIPELINE_STREAM_OUTPUT = 2,
// At least one buffer is queued on the input video node.
    ISP_PIPELINE_QUEUE_INPUT = 4,
// At least one buffer is queued on the output video node.
    ISP_PIPELINE_QUEUE_OUTPUT = 8,
// The input entity is idle, ready to be started.
    ISP_PIPELINE_IDLE_INPUT = 16,
// The output entity is idle, ready to be started.
    ISP_PIPELINE_IDLE_OUTPUT = 32,
// The pipeline is currently streaming.
    ISP_PIPELINE_STREAM = 64,
}

//
// struct isp_pipeline - An ISP hardware pipeline
// @field: The field being processed by the pipeline
// @error: A hardware error occurred during capture
// @ent_enum: Entities in the pipeline
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_pipeline {
    pub pipe: media_pipeline,
    pub /: *mut *mut spinlock_t lock; / Pipeline state and queue flags,
    pub state: c_uint,
    pub stream_state: isp_pipeline_stream_state,
    pub input: *mut isp_video,
    pub output: *mut isp_video,
    pub ent_enum: media_entity_enum,
    pub l3_ick: c_ulong,
    pub max_rate: c_uint,
    pub field: v4l2_field,
    pub frame_number: core::sync::atomic::AtomicI32,
    pub /: *mut *mut bool do_propagation; / of frame number,
    pub error: bool,
    pub max_timeperframe: v4l2_fract,
    pub external: *mut v4l2_subdev,
    pub external_rate: c_uint,
    pub external_width: c_uint,
}

extern "C" {
    pub fn container_of(_arg: pipe, isp_pipeline: struct, _arg: pipe) -> return;
}
//
// struct isp_buffer - ISP video buffer
// @vb: videobuf2 buffer
// @irqlist: List head for insertion into IRQ queue
// @dma: DMA address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_buffer {
    pub vb: vb2_v4l2_buffer,
    pub irqlist: list_head,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp_video_dmaqueue_flags {
// Set if DMA queue becomes empty when ISP_PIPELINE_STREAM_CONTINUOUS
    ISP_VIDEO_DMAQUEUE_UNDERRUN = (1 << 0),
// Set when queuing buffer to an empty DMA queue
    ISP_VIDEO_DMAQUEUE_QUEUED = (1 << 1),
}

//
// struct isp_video_operations - ISP video operations
// @queue:	Resume streaming when a buffer is queued. Called on VIDIOC_QBUF
// if there was no buffer previously queued.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_video_operations {
    pub buffer): *mut *mut *mut int(queue)(struct isp_video video, struct isp_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_video {
    pub video: video_device,
    pub type: v4l2_buf_type,
    pub pad: media_pad,
    pub /: *mut *mut mutex mutex; / format and crop settings,
    pub active: core::sync::atomic::AtomicI32,
    pub isp: *mut isp_device,
    pub capture_mem: c_uint,
    pub /: *mut *mut unsigned int bpl_alignment; / alignment value,
    pub /: *mut *mut unsigned int bpl_zero_padding; / whether the alignment is optional,
    pub /: *mut *mut unsigned int bpl_max; / maximum bytes per line value,
    pub /: *mut *mut unsigned int bpl_value; / bytes per line value,
    pub /: *mut *mut unsigned int bpl_padding; / padding at end of line,
// Pipeline state
    pub pipe: isp_pipeline,
    pub /: *mut *mut mutex stream_lock; / pipeline and stream states,
    pub error: bool,
// Video buffers queue
    pub queue: *mut vb2_queue,
    pub /: *mut *mut mutex queue_lock; / protects the queue,
    pub /: *mut *mut spinlock_t irqlock; / protects dmaqueue,
    pub dmaqueue: list_head,
    pub dmaqueue_flags: isp_video_dmaqueue_flags,
    pub ops: *const isp_video_operations,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_video_fh {
    pub vfh: v4l2_fh,
    pub video: *mut isp_video,
    pub queue: vb2_queue,
    pub format: v4l2_format,
    pub timeperframe: v4l2_fract,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), isp_video_fh: struct, _arg: vfh) -> return;
}

extern "C" {
    pub fn omap3isp_video_init(video: *mut isp_video, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn omap3isp_video_cleanup(video: *mut isp_video);
}
extern "C" {
    pub fn omap3isp_video_unregister(video: *mut isp_video);
}
extern "C" {
    pub fn omap3isp_video_cancel_stream(video: *mut isp_video);
}
extern "C" {
    pub fn omap3isp_video_resume(video: *mut isp_video, continuous: c_int);
}

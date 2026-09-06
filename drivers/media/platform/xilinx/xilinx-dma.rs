//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/xilinx/xilinx-dma.h
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
// Xilinx Video DMA
//
// Copyright (C) 2013-2015 Ideas on Board
// Copyright (C) 2013-2015 Xilinx, Inc.
//
// Contacts: Hyun Kwon <hyun.kwon@xilinx.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

//
// struct xvip_pipeline - Xilinx Video IP pipeline structure
// @pipe: media pipeline
// @lock: protects the pipeline @stream_count
// @use_count: number of DMA engines using the pipeline
// @stream_count: number of DMA engines currently streaming
// @num_dmas: number of DMA engines in the pipeline
// @output: DMA engine at the output of the pipeline
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xvip_pipeline {
    pub pipe: media_pipeline,
    pub lock: mutex,
    pub use_count: c_uint,
    pub stream_count: c_uint,
    pub num_dmas: c_uint,
    pub output: *mut xvip_dma,
}

extern "C" {
    pub fn container_of(_arg: pipe, xvip_pipeline: struct, _arg: pipe) -> return;
}
//
// struct xvip_dma - Video DMA channel
// @list: list entry in a composite device dmas list
// @video: V4L2 video device associated with the DMA channel
// @pad: media pad for the video device entity
// @xdev: composite device the DMA channel belongs to
// @pipe: pipeline belonging to the DMA channel
// @port: composite device DT node port number for the DMA channel
// @lock: protects the @format, @fmtinfo and @queue fields
// @format: active V4L2 pixel format
// @fmtinfo: format information corresponding to the active @format
// @queue: vb2 buffers queue
// @sequence: V4L2 buffers sequence number
// @queued_bufs: list of queued buffers
// @queued_lock: protects the buf_queued list
// @dma: DMA engine channel
// @align: transfer alignment required by the DMA channel (in bytes)
// @xt: dma interleaved template for dma configuration
// @sgl: data chunk structure for dma_interleaved_template
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xvip_dma {
    pub list: list_head,
    pub video: video_device,
    pub pad: media_pad,
    pub xdev: *mut xvip_composite_device,
    pub pipe: xvip_pipeline,
    pub port: c_uint,
    pub lock: mutex,
    pub format: v4l2_pix_format,
    pub fmtinfo: *const xvip_video_format,
    pub queue: vb2_queue,
    pub sequence: c_uint,
    pub queued_bufs: list_head,
    pub queued_lock: spinlock_t,
    pub dma: *mut dma_chan,
    pub align: c_uint,
    pub xt: dma_interleaved_template,
    pub sgl: data_chunk,
}

extern "C" {
    pub fn xvip_dma_cleanup(dma: *mut xvip_dma);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_pipe.h
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
// vsp1_pipe.h  --  R-Car VSP1 Pipeline
//
// Copyright (C) 2013-2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

//
// struct vsp1_format_info - VSP1 video format description
// @fourcc: V4L2 pixel format FCC identifier
// @mbus: media bus format code
// @hwfmt: VSP1 hardware format
// @swap: swap register control
// @planes: number of planes
// @bpp: bits per pixel
// @swap_yc: the Y and C components are swapped (Y comes before C)
// @swap_uv: the U and V components are swapped (V comes before U)
// @hsub: horizontal subsampling factor
// @vsub: vertical subsampling factor
// @alpha: has an alpha channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_format_info {
    pub fourcc: u32,
    pub mbus: c_uint,
    pub hwfmt: c_uint,
    pub swap: c_uint,
    pub planes: c_uint,
    pub bpp: [c_uint; 3],
    pub swap_yc: bool,
    pub swap_uv: bool,
    pub hsub: c_uint,
    pub vsub: c_uint,
    pub alpha: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vsp1_pipeline_state {
    VSP1_PIPELINE_STOPPED,
    VSP1_PIPELINE_RUNNING,
    VSP1_PIPELINE_STOPPING,
}

//
// struct vsp1_partition - A description of a slice for the partition algorithm
// @rpf: The RPF partition window configuration
// @uds_sink: The UDS input partition window configuration
// @uds_source: The UDS output partition window configuration
// @sru: The SRU partition window configuration
// @wpf: The WPF partition window configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_partition {
    pub rpf: [v4l2_rect; VSP1_MAX_RPF],
    pub uds_sink: v4l2_rect,
    pub uds_source: v4l2_rect,
    pub sru: v4l2_rect,
    pub wpf: v4l2_rect,
}

//
// struct vsp1_pipeline - A VSP1 hardware pipeline
// @pipe: the media pipeline
// @irqlock: protects the pipeline state
// @state: current state
// @wq: wait queue to wait for state change completion
// @frame_end: frame end interrupt handler
// @lock: protects the pipeline use count and stream count
// @kref: pipeline reference count
// @stream_count: number of streaming video nodes
// @buffers_ready: bitmask of RPFs and WPFs with at least one buffer available
// @sequence: frame sequence number
// @num_inputs: number of RPFs
// @inputs: array of RPFs in the pipeline (indexed by RPF index)
// @output: WPF at the output of the pipeline
// @brx: BRx entity, if present
// @hgo: HGO entity, if present
// @hgt: HGT entity, if present
// @lif: LIF entity, if present
// @uds: UDS entity, if present
// @uds_input: entity at the input of the UDS, if the UDS is present
// @entities: list of entities in the pipeline
// @stream_config: cached stream configuration for video pipelines
// @configured: when false the @stream_config shall be written to the hardware
// @interlaced: True when the pipeline is configured in interlaced mode
// @partitions: The number of partitions used to process one frame
// @part_table: The pre-calculated partitions used by the pipeline
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_pipeline {
    pub pipe: media_pipeline,
    pub irqlock: spinlock_t,
    pub state: vsp1_pipeline_state,
    pub wq: wait_queue_head_t,
    pub completion): *mut *mut *mut void (frame_end)(struct vsp1_pipeline pipe, unsigned int,
    pub lock: mutex,
    pub kref: kref,
    pub stream_count: c_uint,
    pub buffers_ready: c_uint,
    pub sequence: c_uint,
    pub num_inputs: c_uint,
    pub inputs: [*mut vsp1_rwpf; VSP1_MAX_RPF],
    pub output: *mut vsp1_rwpf,
    pub brx: *mut vsp1_entity,
    pub hgo: *mut vsp1_entity,
    pub hgt: *mut vsp1_entity,
    pub iif: *mut vsp1_entity,
    pub lif: *mut vsp1_entity,
    pub uds: *mut vsp1_entity,
    pub uds_input: *mut vsp1_entity,
//
// The order of this list must be identical to the order of the entities
// in the pipeline, as it is assumed by the partition algorithm that we
// can walk this list in sequence.
//
    pub entities: list_head,
    pub stream_config: *mut vsp1_dl_body,
    pub configured: bool,
    pub interlaced: bool,
    pub partitions: c_uint,
    pub part_table: *mut vsp1_partition,
    pub underrun_count: u32,
}

extern "C" {
    pub fn vsp1_pipeline_reset(pipe: *mut vsp1_pipeline);
}
extern "C" {
    pub fn vsp1_pipeline_init(pipe: *mut vsp1_pipeline);
}

extern "C" {
    pub fn vsp1_pipeline_run(pipe: *mut vsp1_pipeline);
}
extern "C" {
    pub fn vsp1_pipeline_stopped(pipe: *mut vsp1_pipeline) -> bool;
}
extern "C" {
    pub fn vsp1_pipeline_stop(pipe: *mut vsp1_pipeline) -> c_int;
}
extern "C" {
    pub fn vsp1_pipeline_ready(pipe: *mut vsp1_pipeline) -> bool;
}
extern "C" {
    pub fn vsp1_pipeline_frame_end(pipe: *mut vsp1_pipeline);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/rcar-isp/risp-core.h
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
// Copyright (C) 2026 Renesas Electronics Corp.
// Copyright (C) 2026 Ideas on Board Oy
// Copyright (C) 2026 Ragnatech AB
//

// Max 2048 address + value pairs in one VSPX buffer, increase if needed.
pub const RISP_IO_PARAMS_BUF_SIZE: c_int = 16384;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum risp_core_pads {
    RISP_CORE_INPUT1,
    RISP_CORE_PARAMS,
    RISP_CORE_STATS,
    RISP_CORE_OUTPUT1,
    RISP_CORE_NUM_PADS,
}

//
// struct risp_buffer - Describe an IO buffer
// @vb:		The VB2 buffer
// @list:	List of buffers queued to the IO queue
// @vsp_buffer:	Buffer mapped from VSP-X, only used for params IO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct risp_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub vsp_buffer: vsp1_isp_buffer_desc,
}

//
// struct rcar_isp_core_io - Information for a IO video devices
// @core:	Backlink to the common ISP core structure
//
// @lock:	Protects @vdev, @pad and @queue + open/close fops
// @vdev:	V4L2 video device associated with this IO port
// @pad:	Media pad for @vdev
// @queue:	VB2 buffers queue for $@vdev
//
// @streaming:	Flag to indicate if device is streaming, or not
// @buffers:	List of buffers queued to the device
//
// @format:	The active V4L2 format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_isp_core_io {
    pub core: *mut rcar_isp_core,
    pub /: *mut *mut mutex lock; / See KDoc block.,
    pub vdev: video_device,
    pub pad: media_pad,
    pub queue: vb2_queue,
    pub streaming: bool,
    pub buffers: list_head,
    pub format: v4l2_format,
}

//
// struct rcar_isp_job - R-Car ISP job description
//
// Both done_isp and done_vspx shall be set before the job can be considered
// completely done.
//
// @buffers: IO buffers that form a job
// @vspx_job: VSPX job description
// @job_queue: list handle
// @done_isp: Flag to indicate the ISP is done with the job
// @done_vspx: Flag to indicate the VSPX is done with the job
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_isp_job {
    pub buffers: [*mut risp_buffer; RISP_CORE_NUM_PADS],
    pub vspx_job: vsp1_isp_job_desc,
    pub job_queue: list_head,
    pub done_isp: bool,
    pub done_vspx: bool,
}

//
// struct rcar_isp_vspx - R-Car ISP job description
//
// @dev: Device reference to VSPX
// @job: Job currently being processed by VSPX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_isp_vspx {
    pub dev: *mut device,
    pub job: *mut rcar_isp_job,
}

//
// struct rcar_isp_core - ISP Core
// @dev:	(OF) device
// @rppaddr:	Hardware address of the RPP ISP (from OF)
// @clk:	The clock for the ISP CORE
// @rstc:	The reset for the ISP Core
// @csrstc:	The reset for the ISP Channel Selector
//
// @base:	MMIO base of the ISP CORE
// @csbase:	MMIO base of the ISP CS
//
// @subdev:	V4L2 subdevice to represent the ISP CORE
// @pads:	Media pad for @subdev
//
// @v4l2_dev:	V4L2 device
// @rpp:	Handle to the RPP ISP connected to the ISP CORE
//
// @io_lock:	Protect io[*].streaming and io[*].buffers
// @io:		Array of IO ports to the ISP CORE
//
// @lock:	Protects @vspx, @risp_jobs, @sequence and @streaming
// @vspx:	Handle to the resources used by VSPX connected to the ISP CORE
// @risp_jobs:	Queue of VSPX transfer jobs
// @sequence:	V4L2 buffers sequence number
// @streaming:	Tracks if the device is streaming
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_isp_core {
    pub dev: *mut device,
    pub rppaddr: u32,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
    pub csrstc: *mut reset_control,
    pub base: *mut void __iomem,
    pub csbase: *mut void __iomem,
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; RISP_CORE_NUM_PADS],
    pub v4l2_dev: v4l2_device,
    pub rpp: *mut rppx1,
    pub /: *mut *mut mutex io_lock; / See KDoc block.,
    pub io: [rcar_isp_core_io; RISP_CORE_NUM_PADS],
    pub /: *mut *mut spinlock_t lock; / See KDoc block.,
    pub vspx: rcar_isp_vspx,
    pub risp_jobs: list_head,
    pub sequence: c_uint,
    pub streaming: bool,
}

extern "C" {
    pub fn risp_core_remove(core: *mut rcar_isp_core);
}
extern "C" {
    pub fn risp_core_registered(core: *mut rcar_isp_core, sd: *mut v4l2_subdev) -> c_int;
}
extern "C" {
    pub fn risp_core_job_prepare(core: *mut rcar_isp_core) -> c_int;
}
extern "C" {
    pub fn risp_core_start_streaming(core: *mut rcar_isp_core) -> c_int;
}
extern "C" {
    pub fn risp_core_stop_streaming(core: *mut rcar_isp_core);
}
extern "C" {
    pub fn risp_core_io_destroy(io: *mut rcar_isp_core_io);
}

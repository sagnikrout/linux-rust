//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/bdisp/bdisp.h
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
// Copyright (C) STMicroelectronics SA 2014
// Authors: Fabien Dessenne <fabien.dessenne@st.com> for STMicroelectronics.
//

//
// Max nb of nodes in node-list:
// - 2 nodes to handle wide 4K pictures
// - 2 nodes to handle two planes (Y & CbCr)
pub const MAX_OUTPUT_PLANES: c_int = 2;
pub const MAX_VERTICAL_STRIDES: c_int = 2;

// struct bdisp_ctrls - bdisp control set
// @hflip:      horizontal flip
// @vflip:      vertical flip
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_ctrls {
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
}

//
// struct bdisp_fmt - driver's internal color format data
// @pixelformat:fourcc code for this format
// @nb_planes:  number of planes  (ex: [0]=RGB/Y - [1]=Cb/Cr, ...)
// @bpp:        bits per pixel (general)
// @bpp_plane0: byte per pixel for the 1st plane
// @w_align:    width alignment in pixel (multiple of)
// @h_align:    height alignment in pixel (multiple of)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_fmt {
    pub pixelformat: u32,
    pub nb_planes: u8,
    pub bpp: u8,
    pub bpp_plane0: u8,
    pub w_align: u8,
    pub h_align: u8,
}

//
// struct bdisp_frame - frame properties
//
// @width:      frame width (including padding)
// @height:     frame height (including padding)
// @fmt:        pointer to frame format descriptor
// @field:      frame / field type
// @bytesperline: stride of the 1st plane
// @sizeimage:  image size in bytes
// @colorspace: colorspace
// @crop:       crop area
// @paddr:      image physical addresses per plane ([0]=RGB/Y - [1]=Cb/Cr, ...)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_frame {
    pub width: u32,
    pub height: u32,
    pub fmt: *const bdisp_fmt,
    pub field: v4l2_field,
    pub bytesperline: u32,
    pub sizeimage: u32,
    pub colorspace: v4l2_colorspace,
    pub crop: v4l2_rect,
    pub paddr: [dma_addr_t; 4],
}

//
// struct bdisp_request - bdisp request
//
// @src:        source frame properties
// @dst:        destination frame properties
// @hflip:      horizontal flip
// @vflip:      vertical flip
// @nb_req:     number of run request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_request {
    pub src: bdisp_frame,
    pub dst: bdisp_frame,
    pub hflip:1: c_uint,
    pub vflip:1: c_uint,
    pub nb_req: c_int,
}

//
// struct bdisp_ctx - device context data
//
// @src:        source frame properties
// @dst:        destination frame properties
// @state:      flags to keep track of user configuration
// @hflip:      horizontal flip
// @vflip:      vertical flip
// @bdisp_dev:  the device this context applies to
// @node:       node array
// @node_paddr: node physical address array
// @fh:         v4l2 file handle
// @ctrl_handler: v4l2 controls handler
// @bdisp_ctrls: bdisp control set
// @ctrls_rdy:  true if the control handler is initialized
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_ctx {
    pub src: bdisp_frame,
    pub dst: bdisp_frame,
    pub state: u32,
    pub hflip:1: c_uint,
    pub vflip:1: c_uint,
    pub bdisp_dev: *mut bdisp_dev,
    pub node: [*mut bdisp_node; MAX_NB_NODE],
    pub node_paddr: [dma_addr_t; MAX_NB_NODE],
    pub fh: v4l2_fh,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub bdisp_ctrls: bdisp_ctrls,
    pub ctrls_rdy: bool,
}

//
// struct bdisp_m2m_device - v4l2 memory-to-memory device data
//
// @vdev:       video device node for v4l2 m2m mode
// @m2m_dev:    v4l2 m2m device data
// @ctx:        hardware context data
// @refcnt:     reference counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_m2m_device {
    pub vdev: *mut video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub ctx: *mut bdisp_ctx,
    pub refcnt: c_int,
}

//
// struct bdisp_dbg - debug info
//
// @debugfs_entry: debugfs
// @copy_node:     array of last used nodes
// @copy_request:  last bdisp request
// @hw_start:      start time of last HW request
// @last_duration: last HW processing duration in microsecs
// @min_duration:  min HW processing duration in microsecs
// @max_duration:  max HW processing duration in microsecs
// @tot_duration:  total HW processing duration in microsecs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_dbg {
    pub debugfs_entry: *mut dentry,
    pub copy_node: [*mut bdisp_node; MAX_NB_NODE],
    pub copy_request: bdisp_request,
    pub hw_start: ktime_t,
    pub last_duration: i64,
    pub min_duration: i64,
    pub max_duration: i64,
    pub tot_duration: i64,
}

//
// struct bdisp_dev - abstraction for bdisp entity
//
// @v4l2_dev:   v4l2 device
// @vdev:       video device
// @pdev:       platform device
// @dev:        device
// @lock:       mutex protecting this data structure
// @slock:      spinlock protecting this data structure
// @id:         device index
// @m2m:        memory-to-memory V4L2 device information
// @state:      flags used to synchronize m2m and capture mode operation
// @clock:      IP clock
// @regs:       registers
// @irq_queue:  interrupt handler waitqueue
// @work_queue: workqueue to handle timeouts
// @timeout_work: IRQ timeout structure
// @dbg:        debug info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_dev {
    pub v4l2_dev: v4l2_device,
    pub vdev: video_device,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub slock: spinlock_t,
    pub lock: mutex,
    pub id: u16,
    pub m2m: bdisp_m2m_device,
    pub state: c_ulong,
    pub clock: *mut clk,
    pub regs: *mut void __iomem,
    pub irq_queue: wait_queue_head_t,
    pub work_queue: *mut workqueue_struct,
    pub timeout_work: delayed_work,
    pub dbg: bdisp_dbg,
}

extern "C" {
    pub fn bdisp_hw_free_nodes(ctx: *mut bdisp_ctx);
}
extern "C" {
    pub fn bdisp_hw_alloc_nodes(ctx: *mut bdisp_ctx) -> c_int;
}
extern "C" {
    pub fn bdisp_hw_free_filters(dev: *mut device);
}
extern "C" {
    pub fn bdisp_hw_alloc_filters(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn bdisp_hw_reset(bdisp: *mut bdisp_dev) -> c_int;
}
extern "C" {
    pub fn bdisp_hw_get_and_clear_irq(bdisp: *mut bdisp_dev) -> c_int;
}
extern "C" {
    pub fn bdisp_hw_update(ctx: *mut bdisp_ctx) -> c_int;
}
extern "C" {
    pub fn bdisp_debugfs_remove(bdisp: *mut bdisp_dev);
}
extern "C" {
    pub fn bdisp_debugfs_create(bdisp: *mut bdisp_dev);
}
extern "C" {
    pub fn bdisp_dbg_perf_begin(bdisp: *mut bdisp_dev);
}
extern "C" {
    pub fn bdisp_dbg_perf_end(bdisp: *mut bdisp_dev);
}

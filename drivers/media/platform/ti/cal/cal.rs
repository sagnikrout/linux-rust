//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/cal/cal.h
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
// TI Camera Access Layer (CAL)
//
// Copyright (c) 2015-2020 Texas Instruments Inc.
//
// Authors:
// Benoit Parrot <bparrot@ti.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

pub const CAL_MAX_NUM_CONTEXT: c_int = 8;
pub const CAL_NUM_CSI2_PORTS: c_int = 2;
//
// The width is limited by the size of the CAL_WR_DMA_XSIZE_j.XSIZE field,
// expressed in multiples of 64 bits. The height is limited by the size of the
// CAL_CSI2_CTXi_j.CTXi_LINES and CAL_WR_DMA_CTRL_j.YSIZE fields, expressed in
// lines.
//
pub const CAL_MIN_WIDTH_BYTES: c_int = 16;

pub const CAL_MIN_HEIGHT_LINES: c_int = 1;
pub const CAL_MAX_HEIGHT_LINES: c_int = 16383;
pub const CAL_CAMERARX_PAD_SINK: c_int = 0;
pub const CAL_CAMERARX_PAD_FIRST_SOURCE: c_int = 1;
pub const CAL_CAMERARX_NUM_SOURCE_PADS: c_int = 8;

// Camera RX has 1 sink pad, and N source pads
// CTRL_CORE_CAMERRX_CONTROL register field id
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cal_camerarx_field {
    F_CTRLCLKEN,
    F_CAMMODE,
    F_LANEENABLE,
    F_CSI_MODE,
    F_MAX_FIELDS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cal_dma_state {
    CAL_DMA_RUNNING,
    CAL_DMA_STOP_REQUESTED,
    CAL_DMA_STOP_PENDING,
    CAL_DMA_STOPPED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_format_info {
    pub fourcc: u32,
    pub code: u32,
// Bits per pixel
    pub bpp: u8,
    pub meta: bool,
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

//
// struct cal_dmaqueue - Queue of DMA buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_dmaqueue {
//
// @lock: Protects all fields in the cal_dmaqueue.
//
    pub lock: spinlock_t,
//
// @queue: Buffers queued to the driver and waiting for DMA processing.
// Buffers are added to the list by the vb2 .buffer_queue() operation,
// and move to @pending when they are scheduled for the next frame.
//
    pub queue: list_head,
//
// @pending: Buffer provided to the hardware to DMA the next frame.
// Will move to @active at the end of the current frame.
//
    pub pending: *mut cal_buffer,
//
// @active: Buffer being DMA'ed to for the current frame. Will be
// retired and given back to vb2 at the end of the current frame if
// a @pending buffer has been scheduled to replace it.
//
    pub active: *mut cal_buffer,
// @state: State of the DMA engine.
    pub state: cal_dma_state,
// @wait: Wait queue to signal a @state transition to CAL_DMA_STOPPED.
    pub wait: wait_queue_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_camerarx_data {
    pub lsb: c_uint,
    pub msb: c_uint,
    pub fields: [}; F_MAX_FIELDS],
    pub num_lanes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_data {
    pub camerarx: *const cal_camerarx_data,
    pub num_csi2_phy: c_uint,
    pub flags: c_uint,
}

//
// The Camera Adaptation Layer (CAL) module is paired with one or more complex
// I/O PHYs (CAMERARX). It contains multiple instances of CSI-2, processing and
// DMA contexts.
//
// The cal_dev structure represents the whole subsystem, including the CAL and
// the CAMERARX instances. Instances of struct cal_dev are named cal through the
// driver.
//
// The cal_camerarx structure represents one CAMERARX instance. Instances of
// cal_camerarx are named phy through the driver.
//
// The cal_ctx structure represents the combination of one CSI-2 context, one
// processing context and one DMA context. Instance of struct cal_ctx are named
// ctx through the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_camerarx {
    pub base: *mut void __iomem,
    pub res: *mut resource,
    pub fields: [*mut regmap_field; F_MAX_FIELDS],
    pub cal: *mut cal_dev,
    pub instance: c_uint,
    pub endpoint: v4l2_fwnode_endpoint,
    pub source_ep_node: *mut device_node,
    pub source_node: *mut device_node,
    pub source: *mut v4l2_subdev,
    pub source_pad: c_uint,
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; CAL_CAMERARX_NUM_PADS],
// protects the vc_* fields below
    pub vc_lock: spinlock_t,
    pub vc_enable_count: [u8; 4],
    pub vc_frame_number: [u16; 4],
    pub vc_sequence: [u32; 4],
    pub enable_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_dev {
    pub fclk: *mut clk,
    pub irq: c_int,
    pub base: *mut void __iomem,
    pub res: *mut resource,
    pub dev: *mut device,
    pub data: *const cal_data,
    pub revision: u32,
// Control Module handle
    pub syscon_camerrx: *mut regmap,
    pub syscon_camerrx_offset: u32,
// Camera Core Module handle
    pub phy: [*mut cal_camerarx; CAL_NUM_CSI2_PORTS],
    pub num_contexts: u32,
    pub ctx: [*mut cal_ctx; CAL_MAX_NUM_CONTEXT],
    pub mdev: media_device,
    pub v4l2_dev: v4l2_device,
    pub notifier: v4l2_async_notifier,
    pub reserved_pix_proc_mask: c_ulong,
}

//
// There is one cal_ctx structure for each camera core context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cal_ctx {
    pub ctrl_handler: v4l2_ctrl_handler,
    pub vdev: video_device,
    pub pad: media_pad,
    pub cal: *mut cal_dev,
    pub phy: *mut cal_camerarx,
// v4l2_ioctl mutex
    pub mutex: mutex,
    pub dma: cal_dmaqueue,
// video capture
    pub fmtinfo: *const cal_format_info,
// Used to store current pixel format
    pub v_fmt: v4l2_format,
// Current subdev enumerated format (legacy)
    pub active_fmt: *const cal_format_info,
    pub num_active_fmt: c_uint,
    pub vb_vidq: vb2_queue,
    pub dma_ctx: u8,
    pub cport: u8,
    pub csi2_ctx: u8,
    pub pix_proc: u8,
    pub vc: u8,
    pub datatype: u8,
    pub use_pix_proc: bool,
}

extern "C" {
    pub fn ioread32(offset: cal->base +) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: mask, _arg: cal_read(cal, _arg: offset)) -> return;
}
// valp = val;
extern "C" {
    pub fn cal_quickdump_regs(cal: *mut cal_dev);
}
extern "C" {
    pub fn cal_camerarx_disable(phy: *mut cal_camerarx);
}
extern "C" {
    pub fn cal_camerarx_i913_errata(phy: *mut cal_camerarx);
}
extern "C" {
    pub fn cal_camerarx_destroy(phy: *mut cal_camerarx);
}
extern "C" {
    pub fn cal_ctx_prepare(ctx: *mut cal_ctx) -> c_int;
}
extern "C" {
    pub fn cal_ctx_unprepare(ctx: *mut cal_ctx);
}
extern "C" {
    pub fn cal_ctx_set_dma_addr(ctx: *mut cal_ctx, addr: dma_addr_t);
}
extern "C" {
    pub fn cal_ctx_start(ctx: *mut cal_ctx);
}
extern "C" {
    pub fn cal_ctx_stop(ctx: *mut cal_ctx);
}
extern "C" {
    pub fn cal_ctx_v4l2_register(ctx: *mut cal_ctx) -> c_int;
}
extern "C" {
    pub fn cal_ctx_v4l2_unregister(ctx: *mut cal_ctx);
}
extern "C" {
    pub fn cal_ctx_v4l2_init(ctx: *mut cal_ctx) -> c_int;
}
extern "C" {
    pub fn cal_ctx_v4l2_cleanup(ctx: *mut cal_ctx);
}

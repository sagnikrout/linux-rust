//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amlogic/c3/isp/c3-isp-common.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2024 Amlogic, Inc. All rights reserved
//

pub const C3_ISP_CLOCK_NUM_MAX: c_int = 3;
pub const C3_ISP_DEFAULT_WIDTH: c_int = 1920;
pub const C3_ISP_DEFAULT_HEIGHT: c_int = 1080;
pub const C3_ISP_MAX_WIDTH: c_int = 2888;
pub const C3_ISP_MAX_HEIGHT: c_int = 2240;
pub const C3_ISP_MIN_WIDTH: c_int = 160;
pub const C3_ISP_MIN_HEIGHT: c_int = 120;
pub const C3_ISP_DMA_SIZE_ALIGN_BYTES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_core_pads {
    C3_ISP_CORE_PAD_SINK_VIDEO,
    C3_ISP_CORE_PAD_SINK_PARAMS,
    C3_ISP_CORE_PAD_SOURCE_STATS,
    C3_ISP_CORE_PAD_SOURCE_VIDEO_0,
    C3_ISP_CORE_PAD_SOURCE_VIDEO_1,
    C3_ISP_CORE_PAD_SOURCE_VIDEO_2,
    C3_ISP_CORE_PAD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_resizer_ids {
    C3_ISP_RSZ_0,
    C3_ISP_RSZ_1,
    C3_ISP_RSZ_2,
    C3_ISP_NUM_RSZ
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_resizer_pads {
    C3_ISP_RSZ_PAD_SINK,
    C3_ISP_RSZ_PAD_SOURCE,
    C3_ISP_RSZ_PAD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_cap_devs {
    C3_ISP_CAP_DEV_0,
    C3_ISP_CAP_DEV_1,
    C3_ISP_CAP_DEV_2,
    C3_ISP_NUM_CAP_DEVS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c3_isp_planes {
    C3_ISP_PLANE_Y,
    C3_ISP_PLANE_UV,
    C3_ISP_NUM_PLANES
}

//
// struct c3_isp_cap_format_info - The image format of capture device
//
// @mbus_code: the mbus code
// @fourcc: the pixel format
// @format: defines the output format of hardware
// @planes: defines the mutil plane of hardware
// @ch0_pix_bits: defines the channel 0 pixel bits mode of hardware
// @uv_swap: defines the uv swap flag of hardware
// @in_bits: defines the input bits of hardware
// @hdiv: horizontal chroma subsampling factor of hardware
// @vdiv: vertical chroma subsampling factor of hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_cap_format_info {
    pub mbus_code: u32,
    pub fourcc: u32,
    pub format: u32,
    pub planes: u32,
    pub ch0_pix_bits: u32,
    pub uv_swap: u8,
    pub in_bits: u8,
    pub hdiv: u8,
    pub vdiv: u8,
}

//
// struct c3_isp_cap_buffer - A container of vb2 buffer used by the video
// devices: capture video devices
//
// @vb: vb2 buffer
// @dma_addr: buffer physical address
// @list: entry of the buffer in the queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_cap_buffer {
    pub vb: vb2_v4l2_buffer,
    pub dma_addr: [dma_addr_t; C3_ISP_NUM_PLANES],
    pub list: list_head,
}

//
// struct c3_isp_stats_dma_buffer - A container of vb2 buffer used by the video
// devices: stats video devices
//
// @vb: vb2 buffer
// @dma_addr: buffer physical address
// @list: entry of the buffer in the queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_stats_buffer {
    pub vb: vb2_v4l2_buffer,
    pub dma_addr: dma_addr_t,
    pub list: list_head,
}

//
// struct c3_isp_params_buffer - A container of vb2 buffer used by the
// params video device
//
// @vb: vb2 buffer
// @cfg: scratch buffer used for caching the ISP configuration parameters
// @list: entry of the buffer in the queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params_buffer {
    pub vb: vb2_v4l2_buffer,
    pub cfg: *mut c_void,
    pub list: list_head,
}

//
// struct c3_isp_dummy_buffer - A buffer to write the next frame to in case
// there are no vb2 buffers available.
//
// @vaddr:	return value of call to dma_alloc_attrs
// @dma_addr:	dma address of the buffer
// @size:	size of the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_dummy_buffer {
    pub vaddr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: u32,
}

//
// struct c3_isp_core - ISP core subdev
//
// @sd: ISP sub-device
// @pads: ISP sub-device pads
// @src_pad: source sub-device pad
// @isp: pointer to c3_isp_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_core {
    pub sd: v4l2_subdev,
    pub pads: [media_pad; C3_ISP_CORE_PAD_MAX],
    pub src_pad: *mut media_pad,
    pub isp: *mut c3_isp_device,
}

//
// struct c3_isp_resizer - ISP resizer subdev
//
// @id: resizer id
// @sd: resizer sub-device
// @pads: resizer sub-device pads
// @src_sd: source sub-device
// @isp: pointer to c3_isp_device
// @src_pad: the pad of source sub-device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_resizer {
    pub id: c3_isp_resizer_ids,
    pub sd: v4l2_subdev,
    pub pads: [media_pad; C3_ISP_RSZ_PAD_MAX],
    pub src_sd: *mut v4l2_subdev,
    pub isp: *mut c3_isp_device,
    pub src_pad: u32,
}

//
// struct c3_isp_stats - ISP statistics device
//
// @vb2_q: vb2 buffer queue
// @vdev: video node
// @vfmt: v4l2_format of the metadata format
// @pad: media pad
// @lock: protects vb2_q, vdev
// @isp: pointer to c3_isp_device
// @buff: in use buffer
// @buff_lock: protects stats buffer
// @pending: stats buffer list head
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_stats {
    pub vb2_q: vb2_queue,
    pub vdev: video_device,
    pub vfmt: v4l2_format,
    pub pad: media_pad,
    pub /: *mut *mut mutex lock; / Protects vb2_q, vdev,
    pub isp: *mut c3_isp_device,
    pub buff: *mut c3_isp_stats_buffer,
    pub /: *mut *mut spinlock_t buff_lock; / Protects stats buffer,
    pub pending: list_head,
}

//
// struct c3_isp_params - ISP parameters device
//
// @vb2_q: vb2 buffer queue
// @vdev: video node
// @vfmt: v4l2_format of the metadata format
// @pad: media pad
// @lock: protects vb2_q, vdev
// @isp: pointer to c3_isp_device
// @buff: in use buffer
// @buff_lock: protects stats buffer
// @pending: stats buffer list head
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_params {
    pub vb2_q: vb2_queue,
    pub vdev: video_device,
    pub vfmt: v4l2_format,
    pub pad: media_pad,
    pub /: *mut *mut mutex lock; / Protects vb2_q, vdev,
    pub isp: *mut c3_isp_device,
    pub buff: *mut c3_isp_params_buffer,
    pub /: *mut *mut spinlock_t buff_lock; / Protects params buffer,
    pub pending: list_head,
}

//
// struct c3_isp_capture - ISP capture device
//
// @id: capture device ID
// @vb2_q: vb2 buffer queue
// @vdev: video node
// @pad: media pad
// @lock: protects vb2_q, vdev
// @isp: pointer to c3_isp_device
// @rsz: pointer to c3_isp_resizer
// @buff: in use buffer
// @buff_lock: protects capture buffer
// @pending: capture buffer list head
// @format.info: a pointer to the c3_isp_capture_format of the pixel format
// @format.fmt: buffer format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_capture {
    pub id: c3_isp_cap_devs,
    pub vb2_q: vb2_queue,
    pub vdev: video_device,
    pub pad: media_pad,
    pub /: *mut *mut mutex lock; / Protects vb2_q, vdev,
    pub isp: *mut c3_isp_device,
    pub rsz: *mut c3_isp_resizer,
    pub dummy_buff: c3_isp_dummy_buffer,
    pub buff: *mut c3_isp_cap_buffer,
    pub /: *mut *mut spinlock_t buff_lock; / Protects stream buffer,
    pub pending: list_head,
    pub info: *const c3_isp_cap_format_info,
    pub pix_mp: v4l2_pix_format_mplane,
    pub format: },
}

//
// struct c3_isp_info - ISP information
//
// @clocks: array of ISP clock names
// @clock_num: actual clock number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_info {
    pub clocks: [*mut c_char; C3_ISP_CLOCK_NUM_MAX],
    pub clock_num: u32,
}

//
// struct c3_isp_device - ISP platform device
//
// @dev: pointer to the struct device
// @base: base register address
// @clks: array of clocks
// @notifier: notifier to register on the v4l2-async API
// @v4l2_dev: v4l2_device variable
// @media_dev: media device variable
// @pipe: media pipeline
// @core: ISP core subdev
// @resizers: ISP resizer subdev
// @stats: ISP stats device
// @params: ISP params device
// @caps: array of ISP capture device
// @frm_sequence: used to record frame id
// @info: version-specific ISP information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c3_isp_device {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub clks: [clk_bulk_data; C3_ISP_CLOCK_NUM_MAX],
    pub notifier: v4l2_async_notifier,
    pub v4l2_dev: v4l2_device,
    pub media_dev: media_device,
    pub pipe: media_pipeline,
    pub core: c3_isp_core,
    pub resizers: [c3_isp_resizer; C3_ISP_NUM_RSZ],
    pub stats: c3_isp_stats,
    pub params: c3_isp_params,
    pub caps: [c3_isp_capture; C3_ISP_NUM_CAP_DEVS],
    pub frm_sequence: u32,
    pub info: *const c3_isp_info,
}

extern "C" {
    pub fn c3_isp_read(isp: *mut c3_isp_device, reg: u32) -> u32;
}
extern "C" {
    pub fn c3_isp_write(isp: *mut c3_isp_device, reg: u32, val: u32);
}
extern "C" {
    pub fn c3_isp_update_bits(isp: *mut c3_isp_device, reg: u32, mask: u32, val: u32);
}
extern "C" {
    pub fn c3_isp_core_queue_sof(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_core_register(isp: *mut c3_isp_device) -> c_int;
}
extern "C" {
    pub fn c3_isp_core_unregister(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_resizers_register(isp: *mut c3_isp_device) -> c_int;
}
extern "C" {
    pub fn c3_isp_resizers_unregister(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_captures_register(isp: *mut c3_isp_device) -> c_int;
}
extern "C" {
    pub fn c3_isp_captures_unregister(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_captures_isr(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_stats_pre_cfg(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_stats_register(isp: *mut c3_isp_device) -> c_int;
}
extern "C" {
    pub fn c3_isp_stats_unregister(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_stats_isr(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_params_pre_cfg(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_params_register(isp: *mut c3_isp_device) -> c_int;
}
extern "C" {
    pub fn c3_isp_params_unregister(isp: *mut c3_isp_device);
}
extern "C" {
    pub fn c3_isp_params_isr(isp: *mut c3_isp_device);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkcif/rkcif-common.h
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
// Rockchip Camera Interface (CIF) Driver
//
// Copyright (C) 2018 Rockchip Electronics Co., Ltd.
// Copyright (C) 2023 Mehdi Djait <mehdi.djait@bootlin.com>
// Copyright (C) 2025 Michael Riesch <michael.riesch@wolfvision.net>
// Copyright (C) 2025 Collabora, Ltd.
//

pub const RKCIF_CLK_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_format_type {
    RKCIF_FMT_TYPE_INVALID,
    RKCIF_FMT_TYPE_YUV,
    RKCIF_FMT_TYPE_RAW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_id_index {
    RKCIF_ID0,
    RKCIF_ID1,
    RKCIF_ID2,
    RKCIF_ID3,
    RKCIF_ID_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_interface_index {
    RKCIF_DVP,
    RKCIF_MIPI_BASE,
    RKCIF_MIPI1 = RKCIF_MIPI_BASE,
    RKCIF_MIPI2,
    RKCIF_MIPI3,
    RKCIF_MIPI4,
    RKCIF_MIPI5,
    RKCIF_MIPI6,
    RKCIF_MIPI_MAX,
    RKCIF_IF_MAX = RKCIF_MIPI_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_interface_pad_index {
    RKCIF_IF_PAD_SINK,
    RKCIF_IF_PAD_SRC,
    RKCIF_IF_PAD_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_interface_status {
    RKCIF_IF_INACTIVE,
    RKCIF_IF_ACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_interface_type {
    RKCIF_IF_INVALID,
    RKCIF_IF_DVP,
    RKCIF_IF_MIPI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_mipi_format_type {
    RKCIF_MIPI_TYPE_INVALID,
    RKCIF_MIPI_TYPE_RAW8,
    RKCIF_MIPI_TYPE_RAW10,
    RKCIF_MIPI_TYPE_RAW12,
    RKCIF_MIPI_TYPE_RGB888,
    RKCIF_MIPI_TYPE_YUV422SP,
    RKCIF_MIPI_TYPE_YUV420SP,
    RKCIF_MIPI_TYPE_YUV400,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_buffer {
    pub vb: vb2_v4l2_buffer,
    pub queue: list_head,
    pub buff_addr: [dma_addr_t; VIDEO_MAX_PLANES],
    pub is_dummy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_dummy_buffer {
    pub buffer: rkcif_buffer,
    pub vaddr: *mut c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rkcif_plane_index {
    RKCIF_PLANE_Y,
    RKCIF_PLANE_UV,
    RKCIF_PLANE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_input_fmt {
    pub mbus_code: u32,
    pub fmt_type: rkcif_format_type,
    pub field: v4l2_field,
    pub dvp_fmt_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_output_fmt {
    pub fourcc: u32,
    pub mbus_code: u32,
    pub cplanes: u8,
    pub depth: u8,
    pub dvp_fmt_val: u32,
    pub dt: u8,
    pub compact: bool,
    pub type: rkcif_mipi_format_type,
    pub mipi: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_remote {
    pub async_conn: v4l2_async_connection,
    pub sd: *mut v4l2_subdev,
    pub interface: *mut rkcif_interface,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_stream {
    pub id: rkcif_id_index,
    pub rkcif: *mut rkcif_device,
    pub interface: *mut rkcif_interface,
    pub out_fmts: *const rkcif_output_fmt,
    pub out_fmts_num: c_uint,
// in ping-pong mode, two buffers can be provided to the HW
    pub buffers: [*mut rkcif_buffer; 2],
    pub frame_idx: c_int,
    pub frame_phase: c_int,
// in case of no available buffer, HW can write to the dummy buffer
    pub dummy: rkcif_dummy_buffer,
    pub stopping: bool,
    pub wq_stopped: wait_queue_head_t,
// queue of available buffers plus spinlock that protects it
    pub driver_queue_lock: spinlock_t,
    pub driver_queue: list_head,
// lock used by the V4L2 core
    pub vlock: mutex,
    pub pad: media_pad,
    pub pipeline: media_pipeline,
    pub pix: v4l2_pix_format_mplane,
    pub buf_queue: vb2_queue,
    pub vdev: video_device,
    pub index): *mut *mut *mut void (queue_buffer)(struct rkcif_stream stream, unsigned int,
    pub stream): *mut *mut int (start_streaming)(struct rkcif_stream,
    pub stream): *mut *mut void (stop_streaming)(struct rkcif_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_dvp {
    pub dvp_clk_delay: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_interface {
    pub type: rkcif_interface_type,
    pub status: rkcif_interface_status,
    pub index: rkcif_interface_index,
    pub rkcif: *mut rkcif_device,
    pub remote: *mut rkcif_remote,
    pub streams: [rkcif_stream; RKCIF_ID_MAX],
    pub streams_num: c_uint,
    pub in_fmts: *const rkcif_input_fmt,
    pub in_fmts_num: c_uint,
    pub pads: [media_pad; RKCIF_IF_PAD_MAX],
    pub vep: v4l2_fwnode_endpoint,
    pub sd: v4l2_subdev,
    pub dvp: rkcif_dvp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_mipi_match_data {
    pub mipi_num: c_uint,
    pub regs: [c_uint; RKCIF_MIPI_REGISTER_MAX],
    pub regs_id: [c_uint; RKCIF_ID_MAX][RKCIF_MIPI_ID_REGISTER_MAX],
    pub active_out_fmt): *const rkcif_output_fmt,
    pub offset: c_uint,
    pub RKCIF_MIPI_BASE]: } blocks[RKCIF_MIPI_MAX -,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_dvp_match_data {
    pub in_fmts: *const rkcif_input_fmt,
    pub in_fmts_num: c_uint,
    pub out_fmts: *const rkcif_output_fmt,
    pub out_fmts_num: c_uint,
    pub rkcif): *mut *mut void (setup)(struct rkcif_device,
    pub has_scaler: bool,
    pub has_ids: bool,
    pub regs: [c_uint; RKCIF_DVP_REGISTER_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_match_data {
    pub clks: *const *const c_char,
    pub clks_num: c_uint,
    pub dvp: *const rkcif_dvp_match_data,
    pub mipi: *const rkcif_mipi_match_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkcif_device {
    pub dev: *mut device,
    pub match_data: *const rkcif_match_data,
    pub clks: [clk_bulk_data; RKCIF_CLK_MAX],
    pub clks_num: c_uint,
    pub grf: *mut regmap,
    pub reset: *mut reset_control,
    pub base_addr: *mut void __iomem,
    pub interfaces: [rkcif_interface; RKCIF_IF_MAX],
    pub media_dev: media_device,
    pub v4l2_dev: v4l2_device,
    pub notifier: v4l2_async_notifier,
}

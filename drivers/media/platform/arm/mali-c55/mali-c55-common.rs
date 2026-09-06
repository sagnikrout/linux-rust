//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/arm/mali-c55/mali-c55-common.h
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
// ARM Mali-C55 ISP Driver - Common definitions
//
// Copyright (C) 2025 Ideas on Board Oy
//

// min and max values for the image sizes

pub const MALI_C55_NUM_CLKS: c_int = 3;
pub const MALI_C55_NUM_RESETS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mali_c55_isp_pads {
    MALI_C55_ISP_PAD_SINK_VIDEO,
    MALI_C55_ISP_PAD_SOURCE_VIDEO,
    MALI_C55_ISP_PAD_SOURCE_BYPASS,
    MALI_C55_ISP_PAD_SOURCE_STATS,
    MALI_C55_ISP_PAD_SINK_PARAMS,
    MALI_C55_ISP_NUM_PADS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_tpg {
    pub mali_c55: *mut mali_c55,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_tpg_ctrls {
    pub handler: v4l2_ctrl_handler,
    pub vblank: *mut v4l2_ctrl,
    pub ctrls: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_isp {
    pub mali_c55: *mut mali_c55,
    pub sd: v4l2_subdev,
    pub pads: [media_pad; MALI_C55_ISP_NUM_PADS],
    pub handler: v4l2_ctrl_handler,
    pub remote_src: *mut media_pad,
// Mutex to guard vb2 start/stop streaming
    pub capture_lock: mutex,
    pub frame_sequence: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mali_c55_resizer_ids {
    MALI_C55_RSZ_FR,
    MALI_C55_RSZ_DS,
    MALI_C55_NUM_RSZS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mali_c55_rsz_pads {
    MALI_C55_RSZ_SINK_PAD,
    MALI_C55_RSZ_SOURCE_PAD,
    MALI_C55_RSZ_SINK_BYPASS_PAD,
    MALI_C55_RSZ_NUM_PADS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_resizer {
    pub mali_c55: *mut mali_c55,
    pub cap_dev: *mut mali_c55_cap_dev,
    pub id: mali_c55_resizer_ids,
    pub sd: v4l2_subdev,
    pub pads: [media_pad; MALI_C55_RSZ_NUM_PADS],
    pub num_routes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mali_c55_cap_devs {
    MALI_C55_CAP_DEV_FR,
    MALI_C55_CAP_DEV_DS,
    MALI_C55_NUM_CAP_DEVS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_format_info {
    pub fourcc: u32,
//
// The output formats can be produced by a couple of different media bus
// formats, depending on how the ISP is configured.
//
    pub mbus_codes: [c_uint; 2],
    pub is_raw: bool,
    pub base_mode: u32,
    pub uv_plane: u32,
    pub registers: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_isp_format_info {
    pub code: u32,
    pub shifted_code: u32,
    pub bypass: bool,
    pub order: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mali_c55_planes {
    MALI_C55_PLANE_Y,
    MALI_C55_PLANE_UV,
    MALI_C55_NUM_PLANES
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_buffer {
    pub vb: vb2_v4l2_buffer,
    pub planes_pending: c_uint,
    pub queue: list_head,
    pub addrs: [dma_addr_t; MALI_C55_NUM_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_cap_dev {
    pub mali_c55: *mut mali_c55,
    pub rsz: *mut mali_c55_resizer,
    pub vdev: video_device,
    pub pad: media_pad,
    pub queue: vb2_queue,
// Mutex to provide to vb2
    pub lock: mutex,
    pub reg_offset: c_uint,
    pub info: *const mali_c55_format_info,
    pub format: v4l2_pix_format_mplane,
    pub format: },
// Spinlock to guard buffer queue
    pub lock: spinlock_t,
// Spinlock to guard the queue of buffers being processed
    pub processing_lock: spinlock_t,
    pub input: list_head,
    pub processing: list_head,
    pub buffers: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_stats_buf {
    pub vb: vb2_v4l2_buffer,
    pub segments_remaining: c_uint,
    pub queue: list_head,
    pub failed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_params_buf {
    pub vb: vb2_v4l2_buffer,
    pub queue: list_head,
    pub config: *mut v4l2_isp_params_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_stats {
    pub mali_c55: *mut mali_c55,
    pub vdev: video_device,
    pub queue: vb2_queue,
    pub pad: media_pad,
// Mutex to provide to vb2
    pub lock: mutex,
// Spinlock to guard buffer queue
    pub lock: spinlock_t,
    pub queue: list_head,
    pub buffers: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_params {
    pub mali_c55: *mut mali_c55,
    pub vdev: video_device,
    pub queue: vb2_queue,
    pub pad: media_pad,
// Mutex to provide to vb2
    pub lock: mutex,
// Spinlock to guard buffer queue
    pub lock: spinlock_t,
    pub queue: list_head,
    pub buffers: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mali_c55_config_spaces {
    MALI_C55_CONFIG_PONG,
    MALI_C55_CONFIG_PING,
}

//
// struct mali_c55_context - Fields relating to a single camera context
//
// @mali_c55:	Pointer to the main struct mali_c55
// @registers:	A pointer to some allocated memory holding register
// values to be written to the hardware at frame interrupt
// @base:	Base address of the config space in the hardware
// @lock:	A spinlock to protect against writes to @registers whilst that
// space is being copied to the hardware
// @list:	A list head to facilitate a context queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55_context {
    pub mali_c55: *mut mali_c55,
    pub registers: *mut u32,
    pub base: phys_addr_t,
// Spinlock to prevent simultaneous access of register space
    pub lock: spinlock_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mali_c55 {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub clks: [clk_bulk_data; MALI_C55_NUM_CLKS],
    pub resets: [reset_control_bulk_data; MALI_C55_NUM_RESETS],
    pub irqnum: c_int,
    pub capabilities: u16,
    pub inline_mode: bool,
    pub media_dev: media_device,
    pub v4l2_dev: v4l2_device,
    pub notifier: v4l2_async_notifier,
    pub pipe: media_pipeline,
    pub tpg: mali_c55_tpg,
    pub isp: mali_c55_isp,
    pub resizers: [mali_c55_resizer; MALI_C55_NUM_RSZS],
    pub cap_devs: [mali_c55_cap_dev; MALI_C55_NUM_CAP_DEVS],
    pub params: mali_c55_params,
    pub stats: mali_c55_stats,
    pub context: mali_c55_context,
    pub next_config: u32,
}

extern "C" {
    pub fn mali_c55_write(mali_c55: *mut mali_c55, addr: c_uint, val: u32);
}
extern "C" {
    pub fn mali_c55_read(mali_c55: *mut mali_c55, addr: c_uint) -> u32;
}
extern "C" {
    pub fn mali_c55_ctx_write(mali_c55: *mut mali_c55, addr: c_uint, val: u32);
}
extern "C" {
    pub fn mali_c55_ctx_read(mali_c55: *mut mali_c55, addr: c_uint) -> u32;
}
extern "C" {
    pub fn mali_c55_register_isp(mali_c55: *mut mali_c55) -> c_int;
}
extern "C" {
    pub fn mali_c55_register_tpg(mali_c55: *mut mali_c55) -> c_int;
}
extern "C" {
    pub fn mali_c55_unregister_tpg(mali_c55: *mut mali_c55);
}
extern "C" {
    pub fn mali_c55_unregister_isp(mali_c55: *mut mali_c55);
}
extern "C" {
    pub fn mali_c55_register_resizers(mali_c55: *mut mali_c55) -> c_int;
}
extern "C" {
    pub fn mali_c55_unregister_resizers(mali_c55: *mut mali_c55);
}
extern "C" {
    pub fn mali_c55_register_capture_devs(mali_c55: *mut mali_c55) -> c_int;
}
extern "C" {
    pub fn mali_c55_unregister_capture_devs(mali_c55: *mut mali_c55);
}
extern "C" {
    pub fn mali_c55_register_stats(mali_c55: *mut mali_c55) -> c_int;
}
extern "C" {
    pub fn mali_c55_unregister_stats(mali_c55: *mut mali_c55);
}
extern "C" {
    pub fn mali_c55_register_params(mali_c55: *mut mali_c55) -> c_int;
}
extern "C" {
    pub fn mali_c55_unregister_params(mali_c55: *mut mali_c55);
}
extern "C" {
    pub fn mali_c55_set_next_buffer(cap_dev: *mut mali_c55_cap_dev);
}
extern "C" {
    pub fn mali_c55_isp_queue_event_sof(mali_c55: *mut mali_c55);
}
extern "C" {
    pub fn mali_c55_format_is_raw(mbus_code: c_uint) -> bool;
}
extern "C" {
    pub fn mali_c55_pipeline_ready(mali_c55: *mut mali_c55) -> bool;
}
extern "C" {
    pub fn mali_c55_params_write_config(mali_c55: *mut mali_c55);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/isppreview.h
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
// isppreview.h
//
// TI OMAP3 ISP - Preview module
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc.
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

pub const ISPPRV_BRIGHT_STEP: c_uint = 0x1;
pub const ISPPRV_BRIGHT_DEF: c_uint = 0x0;
pub const ISPPRV_BRIGHT_LOW: c_uint = 0x0;
pub const ISPPRV_BRIGHT_HIGH: c_uint = 0xFF;
pub const ISPPRV_BRIGHT_UNITS: c_uint = 0x1;
pub const ISPPRV_CONTRAST_STEP: c_uint = 0x1;
pub const ISPPRV_CONTRAST_DEF: c_uint = 0x10;
pub const ISPPRV_CONTRAST_LOW: c_uint = 0x0;
pub const ISPPRV_CONTRAST_HIGH: c_uint = 0xFF;
pub const ISPPRV_CONTRAST_UNITS: c_uint = 0x1;
// Additional features not listed in linux/omap3isp.h

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum preview_input_entity {
    PREVIEW_INPUT_NONE,
    PREVIEW_INPUT_CCDC,
    PREVIEW_INPUT_MEMORY,
}

// Configure byte layout of YUV image
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum preview_ycpos_mode {
    YCPOS_YCrYCb = 0,
    YCPOS_YCbYCr = 1,
    YCPOS_CbYCrY = 2,
    YCPOS_CrYCbY = 3
}

//
// struct prev_params - Structure for all configuration
// @busy: Bitmask of busy parameters (being updated or used)
// @update: Bitmask of the parameters to be updated
// @features: Set of features enabled.
// @cfa: CFA coefficients.
// @csup: Chroma suppression coefficients.
// @luma: Luma enhancement coefficients.
// @nf: Noise filter coefficients.
// @dcor: Noise filter coefficients.
// @gamma: Gamma coefficients.
// @wbal: White Balance parameters.
// @blkadj: Black adjustment parameters.
// @rgb2rgb: RGB blending parameters.
// @csc: Color space conversion (RGB to YCbCr) parameters.
// @hmed: Horizontal median filter.
// @yclimit: YC limits parameters.
// @contrast: Contrast.
// @brightness: Brightness.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prev_params {
    pub busy: u32,
    pub update: u32,
    pub features: u32,
    pub cfa: omap3isp_prev_cfa,
    pub csup: omap3isp_prev_csup,
    pub luma: omap3isp_prev_luma,
    pub nf: omap3isp_prev_nf,
    pub dcor: omap3isp_prev_dcor,
    pub gamma: omap3isp_prev_gtables,
    pub wbal: omap3isp_prev_wbal,
    pub blkadj: omap3isp_prev_blkadj,
    pub rgb2rgb: omap3isp_prev_rgbtorgb,
    pub csc: omap3isp_prev_csc,
    pub hmed: omap3isp_prev_hmed,
    pub yclimit: omap3isp_prev_yclimit,
    pub contrast: u8,
    pub brightness: u8,
}

// Sink and source previewer pads
pub const PREV_PAD_SINK: c_int = 0;
pub const PREV_PAD_SOURCE: c_int = 1;
pub const PREV_PADS_NUM: c_int = 2;
//
// struct isp_prev_device - Structure for storing ISP Preview module information
// @subdev: V4L2 subdevice
// @pads: Media entity pads
// @formats: Active formats at the subdev pad
// @crop: Active crop rectangle
// @input: Module currently connected to the input pad
// @output: Bitmask of the active output
// @video_in: Input video entity
// @video_out: Output video entity
// @params.params : Active and shadow parameters sets
// @params.active: Bitmask of parameters active in set 0
// @params.lock: Parameters lock, protects params.active and params.shadow
// @underrun: Whether the preview entity has queued buffers on the output
// @state: Current preview pipeline state
//
// This structure is used to store the OMAP ISP Preview module Information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_prev_device {
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; PREV_PADS_NUM],
    pub formats: [v4l2_mbus_framefmt; PREV_PADS_NUM],
    pub crop: v4l2_rect,
    pub ctrls: v4l2_ctrl_handler,
    pub input: preview_input_entity,
    pub output: c_uint,
    pub video_in: isp_video,
    pub video_out: isp_video,
    pub cfa_order: c_uint,
    pub params: [prev_params; 2],
    pub active: u32,
    pub lock: spinlock_t,
    pub params: },
    pub state: isp_pipeline_stream_state,
    pub wait: wait_queue_head_t,
    pub stopping: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn omap3isp_preview_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_preview_cleanup(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_preview_unregister_entities(prv: *mut isp_prev_device);
}
extern "C" {
    pub fn omap3isp_preview_isr_frame_sync(prev: *mut isp_prev_device);
}
extern "C" {
    pub fn omap3isp_preview_isr(prev: *mut isp_prev_device);
}
extern "C" {
    pub fn omap3isp_preview_busy(isp_prev: *mut isp_prev_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_preview_restore_context(isp: *mut isp_device);
}

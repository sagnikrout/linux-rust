//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/omap3isp/ispresizer.h
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
// ispresizer.h
//
// TI OMAP3 ISP - Resizer module
//
// Copyright (C) 2010 Nokia Corporation
// Copyright (C) 2009 Texas Instruments, Inc
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//

//
// Constants for filter coefficients count
//
pub const COEFF_CNT: c_int = 32;
//
// struct isprsz_coef - Structure for resizer filter coefficients.
// @h_filter_coef_4tap: Horizontal filter coefficients for 8-phase/4-tap
// mode (.5x-4x)
// @v_filter_coef_4tap: Vertical filter coefficients for 8-phase/4-tap
// mode (.5x-4x)
// @h_filter_coef_7tap: Horizontal filter coefficients for 4-phase/7-tap
// mode (.25x-.5x)
// @v_filter_coef_7tap: Vertical filter coefficients for 4-phase/7-tap
// mode (.25x-.5x)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isprsz_coef {
    pub h_filter_coef_4tap: [u16; 32],
    pub v_filter_coef_4tap: [u16; 32],
// Every 8th value is a dummy value in the following arrays:
    pub h_filter_coef_7tap: [u16; 32],
    pub v_filter_coef_7tap: [u16; 32],
}

// Chrominance horizontal algorithm
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resizer_chroma_algo {
    RSZ_THE_SAME = 0,	/* Chrominance the same as Luminance */
    RSZ_BILINEAR = 1,	/* Chrominance uses bilinear interpolation */
}

// Resizer input type select
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resizer_colors_type {
    RSZ_YUV422 = 0,		/* YUV422 color is interleaved */
    RSZ_COLOR8 = 1,		/* Color separate data on 8 bits */
}

//
// Structure for horizontal and vertical resizing value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resizer_ratio {
    pub horz: u32,
    pub vert: u32,
}

//
// Structure for luminance enhancer parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resizer_luma_yenh {
    pub /: *mut *mut u8 algo; / algorithm select.,
    pub /: *mut *mut u8 gain; / maximum gain.,
    pub /: *mut *mut u8 slope; / slope.,
    pub /: *mut *mut u8 core; / core offset.,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resizer_input_entity {
    RESIZER_INPUT_NONE,
    RESIZER_INPUT_VP,	/* input video port - prev or ccdc */
    RESIZER_INPUT_MEMORY,
}

// Sink and source resizer pads
pub const RESZ_PAD_SINK: c_int = 0;
pub const RESZ_PAD_SOURCE: c_int = 1;
pub const RESZ_PADS_NUM: c_int = 2;
//
// struct isp_res_device - OMAP3 ISP resizer module
// @lock: Protects formats and crop rectangles between set_selection and IRQ
// @crop.request: Crop rectangle requested by the user
// @crop.active: Active crop rectangle (based on hardware requirements)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp_res_device {
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; RESZ_PADS_NUM],
    pub formats: [v4l2_mbus_framefmt; RESZ_PADS_NUM],
    pub input: resizer_input_entity,
    pub video_in: isp_video,
    pub video_out: isp_video,
    pub /: *mut *mut u32 addr_base; / stored source buffer address in memory mode,
    pub /: *mut *mut u32 crop_offset; / additional offset for crop in memory mode,
    pub ratio: resizer_ratio,
    pub pm_state: c_int,
    pub applycrop:1: c_uint,
    pub state: isp_pipeline_stream_state,
    pub wait: wait_queue_head_t,
    pub stopping: core::sync::atomic::AtomicI32,
    pub lock: spinlock_t,
    pub request: v4l2_rect,
    pub active: v4l2_rect,
    pub crop: },
}

extern "C" {
    pub fn omap3isp_resizer_init(isp: *mut isp_device) -> c_int;
}
extern "C" {
    pub fn omap3isp_resizer_cleanup(isp: *mut isp_device);
}
extern "C" {
    pub fn omap3isp_resizer_unregister_entities(res: *mut isp_res_device);
}
extern "C" {
    pub fn omap3isp_resizer_isr_frame_sync(res: *mut isp_res_device);
}
extern "C" {
    pub fn omap3isp_resizer_isr(isp_res: *mut isp_res_device);
}
extern "C" {
    pub fn omap3isp_resizer_suspend(isp_res: *mut isp_res_device);
}
extern "C" {
    pub fn omap3isp_resizer_resume(isp_res: *mut isp_res_device);
}
extern "C" {
    pub fn omap3isp_resizer_busy(isp_res: *mut isp_res_device) -> c_int;
}

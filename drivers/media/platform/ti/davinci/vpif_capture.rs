//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/davinci/vpif_capture.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2009 Texas Instruments Inc
//
// Header files

// Macros

pub const VPIF_CAPTURE_MAX_DEVICES: c_int = 2;
pub const VPIF_VIDEO_INDEX: c_int = 0;
pub const VPIF_NUMBER_OF_OBJECTS: c_int = 1;
// Enumerated data type to give id to each device per channel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpif_channel_id {
    VPIF_CHANNEL0_VIDEO = 0,
    VPIF_CHANNEL1_VIDEO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_obj {
    pub buf_field: v4l2_field,
// Currently selected or default standard
    pub stdid: v4l2_std_id,
    pub dv_timings: v4l2_dv_timings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_cap_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_obj {
// Pointer pointing to current v4l2_buffer
    pub cur_frm: *mut vpif_cap_buffer,
// Pointer pointing to current v4l2_buffer
    pub next_frm: *mut vpif_cap_buffer,
// Used to store pixel format
    pub fmt: v4l2_format,
// Buffer queue used in vb2
    pub buffer_queue: vb2_queue,
// Queue of filled frames
    pub dma_queue: list_head,
// Protects the dma_queue field
    pub irqlock: spinlock_t,
// lock used to access this structure
    pub lock: mutex,
// Function pointer to set the addresses
    pub long): unsigned,
// offset where Y top starts from the starting of the buffer
    pub ytop_off: u32,
// offset where Y bottom starts from the starting of the buffer
    pub ybtm_off: u32,
// offset where C top starts from the starting of the buffer
    pub ctop_off: u32,
// offset where C bottom starts from the starting of the buffer
    pub cbtm_off: u32,
// Indicates width of the image data
    pub width: u32,
// Indicates height of the image data
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_obj {
// Identifies video device for this channel
    pub video_dev: video_device,
// Indicates id of the field which is being displayed
    pub field_id: u32,
// flag to indicate whether decoder is initialized
    pub initialized: u8,
// Identifies channel
    pub channel_id: vpif_channel_id,
// Current input
    pub input_idx: u32,
// subdev corresponding to the current input, may be NULL
    pub sd: *mut v4l2_subdev,
// vpif configuration params
    pub vpifparams: vpif_params,
// common object array
    pub common: [common_obj; VPIF_NUMBER_OF_OBJECTS],
// video object
    pub video: video_obj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_device {
    pub v4l2_dev: v4l2_device,
    pub dev: [*mut channel_obj; VPIF_CAPTURE_NUM_CHANNELS],
    pub sd: *mut v4l2_subdev,
    pub notifier: v4l2_async_notifier,
    pub config: *mut vpif_capture_config,
}

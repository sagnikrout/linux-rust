//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/ti/davinci/vpif_display.h
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
// VPIF display header file
//
// Copyright (C) 2009 Texas Instruments Incorporated - https://www.ti.com
//
// Header files

// Macros

// Setting it to 1 as HBI/VBI support yet to be added , else 3

// Macros

// enumerated data types
// Enumerated data type to give id to each device per channel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpif_channel_id {
    VPIF_CHANNEL2_VIDEO = 0,	/* Channel2 Video */
    VPIF_CHANNEL3_VIDEO,		/* Channel3 Video */
}

// structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_obj {
    pub buf_field: v4l2_field,
    pub return: *mut *mut u32 latest_only; / indicate whether to,
// most recent displayed frame only
    pub default: *mut *mut v4l2_std_id stdid; / Currently selected or,
// standard
    pub dv_timings: v4l2_dv_timings,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_disp_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_obj {
    pub current: *mut *mut *mut vpif_disp_buffer cur_frm; / Pointer pointing to,
// vb2_buffer
    pub next: *mut *mut *mut vpif_disp_buffer next_frm; / Pointer pointing to,
// vb2_buffer
    pub /: *mut *mut v4l2_format fmt; / Used to store the format,
    pub /: *mut *mut vb2_queue buffer_queue; / Buffer queue used in vb2,
    pub /: *mut *mut list_head dma_queue; / Queue of filled frames,
    pub buffer: *mut *mut spinlock_t irqlock; / Used for video,
// handling
// channel specific parameters
    pub this: *mut *mut mutex lock; / lock used to access,
// structure
    pub the: *mut *mut u32 ytop_off; / offset of Y top from,
// starting of the buffer
    pub the: *mut *mut u32 ybtm_off; / offset of Y bottom from,
// starting of the buffer
    pub the: *mut *mut u32 ctop_off; / offset of C top from,
// starting of the buffer
    pub the: *mut *mut u32 cbtm_off; / offset of C bottom from,
// starting of the buffer
// Function pointer to set the addresses
    pub long): unsigned long, unsigned,
    pub height: u32,
    pub width: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_obj {
// V4l2 specific parameters
    pub for: *mut *mut video_device video_dev; / Identifies video device,
// this channel
    pub field: *mut *mut u32 field_id; / Indicates id of the,
// which is being displayed
    pub whether: *mut *mut u8 initialized; / flag to indicate,
// encoder is initialized
    pub /: *mut *mut u32 output_idx; / Current output index,
    pub /: *mut *mut *mut v4l2_subdev sd; / Current output subdev(may be NULL),
    pub /: *mut *mut vpif_channel_id channel_id;/ Identifies channel,
    pub vpifparams: vpif_params,
    pub common: [common_obj; VPIF_NUMOBJECTS],
    pub video: video_obj,
}

// vpif device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpif_device {
    pub v4l2_dev: v4l2_device,
    pub dev: [*mut channel_obj; VPIF_DISPLAY_NUM_CHANNELS],
    pub sd: *mut v4l2_subdev,
    pub config: *mut vpif_display_config,
}

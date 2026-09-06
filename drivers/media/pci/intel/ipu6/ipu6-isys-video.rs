//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-isys-video.h
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
// Copyright (C) 2013--2024 Intel Corporation

pub const IPU6_ISYS_OUTPUT_PINS: c_int = 11;
pub const IPU6_ISYS_MAX_PARALLEL_SOF: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_pixelformat {
    pub pixelformat: u32,
    pub bpp: u32,
    pub bpp_packed: u32,
    pub code: u32,
    pub css_pixelformat: u32,
    pub is_meta: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sequence_info {
    pub sequence: c_uint,
    pub timestamp: u64,
}

//
// Align with firmware stream. Each stream represents a CSI virtual channel.
// May map to multiple video devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_stream {
    pub mutex: mutex,
    pub sequence: core::sync::atomic::AtomicI32,
    pub seq_index: c_uint,
    pub seq: [sequence_info; IPU6_ISYS_MAX_PARALLEL_SOF],
    pub stream_source: c_int,
    pub stream_handle: c_int,
    pub nr_output_pins: c_uint,
    pub asd: *mut ipu6_isys_subdev,
    pub /: *mut *mut int nr_queues; / Number of capture queues,
    pub nr_streaming: c_int,
    pub /: *mut *mut int streaming; / Has streaming been really started?,
    pub queues: list_head,
    pub stream_open_completion: completion,
    pub stream_close_completion: completion,
    pub stream_start_completion: completion,
    pub stream_stop_completion: completion,
    pub isys: *mut ipu6_isys,
    pub output_pins_queue: [*mut ipu6_isys_queue; IPU6_ISYS_OUTPUT_PINS],
    pub error: c_int,
    pub vc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_stream_watermark {
    pub width: u32,
    pub height: u32,
    pub hblank: u32,
    pub frame_rate: u32,
    pub pixel_rate: u64,
    pub stream_data_rate: u64,
    pub sram_gran_shift: u16,
    pub sram_gran_size: u16,
    pub stream_node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_video {
    pub aq: ipu6_isys_queue,
// Serialise access to other fields in the struct.
    pub mutex: mutex,
    pub pad: media_pad,
    pub vdev: video_device,
    pub pix_fmt: v4l2_pix_format,
    pub meta_fmt: v4l2_meta_format,
    pub isys: *mut ipu6_isys,
    pub csi2: *mut ipu6_isys_csi2,
    pub stream: *mut ipu6_isys_stream,
    pub streaming: c_uint,
    pub watermark: video_stream_watermark,
    pub source_stream: u32,
    pub vc: u8,
    pub dt: u8,
}

extern "C" {
    pub fn ipu6_isys_fw_open(isys: *mut ipu6_isys) -> c_int;
}
extern "C" {
    pub fn ipu6_isys_fw_close(isys: *mut ipu6_isys);
}
extern "C" {
    pub fn ipu6_isys_video_init(av: *mut ipu6_isys_video) -> c_int;
}
extern "C" {
    pub fn ipu6_isys_video_cleanup(av: *mut ipu6_isys_video);
}
extern "C" {
    pub fn ipu6_isys_put_stream(stream: *mut ipu6_isys_stream);
}
extern "C" {
    pub fn ipu6_isys_update_stream_watermark(av: *mut ipu6_isys_video, state: bool);
}
extern "C" {
    pub fn ipu6_isys_get_format(av: *mut ipu6_isys_video) -> u32;
}
extern "C" {
    pub fn ipu6_isys_get_data_size(av: *mut ipu6_isys_video) -> u32;
}
extern "C" {
    pub fn ipu6_isys_get_bytes_per_line(av: *mut ipu6_isys_video) -> u32;
}
extern "C" {
    pub fn ipu6_isys_get_frame_width(av: *mut ipu6_isys_video) -> u32;
}
extern "C" {
    pub fn ipu6_isys_get_frame_height(av: *mut ipu6_isys_video) -> u32;
}

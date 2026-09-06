//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_histo.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// vsp1_histo.h  --  R-Car VSP1 Histogram API
//
// Copyright (C) 2016 Renesas Electronics Corporation
// Copyright (C) 2016 Laurent Pinchart
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

pub const HISTO_PAD_SINK: c_int = 0;
pub const HISTO_PAD_SOURCE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_histogram_buffer {
    pub buf: vb2_v4l2_buffer,
    pub queue: list_head,
    pub addr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_histogram {
    pub entity: vsp1_entity,
    pub video: video_device,
    pub pad: media_pad,
    pub data_size: usize,
    pub meta_format: u32,
    pub lock: mutex,
    pub queue: vb2_queue,
    pub irqlock: spinlock_t,
    pub irqqueue: list_head,
    pub wait_queue: wait_queue_head_t,
    pub readout: bool,
}

extern "C" {
    pub fn container_of(_arg: vdev, vsp1_histogram: struct, _arg: video) -> return;
}
extern "C" {
    pub fn container_of(_arg: subdev, vsp1_histogram: struct, _arg: entity.subdev) -> return;
}
extern "C" {
    pub fn vsp1_histogram_destroy(entity: *mut vsp1_entity);
}

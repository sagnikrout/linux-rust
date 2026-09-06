//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1_video.h
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
// vsp1_video.h  --  R-Car VSP1 Video Node
//
// Copyright (C) 2013-2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_vb2_buffer {
    pub buf: vb2_v4l2_buffer,
    pub queue: list_head,
    pub mem: vsp1_rwpf_memory,
}

extern "C" {
    pub fn container_of(_arg: vbuf, vsp1_vb2_buffer: struct, _arg: buf) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_video {
    pub list: list_head,
    pub vsp1: *mut vsp1_device,
    pub rwpf: *mut vsp1_rwpf,
    pub video: video_device,
    pub type: v4l2_buf_type,
    pub pad: media_pad,
    pub lock: mutex,
    pub pipe_index: c_uint,
    pub queue: vb2_queue,
    pub irqlock: spinlock_t,
    pub irqqueue: list_head,
}

extern "C" {
    pub fn container_of(_arg: vdev, vsp1_video: struct, _arg: video) -> return;
}
extern "C" {
    pub fn vsp1_video_suspend(vsp1: *mut vsp1_device);
}
extern "C" {
    pub fn vsp1_video_resume(vsp1: *mut vsp1_device);
}
extern "C" {
    pub fn vsp1_video_cleanup(video: *mut vsp1_video);
}

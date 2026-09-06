//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/xilinx/xilinx-vipp.h
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
// Xilinx Video IP Composite Device
//
// Copyright (C) 2013-2015 Ideas on Board
// Copyright (C) 2013-2015 Xilinx, Inc.
//
// Contacts: Hyun Kwon <hyun.kwon@xilinx.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

//
// struct xvip_composite_device - Xilinx Video IP device structure
// @v4l2_dev: V4L2 device
// @media_dev: media device
// @dev: (OF) device
// @notifier: V4L2 asynchronous subdevs notifier
// @dmas: list of DMA channels at the pipeline output and input
// @v4l2_caps: V4L2 capabilities of the whole device (see VIDIOC_QUERYCAP)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xvip_composite_device {
    pub v4l2_dev: v4l2_device,
    pub media_dev: media_device,
    pub dev: *mut device,
    pub notifier: v4l2_async_notifier,
    pub dmas: list_head,
    pub v4l2_caps: u32,
}

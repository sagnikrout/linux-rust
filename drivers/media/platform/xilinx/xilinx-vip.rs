//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/xilinx/xilinx-vip.h
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
// Xilinx Video IP Core
//
// Copyright (C) 2013-2015 Ideas on Board
// Copyright (C) 2013-2015 Xilinx, Inc.
//
// Contacts: Hyun Kwon <hyun.kwon@xilinx.com>
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

//
// Minimum and maximum width and height common to most video IP cores. IP
// cores with different requirements must define their own values.
//
pub const XVIP_MIN_WIDTH: c_int = 32;
pub const XVIP_MAX_WIDTH: c_int = 7680;
pub const XVIP_MIN_HEIGHT: c_int = 32;
pub const XVIP_MAX_HEIGHT: c_int = 7680;
//
// Pad IDs. IP cores with multiple inputs or outputs should define their own
// values.
//
pub const XVIP_PAD_SINK: c_int = 0;
pub const XVIP_PAD_SOURCE: c_int = 1;
// Xilinx Video IP Control Registers
pub const XVIP_CTRL_CONTROL: c_uint = 0x0000;

pub const XVIP_CTRL_STATUS: c_uint = 0x0004;

pub const XVIP_CTRL_ERROR: c_uint = 0x0008;

pub const XVIP_CTRL_IRQ_ENABLE: c_uint = 0x000c;

pub const XVIP_CTRL_VERSION: c_uint = 0x0010;

pub const XVIP_CTRL_VERSION_MAJOR_SHIFT: c_int = 24;

pub const XVIP_CTRL_VERSION_MINOR_SHIFT: c_int = 16;

pub const XVIP_CTRL_VERSION_REVISION_SHIFT: c_int = 12;

pub const XVIP_CTRL_VERSION_PATCH_SHIFT: c_int = 8;

pub const XVIP_CTRL_VERSION_INTERNAL_SHIFT: c_int = 0;
// Xilinx Video IP Timing Registers
pub const XVIP_ACTIVE_SIZE: c_uint = 0x0020;

pub const XVIP_ACTIVE_VSIZE_SHIFT: c_int = 16;

pub const XVIP_ACTIVE_HSIZE_SHIFT: c_int = 0;
pub const XVIP_ENCODING: c_uint = 0x0028;

pub const XVIP_ENCODING_NBITS_SHIFT: c_int = 4;

pub const XVIP_ENCODING_VIDEO_FORMAT_SHIFT: c_int = 0;
//
// struct xvip_device - Xilinx Video IP device structure
// @subdev: V4L2 subdevice
// @dev: (OF) device
// @iomem: device I/O register space remapped to kernel virtual memory
// @clk: video core clock
// @saved_ctrl: saved control register for resume / suspend
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xvip_device {
    pub subdev: v4l2_subdev,
    pub dev: *mut device,
    pub iomem: *mut void __iomem,
    pub clk: *mut clk,
    pub saved_ctrl: u32,
}

//
// struct xvip_video_format - Xilinx Video IP video format description
// @vf_code: AXI4 video format code
// @width: AXI4 format width in bits per component
// @pattern: CFA pattern for Mono/Sensor formats
// @code: media bus format code
// @bpp: bytes per pixel (when stored in memory)
// @fourcc: V4L2 pixel format FCC identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xvip_video_format {
    pub vf_code: c_uint,
    pub width: c_uint,
    pub pattern: *const c_char,
    pub code: c_uint,
    pub bpp: c_uint,
    pub fourcc: u32,
}

extern "C" {
    pub fn ioread32(addr: xvip->iomem +) -> return;
}
extern "C" {
    pub fn xvip_clr_or_set(xvip: *mut xvip_device, addr: u32, mask: u32, set: bool);
}
extern "C" {
    pub fn xvip_clr_and_set(xvip: *mut xvip_device, addr: u32, clr: u32, set: u32);
}
extern "C" {
    pub fn xvip_init_resources(xvip: *mut xvip_device) -> c_int;
}
extern "C" {
    pub fn xvip_cleanup_resources(xvip: *mut xvip_device);
}

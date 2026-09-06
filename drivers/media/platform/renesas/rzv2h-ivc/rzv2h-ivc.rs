//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/rzv2h-ivc/rzv2h-ivc.h
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
// Renesas RZ/V2H(P) Input Video Control Block driver
//
// Copyright (C) 2025 Ideas on Board Oy
//

pub const RZV2H_IVC_REG_AXIRX_PLNUM: c_uint = 0x0000;
pub const RZV2H_IVC_ONE_EXPOSURE: c_uint = 0x00;
pub const RZV2H_IVC_TWO_EXPOSURE: c_uint = 0x01;
pub const RZV2H_IVC_REG_AXIRX_PXFMT: c_uint = 0x0004;

pub const RZV2H_IVC_CLFMT_MIPI: c_int = 0;
pub const RZV2H_IVC_CLFMT_CRU_PACKED: c_int = 1;

pub const RZV2H_IVC_REG_AXIRX_SADDL_P0: c_uint = 0x0010;
pub const RZV2H_IVC_REG_AXIRX_SADDH_P0: c_uint = 0x0014;
pub const RZV2H_IVC_REG_AXIRX_SADDL_P1: c_uint = 0x0018;
pub const RZV2H_IVC_REG_AXIRX_SADDH_P1: c_uint = 0x001c;
pub const RZV2H_IVC_REG_AXIRX_HSIZE: c_uint = 0x0020;
pub const RZV2H_IVC_REG_AXIRX_VSIZE: c_uint = 0x0024;
pub const RZV2H_IVC_REG_AXIRX_BLANK: c_uint = 0x0028;

pub const RZV2H_IVC_REG_AXIRX_STRD: c_uint = 0x0030;
pub const RZV2H_IVC_REG_AXIRX_ISSU: c_uint = 0x0040;
pub const RZV2H_IVC_REG_AXIRX_ERACT: c_uint = 0x0048;
pub const RZV2H_IVC_REG_FM_CONTEXT: c_uint = 0x0100;
pub const RZV2H_IVC_SOFTWARE_CFG: c_uint = 0x00;

pub const RZV2H_IVC_REG_FM_MCON: c_uint = 0x0104;
pub const RZV2H_IVC_REG_FM_FRCON: c_uint = 0x0108;
pub const RZV2H_IVC_REG_FM_STOP: c_uint = 0x010c;

pub const RZV2H_IVC_REG_FM_INT_EN: c_uint = 0x0120;

pub const RZV2H_IVC_REG_FM_INT_STA: c_uint = 0x0124;
pub const RZV2H_IVC_REG_AXIRX_FIFOCAP0: c_uint = 0x0208;
pub const RZV2H_IVC_REG_CORE_CAPCON: c_uint = 0x020c;
pub const RZV2H_IVC_REG_CORE_FIFOCAP0: c_uint = 0x0228;
pub const RZV2H_IVC_REG_CORE_FIFOCAP1: c_uint = 0x022c;
pub const RZV2H_IVC_MIN_WIDTH: c_int = 640;
pub const RZV2H_IVC_MAX_WIDTH: c_int = 4096;
pub const RZV2H_IVC_MIN_HEIGHT: c_int = 480;
pub const RZV2H_IVC_MAX_HEIGHT: c_int = 4096;
pub const RZV2H_IVC_DEFAULT_WIDTH: c_int = 1920;
pub const RZV2H_IVC_DEFAULT_HEIGHT: c_int = 1080;
pub const RZV2H_IVC_NUM_HW_RESOURCES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rzv2h_ivc_subdev_pads {
    RZV2H_IVC_SUBDEV_SINK_PAD,
    RZV2H_IVC_SUBDEV_SOURCE_PAD,
    RZV2H_IVC_NUM_SUBDEV_PADS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_ivc_format {
    pub fourcc: u32,
//
// The CRU packed pixel formats are bayer-order agnostic, so each could
// support any one of the 4 possible media bus formats.
//
    pub mbus_codes: [u32; 4],
    pub dtype: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_ivc {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub clks: [clk_bulk_data; RZV2H_IVC_NUM_HW_RESOURCES],
    pub resets: [reset_control_bulk_data; RZV2H_IVC_NUM_HW_RESOURCES],
    pub irqnum: c_int,
    pub vvalid_ifp: u8,
    pub dev: video_device,
    pub vb2q: vb2_queue,
    pub pad: media_pad,
    pub vdev: },
    pub sd: v4l2_subdev,
    pub pads: [media_pad; RZV2H_IVC_NUM_SUBDEV_PADS],
    pub subdev: },
// Spinlock to guard buffer queue
    pub lock: spinlock_t,
    pub queue: list_head,
    pub curr: *mut rzv2h_ivc_buf,
    pub sequence: c_uint,
    pub buffers: },
    pub pix: v4l2_pix_format_mplane,
    pub fmt: *const rzv2h_ivc_format,
    pub format: },
// Mutex to provide to vb2
    pub lock: mutex,
// Lock to protect the interrupt counter
    pub spinlock: spinlock_t,
}

extern "C" {
    pub fn rzv2h_ivc_init_vdev(ivc: *mut rzv2h_ivc, v4l2_dev: *mut v4l2_device) -> c_int;
}
extern "C" {
    pub fn rzv2h_deinit_video_dev_and_queue(ivc: *mut rzv2h_ivc);
}
extern "C" {
    pub fn rzv2h_ivc_buffer_done(ivc: *mut rzv2h_ivc);
}
extern "C" {
    pub fn rzv2h_ivc_initialise_subdevice(ivc: *mut rzv2h_ivc) -> c_int;
}
extern "C" {
    pub fn rzv2h_ivc_deinit_subdevice(ivc: *mut rzv2h_ivc);
}
extern "C" {
    pub fn rzv2h_ivc_write(ivc: *mut rzv2h_ivc, addr: u32, val: u32);
}
extern "C" {
    pub fn rzv2h_ivc_transfer_buffer(ivc: *mut rzv2h_ivc);
}

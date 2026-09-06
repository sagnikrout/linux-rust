//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun4i-csi/sun4i_csi.h
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
// Copyright (C) 2016 NextThing Co
// Copyright (C) 2016-2019 Bootlin
//
// Author: Maxime Ripard <maxime.ripard@bootlin.com>
//

pub const CSI_EN_REG: c_uint = 0x00;
pub const CSI_CFG_REG: c_uint = 0x04;

pub const CSI_CPT_CTRL_REG: c_uint = 0x08;

pub const CSI_BUF_CTRL_REG: c_uint = 0x28;

pub const CSI_INT_EN_REG: c_uint = 0x30;

pub const CSI_INT_STA_REG: c_uint = 0x34;
pub const CSI_WIN_CTRL_W_REG: c_uint = 0x40;

pub const CSI_WIN_CTRL_H_REG: c_uint = 0x44;

pub const CSI_BUF_LEN_REG: c_uint = 0x48;
pub const CSI_MAX_BUFFER: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi_input {
    CSI_INPUT_RAW	= 0,
    CSI_INPUT_BT656	= 2,
    CSI_INPUT_YUV	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi_output_raw {
    CSI_OUTPUT_RAW_PASSTHROUGH = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi_output_yuv {
    CSI_OUTPUT_YUV_422_PLANAR	= 0,
    CSI_OUTPUT_YUV_420_PLANAR	= 1,
    CSI_OUTPUT_YUV_422_UV		= 4,
    CSI_OUTPUT_YUV_420_UV		= 5,
    CSI_OUTPUT_YUV_422_MACRO	= 8,
    CSI_OUTPUT_YUV_420_MACRO	= 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi_yuv_data_seq {
    CSI_YUV_DATA_SEQ_YUYV	= 0,
    CSI_YUV_DATA_SEQ_YVYU	= 1,
    CSI_YUV_DATA_SEQ_UYVY	= 2,
    CSI_YUV_DATA_SEQ_VYUY	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum csi_subdev_pads {
    CSI_SUBDEV_SINK,
    CSI_SUBDEV_SOURCE,

    CSI_SUBDEV_PADS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_csi_format {
    pub mbus: u32,
    pub fourcc: u32,
    pub input: csi_input,
    pub output: u32,
    pub num_planes: c_uint,
    pub bpp: [u8; 3],
    pub hsub: c_uint,
    pub vsub: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_csi {
// Device resources
    pub dev: *mut device,
    pub traits: *const sun4i_csi_traits,
    pub regs: *mut void __iomem,
    pub bus_clk: *mut clk,
    pub isp_clk: *mut clk,
    pub ram_clk: *mut clk,
    pub rst: *mut reset_control,
    pub current_buf: [*mut vb2_v4l2_buffer; CSI_MAX_BUFFER],
    pub size: usize,
    pub vaddr: *mut c_void,
    pub paddr: dma_addr_t,
    pub scratch: },
    pub bus: v4l2_mbus_config_parallel,
// Main Device
    pub v4l: v4l2_device,
    pub mdev: media_device,
    pub vdev: video_device,
    pub vdev_pad: media_pad,
    pub fmt: v4l2_pix_format_mplane,
// Local subdev
    pub subdev: v4l2_subdev,
    pub subdev_pads: [media_pad; CSI_SUBDEV_PADS],
    pub subdev_fmt: v4l2_mbus_framefmt,
// V4L2 Async variables
    pub notifier: v4l2_async_notifier,
    pub src_subdev: *mut v4l2_subdev,
    pub src_pad: c_int,
// V4L2 variables
    pub lock: mutex,
// Videobuf2
    pub queue: vb2_queue,
    pub buf_list: list_head,
    pub qlock: spinlock_t,
    pub sequence: c_uint,
}

extern "C" {
    pub fn sun4i_csi_dma_register(csi: *mut sun4i_csi, irq: c_int) -> c_int;
}
extern "C" {
    pub fn sun4i_csi_dma_unregister(csi: *mut sun4i_csi);
}
extern "C" {
    pub fn sun4i_csi_v4l2_register(csi: *mut sun4i_csi) -> c_int;
}

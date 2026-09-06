//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/rzg2l-cru/rzg2l-cru.h
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
// Driver for Renesas RZ/G2L CRU
//
// Copyright (C) 2022 Renesas Electronics Corp.
//

// Number of HW buffers
pub const RZG2L_CRU_HW_BUFFER_MAX: c_int = 8;
pub const RZG2L_CRU_HW_BUFFER_DEFAULT: c_int = 3;
// Address alignment mask for HW buffers
pub const RZG2L_CRU_HW_BUFFER_MASK: c_uint = 0x1ff;
// Maximum number of CSI2 virtual channels
pub const RZG2L_CRU_CSI2_VCHANNEL: c_int = 4;
pub const RZG2L_CRU_MIN_INPUT_WIDTH: c_int = 320;
pub const RZG2L_CRU_MIN_INPUT_HEIGHT: c_int = 240;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rzg2l_csi2_pads {
    RZG2L_CRU_IP_SINK = 0,
    RZG2L_CRU_IP_SOURCE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_cru_csi {
    pub asd: *mut v4l2_async_connection,
    pub subdev: *mut v4l2_subdev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_cru_ip {
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; 2],
    pub notifier: v4l2_async_notifier,
    pub remote: *mut v4l2_subdev,
}

//
// struct rzg2l_cru_ip_format - CRU IP format
// @codes: Array of up to four media bus codes
// @datatype: MIPI CSI2 data type
// @format: 4CC format identifier (V4L2_PIX_FMT_*)
// @icndmr: ICnDMR register value
// @yuv: Flag to indicate whether the format is YUV-based.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_cru_ip_format {
//
// RAW output formats might be produced by RAW media codes with any one
// of the 4 common bayer patterns.
//
    pub codes: [u32; 4],
    pub datatype: u32,
    pub format: u32,
    pub icndmr: u32,
    pub yuv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_cru_info {
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub image_conv: u16,
    pub regs: *const u16,
    pub has_stride: bool,
    pub data): *mut *mut irqreturn_t (irq_handler)(int irq, void,
    pub cru): *mut *mut void (enable_interrupts)(struct rzg2l_cru_dev,
    pub cru): *mut *mut void (disable_interrupts)(struct rzg2l_cru_dev,
    pub cru): *mut *mut bool (fifo_empty)(struct rzg2l_cru_dev,
}

//
// struct rzg2l_cru_dev - Renesas CRU device structure
// @dev:		(OF) device
// @base:		device I/O register space remapped to virtual memory
// @info:		info about CRU instance
//
// @presetn:		CRU_PRESETN reset line
// @aresetn:		CRU_ARESETN reset line
//
// @vclk:		CRU Main clock
//
// @vdev:		V4L2 video device associated with CRU
// @v4l2_dev:		V4L2 device
// @num_buf:		Holds the current number of buffers enabled
// @svc_channel:	SVC0/1/2/3 to use for RZ/G3E
// @notifier:		V4L2 asynchronous subdevs notifier
//
// @ip:			Image processing subdev info
// @csi:		CSI info
// @mdev:		media device
// @mdev_lock:		protects the count, notifier and csi members
// @pad:		media pad for the video device entity
//
// @hw_lock:		protects the @active_slot counter, hardware programming
// of slot addresses and the @buf_addr[] list
// @buf_addr:		Memory addresses where current video data is written
// @active_slot:	The slot in use
//
// @lock:		protects @queue
// @queue:		vb2 buffers queue
// @scratch:		cpu address for scratch buffer
// @scratch_phys:	physical address of the scratch buffer
//
// @qlock:		protects @queue_buf, @buf_list, @sequence
// @queue_buf:		Keeps track of buffers given to HW slot
// @buf_list:		list of queued buffers
// @sequence:		V4L2 buffers sequence number
//
// @format:		active V4L2 pixel format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_cru_dev {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub info: *const rzg2l_cru_info,
    pub presetn: *mut reset_control,
    pub aresetn: *mut reset_control,
    pub vclk: *mut clk,
    pub vdev: video_device,
    pub v4l2_dev: v4l2_device,
    pub num_buf: u8,
    pub svc_channel: u8,
    pub notifier: v4l2_async_notifier,
    pub ip: rzg2l_cru_ip,
    pub csi: rzg2l_cru_csi,
    pub mdev: media_device,
    pub mdev_lock: mutex,
    pub pad: media_pad,
    pub hw_lock: spinlock_t,
    pub buf_addr: [dma_addr_t; RZG2L_CRU_HW_BUFFER_DEFAULT],
    pub active_slot: c_uint,
    pub lock: mutex,
    pub queue: vb2_queue,
    pub scratch: *mut c_void,
    pub scratch_phys: dma_addr_t,
    pub qlock: spinlock_t,
    pub queue_buf: [*mut vb2_v4l2_buffer; RZG2L_CRU_HW_BUFFER_MAX],
    pub buf_list: list_head,
    pub sequence: c_uint,
    pub format: v4l2_pix_format,
}

extern "C" {
    pub fn rzg2l_cru_start_image_processing(cru: *mut rzg2l_cru_dev) -> c_int;
}
extern "C" {
    pub fn rzg2l_cru_stop_image_processing(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg2l_cru_dma_register(cru: *mut rzg2l_cru_dev) -> c_int;
}
extern "C" {
    pub fn rzg2l_cru_dma_unregister(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg2l_cru_video_register(cru: *mut rzg2l_cru_dev) -> c_int;
}
extern "C" {
    pub fn rzg2l_cru_video_unregister(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg2l_cru_irq(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn rzg3e_cru_irq(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn rzg2l_cru_ip_subdev_register(cru: *mut rzg2l_cru_dev) -> c_int;
}
extern "C" {
    pub fn rzg2l_cru_ip_subdev_unregister(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg2l_cru_enable_interrupts(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg2l_cru_disable_interrupts(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg3e_cru_enable_interrupts(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg3e_cru_disable_interrupts(cru: *mut rzg2l_cru_dev);
}
extern "C" {
    pub fn rzg2l_fifo_empty(cru: *mut rzg2l_cru_dev) -> bool;
}
extern "C" {
    pub fn rzg3e_fifo_empty(cru: *mut rzg2l_cru_dev) -> bool;
}

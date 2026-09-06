//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/rcar-vin/rcar-vin.h
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
// Driver for Renesas R-Car VIN
//
// Copyright (C) 2025 Niklas Söderlund <niklas.soderlund@ragnatech.se>
// Copyright (C) 2016 Renesas Electronics Corp.
// Copyright (C) 2011-2013 Renesas Solutions Corp.
// Copyright (C) 2013 Cogent Embedded, Inc., <source@cogentembedded.com>
// Copyright (C) 2008 Magnus Damm
//

// Number of HW buffers
pub const HW_BUFFER_NUM: c_int = 3;
// Address alignment mask for HW buffers
pub const HW_BUFFER_MASK: c_uint = 0x7f;
// Max number on VIN instances that can be in a system
pub const RCAR_VIN_NUM: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum model_id {
    RCAR_H1,
    RCAR_M1,
    RCAR_GEN2,
    RCAR_GEN3,
    RCAR_GEN4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvin_csi_id {
    RVIN_CSI20,
    RVIN_CSI21,
    RVIN_CSI40,
    RVIN_CSI41,
    RVIN_CSI_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvin_isp_id {
    RVIN_ISP0,
    RVIN_ISP1,
    RVIN_ISP2,
    RVIN_ISP4,
    RVIN_ISP_MAX,
}

//
// struct rvin_video_format - Data format stored in memory
// @fourcc:	Pixelformat
// @bpp:	Bytes per pixel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvin_video_format {
    pub fourcc: u32,
    pub bpp: u8,
}

//
// struct rvin_parallel_entity - Parallel video input endpoint descriptor
// @asc:	async connection descriptor for async framework
// @subdev:	subdevice matched using async framework
// @mbus_type:	media bus type
// @bus:	media bus parallel configuration
// @source_pad:	source pad of remote subdevice
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvin_parallel_entity {
    pub asc: *mut v4l2_async_connection,
    pub subdev: *mut v4l2_subdev,
    pub mbus_type: v4l2_mbus_type,
    pub bus: v4l2_mbus_config_parallel,
    pub source_pad: c_uint,
}

//
// struct rvin_group_route - describes a route from a channel of a
// CSI-2 receiver to a VIN
//
// @master:	VIN group master ID.
// @csi:	CSI-2 receiver ID.
// @chsel:	CHSEL register values that connects VIN group to CSI-2.
//
// .. note::
// Each R-Car CSI-2 receiver has four output channels facing the VIN
// devices, each channel can carry one CSI-2 Virtual Channel (VC).
// There is no correlation between channel number and CSI-2 VC. It's
// up to the CSI-2 receiver driver to configure which VC is output
// on which channel, the VIN devices only care about output channels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvin_group_route {
    pub master: c_uint,
    pub csi: rvin_csi_id,
    pub chsel: c_uint,
}

//
// struct rvin_info - Information about the particular VIN implementation
// @model:		VIN model
// @use_isp:		the VIN is connected to the ISP and not to the CSI-2
// @nv12:		support outputting NV12 pixel format
// @raw10:		support outputting RAW10 pixel format
// @max_width:		max input width the VIN supports
// @max_height:		max input height the VIN supports
// @routes:		list of possible routes from the CSI-2 recivers to
// all VINs. The list mush be NULL terminated.
// @scaler:		Optional scaler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvin_info {
    pub model: model_id,
    pub use_isp: bool,
    pub nv12: bool,
    pub raw10: bool,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub routes: *const rvin_group_route,
    pub vin): *mut *mut void (scaler)(struct rvin_dev,
}

//
// struct rvin_dev - Renesas VIN device structure
// @dev:		(OF) device
// @base:		device I/O register space remapped to virtual memory
// @info:		info about VIN instance
//
// @vdev:		V4L2 video device associated with VIN
// @v4l2_dev:		V4L2 device
// @ctrl_handler:	V4L2 control handler
//
// @parallel:		parallel input subdevice descriptor
//
// @group:		Gen3 CSI group
// @id:			Gen3 group id for this VIN
// @pad:		media pad for the video device entity
//
// @lock:		protects @queue
// @queue:		vb2 buffers queue
// @scratch:		cpu address for scratch buffer
// @scratch_phys:	physical address of the scratch buffer
//
// @qlock:		Protects @buf_hw, @buf_list, @sequence and @running
// @buf_hw:		Keeps track of buffers given to HW slot
// @buf_list:		list of queued buffers
// @sequence:		V4L2 buffers sequence number
// @running:		Keeps track of if the VIN is running
//
// @is_csi:		flag to mark the VIN as using a CSI-2 subdevice
// @chsel:		Cached value of the current CSI-2 channel selection
//
// @mbus_code:		media bus format code
// @format:		active V4L2 pixel format
//
// @crop:		active cropping
// @compose:		active composing
// @scaler:		Optional scaler
//
// @alpha:		Alpha component to fill in for supported pixel formats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvin_dev {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub info: *const rvin_info,
    pub vdev: video_device,
    pub v4l2_dev: v4l2_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub parallel: rvin_parallel_entity,
    pub group: *mut rvin_group,
    pub id: c_uint,
    pub pad: media_pad,
    pub lock: mutex,
    pub queue: vb2_queue,
    pub scratch: *mut c_void,
    pub scratch_phys: dma_addr_t,
    pub qlock: spinlock_t,
    pub buffer: *mut vb2_v4l2_buffer,
    pub phys: dma_addr_t,
    pub buf_hw: [}; HW_BUFFER_NUM],
    pub buf_list: list_head,
    pub sequence: c_uint,
    pub running: bool,
    pub is_csi: bool,
    pub chsel: c_uint,
    pub mbus_code: u32,
    pub format: v4l2_pix_format,
    pub crop: v4l2_rect,
    pub compose: v4l2_rect,
    pub vin): *mut *mut void (scaler)(struct rvin_dev,
    pub alpha: c_uint,
}

// Debug

//
// struct rvin_group - VIN CSI2 group information
// @refcount:		number of VIN instances using the group
//
// @mdev:		media device which represents the group
//
// @lock:		protects the count, notifier, vin and csi members
// @count:		number of enabled VIN instances found in DT
// @notifier:		group notifier for CSI-2 async connections
// @info:		Platform dependent information about the VIN instances
// @vin:		VIN instances which are part of the group
// @link_setup:		Callback to create all links for the media graph
// @remotes:		array of pairs of async connection and subdev pointers
// to all remote subdevices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rvin_group {
    pub refcount: kref,
    pub mdev: media_device,
    pub lock: mutex,
    pub count: c_uint,
    pub notifier: v4l2_async_notifier,
    pub info: *const rvin_info,
    pub vin: [*mut rvin_dev; RCAR_VIN_NUM],
    pub group): *mut *mut int (link_setup)(struct rvin_group,
    pub asc: *mut v4l2_async_connection,
    pub subdev: *mut v4l2_subdev,
    pub remotes: [}; RVIN_REMOTES_MAX],
}

extern "C" {
    pub fn rvin_dma_register(vin: *mut rvin_dev, irq: c_int) -> c_int;
}
extern "C" {
    pub fn rvin_dma_unregister(vin: *mut rvin_dev);
}
extern "C" {
    pub fn rvin_v4l2_register(vin: *mut rvin_dev) -> c_int;
}
extern "C" {
    pub fn rvin_v4l2_unregister(vin: *mut rvin_dev);
}
// Cropping, composing and scaling
extern "C" {
    pub fn rvin_scaler_gen2(vin: *mut rvin_dev);
}
extern "C" {
    pub fn rvin_scaler_gen3(vin: *mut rvin_dev);
}
extern "C" {
    pub fn rvin_crop_scale_comp(vin: *mut rvin_dev);
}
extern "C" {
    pub fn rvin_set_channel_routing(vin: *mut rvin_dev, chsel: u8) -> c_int;
}
extern "C" {
    pub fn rvin_set_alpha(vin: *mut rvin_dev, alpha: c_uint);
}
extern "C" {
    pub fn rvin_start_streaming(vin: *mut rvin_dev) -> c_int;
}
extern "C" {
    pub fn rvin_stop_streaming(vin: *mut rvin_dev);
}

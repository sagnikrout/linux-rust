//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/stm32/stm32-dcmipp/dcmipp-common.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for STM32 Digital Camera Memory Interface Pixel Processor
//
// Copyright (C) STMicroelectronics SA 2023
// Authors: Hugues Fruchet <hugues.fruchet@foss.st.com>
// Alain Volmat <alain.volmat@foss.st.com>
// for STMicroelectronics.
//

pub const DCMIPP_FRAME_MAX_WIDTH: c_int = 4096;
pub const DCMIPP_FRAME_MAX_HEIGHT: c_int = 2160;
pub const DCMIPP_FRAME_MIN_WIDTH: c_int = 16;
pub const DCMIPP_FRAME_MIN_HEIGHT: c_int = 16;
pub const DCMIPP_FMT_WIDTH_DEFAULT: c_int = 640;
pub const DCMIPP_FMT_HEIGHT_DEFAULT: c_int = 480;

//
// dcmipp_colorimetry_clamp() - Adjust colorimetry parameters
//
// @fmt:		the pointer to struct v4l2_pix_format or
// struct v4l2_mbus_framefmt
//
// Entities must check if colorimetry given by the userspace is valid, if not
// then set them as DEFAULT
//

//
// struct dcmipp_ent_device - core struct that represents a node in the topology
//
// @ent:		the pointer to struct media_entity for the node
// @pads:		the list of pads of the node
// @bus:		struct v4l2_mbus_config_parallel describing input bus
// @bus_type:		type of input bus (parallel or BT656)
// @handler:		irq handler dedicated to the subdev
// @handler_ret:	value returned by the irq handler
// @thread_fn:		threaded irq handler
//
// The DCMIPP provides a single IRQ line and a IRQ status registers for all
// subdevs, hence once the main irq handler (registered at probe time) is
// called, it will chain calls to the irq handler of each the subdevs of the
// pipelines, using the handler/handler_ret/thread_fn variables.
//
// Each node of the topology must create a dcmipp_ent_device struct.
// Depending on the node it will be of an instance of v4l2_subdev or
// video_device struct where both contains a struct media_entity.
// Those structures should embedded the dcmipp_ent_device struct through
// v4l2_set_subdevdata() and video_set_drvdata() respectivaly, allowing the
// dcmipp_ent_device struct to be retrieved from the corresponding struct
// media_entity
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcmipp_ent_device {
    pub ent: *mut media_entity,
    pub pads: *mut media_pad,
// Parallel input device
    pub bus: v4l2_mbus_config_parallel,
    pub bus_type: v4l2_mbus_type,
    pub handler: irq_handler_t,
    pub handler_ret: irqreturn_t,
    pub thread_fn: irq_handler_t,
}

//
// dcmipp_pads_init - initialize pads
//
// @num_pads:	number of pads to initialize
// @pads_flags:	flags to use in each pad
//
// Helper functions to allocate/initialize pads
//
// dcmipp_pads_cleanup - free pads
//
// @pads: pointer to the pads
//
// Helper function to free the pads initialized with dcmipp_pads_init
//
// dcmipp_ent_sd_register - initialize and register a subdev node
//
// @ved:	the dcmipp_ent_device struct to be initialize
// @sd:		the v4l2_subdev struct to be initialize and registered
// @v4l2_dev:	the v4l2 device to register the v4l2_subdev
// @name:	name of the sub-device. Please notice that the name must be
// unique.
// @function:	media entity function defined by MEDIA_ENT_F_* macros
// @num_pads:	number of pads to initialize
// @pads_flag:	flags to use in each pad
// @sd_int_ops:	pointer to &struct v4l2_subdev_internal_ops
// @sd_ops:	pointer to &struct v4l2_subdev_ops.
// @handler:	func pointer of the irq handler
// @thread_fn:	func pointer of the threaded irq handler
//
// Helper function initialize and register the struct dcmipp_ent_device and
// struct v4l2_subdev which represents a subdev node in the topology
//
// dcmipp_ent_sd_unregister - cleanup and unregister a subdev node
//
// @ved:	the dcmipp_ent_device struct to be cleaned up
// @sd:		the v4l2_subdev struct to be unregistered
//
// Helper function cleanup and unregister the struct dcmipp_ent_device and
// struct v4l2_subdev which represents a subdev node in the topology
//

// DCMIPP subdev init / release entry points
extern "C" {
    pub fn dcmipp_inp_ent_release(ved: *mut dcmipp_ent_device);
}
extern "C" {
    pub fn dcmipp_byteproc_ent_release(ved: *mut dcmipp_ent_device);
}
extern "C" {
    pub fn dcmipp_bytecap_ent_release(ved: *mut dcmipp_ent_device);
}

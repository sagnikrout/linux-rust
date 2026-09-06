//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vimc/vimc-common.h
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
// vimc-common.h Virtual Media Controller Driver
//
// Copyright (C) 2015-2017 Helen Koike <helen.fornazier@gmail.com>
//

// VIMC-specific controls

pub const VIMC_FRAME_MAX_WIDTH: c_int = 4096;
pub const VIMC_FRAME_MAX_HEIGHT: c_int = 2160;
pub const VIMC_FRAME_MIN_WIDTH: c_int = 16;
pub const VIMC_FRAME_MIN_HEIGHT: c_int = 16;

pub const VIMC_HBLANK_FIXED: c_int = 800;
// VBLANK - vertical blanking (primary FPS control)
pub const VIMC_VBLANK_MIN: c_int = 4;
pub const VIMC_VBLANK_MAX: c_int = 65535;
pub const VIMC_VBLANK_STEP: c_int = 1;

// Source and sink pad checks

pub const VIMC_PIX_FMT_MAX_CODES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vimc_allocator_type {
    VIMC_ALLOCATOR_VMALLOC = 0,
    VIMC_ALLOCATOR_DMA_CONTIG = 1,
}

//
// vimc_colorimetry_clamp - Adjust colorimetry parameters
//
// @fmt:		the pointer to struct v4l2_pix_format or
// struct v4l2_mbus_framefmt
//
// Entities must check if colorimetry given by the userspace is valid, if not
// then set them as DEFAULT
//

//
// struct vimc_pix_map - maps media bus code with v4l2 pixel format
//
// @code:		media bus format code defined by MEDIA_BUS_FMT_* macros
// @bpp:		number of bytes each pixel occupies
// @pixelformat:	pixel format defined by V4L2_PIX_FMT_* macros
// @bayer:		true if this is a bayer format
//
// Struct which matches the MEDIA_BUS_FMT_* codes with the corresponding
// V4L2_PIX_FMT_* fourcc pixelformat and its bytes per pixel (bpp)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_pix_map {
    pub code: [c_uint; VIMC_PIX_FMT_MAX_CODES],
    pub bpp: c_uint,
    pub pixelformat: u32,
    pub bayer: bool,
}

//
// struct vimc_ent_device - core struct that represents an entity in the
// topology
//
// @dev:		a pointer of the device struct of the driver
// @ent:		the pointer to struct media_entity for the node
// @process_frame:	callback send a frame to that node
// @vdev_get_format:	callback that returns the current format a pad, used
// only when is_media_entity_v4l2_video_device(ent) returns
// true
//
// Each node of the topology must create a vimc_ent_device struct. Depending on
// the node it will be of an instance of v4l2_subdev or video_device struct
// where both contains a struct media_entity.
// Those structures should embedded the vimc_ent_device struct through
// v4l2_set_subdevdata() and video_set_drvdata() respectively, allowing the
// vimc_ent_device struct to be retrieved from the corresponding struct
// media_entity
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_ent_device {
    pub dev: *mut device,
    pub ent: *mut media_entity,
    pub frame): *const c_void,
    pub fmt): *mut v4l2_pix_format,
}

//
// struct vimc_device - main device for vimc driver
//
// @pipe_cfg:	pointer to the vimc pipeline configuration structure
// @ent_devs:	array of vimc_ent_device pointers
// @mdev:	the associated media_device parent
// @v4l2_dev:	Internal v4l2 parent device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_device {
    pub pipe_cfg: *const vimc_pipeline_config,
    pub ent_devs: *mut vimc_ent_device,
    pub mdev: media_device,
    pub v4l2_dev: v4l2_device,
}

//
// struct vimc_ent_type		Structure for the callbacks of the entity types
//
// @add:			initializes and registers
// vimc entity - called from vimc-core
// @unregister:			unregisters vimc entity - called from vimc-core
// @release:			releases vimc entity - called from the v4l2_dev
// release callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_ent_type {
    pub vcfg_name): *const c_char,
    pub ved): *mut *mut void (unregister)(struct vimc_ent_device,
    pub ved): *mut *mut void (release)(struct vimc_ent_device,
}

//
// struct vimc_ent_config	Structure which describes individual
// configuration for each entity
//
// @name:			entity name
// @type:			contain the callbacks of this entity type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_ent_config {
    pub name: *const c_char,
    pub type: *const vimc_ent_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vimc_sensor_osd_mode {
    VIMC_SENSOR_OSD_SHOW_ALL = 0,
    VIMC_SENSOR_OSD_SHOW_COUNTERS = 1,
    VIMC_SENSOR_OSD_SHOW_NONE = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_sensor_device {
    pub ved: vimc_ent_device,
    pub sd: v4l2_subdev,
    pub tpg: tpg_data,
    pub hdl: v4l2_ctrl_handler,
    pub pad: media_pad,
    pub pixel_rate: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub frame: *mut u8,
//
// Virtual "hardware" configuration, filled when the stream starts or
// when controls are set.
//
    pub size: v4l2_area,
    pub osd_value: vimc_sensor_osd_mode,
    pub start_stream_ts: u64,
    pub fps_jiffies: c_ulong,
    pub hw: },
}

//
// vimc_is_source - returns true if the entity has only source pads
//
// @ent: pointer to &struct media_entity
//
extern "C" {
    pub fn vimc_is_source(ent: *mut media_entity) -> bool;
}
//
// vimc_pix_map_by_index - get vimc_pix_map struct by its index
//
// @i:			index of the vimc_pix_map struct in vimc_pix_map_list
//
// vimc_mbus_code_by_index - get mbus code by its index
//
// @index:		index of the mbus code in vimc_pix_map_list
//
// Returns 0 if no mbus code is found for the given index.
//
extern "C" {
    pub fn vimc_mbus_code_by_index(index: c_uint) -> u32;
}
//
// vimc_pix_map_by_code - get vimc_pix_map struct by media bus code
//
// @code:		media bus format code defined by MEDIA_BUS_FMT_* macros
//
// vimc_pix_map_by_pixelformat - get vimc_pix_map struct by v4l2 pixel format
//
// @pixelformat:	pixel format defined by V4L2_PIX_FMT_* macros
//
// vimc_ent_sd_register - initialize and register a subdev node
//
// @ved:	the vimc_ent_device struct to be initialize
// @sd:		the v4l2_subdev struct to be initialize and registered
// @v4l2_dev:	the v4l2 device to register the v4l2_subdev
// @name:	name of the sub-device. Please notice that the name must be
// unique.
// @function:	media entity function defined by MEDIA_ENT_F_* macros
// @num_pads:	number of pads to initialize
// @pads:	the array of pads of the entity, the caller should set the
// flags of the pads
// @int_ops:	pointer to &struct v4l2_subdev_internal_ops.
// @sd_ops:	pointer to &struct v4l2_subdev_ops.
//
// Helper function initialize and register the struct vimc_ent_device and struct
// v4l2_subdev which represents a subdev node in the topology
//
// vimc_vdev_link_validate - validates a media link
//
// @link: pointer to &struct media_link
//
// This function calls validates if a media link is valid for streaming.
//
extern "C" {
    pub fn vimc_vdev_link_validate(link: *mut media_link) -> c_int;
}

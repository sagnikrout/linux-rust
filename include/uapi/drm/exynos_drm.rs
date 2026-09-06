//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/exynos_drm.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
// exynos_drm.h
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Authors:
// Inki Dae <inki.dae@samsung.com>
// Joonyoung Shim <jy0922.shim@samsung.com>
// Seung-Woo Kim <sw0312.kim@samsung.com>
//
// This program is free software; you can redistribute  it and/or modify it
// under  the terms of  the GNU General  Public License as published by the
// Free Software Foundation;  either version 2 of the  License, or (at your
// option) any later version.
//

//
// User-desired buffer creation information structure.
//
// @size: user-desired memory allocation size.
// - this size value would be page-aligned internally.
// @flags: user request for setting memory type or cache attributes.
// @handle: returned a handle to created gem object.
// - this handle will be set by gem module of kernel side.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_gem_create {
    pub size: __u64,
    pub flags: __u32,
    pub handle: __u32,
}

//
// A structure for getting a fake-offset that can be used with mmap.
//
// @handle: handle of gem object.
// @reserved: just padding to be 64-bit aligned.
// @offset: a fake-offset of gem object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_gem_map {
    pub handle: __u32,
    pub reserved: __u32,
    pub offset: __u64,
}

//
// A structure to gem information.
//
// @handle: a handle to gem object created.
// @flags: flag value including memory type and cache attribute and
// this value would be set by driver.
// @size: size to memory region allocated by gem and this size would
// be set by driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_gem_info {
    pub handle: __u32,
    pub flags: __u32,
    pub size: __u64,
}

//
// A structure for user connection request of virtual display.
//
// @connection: indicate whether doing connection or not by user.
// @extensions: if this value is 1 then the vidi driver would need additional
// 128bytes edid data.
// @edid: the edid data pointer from user side.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_vidi_connection {
    pub connection: __u32,
    pub extensions: __u32,
    pub edid: __u64,
}

// memory type definitions.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e_drm_exynos_gem_mem_type {
// Physically Continuous memory and used as default.
    EXYNOS_BO_CONTIG	= 0 << 0,
// Physically Non-Continuous memory.
    EXYNOS_BO_NONCONTIG	= 1 << 0,
// non-cachable mapping and used as default.
    EXYNOS_BO_NONCACHABLE	= 0 << 1,
// cachable mapping.
    EXYNOS_BO_CACHABLE	= 1 << 1,
// write-combine mapping.
    EXYNOS_BO_WC		= 1 << 2,
    EXYNOS_BO_MASK		= EXYNOS_BO_NONCONTIG | EXYNOS_BO_CACHABLE |
    EXYNOS_BO_WC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_g2d_get_ver {
    pub major: __u32,
    pub minor: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_g2d_cmd {
    pub offset: __u32,
    pub data: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_exynos_g2d_buf_type {
    G2D_BUF_USERPTR = 1 << 31,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_exynos_g2d_event_type {
    G2D_EVENT_NOT,
    G2D_EVENT_NONSTOP,
    G2D_EVENT_STOP,		/* not yet */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_g2d_userptr {
    pub userptr: c_ulong,
    pub size: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_g2d_set_cmdlist {
    pub cmd: __u64,
    pub cmd_buf: __u64,
    pub cmd_nr: __u32,
    pub cmd_buf_nr: __u32,
// for g2d event
    pub event_type: __u64,
    pub user_data: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_g2d_exec {
    pub async: __u64,
}

// Exynos DRM IPP v2 API
//
// Enumerate available IPP hardware modules.
//
// @count_ipps: size of ipp_id array / number of ipp modules (set by driver)
// @reserved: padding
// @ipp_id_ptr: pointer to ipp_id array or NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ioctl_ipp_get_res {
    pub count_ipps: __u32,
    pub reserved: __u32,
    pub ipp_id_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_exynos_ipp_format_type {
    DRM_EXYNOS_IPP_FORMAT_SOURCE		= 0x01,
    DRM_EXYNOS_IPP_FORMAT_DESTINATION	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_format {
    pub fourcc: __u32,
    pub type: __u32,
    pub modifier: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_exynos_ipp_capability {
    DRM_EXYNOS_IPP_CAP_CROP		= 0x01,
    DRM_EXYNOS_IPP_CAP_ROTATE	= 0x02,
    DRM_EXYNOS_IPP_CAP_SCALE	= 0x04,
    DRM_EXYNOS_IPP_CAP_CONVERT	= 0x08,
}

//
// Get IPP hardware capabilities and supported image formats.
//
// @ipp_id: id of IPP module to query
// @capabilities: bitmask of drm_exynos_ipp_capability (set by driver)
// @reserved: padding
// @formats_count: size of formats array (in entries) / number of filled
// formats (set by driver)
// @formats_ptr: pointer to formats array or NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ioctl_ipp_get_caps {
    pub ipp_id: __u32,
    pub capabilities: __u32,
    pub reserved: __u32,
    pub formats_count: __u32,
    pub formats_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_exynos_ipp_limit_type {
// size (horizontal/vertial) limits, in pixels (min, max, alignment)
    DRM_EXYNOS_IPP_LIMIT_TYPE_SIZE		= 0x0001,
// scale ratio (horizonta/vertial), 16.16 fixed point (min, max)
    DRM_EXYNOS_IPP_LIMIT_TYPE_SCALE		= 0x0002,

// image buffer area
    DRM_EXYNOS_IPP_LIMIT_SIZE_BUFFER	= 0x0001 << 16,
// src/dst rectangle area
    DRM_EXYNOS_IPP_LIMIT_SIZE_AREA		= 0x0002 << 16,
// src/dst rectangle area when rotation enabled
    DRM_EXYNOS_IPP_LIMIT_SIZE_ROTATED	= 0x0003 << 16,

    DRM_EXYNOS_IPP_LIMIT_TYPE_MASK		= 0x000f,
    DRM_EXYNOS_IPP_LIMIT_SIZE_MASK		= 0x000f << 16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_limit_val {
    pub min: __u32,
    pub max: __u32,
    pub align: __u32,
    pub reserved: __u32,
}

//
// IPP module limitation.
//
// @type: limit type (see drm_exynos_ipp_limit_type enum)
// @reserved: padding
// @h: horizontal limits
// @v: vertical limits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_limit {
    pub type: __u32,
    pub reserved: __u32,
    pub h: drm_exynos_ipp_limit_val,
    pub v: drm_exynos_ipp_limit_val,
}

//
// Get IPP limits for given image format.
//
// @ipp_id: id of IPP module to query
// @fourcc: image format code (see DRM_FORMAT_* in drm_fourcc.h)
// @modifier: image format modifier (see DRM_FORMAT_MOD_* in drm_fourcc.h)
// @type: source/destination identifier (drm_exynos_ipp_format_flag enum)
// @limits_count: size of limits array (in entries) / number of filled entries
// (set by driver)
// @limits_ptr: pointer to limits array or NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ioctl_ipp_get_limits {
    pub ipp_id: __u32,
    pub fourcc: __u32,
    pub modifier: __u64,
    pub type: __u32,
    pub limits_count: __u32,
    pub limits_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_exynos_ipp_task_id {
// buffer described by struct drm_exynos_ipp_task_buffer
    DRM_EXYNOS_IPP_TASK_BUFFER		= 0x0001,
// rectangle described by struct drm_exynos_ipp_task_rect
    DRM_EXYNOS_IPP_TASK_RECTANGLE		= 0x0002,
// transformation described by struct drm_exynos_ipp_task_transform
    DRM_EXYNOS_IPP_TASK_TRANSFORM		= 0x0003,
// alpha configuration described by struct drm_exynos_ipp_task_alpha
    DRM_EXYNOS_IPP_TASK_ALPHA		= 0x0004,

// source image data (for buffer and rectangle chunks)
    DRM_EXYNOS_IPP_TASK_TYPE_SOURCE		= 0x0001 << 16,
// destination image data (for buffer and rectangle chunks)
    DRM_EXYNOS_IPP_TASK_TYPE_DESTINATION	= 0x0002 << 16,
}

//
// Memory buffer with image data.
//
// @id: must be DRM_EXYNOS_IPP_TASK_BUFFER
// other parameters are same as for AddFB2 generic DRM ioctl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_task_buffer {
    pub id: __u32,
    pub fourcc: __u32,
    pub height: __u32 width,,
    pub gem_id: [__u32; 4],
    pub offset: [__u32; 4],
    pub pitch: [__u32; 4],
    pub modifier: __u64,
}

//
// Rectangle for processing.
//
// @id: must be DRM_EXYNOS_IPP_TASK_RECTANGLE
// @reserved: padding
// @x,@y: left corner in pixels
// @w,@h: width/height in pixels
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_task_rect {
    pub id: __u32,
    pub reserved: __u32,
    pub x: __u32,
    pub y: __u32,
    pub w: __u32,
    pub h: __u32,
}

//
// Image tranformation description.
//
// @id: must be DRM_EXYNOS_IPP_TASK_TRANSFORM
// @rotation: DRM_MODE_ROTATE_* and DRM_MODE_REFLECT_* values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_task_transform {
    pub id: __u32,
    pub rotation: __u32,
}

//
// Image global alpha configuration for formats without alpha values.
//
// @id: must be DRM_EXYNOS_IPP_TASK_ALPHA
// @value: global alpha value (0-255)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_task_alpha {
    pub id: __u32,
    pub value: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_exynos_ipp_flag {
// generate DRM event after processing
    DRM_EXYNOS_IPP_FLAG_EVENT	= 0x01,
// dry run, only check task parameters
    DRM_EXYNOS_IPP_FLAG_TEST_ONLY	= 0x02,
// non-blocking processing
    DRM_EXYNOS_IPP_FLAG_NONBLOCK	= 0x04,
}

//
// Perform image processing described by array of drm_exynos_ipp_task_
// structures (parameters array).
//
// @ipp_id: id of IPP module to run the task
// @flags: bitmask of drm_exynos_ipp_flag values
// @reserved: padding
// @params_size: size of parameters array (in bytes)
// @params_ptr: pointer to parameters array or NULL
// @user_data: (optional) data for drm event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ioctl_ipp_commit {
    pub ipp_id: __u32,
    pub flags: __u32,
    pub reserved: __u32,
    pub params_size: __u32,
    pub params_ptr: __u64,
    pub user_data: __u64,
}

pub const DRM_EXYNOS_GEM_CREATE: c_uint = 0x00;
pub const DRM_EXYNOS_GEM_MAP: c_uint = 0x01;
// Reserved 0x03 ~ 0x05 for exynos specific gem ioctl
pub const DRM_EXYNOS_GEM_GET: c_uint = 0x04;
pub const DRM_EXYNOS_VIDI_CONNECTION: c_uint = 0x07;
// G2D
pub const DRM_EXYNOS_G2D_GET_VER: c_uint = 0x20;
pub const DRM_EXYNOS_G2D_SET_CMDLIST: c_uint = 0x21;
pub const DRM_EXYNOS_G2D_EXEC: c_uint = 0x22;
// Reserved 0x30 ~ 0x33 for obsolete Exynos IPP ioctls
// IPP - Image Post Processing
pub const DRM_EXYNOS_IPP_GET_RESOURCES: c_uint = 0x40;
pub const DRM_EXYNOS_IPP_GET_CAPS: c_uint = 0x41;
pub const DRM_EXYNOS_IPP_GET_LIMITS: c_uint = 0x42;
pub const DRM_EXYNOS_IPP_COMMIT: c_uint = 0x43;

// Exynos specific events
pub const DRM_EXYNOS_G2D_EVENT: c_uint = 0x80000000;
pub const DRM_EXYNOS_IPP_EVENT: c_uint = 0x80000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_g2d_event {
    pub base: drm_event,
    pub user_data: __u64,
    pub tv_sec: __u32,
    pub tv_usec: __u32,
    pub cmdlist_no: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_exynos_ipp_event {
    pub base: drm_event,
    pub user_data: __u64,
    pub tv_sec: __u32,
    pub tv_usec: __u32,
    pub ipp_id: __u32,
    pub sequence: __u32,
    pub reserved: __u64,
}


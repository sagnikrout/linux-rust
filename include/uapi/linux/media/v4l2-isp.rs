//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/media/v4l2-isp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Video4Linux2 generic ISP parameters and statistics support
//
// Copyright (C) 2025 Ideas On Board Oy
// Author: Jacopo Mondi <jacopo.mondi@ideasonboard.com>
//

//
// enum v4l2_isp_version - V4L2 ISP serialization format versioning
//
// @V4L2_ISP_VERSION_V0: First version of the V4L2 ISP serialization format
// (for compatibility)
// @V4L2_ISP_VERSION_V1: First version of the V4L2 ISP serialization format
//
// V0 and V1 are identical in order to support drivers compatible with the V4L2
// ISP format already upstreamed which use either 0 or 1 as their versioning
// identifier. Both V0 and V1 refers to the first version of the V4L2 ISP
// serialization format.
//
// Future revisions of the V4L2 ISP serialization format should start from the
// value of 2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_isp_version {
    V4L2_ISP_VERSION_V0 = 0,
    V4L2_ISP_VERSION_V1
}

//
// Compatibility with existing users of v4l2_isp_params which pre-date the
// introduction of v4l2_isp_stats.
//

//
// Reserve the first 8 bits for V4L2_ISP_PARAMS_FL_* flag.
//
// Driver-specific flags should be defined as:
// #define DRIVER_SPECIFIC_FLAG0     ((1U << V4L2_ISP_FL_DRIVER_FLAGS(0))
// #define DRIVER_SPECIFIC_FLAG1     ((1U << V4L2_ISP_FL_DRIVER_FLAGS(1))
//

//
// struct v4l2_isp_block_header - V4L2 extensible block header
// @type: The parameters or statistics block type (driver-specific)
// @flags: A bitmask of block flags (driver-specific)
// @size: Size (in bytes) of the block, including this header
//
// This structure represents the common part of all the ISP configuration or
// statistic blocks. Each block shall embed an instance of this structure type
// as its first member, followed by the block-specific configuration or
// statistic data.
//
// The @type field is an ISP driver-specific value that identifies the block
// type. The @size field specifies the size of the block, including this
// header.
//
// The @flags field is a bitmask of per-block flags. If a block is used for
// configuration parameters this field can be a combination of
// V4L2_ISP_PARAMS_FL_* and driver-specific flags. If a block is used
// for statistics this fields is used to report optional
// driver-specific flags, if any.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_isp_block_header {
    pub type: __u16,
    pub flags: __u16,
    pub size: __u32,
    pub __attribute__((aligned(8))): },
//
// v4l2_isp_params_block_header - V4L2 extensible parameters block header
//
// Compatibility with existing users of v4l2_isp_params_block_header
// which pre-date the introduction of v4l2_isp_block_header.
//

//
// struct v4l2_isp_buffer - V4L2 extensible buffer
// @version: The extensible buffer version (driver-specific)
// @data_size: The data effective size, excluding this header
// @data: The configuration or statistics data
//
// This structure contains ISP configuration parameters or ISP hardware
// statistics serialized into a data buffer. Each block is represented by a
// block-specific structure which contains a :c:type:`v4l2_isp_block_header`
// entry as first member.
//
// When used for ISP parameters, userspace populates the @data buffer with
// configuration parameters for the blocks that it intends to configure. As a
// consequence, the data buffer effective size changes according to the number
// of ISP blocks that userspace intends to configure.
//
// When used to report ISP statistics, the driver populates the @data buffer
// with statistics for each supported measurement block.
//
// The buffer is versioned by the @version field to allow modifying
// and extending its definition. The writer shall populate the @version field
// to inform the reader about the version it intends to use. The reader will
// parse and handle the @data buffer according to the data layout specific to
// the indicated version and return an error if the desired version is not
// supported.
//
// For each ISP block, a block-specific structure is appended to the @data
// buffer, one after the other without gaps in between. The writer shall
// populate the @data_size field with the effective size, in bytes, of the
// @data buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_isp_buffer {
    pub version: __u32,
    pub data_size: __u32,
    pub __counted_by(data_size): __u8 data[],
}

//
// v4l2_isp_params_buffer - V4L2 extensible parameters compatibility
//
// Compatibility with existing users of v4l2_isp_params_buffer which
// pre-date the introduction of v4l2_isp_buffer.
//


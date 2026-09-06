//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-isp.h
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
// Video4Linux2 generic ISP parameters and statistics support
//
// Copyright (C) 2025 Ideas On Board Oy
// Author: Jacopo Mondi <jacopo.mondi@ideasonboard.com>
//

//
// v4l2_isp_buffer_size - Calculate size of v4l2_isp_buffer
// @max_size: The total size of the ISP configuration or statistics blocks
//
// Users of v4l2-isp will have differing sized data arrays for parameters and
// statistics, depending on their specific blocks. Drivers need to be able to
// calculate the appropriate size of the buffer to accommodate all ISP blocks
// supported by the platform. This macro provides a convenient tool for the
// calculation.
//
// The intended users of this function are drivers initializing the size
// of their metadata (parameters and statistics) buffers.
//

//
// v4l2_isp_params_validate_buffer_size - Validate a V4L2 ISP buffer sizes
// @dev: the driver's device pointer
// @vb: the videobuf2 buffer
// @max_size: the maximum allowed buffer size
//
// This function performs validation of the size of a V4L2 ISP parameters buffer
// before the driver can access the actual data buffer content.
//
// After the sizes validation, drivers should copy the buffer content to a
// kernel-only memory area to prevent userspace from modifying it,
// before completing validation using v4l2_isp_params_validate_buffer().
//
// The @vb buffer as received from the vb2 .buf_prepare() operation is checked
// against @max_size and it's validated to be large enough to accommodate at
// least one ISP configuration block.
//
// struct v4l2_isp_params_block_type_info - V4L2 ISP params per-block-type info
// @size: the block type expected size
// @block_validate: driver's callback to implement per-block validation
//
// The v4l2_isp_params_block_type_info collects information of the ISP
// configuration block types for validation purposes. It contains the expected
// block type size and a function pointer where drivers can register a callback
// for additional per-block validation purposes. The validation function is
// expected to return 0 on success or a negative error number for errors.
//
// Drivers shall prepare a list of block type info, indexed by block type, one
// for each supported ISP block type and correctly populate them with the
// expected block type size and the optional callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_isp_params_block_type_info {
    pub size: usize,
    pub block): *const v4l2_isp_block_header,
}

//
// v4l2_isp_params_validate_buffer - Validate a V4L2 ISP parameters buffer
// @dev: the driver's device pointer
// @vb: the videobuf2 buffer
// @buffer: the V4L2 ISP parameters buffer
// @type_info: the array of per-block-type validation info
// @num_block_types: the number of block types in the type_info array
//
// This function completes the validation of a V4L2 ISP parameters buffer,
// verifying each configuration block correctness before the driver can use
// them to program the hardware.
//
// Drivers should use this function after having validated the correctness of
// the vb2 buffer sizes by using the v4l2_isp_params_validate_buffer_size()
// helper first. Once the buffer size has been validated, drivers should
// perform a copy of the user provided buffer into a kernel-only memory buffer
// to prevent userspace from modifying its content after it has been submitted
// to the driver, and then call this function to complete validation.
//
// struct v4l2_isp_stats_block_type_info - V4L2 ISP stats per-block-type info
// @size: the block type expected size
//
// The v4l2_isp_stats_block_type_info collects information of the ISP
// statistics block types for validation purposes. It currently only contains
// the expected block size.
//
// Drivers shall prepare a list of statistics block type info, indexed by block
// type, one for each supported ISP statistics block type and correctly populate
// them with the expected block size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_isp_stats_block_type_info {
    pub size: usize,
}

//
// v4l2_isp_stats_init_buffer - Initialize a statistics buffer
//
// Initialize a buffer of statistics. Only set the 'version' field and reset
// 'data_size' to 0.
//
// @buf: the v4l2_isp_buffer to initialize
// @version: the v4l2-isp serialization format version used by the driver
//
// v4l2_isp_stats_init_block - Create and initialize a new block in a statistics
// buffer
// @dev: the driver's device pointer
// @buf: the v4l2_isp_buffer where statistics are serialized
// @type_info: the array of per-block-type validation info
// @num_block_types: the number of block types in the type_info array
// @block_type: the type of the statistics block to initialize
// @max_size: the maximum size of the data[] member of @buf
//
// This function locates and initialize a new statistics block in @buf for the
// driver to populate its content. The function checks that enough space for the
// requested @block_type is available in @buf and increments the 'data_size'
// member of @buf. The newly created statistics block's header is initialized
// with the size and type information provided by the caller in @type_info.
//
// Drivers should call this function before populating a new statistics block
// content.
//
// Returns a pointer to the next available location in @buf, or an error pointer
// if the requested @block_size is not available in @buf or @block_type is not
// valid.
//

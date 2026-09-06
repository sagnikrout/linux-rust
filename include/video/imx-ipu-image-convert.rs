//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/imx-ipu-image-convert.h
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
// Copyright (C) 2012-2016 Mentor Graphics Inc.
//
// i.MX Queued image conversion support, with tiling and rotation.
//

//
// struct ipu_image_convert_run - image conversion run request struct
//
// @ctx:	the conversion context
// @in_phys:	dma addr of input image buffer for this run
// @out_phys:	dma addr of output image buffer for this run
// @status:	completion status of this run
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_image_convert_run {
    pub ctx: *mut ipu_image_convert_ctx,
    pub in_phys: dma_addr_t,
    pub out_phys: dma_addr_t,
    pub status: c_int,
// private:
// internal to image converter, callers don't touch
    pub list: list_head,
}

//
// typedef ipu_image_convert_cb_t - conversion callback function prototype
//
// @run:	the completed conversion run pointer
// @ctx:	a private context pointer for the callback
//
// ipu_image_convert_adjust() - adjust input/output images to IPU restrictions.
//
// @in:		input image format, adjusted on return
// @out:	output image format, adjusted on return
// @rot_mode:	rotation mode
//
// In V4L2, drivers can call ipu_image_convert_adjust() in .try_fmt.
//
// ipu_image_convert_verify() - verify that input/output image formats
// and rotation mode meet IPU restrictions.
//
// @in:		input image format
// @out:	output image format
// @rot_mode:	rotation mode
//
// Returns: 0 if the formats and rotation mode meet IPU restrictions,
// -EINVAL otherwise.
//
// ipu_image_convert_prepare() - prepare a conversion context.
//
// @ipu:	the IPU handle to use for the conversions
// @ic_task:	the IC task to use for the conversions
// @in:		input image format
// @out:	output image format
// @rot_mode:	rotation mode
// @complete:	run completion callback
// @complete_context:	a context pointer for the completion callback
//
// In V4L2, drivers should call ipu_image_convert_prepare() at streamon.
//
// Returns: an opaque conversion context pointer on success, error pointer
// on failure. The input/output formats and rotation mode must already meet
// IPU retrictions.
//
// ipu_image_convert_unprepare() - unprepare a conversion context.
//
// @ctx: the conversion context pointer to unprepare
//
// Aborts any active or pending conversions for this context and
// frees the context. Any currently active or pending runs belonging
// to this context are returned via the completion callback with an
// error run status.
//
// In V4L2, drivers should call ipu_image_convert_unprepare() at
// streamoff.
//
extern "C" {
    pub fn ipu_image_convert_unprepare(ctx: *mut ipu_image_convert_ctx);
}
//
// ipu_image_convert_queue() - queue a conversion run
//
// @run: the run request pointer
//
// ipu_image_convert_run must be dynamically allocated (_not_ as a local
// var) by callers and filled in with a previously prepared conversion
// context handle and the dma addr's of the input and output image buffers
// for this conversion run.
//
// When this conversion completes, the run pointer is returned via the
// completion callback. The caller is responsible for freeing the run
// object after it completes.
//
// In V4L2, drivers should call ipu_image_convert_queue() while
// streaming to queue the conversion of a received input buffer.
// For example mem2mem devices this would be called in .device_run.
//
// Returns: 0 on success or -errno on error.
//
extern "C" {
    pub fn ipu_image_convert_queue(run: *mut ipu_image_convert_run) -> c_int;
}
//
// ipu_image_convert_abort() - abort conversions
//
// @ctx: the conversion context pointer
//
// This will abort any active or pending conversions for this context.
// Any currently active or pending runs belonging to this context are
// returned via the completion callback with an error run status.
//
extern "C" {
    pub fn ipu_image_convert_abort(ctx: *mut ipu_image_convert_ctx);
}
//
// ipu_image_convert() - asynchronous image conversion request
//
// @ipu:	the IPU handle to use for the conversion
// @ic_task:	the IC task to use for the conversion
// @in:		input image format
// @out:	output image format
// @rot_mode:	rotation mode
// @complete:	run completion callback
// @complete_context:	a context pointer for the completion callback
//
// Request a single image conversion. Returns the run that has been queued.
// A conversion context is automatically created and is available in run->ctx.
// As with ipu_image_convert_prepare(), the input/output formats and rotation
// mode must already meet IPU retrictions.
//
// On successful return the caller can queue more run requests if needed, using
// the prepared context in run->ctx. The caller is responsible for unpreparing
// the context when no more conversion requests are needed.
//
// Returns: pointer to the created &struct ipu_image_convert_run that has
// been queued on success; an ERR_PTR(errno) on error.
//

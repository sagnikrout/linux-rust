//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vimc/vimc-streamer.h
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
// vimc-streamer.h Virtual Media Controller Driver
//
// Copyright (C) 2018 Lucas A. M. Magalhães <lucmaga@gmail.com>
//

pub const VIMC_STREAMER_PIPELINE_MAX_SIZE: c_int = 16;
//
// struct vimc_stream - struct that represents a stream in the pipeline
//
// @pipe:		the media pipeline object associated with this stream
// @ved_pipeline:	array containing all the entities participating in the
// stream. The order is from a video device (usually a
// capture device) where stream_on was called, to the
// entity generating the first base image to be
// processed in the pipeline.
// @pipe_size:		size of @ved_pipeline
// @kthread:		thread that generates the frames of the stream.
//
// When the user call stream_on in a video device, struct vimc_stream is
// used to keep track of all entities and subdevices that generates and
// process frames for the stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vimc_stream {
    pub pipe: media_pipeline,
    pub ved_pipeline: [*mut vimc_ent_device; VIMC_STREAMER_PIPELINE_MAX_SIZE],
    pub pipe_size: c_uint,
    pub kthread: *mut task_struct,
}

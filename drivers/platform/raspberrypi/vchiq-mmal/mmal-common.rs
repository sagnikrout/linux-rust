//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-mmal/mmal-common.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Broadcom BCM2835 V4L2 driver
//
// Copyright © 2013 Raspberry Pi (Trading) Ltd.
//
// Authors: Vincent Sanders @ Collabora
// Dave Stevenson @ Broadcom
// (now dave.stevenson@raspberrypi.org)
// Simon Mellor @ Broadcom
// Luke Diamand @ Broadcom
//
// MMAL structures
//

// Special value signalling that time is not known

// mapping between v4l and mmal video modes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_fmt {
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub /: *mut *mut int flags; / v4l2 flags field,
    pub mmal: u32,
    pub depth: c_int,
    pub /: *mut *mut u32 mmal_component; / MMAL component index to be used to encode,
    pub /: *mut *mut u32 ybbp; / depth of first Y plane for planar formats,
    pub padding,: *mut *mut bool remove_padding; / Does the GPU have to remove,
// or can we do hide padding via bytesperline.
//
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_buffer {
// v4l buffer data -- must be first
    pub vb: vb2_v4l2_buffer,
// list of buffers available
    pub list: list_head,
    pub /: *mut *mut *mut void buffer; / buffer pointer,
    pub /: *mut *mut unsigned long buffer_size; / size of allocated buffer,
    pub msg_context: *mut mmal_msg_context,
    pub length: c_ulong,
    pub mmal_flags: u32,
    pub dts: i64,
    pub pts: i64,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_colourfx {
    pub enable: i32,
    pub u: u32,
    pub v: u32,
}

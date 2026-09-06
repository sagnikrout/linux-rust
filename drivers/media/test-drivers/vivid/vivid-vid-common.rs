//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-vid-common.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// vivid-vid-common.h - common video support functions.
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
extern "C" {
    pub fn int(file: *mut *mut fmtfunc)(struct file, priv: *mut c_void, f: *mut v4l2_format) -> typedef;
}
//
// Conversion function that converts a single-planar format to a
// single-plane multiplanar format.
//
extern "C" {
    pub fn fmt_sp2mp(sp_fmt: *const v4l2_format, mp_fmt: *mut v4l2_format);
}
extern "C" {
    pub fn vivid_vid_can_loop(dev: *mut vivid_dev) -> bool;
}
extern "C" {
    pub fn vivid_send_source_change(dev: *mut vivid_dev, type: c_uint);
}
extern "C" {
    pub fn vivid_send_input_source_change(dev: *mut vivid_dev, input_index: c_uint);
}
extern "C" {
    pub fn vivid_vid_adjust_sel(flags: unsigned, r: *mut v4l2_rect) -> c_int;
}
extern "C" {
    pub fn vivid_enum_fmt_vid(file: *mut file, priv: *mut c_void, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn vidioc_g_std(file: *mut file, priv: *mut c_void, id: *mut v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn vidioc_g_dv_timings(file: *mut file, priv: *mut c_void, timings: *mut v4l2_dv_timings) -> c_int;
}
extern "C" {
    pub fn vidioc_enum_dv_timings(file: *mut file, priv: *mut c_void, timings: *mut v4l2_enum_dv_timings) -> c_int;
}
extern "C" {
    pub fn vidioc_dv_timings_cap(file: *mut file, priv: *mut c_void, cap: *mut v4l2_dv_timings_cap) -> c_int;
}
extern "C" {
    pub fn vidioc_g_edid(file: *mut file, priv: *mut c_void, edid: *mut v4l2_edid) -> c_int;
}
extern "C" {
    pub fn vidioc_subscribe_event(fh: *mut v4l2_fh, sub: *const v4l2_event_subscription) -> c_int;
}

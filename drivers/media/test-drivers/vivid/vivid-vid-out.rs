//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-vid-out.h
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
// vivid-vid-out.h - video output support functions.
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
extern "C" {
    pub fn vivid_update_format_out(dev: *mut vivid_dev);
}
extern "C" {
    pub fn vivid_g_fmt_vid_out(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_try_fmt_vid_out(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_s_fmt_vid_out(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_g_fmt_vid_out_mplane(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_try_fmt_vid_out_mplane(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_s_fmt_vid_out_mplane(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_g_fmt_vid_out(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_try_fmt_vid_out(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_s_fmt_vid_out(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_g_selection(file: *mut file, priv: *mut c_void, sel: *mut v4l2_selection) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_s_selection(file: *mut file, priv: *mut c_void, s: *mut v4l2_selection) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_g_pixelaspect(file: *mut file, priv: *mut c_void, type: c_int, f: *mut v4l2_fract) -> c_int;
}
extern "C" {
    pub fn vidioc_enum_fmt_vid_out_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn vidioc_g_fmt_vid_out_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_try_fmt_vid_out_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_s_fmt_vid_out_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_overlay(file: *mut file, priv: *mut c_void, i: unsigned) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_g_fbuf(file: *mut file, priv: *mut c_void, a: *mut v4l2_framebuffer) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_s_fbuf(file: *mut file, priv: *mut c_void, a: *const v4l2_framebuffer) -> c_int;
}
extern "C" {
    pub fn vidioc_enum_output(file: *mut file, priv: *mut c_void, out: *mut v4l2_output) -> c_int;
}
extern "C" {
    pub fn vidioc_g_output(file: *mut file, priv: *mut c_void, i: *mut unsigned) -> c_int;
}
extern "C" {
    pub fn vidioc_s_output(file: *mut file, priv: *mut c_void, i: unsigned) -> c_int;
}
extern "C" {
    pub fn vidioc_enumaudout(file: *mut file, priv: *mut c_void, vout: *mut v4l2_audioout) -> c_int;
}
extern "C" {
    pub fn vidioc_g_audout(file: *mut file, priv: *mut c_void, vout: *mut v4l2_audioout) -> c_int;
}
extern "C" {
    pub fn vidioc_s_audout(file: *mut file, priv: *mut c_void, vout: *const v4l2_audioout) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_s_std(file: *mut file, priv: *mut c_void, id: v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_s_dv_timings(file: *mut file, priv: *mut c_void, timings: *mut v4l2_dv_timings) -> c_int;
}
extern "C" {
    pub fn vivid_vid_out_g_parm(file: *mut file, priv: *mut c_void, parm: *mut v4l2_streamparm) -> c_int;
}

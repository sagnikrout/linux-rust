//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-vid-cap.h
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
// vivid-vid-cap.h - video capture support functions.
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
extern "C" {
    pub fn vivid_update_quality(dev: *mut vivid_dev);
}
extern "C" {
    pub fn vivid_update_reduced_fps(dev: *mut vivid_dev);
}
extern "C" {
    pub fn vivid_update_format_cap(dev: *mut vivid_dev, keep_controls: bool);
}
extern "C" {
    pub fn vivid_update_outputs(dev: *mut vivid_dev);
}
extern "C" {
    pub fn vivid_update_connected_outputs(dev: *mut vivid_dev);
}
extern "C" {
    pub fn vivid_get_video_aspect(dev: *const vivid_dev) -> tpg_video_aspect;
}
extern "C" {
    pub fn vivid_g_fmt_vid_cap(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_try_fmt_vid_cap(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_s_fmt_vid_cap(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_g_fmt_vid_cap_mplane(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_try_fmt_vid_cap_mplane(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_s_fmt_vid_cap_mplane(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_g_fmt_vid_cap(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_try_fmt_vid_cap(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_s_fmt_vid_cap(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vivid_vid_cap_g_selection(file: *mut file, priv: *mut c_void, sel: *mut v4l2_selection) -> c_int;
}
extern "C" {
    pub fn vivid_vid_cap_s_selection(file: *mut file, priv: *mut c_void, s: *mut v4l2_selection) -> c_int;
}
extern "C" {
    pub fn vivid_vid_cap_g_pixelaspect(file: *mut file, priv: *mut c_void, type: c_int, f: *mut v4l2_fract) -> c_int;
}
extern "C" {
    pub fn vidioc_enum_fmt_vid_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn vidioc_g_fmt_vid_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_try_fmt_vid_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_s_fmt_vid_overlay(file: *mut file, priv: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_enum_input(file: *mut file, priv: *mut c_void, inp: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn vidioc_g_input(file: *mut file, priv: *mut c_void, i: *mut unsigned) -> c_int;
}
extern "C" {
    pub fn vidioc_s_input(file: *mut file, priv: *mut c_void, i: unsigned) -> c_int;
}
extern "C" {
    pub fn vidioc_enumaudio(file: *mut file, priv: *mut c_void, vin: *mut v4l2_audio) -> c_int;
}
extern "C" {
    pub fn vidioc_g_audio(file: *mut file, priv: *mut c_void, vin: *mut v4l2_audio) -> c_int;
}
extern "C" {
    pub fn vidioc_s_audio(file: *mut file, priv: *mut c_void, vin: *const v4l2_audio) -> c_int;
}
extern "C" {
    pub fn vivid_video_g_frequency(file: *mut file, priv: *mut c_void, vf: *mut v4l2_frequency) -> c_int;
}
extern "C" {
    pub fn vivid_video_s_frequency(file: *mut file, priv: *mut c_void, vf: *const v4l2_frequency) -> c_int;
}
extern "C" {
    pub fn vivid_video_s_tuner(file: *mut file, priv: *mut c_void, vt: *const v4l2_tuner) -> c_int;
}
extern "C" {
    pub fn vivid_video_g_tuner(file: *mut file, priv: *mut c_void, vt: *mut v4l2_tuner) -> c_int;
}
extern "C" {
    pub fn vidioc_querystd(file: *mut file, priv: *mut c_void, id: *mut v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn vivid_vid_cap_s_std(file: *mut file, priv: *mut c_void, id: v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn vivid_vid_cap_s_dv_timings(file: *mut file, priv: *mut c_void, timings: *mut v4l2_dv_timings) -> c_int;
}
extern "C" {
    pub fn vidioc_query_dv_timings(file: *mut file, priv: *mut c_void, timings: *mut v4l2_dv_timings) -> c_int;
}
extern "C" {
    pub fn vidioc_s_edid(file: *mut file, priv: *mut c_void, edid: *mut v4l2_edid) -> c_int;
}
extern "C" {
    pub fn vidioc_enum_framesizes(file: *mut file, priv: *mut c_void, fsize: *mut v4l2_frmsizeenum) -> c_int;
}
extern "C" {
    pub fn vidioc_enum_frameintervals(file: *mut file, priv: *mut c_void, fival: *mut v4l2_frmivalenum) -> c_int;
}
extern "C" {
    pub fn vivid_vid_cap_g_parm(file: *mut file, priv: *mut c_void, parm: *mut v4l2_streamparm) -> c_int;
}
extern "C" {
    pub fn vivid_vid_cap_s_parm(file: *mut file, priv: *mut c_void, parm: *mut v4l2_streamparm) -> c_int;
}

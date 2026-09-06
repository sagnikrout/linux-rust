//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-vbi-cap.h
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
// vivid-vbi-cap.h - vbi capture support functions.
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//
extern "C" {
    pub fn vivid_fill_time_of_day_packet(packet: *mut u8);
}
extern "C" {
    pub fn vivid_raw_vbi_cap_process(dev: *mut vivid_dev, buf: *mut vivid_buffer);
}
extern "C" {
    pub fn vivid_sliced_vbi_cap_process(dev: *mut vivid_dev, buf: *mut vivid_buffer);
}
extern "C" {
    pub fn vivid_sliced_vbi_out_process(dev: *mut vivid_dev, buf: *mut vivid_buffer);
}
extern "C" {
    pub fn vidioc_g_fmt_sliced_vbi_cap(file: *mut file, priv: *mut c_void, fmt: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_try_fmt_sliced_vbi_cap(file: *mut file, priv: *mut c_void, fmt: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_s_fmt_sliced_vbi_cap(file: *mut file, priv: *mut c_void, fmt: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn vidioc_g_sliced_vbi_cap(file: *mut file, priv: *mut c_void, cap: *mut v4l2_sliced_vbi_cap) -> c_int;
}
extern "C" {
    pub fn vivid_fill_service_lines(vbi: *mut v4l2_sliced_vbi_format, service_set: u32);
}

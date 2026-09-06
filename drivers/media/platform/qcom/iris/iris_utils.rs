//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_utils.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_rect_desc {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_hfi_frame_info {
    pub picture_type: u32,
    pub no_output: u32,
    pub data_corrupt: u32,
    pub overflow: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_ts_metadata {
    pub ts_ns: u64,
    pub ts_us: u64,
    pub flags: u32,
    pub tc: v4l2_timecode,
}

extern "C" {
    pub fn iris_get_mbpf(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_split_mode_enabled(inst: *mut iris_inst) -> bool;
}
extern "C" {
    pub fn iris_fmt_is_8bit(pixelformat: u32) -> bool;
}
extern "C" {
    pub fn iris_fmt_is_10bit(pixelformat: u32) -> bool;
}
extern "C" {
    pub fn iris_wait_for_session_response(inst: *mut iris_inst, is_flush: bool) -> c_int;
}
extern "C" {
    pub fn iris_check_core_mbpf(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_check_core_mbps(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn is_rotation_90_or_270(inst: *mut iris_inst) -> bool;
}

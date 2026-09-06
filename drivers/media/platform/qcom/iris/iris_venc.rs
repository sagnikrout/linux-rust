//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_venc.h
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
// Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
//
extern "C" {
    pub fn iris_venc_inst_init(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_venc_enum_fmt(inst: *mut iris_inst, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn iris_venc_try_fmt(inst: *mut iris_inst, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn iris_venc_s_fmt(inst: *mut iris_inst, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn iris_venc_validate_format(inst: *mut iris_inst, pixelformat: u32) -> c_int;
}
extern "C" {
    pub fn iris_venc_subscribe_event(inst: *mut iris_inst, sub: *const v4l2_event_subscription) -> c_int;
}
extern "C" {
    pub fn iris_venc_s_selection(inst: *mut iris_inst, s: *mut v4l2_selection) -> c_int;
}
extern "C" {
    pub fn iris_venc_g_param(inst: *mut iris_inst, s_parm: *mut v4l2_streamparm) -> c_int;
}
extern "C" {
    pub fn iris_venc_s_param(inst: *mut iris_inst, s_parm: *mut v4l2_streamparm) -> c_int;
}
extern "C" {
    pub fn iris_venc_streamon_input(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_venc_streamon_output(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_venc_qbuf(inst: *mut iris_inst, vbuf: *mut vb2_v4l2_buffer) -> c_int;
}
extern "C" {
    pub fn iris_venc_start_cmd(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_venc_stop_cmd(inst: *mut iris_inst) -> c_int;
}

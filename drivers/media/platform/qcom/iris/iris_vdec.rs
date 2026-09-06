//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_vdec.h
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
extern "C" {
    pub fn iris_vdec_inst_init(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_vdec_enum_fmt(inst: *mut iris_inst, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn iris_vdec_try_fmt(inst: *mut iris_inst, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn iris_vdec_s_fmt(inst: *mut iris_inst, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn iris_vdec_validate_format(inst: *mut iris_inst, pixelformat: u32) -> c_int;
}
extern "C" {
    pub fn iris_vdec_subscribe_event(inst: *mut iris_inst, sub: *const v4l2_event_subscription) -> c_int;
}
extern "C" {
    pub fn iris_vdec_src_change(inst: *mut iris_inst);
}
extern "C" {
    pub fn iris_vdec_streamon_input(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_vdec_streamon_output(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_vdec_qbuf(inst: *mut iris_inst, vbuf: *mut vb2_v4l2_buffer) -> c_int;
}
extern "C" {
    pub fn iris_vdec_start_cmd(inst: *mut iris_inst) -> c_int;
}
extern "C" {
    pub fn iris_vdec_stop_cmd(inst: *mut iris_inst) -> c_int;
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_vb2.h
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
    pub fn iris_vb2_buf_init(vb2: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn iris_vb2_start_streaming(q: *mut vb2_queue, count: c_uint) -> c_int;
}
extern "C" {
    pub fn iris_vb2_stop_streaming(q: *mut vb2_queue);
}
extern "C" {
    pub fn iris_vb2_buf_prepare(vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn iris_vb2_buf_out_validate(vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn iris_vb2_buf_queue(vb2: *mut vb2_buffer);
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/wave5/wave5-helper.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Wave5 series multi-standard codec IP - basic types
//
// Copyright (C) 2021-2023 CHIPS&MEDIA INC
//

pub const FMT_TYPES: c_int = 2;
pub const MAX_FMTS: c_int = 16;
extern "C" {
    pub fn wave5_cleanup_instance(inst: *mut vpu_instance, filp: *mut file);
}
extern "C" {
    pub fn wave5_vpu_subscribe_event(fh: *mut v4l2_fh, sub: *const v4l2_event_subscription) -> c_int;
}
extern "C" {
    pub fn wave5_vpu_g_fmt_out(file: *mut file, fh: *mut c_void, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn wave5_to_vpu_std(v4l2_pix_fmt: c_uint, type: vpu_instance_type) -> wave_std;
}
extern "C" {
    pub fn wave5_return_bufs(q: *mut vb2_queue, state: u32);
}
extern "C" {
    pub fn wave5_kfifo_alloc(inst: *mut vpu_instance) -> c_int;
}

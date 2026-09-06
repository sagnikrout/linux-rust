//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-streams.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// cx18 init/start/stop/exit stream functions
//
// Derived from ivtv-streams.h
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
// Copyright (C) 2008  Andy Walls <awalls@md.metrocast.net>
//
extern "C" {
    pub fn cx18_find_handle(cx: *mut cx18) -> u32;
}
extern "C" {
    pub fn cx18_streams_setup(cx: *mut cx18) -> c_int;
}
extern "C" {
    pub fn cx18_streams_register(cx: *mut cx18) -> c_int;
}
extern "C" {
    pub fn cx18_streams_cleanup(cx: *mut cx18, unregister: c_int);
}

extern "C" {
    pub fn cx18_stream_rotate_idx_mdls(cx: *mut cx18);
}
// Related to submission of mdls to firmware
// Put mdl on q_free; the out work handler will move mdl(s) to q_busy
extern "C" {
    pub fn cx18_out_work_handler(work: *mut work_struct);
}
// Capture related
extern "C" {
    pub fn cx18_start_v4l2_encode_stream(s: *mut cx18_stream) -> c_int;
}
extern "C" {
    pub fn cx18_stop_v4l2_encode_stream(s: *mut cx18_stream, gop_end: c_int) -> c_int;
}
extern "C" {
    pub fn cx18_stop_all_captures(cx: *mut cx18);
}

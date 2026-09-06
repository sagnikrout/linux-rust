//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx18-fileops.h
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
// cx18 file operation functions
//
// Derived from ivtv-fileops.h
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
//
// Testing/Debugging
extern "C" {
    pub fn cx18_v4l2_open(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn cx18_v4l2_close(filp: *mut file) -> c_int;
}
extern "C" {
    pub fn cx18_v4l2_enc_poll(filp: *mut file, wait: *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn cx18_start_capture(id: *mut cx18_open_id) -> c_int;
}
extern "C" {
    pub fn cx18_stop_capture(s: *mut cx18_stream, gop_end: c_int);
}
extern "C" {
    pub fn cx18_mute(cx: *mut cx18);
}
extern "C" {
    pub fn cx18_unmute(cx: *mut cx18);
}
extern "C" {
    pub fn cx18_v4l2_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn cx18_clear_queue(s: *mut cx18_stream, state: vb2_buffer_state);
}
extern "C" {
    pub fn cx18_vb_timeout(t: *mut timer_list);
}
// Shared with cx18-alsa module
extern "C" {
    pub fn cx18_claim_stream(id: *mut cx18_open_id, type: c_int) -> c_int;
}
extern "C" {
    pub fn cx18_release_stream(s: *mut cx18_stream);
}

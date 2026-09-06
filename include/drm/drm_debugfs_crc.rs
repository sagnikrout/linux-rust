//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_debugfs_crc.h
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


//
// Copyright © 2016 Collabora Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const DRM_MAX_CRC_NR: c_int = 10;
//
// struct drm_crtc_crc_entry - entry describing a frame's content
// @has_frame_counter: whether the source was able to provide a frame number
// @frame: number of the frame this CRC is about, if @has_frame_counter is true
// @crcs: array of values that characterize the frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_crc_entry {
    pub has_frame_counter: bool,
    pub frame: u32,
    pub crcs: [u32; DRM_MAX_CRC_NR],
}

pub const DRM_CRC_ENTRIES_NR: c_int = 128;
//
// struct drm_crtc_crc - data supporting CRC capture on a given CRTC
// @lock: protects the fields in this struct
// @source: name of the currently configured source of CRCs
// @opened: whether userspace has opened the data file for reading
// @overflow: whether an overflow occurred
// @entries: array of entries, with size of %DRM_CRC_ENTRIES_NR
// @head: head of circular queue
// @tail: tail of circular queue
// @values_cnt: number of CRC values per entry, up to %DRM_MAX_CRC_NR
// @wq: workqueue used to synchronize reading and writing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_crtc_crc {
    pub lock: spinlock_t,
    pub source: *const c_char,
    pub overflow: bool opened,,
    pub entries: *mut drm_crtc_crc_entry,
    pub tail: int head,,
    pub values_cnt: usize,
    pub wq: wait_queue_head_t,
}


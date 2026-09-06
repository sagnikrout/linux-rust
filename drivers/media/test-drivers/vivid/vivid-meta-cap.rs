//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-meta-cap.h
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
// vivid-meta-cap.h - meta capture support functions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vivid_uvc_meta_buf {
    pub ns: __u64,
    pub sof: __u16,
    pub length: __u8,
    pub flags: __u8,
    pub /: *mut *mut __u8 buf[10]; / PTS(4)+STC(4)+SOF(2),
    pub __packed: },
    pub soe): *mut *mut vivid_buffer buf, u64,
    pub f): *mut v4l2_fmtdesc,
    pub f): *mut v4l2_format,
    pub vivid_meta_cap_qops: extern struct vb2_ops,

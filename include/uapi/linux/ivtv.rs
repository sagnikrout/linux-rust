//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ivtv.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//

// ivtv knows several distinct output modes: MPEG streaming,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivtv_dma_frame {
    pub /: *mut *mut v4l2_buf_type type; / V4L2_BUF_TYPE_VIDEO_OUTPUT,
    pub /: *mut *mut __u32 pixelformat; / 0 == same as destination,
    pub V4L2_BUF_TYPE_VIDEO_OUTPUT,: *mut *mut *mut void __user y_source; / if NULL and type ==,
    pub /: *mut *mut *mut void __user uv_source; / Unused for RGB pixelformats,
    pub src: v4l2_rect,
    pub dst: v4l2_rect,
    pub src_width: __u32,
    pub src_height: __u32,
}

// Select the passthrough mode (if the argument is non-zero). In the passthrough

// Deprecated defines: applications should use the defines from videodev2.h


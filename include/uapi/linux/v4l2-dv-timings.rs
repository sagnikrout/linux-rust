//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/v4l2-dv-timings.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// V4L2 DV timings header.
//
// Copyright (C) 2012-2016  Hans Verkuil <hverkuil@kernel.org>
//

// Sadly gcc versions older than 4.6 have a bug in how they initialize

// CEA-861-F timings (i.e. standard HDTV timings)

// Note: these are the nominal timings, for HDMI links this format is typically
// double-clocked to meet the minimum pixelclock requirements.

// Note: these are the nominal timings, for HDMI links this format is typically
// double-clocked to meet the minimum pixelclock requirements.

// VESA Discrete Monitor Timings as per version 1.0, revision 12

// VGA resolutions

// SVGA resolutions

// XGA resolutions

// XGA+ resolution

// WXGA resolutions

// SXGA resolutions

// SXGA+ resolutions

// WXGA+ resolutions

// UXGA resolutions

// WSXGA+ resolutions

// WUXGA resolutions

// WQXGA resolutions

// 4K resolutions

// SDI timings definitions
// SMPTE-125M


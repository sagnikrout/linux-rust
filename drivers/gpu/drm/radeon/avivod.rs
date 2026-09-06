//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/avivod.h
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
// Copyright 2009 Advanced Micro Devices, Inc.
// Copyright 2009 Red Hat Inc.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
pub const D1CRTC_CONTROL: c_uint = 0x6080;

pub const D1CRTC_STATUS: c_uint = 0x609c;
pub const D1CRTC_UPDATE_LOCK: c_uint = 0x60E8;
pub const D1GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x6110;
pub const D1GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x6118;
pub const D2CRTC_CONTROL: c_uint = 0x6880;
pub const D2CRTC_STATUS: c_uint = 0x689c;
pub const D2CRTC_UPDATE_LOCK: c_uint = 0x68E8;
pub const D2GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x6910;
pub const D2GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x6918;
pub const D1VGA_CONTROL: c_uint = 0x0330;

pub const D2VGA_CONTROL: c_uint = 0x0338;
pub const VGA_HDP_CONTROL: c_uint = 0x328;

pub const VGA_MEMORY_BASE_ADDRESS: c_uint = 0x0310;
pub const VGA_RENDER_CONTROL: c_uint = 0x0300;
pub const VGA_VSTATUS_CNTL_MASK: c_uint = 0x00030000;

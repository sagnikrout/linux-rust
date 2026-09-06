//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/resource/dcn31/dcn31_resource.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

// Macro flag: #define TO_DCN31_RES_POOL(pool)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn31_resource_pool {
    pub base: resource_pool,
}

// temp: B0 specific before switch to dcn313 headers

pub const regPHYPLLF_PIXCLK_RESYNC_CNTL: c_uint = 0x007e;
pub const regPHYPLLF_PIXCLK_RESYNC_CNTL_BASE_IDX: c_int = 1;
pub const regPHYPLLG_PIXCLK_RESYNC_CNTL: c_uint = 0x005f;
pub const regPHYPLLG_PIXCLK_RESYNC_CNTL_BASE_IDX: c_int = 1;
// PHYPLLF_PIXCLK_RESYNC_CNTL
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_RESYNC_ENABLE__SHIFT: c_uint = 0x0;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DEEP_COLOR_DTO_ENABLE_STATUS__SHIFT: c_uint = 0x1;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DCCG_DEEP_COLOR_CNTL__SHIFT: c_uint = 0x4;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_ENABLE__SHIFT: c_uint = 0x8;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_DOUBLE_RATE_ENABLE__SHIFT: c_uint = 0x9;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_RESYNC_ENABLE_MASK: c_uint = 0x00000001L;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DEEP_COLOR_DTO_ENABLE_STATUS_MASK: c_uint = 0x00000002L;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_DCCG_DEEP_COLOR_CNTL_MASK: c_uint = 0x00000030L;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_ENABLE_MASK: c_uint = 0x00000100L;
pub const PHYPLLF_PIXCLK_RESYNC_CNTL__PHYPLLF_PIXCLK_DOUBLE_RATE_ENABLE_MASK: c_uint = 0x00000200L;
// PHYPLLG_PIXCLK_RESYNC_CNTL
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_RESYNC_ENABLE__SHIFT: c_uint = 0x0;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DEEP_COLOR_DTO_ENABLE_STATUS__SHIFT: c_uint = 0x1;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DCCG_DEEP_COLOR_CNTL__SHIFT: c_uint = 0x4;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_ENABLE__SHIFT: c_uint = 0x8;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_DOUBLE_RATE_ENABLE__SHIFT: c_uint = 0x9;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_RESYNC_ENABLE_MASK: c_uint = 0x00000001L;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DEEP_COLOR_DTO_ENABLE_STATUS_MASK: c_uint = 0x00000002L;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_DCCG_DEEP_COLOR_CNTL_MASK: c_uint = 0x00000030L;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_ENABLE_MASK: c_uint = 0x00000100L;
pub const PHYPLLG_PIXCLK_RESYNC_CNTL__PHYPLLG_PIXCLK_DOUBLE_RATE_ENABLE_MASK: c_uint = 0x00000200L;


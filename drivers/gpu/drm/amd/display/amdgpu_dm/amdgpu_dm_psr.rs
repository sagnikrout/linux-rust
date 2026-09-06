//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_psr.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2021 Advanced Micro Devices, Inc.
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

// the number of pageflips before enabling psr
pub const AMDGPU_DM_PSR_ENTRY_DELAY: c_int = 5;
extern "C" {
    pub fn amdgpu_dm_set_psr_caps(link: *mut dc_link, aconnector: *mut amdgpu_dm_connector) -> bool;
}
extern "C" {
    pub fn amdgpu_dm_psr_is_active_allowed(dm: *mut amdgpu_display_manager) -> bool;
}

extern "C" {
    pub fn link_supports_psrsu(link: *mut dc_link) -> bool;
}
extern "C" {
    pub fn amdgpu_dm_psr_fill_caps(link: *mut dc_link, caps: *mut psr_caps);
}
extern "C" {
    pub fn amdgpu_dm_psr_get_dc_feature_mask() -> c_uint;
}
extern "C" {
    pub fn amdgpu_dm_psr_set_dc_feature_mask(feature_mask: c_uint);
}
extern "C" {
    pub fn amdgpu_dm_psr_get_dc_debug_mask() -> c_uint;
}
extern "C" {
    pub fn amdgpu_dm_psr_set_dc_debug_mask(debug_mask: c_uint);
}


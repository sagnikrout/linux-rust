//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm_backlight.h
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
// Copyright 2026 Advanced Micro Devices, Inc.
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
pub const AMDGPU_DM_DEFAULT_MIN_BACKLIGHT: c_int = 12;
pub const AMDGPU_DM_DEFAULT_MAX_BACKLIGHT: c_int = 255;

pub const AUX_BL_DEFAULT_TRANSITION_TIME_MS: c_int = 50;
extern "C" {
    pub fn amdgpu_dm_register_backlight_device(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn amdgpu_dm_update_connector_ext_caps(aconnector: *mut amdgpu_dm_connector);
}
extern "C" {
    pub fn amdgpu_dm_should_create_sysfs(aconnector: *mut amdgpu_dm_connector) -> bool;
}

extern "C" {
    pub fn amdgpu_dm_backlight_update_status(bd: *mut backlight_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_backlight_get_level(dm: *mut amdgpu_display_manager, bl_idx: c_int) -> u32;
}
extern "C" {
    pub fn amdgpu_dm_backlight_get_brightness(bd: *mut backlight_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_get_dc_debug_mask() -> c_uint;
}
extern "C" {
    pub fn amdgpu_dm_set_dc_debug_mask(val: c_uint);
}
extern "C" {
    pub fn amdgpu_dm_get_abm_level_param() -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_set_abm_level_param(val: c_int);
}
extern "C" {
    pub fn amdgpu_dm_get_backlight_param() -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_set_backlight_param(val: c_int);
}


//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_crtc_helper.h
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
// Copyright © 2006 Keith Packard
// Copyright © 2007-2008 Dave Airlie
// Copyright © 2007-2008 Intel Corporation
// Jesse Barnes <jesse.barnes@intel.com>
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
// The DRM mode setting helper functions are common code for drivers to use if
// they wish.  Drivers are not forced to use this code in their
// implementations but it would be useful if they code they do use at least
// provides a consistent interface and operation to userspace
//

extern "C" {
    pub fn drm_helper_disable_unused_functions(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_helper_crtc_in_use(crtc: *mut drm_crtc) -> bool;
}
extern "C" {
    pub fn drm_helper_encoder_in_use(encoder: *mut drm_encoder) -> bool;
}
extern "C" {
    pub fn drm_helper_connector_dpms(connector: *mut drm_connector, mode: c_int) -> c_int;
}
extern "C" {
    pub fn drm_helper_resume_force_mode(dev: *mut drm_device);
}
extern "C" {
    pub fn drm_helper_force_disable_all(dev: *mut drm_device) -> c_int;
}

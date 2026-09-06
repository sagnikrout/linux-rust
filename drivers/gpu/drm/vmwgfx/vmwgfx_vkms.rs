//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_vkms.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (c) 2024 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

extern "C" {
    pub fn vmw_vkms_init(vmw: *mut vmw_private);
}
extern "C" {
    pub fn vmw_vkms_cleanup(vmw: *mut vmw_private);
}
extern "C" {
    pub fn vmw_vkms_modeset_lock(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vmw_vkms_modeset_lock_relaxed(crtc: *mut drm_crtc) -> bool;
}
extern "C" {
    pub fn vmw_vkms_vblank_trylock(crtc: *mut drm_crtc) -> bool;
}
extern "C" {
    pub fn vmw_vkms_unlock(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vmw_vkms_handle_vblank_timeout(crtc: *mut drm_crtc) -> bool;
}
extern "C" {
    pub fn vmw_vkms_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn vmw_vkms_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vmw_vkms_crtc_init(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vmw_vkms_crtc_cleanup(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn vmw_vkms_crtc_atomic_flush(crtc: *mut drm_crtc, state: *mut drm_atomic_commit);
}

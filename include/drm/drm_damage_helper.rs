//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_damage_helper.h
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
// Copyright (c) 2018 VMware, Inc., Palo Alto, CA., USA
// All Rights Reserved.
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
// Authors:
// Deepak Rawat <drawat@vmware.com>
//

//
// drm_atomic_for_each_plane_damage - Iterator macro for plane damage.
// @iter: The iterator to advance.
// @rect: Return a rectangle in fb coordinate clipped to plane src.
//
// Note that if the first call to iterator macro return false then no need to do
// plane update. Iterator will return full plane src when damage is not passed
// by user-space.
//

//
// struct drm_atomic_helper_damage_iter - Closure structure for damage iterator.
//
// This structure tracks state needed to walk the list of plane damage clips.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_atomic_helper_damage_iter {
// private: Plane src in whole number.
    pub plane_src: drm_rect,
// private: Rectangles in plane damage blob.
    pub clips: *const drm_rect,
// private: Number of rectangles in plane damage blob.
    pub num_clips: u32,
// private: Current clip iterator is advancing on.
    pub curr_clip: u32,
// private: Whether need full plane update.
    pub full_update: bool,
}

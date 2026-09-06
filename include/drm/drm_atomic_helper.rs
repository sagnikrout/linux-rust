//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_atomic_helper.h
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
// Copyright (C) 2014 Red Hat
// Copyright (C) 2014 Intel Corp.
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
// Authors:
// Rob Clark <robdclark@gmail.com>
// Daniel Vetter <daniel.vetter@ffwll.ch>
//

//
// Drivers that don't allow primary plane scaling may pass this macro in place
// of the min/max scale parameters of the plane-state checker function.
//
// Due to src being in 16.16 fixed point and dest being in integer pixels,
// 1<<16 represents no scaling.
//

extern "C" {
    pub fn drm_atomic_helper_check_crtc_primary_plane(crtc_state: *mut drm_crtc_state) -> c_int;
}
extern "C" {
    pub fn drm_atomic_helper_commit_tail(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn drm_atomic_helper_commit_tail_rpm(state: *mut drm_atomic_commit);
}

extern "C" {
    pub fn drm_atomic_helper_commit_planes_on_crtc(old_crtc_state: *mut drm_crtc_state);
}
// nonblocking commit helpers
extern "C" {
    pub fn drm_atomic_helper_wait_for_dependencies(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn drm_atomic_helper_fake_vblank(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn drm_atomic_helper_commit_hw_done(state: *mut drm_atomic_commit);
}
extern "C" {
    pub fn drm_atomic_helper_commit_cleanup_done(state: *mut drm_atomic_commit);
}
// implementations for legacy interfaces
extern "C" {
    pub fn drm_atomic_helper_shutdown(dev: *mut drm_device);
}
//
// drm_atomic_crtc_for_each_plane - iterate over planes currently attached to CRTC
// @plane: the loop cursor
// @crtc:  the CRTC whose planes are iterated
//
// This iterates over the current state, useful (for example) when applying
// atomic state after it has been checked and swapped.  To iterate over the
// planes which *will* be attached (more useful in code called from
// &drm_mode_config_funcs.atomic_check) see
// drm_atomic_crtc_state_for_each_plane().
//

//
// drm_atomic_crtc_state_for_each_plane - iterate over attached planes in new state
// @plane: the loop cursor
// @crtc_state: the incoming CRTC state
//
// Similar to drm_crtc_for_each_plane(), but iterates the planes that will be
// attached if the specified state is applied.  Useful during for example
// in code called from &drm_mode_config_funcs.atomic_check operations, to
// validate the incoming state.
//

//
// drm_atomic_crtc_state_for_each_plane_state - iterate over attached planes in new state
// @plane: the loop cursor
// @plane_state: loop cursor for the plane's state, must be const
// @crtc_state: the incoming CRTC state
//
// Similar to drm_crtc_for_each_plane(), but iterates the planes that will be
// attached if the specified state is applied.  Useful during for example
// in code called from &drm_mode_config_funcs.atomic_check operations, to
// validate the incoming state.
//
// Compared to just drm_atomic_crtc_state_for_each_plane() this also fills in a
// const plane_state. This is useful when a driver just wants to peek at other
// active planes on this CRTC, but does not need to change it.
//

//
// drm_atomic_plane_enabling - check whether a plane is being enabled
// @old_plane_state: old atomic plane state
// @new_plane_state: new atomic plane state
//
// Checks the atomic state of a plane to determine whether it's being enabled
// or not. This also WARNs if it detects an invalid state (both CRTC and FB
// need to either both be NULL or both be non-NULL).
//
// RETURNS:
// True if the plane is being enabled, false otherwise.
//
// When enabling a plane, CRTC and FB should always be set together.
// Anything else should be considered a bug in the atomic core, so we
// gently warn about it.
//
// drm_atomic_plane_disabling - check whether a plane is being disabled
// @old_plane_state: old atomic plane state
// @new_plane_state: new atomic plane state
//
// Checks the atomic state of a plane to determine whether it's being disabled
// or not. This also WARNs if it detects an invalid state (both CRTC and FB
// need to either both be NULL or both be non-NULL).
//
// RETURNS:
// True if the plane is being disabled, false otherwise.
//
// When disabling a plane, CRTC and FB should always be NULL together.
// Anything else should be considered a bug in the atomic core, so we
// gently warn about it.
//

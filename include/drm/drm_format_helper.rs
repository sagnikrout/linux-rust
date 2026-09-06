//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_format_helper.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Noralf Trønnes
//

//
// struct drm_format_conv_state - Stores format-conversion state
//
// DRM helpers for format conversion store temporary state in
// struct drm_xfrm_buf. The buffer's resources can be reused
// among multiple conversion operations.
//
// All fields are considered private.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_format_conv_state {
// private:
    pub mem: *mut c_void,
    pub size: usize,
    pub preallocated: bool,
    pub tmp: },
}

//
// DRM_FORMAT_CONV_STATE_INIT - Initializer for struct drm_format_conv_state
//
// Initializes an instance of struct drm_format_conv_state to default values.
//

//
// DRM_FORMAT_CONV_STATE_INIT_PREALLOCATED - Initializer for struct drm_format_conv_state
// @_mem: The preallocated memory area
// @_size: The number of bytes in _mem
//
// Initializes an instance of struct drm_format_conv_state to preallocated
// storage. The caller is responsible for releasing the provided memory range.
//

extern "C" {
    pub fn drm_format_conv_state_init(state: *mut drm_format_conv_state);
}
extern "C" {
    pub fn drm_format_conv_state_release(state: *mut drm_format_conv_state);
}

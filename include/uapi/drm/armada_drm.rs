//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/armada_drm.h
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
// Copyright (C) 2012 Russell King
// With inspiration from the i915 driver
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

pub const DRM_ARMADA_GEM_CREATE: c_uint = 0x00;
pub const DRM_ARMADA_GEM_MMAP: c_uint = 0x02;
pub const DRM_ARMADA_GEM_PWRITE: c_uint = 0x03;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_armada_gem_create {
    pub handle: __u32,
    pub size: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_armada_gem_mmap {
    pub handle: __u32,
    pub pad: __u32,
    pub offset: __u64,
    pub size: __u64,
    pub addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_armada_gem_pwrite {
    pub ptr: __u64,
    pub handle: __u32,
    pub offset: __u32,
    pub size: __u32,
}


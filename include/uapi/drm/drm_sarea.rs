//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/drm/drm_sarea.h
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
// \file drm_sarea.h
// \brief SAREA definitions
//
// \author Michel Dänzer <michel@daenzer.net>
//
// Copyright 2002 Tungsten Graphics, Inc., Cedar Park, Texas.
// All Rights Reserved.
//

// SAREA area needs to be at least a page

pub const SAREA_MAX: c_uint = 0x2000U;

pub const SAREA_MAX: c_uint = 0x4000U;

pub const SAREA_MAX: c_uint = 0x10000U	/* 64kB */;

// Intel 830M driver needs at least 8k SAREA
pub const SAREA_MAX: c_uint = 0x2000U;

// Maximum number of drawables in the SAREA
pub const SAREA_MAX_DRAWABLES: c_int = 256;
pub const SAREA_DRAWABLE_CLAIMED_ENTRY: c_uint = 0x80000000;
// SAREA drawable
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sarea_drawable {
    pub stamp: c_uint,
    pub flags: c_uint,
}

// SAREA frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sarea_frame {
    pub x: c_uint,
    pub y: c_uint,
    pub width: c_uint,
    pub height: c_uint,
    pub fullscreen: c_uint,
}

// SAREA
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_sarea {
// first thing is always the DRM locking structure
    pub lock: drm_hw_lock,
// \todo Use readers/writer lock for drm_sarea::drawable_lock
    pub drawable_lock: drm_hw_lock,
    pub /: *mut *mut *mut drm_sarea_drawable drawableTable[SAREA_MAX_DRAWABLES]; /< drawables,
    pub /: *mut *mut *mut drm_sarea_frame frame; /< frame,
    pub dummy_context: drm_context_t,
}

pub type drm_sarea_drawable_t = drm_sarea_drawable;
pub type drm_sarea_frame_t = drm_sarea_frame;
pub type drm_sarea_t = drm_sarea;


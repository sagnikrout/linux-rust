//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/gem.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2014 Patrik Jakobsson
// All Rights Reserved.
//

//
// PSB GEM object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psb_gem_object {
    pub base: drm_gem_object,
    pub /: *mut *mut resource resource; / GTT resource for our allocation,
    pub /: *mut *mut u32 offset; / GTT offset of our object,
    pub /: *mut *mut int in_gart; / Currently in the GART (ref ct),
    pub /: *mut *mut bool stolen; / Backed from stolen RAM,
    pub /: *mut *mut bool mmapping; / Is mmappable,
    pub /: *mut *mut *mut *mut page pages; / Backing pages if present,
}

extern "C" {
    pub fn container_of(_arg: obj, psb_gem_object: struct, _arg: base) -> return;
}
extern "C" {
    pub fn psb_gem_pin(pobj: *mut psb_gem_object) -> c_int;
}
extern "C" {
    pub fn psb_gem_unpin(pobj: *mut psb_gem_object);
}
//
// Memory management
//
extern "C" {
    pub fn psb_gem_mm_init(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn psb_gem_mm_fini(dev: *mut drm_device);
}
extern "C" {
    pub fn psb_gem_mm_resume(dev: *mut drm_device) -> c_int;
}

//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/omap_gem.h
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
// omap_gem.h -- OMAP DRM GEM Object Management
//
// Copyright (C) 2011 Texas Instruments
// Author: Rob Clark <rob@ti.com>
//

// Initialization and Cleanup
extern "C" {
    pub fn omap_gem_init(dev: *mut drm_device);
}
extern "C" {
    pub fn omap_gem_deinit(dev: *mut drm_device);
}

extern "C" {
    pub fn omap_gem_resume(dev: *mut drm_device) -> c_int;
}

extern "C" {
    pub fn omap_gem_describe(obj: *mut drm_gem_object, m: *mut seq_file);
}
extern "C" {
    pub fn omap_gem_describe_objects(list: *mut list_head, m: *mut seq_file);
}

// GEM Object Creation and Deletion
// Dumb Buffers Interface
// mmap() Interface
extern "C" {
    pub fn omap_gem_mmap_offset(obj: *mut drm_gem_object) -> u64;
}
extern "C" {
    pub fn omap_gem_mmap_size(obj: *mut drm_gem_object) -> usize;
}
// PRIME Interface
extern "C" {
    pub fn omap_gem_roll(obj: *mut drm_gem_object, roll: u32) -> c_int;
}
extern "C" {
    pub fn omap_gem_cpu_sync_page(obj: *mut drm_gem_object, pgoff: c_int);
}
extern "C" {
    pub fn omap_gem_pin(obj: *mut drm_gem_object, dma_addr: *mut dma_addr_t) -> c_int;
}
extern "C" {
    pub fn omap_gem_unpin(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn omap_gem_put_pages(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn omap_gem_flags(obj: *mut drm_gem_object) -> u32;
}
extern "C" {
    pub fn omap_gem_tiled_stride(obj: *mut drm_gem_object, orient: u32) -> c_int;
}
extern "C" {
    pub fn omap_gem_put_sg(obj: *mut drm_gem_object, sgt: *mut sg_table);
}

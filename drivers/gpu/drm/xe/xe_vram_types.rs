//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_vram_types.h
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
// Copyright © 2025 Intel Corporation
//

//
// struct xe_vram_region - memory region structure
// This is used to describe a memory region in xe
// device, such as HBM memory or CXL extension memory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_vram_region {
// @xe: Back pointer to xe device
    pub xe: *mut xe_device,
//
// @id: VRAM region instance id
//
// The value should be unique for VRAM region.
//
    pub id: u8,
// @io_start: IO start address of this VRAM instance
    pub io_start: resource_size_t,
//
// @io_size: IO size of this VRAM instance
//
// This represents how much of this VRAM we can access
// via the CPU through the VRAM BAR. This can be smaller
// than @usable_size, in which case only part of VRAM is CPU
// accessible (typically the first 256M). This
// configuration is known as small-bar.
//
    pub io_size: resource_size_t,
// @dpa_base: This memory regions's DPA (device physical address) base
    pub dpa_base: resource_size_t,
//
// @usable_size: usable size of VRAM
//
// Usable size of VRAM excluding reserved portions
// (e.g stolen mem)
//
    pub usable_size: resource_size_t,
//
// @actual_physical_size: Actual VRAM size
//
// Actual VRAM size including reserved portions
// (e.g stolen mem)
//
    pub actual_physical_size: resource_size_t,
// @mapping: pointer to VRAM mappable space
    pub mapping: *mut void __iomem,
// @ttm: VRAM TTM manager
    pub ttm: xe_ttm_vram_mgr,
// @placement: TTM placement dedicated for this region
    pub placement: u32,

// @migrate: Back pointer to migrate
    pub migrate: *mut xe_migrate,
// @dpagemap_cache: drm_pagemap cache.
    pub dpagemap_cache: *mut drm_pagemap_cache,

}

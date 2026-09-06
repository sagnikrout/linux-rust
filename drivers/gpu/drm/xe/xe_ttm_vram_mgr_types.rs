//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_ttm_vram_mgr_types.h
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
// Copyright © 2022 Intel Corporation
//

//
// struct xe_ttm_vram_mgr - Xe TTM VRAM manager
//
// Manages placement of TTM resource in VRAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ttm_vram_mgr {
// @manager: Base TTM resource manager
    pub manager: ttm_resource_manager,
// @mm: DRM buddy allocator which manages the VRAM
    pub mm: gpu_buddy,
// @visible_size: Proped size of the CPU visible portion
    pub visible_size: u64,
// @visible_avail: CPU visible portion still unallocated
    pub visible_avail: u64,
// @default_page_size: default page size
    pub default_page_size: u64,
// @lock: protects allocations of VRAM
    pub lock: mutex,
// @mem_type: The TTM memory type
    pub mem_type: u32,
}

//
// struct xe_ttm_vram_mgr_resource - Xe TTM VRAM resource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_ttm_vram_mgr_resource {
// @base: Base TTM resource
    pub base: ttm_resource,
// @blocks: list of DRM buddy blocks
    pub blocks: list_head,
// @used_visible_size: How many CPU visible bytes this resource is using
    pub used_visible_size: u64,
// @flags: flags associated with the resource
    pub flags: c_ulong,
}

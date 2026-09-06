//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/loongson/lsdc_ttm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2023 Loongson Technology Corporation Limited
//

pub const LSDC_GEM_DOMAIN_SYSTEM: c_uint = 0x1;
pub const LSDC_GEM_DOMAIN_GTT: c_uint = 0x2;
pub const LSDC_GEM_DOMAIN_VRAM: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdc_bo {
    pub tbo: ttm_buffer_object,
// Protected by gem.mutex
    pub list: list_head,
    pub map: iosys_map,
    pub vmap_count: c_uint,
// cross device driver sharing reference count
    pub sharing_count: c_uint,
    pub kmap: ttm_bo_kmap_obj,
    pub kptr: *mut c_void,
    pub is_iomem: bool,
    pub size: usize,
    pub initial_domain: u32,
    pub placement: ttm_placement,
    pub placements: [ttm_place; 4],
}

extern "C" {
    pub fn container_of(_arg: gem, ttm_buffer_object: struct, _arg: base) -> return;
}
extern "C" {
    pub fn container_of(_arg: tbo, lsdc_bo: struct, _arg: tbo) -> return;
}
extern "C" {
    pub fn container_of(_arg: gem, lsdc_bo: struct, _arg: tbo.base) -> return;
}
extern "C" {
    pub fn lsdc_bo_free_kernel_pinned(lbo: *mut lsdc_bo);
}
extern "C" {
    pub fn lsdc_bo_reserve(lbo: *mut lsdc_bo) -> c_int;
}
extern "C" {
    pub fn lsdc_bo_unreserve(lbo: *mut lsdc_bo);
}
extern "C" {
    pub fn lsdc_bo_pin(lbo: *mut lsdc_bo, domain: u32, gpu_addr: *mut u64) -> c_int;
}
extern "C" {
    pub fn lsdc_bo_unpin(lbo: *mut lsdc_bo);
}
extern "C" {
    pub fn lsdc_bo_ref(lbo: *mut lsdc_bo);
}
extern "C" {
    pub fn lsdc_bo_unref(lbo: *mut lsdc_bo);
}
extern "C" {
    pub fn lsdc_bo_gpu_offset(lbo: *mut lsdc_bo) -> u64;
}
extern "C" {
    pub fn lsdc_bo_size(lbo: *mut lsdc_bo) -> usize;
}
extern "C" {
    pub fn lsdc_bo_kmap(lbo: *mut lsdc_bo) -> c_int;
}
extern "C" {
    pub fn lsdc_bo_kunmap(lbo: *mut lsdc_bo);
}
extern "C" {
    pub fn lsdc_bo_clear(lbo: *mut lsdc_bo);
}
extern "C" {
    pub fn lsdc_bo_evict_vram(ddev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn lsdc_ttm_init(ldev: *mut lsdc_device) -> c_int;
}
extern "C" {
    pub fn lsdc_ttm_debugfs_init(ldev: *mut lsdc_device);
}

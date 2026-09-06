//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_svm.h
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
// Copyright © 2024 Intel Corporation
//

// struct xe_svm_range - SVM range
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_svm_range {
// @base: base drm_gpusvm_range
    pub base: drm_gpusvm_range,
// @pages: Page/DMA mapping state for this range (single drm_device).
    pub pages: drm_gpusvm_pages,
//
// @garbage_collector_link: Link into VM's garbage collect SVM range
// list. Protected by VM's garbage collect lock.
//
    pub garbage_collector_link: list_head,
//
// @tile_present: Tile mask of binding is present for this range.
// Protected by GPU SVM notifier lock.
//
    pub tile_present: u8,
//
// @tile_invalidated: Tile mask of binding is invalidated for this
// range. Protected by GPU SVM notifier lock.
//
    pub tile_invalidated: u8,
}

//
// struct xe_pagemap - Manages xe device_private memory for SVM.
// @pagemap: The struct dev_pagemap providing the struct pages.
// @dpagemap: The drm_pagemap managing allocation and migration.
// @destroy_work: Handles asnynchronous destruction and caching.
// @peer: Used for pagemap owner computation.
// @hpa_base: The host physical address base for the managemd memory.
// @vr: Backpointer to the xe_vram region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_pagemap {
    pub pagemap: dev_pagemap,
    pub dpagemap: drm_pagemap,
    pub destroy_work: work_struct,
    pub peer: drm_pagemap_peer,
    pub hpa_base: resource_size_t,
    pub vr: *mut xe_vram_region,
}

//
// xe_svm_range_pages_valid() - SVM range pages valid
// @range: SVM range
//
// Return: True if SVM range pages are valid, False otherwise
//
extern "C" {
    pub fn drm_gpusvm_pages_valid(_arg: range->base.gpusvm, _arg: &range->pages) -> return;
}
extern "C" {
    pub fn xe_devm_add(tile: *mut xe_tile, vr: *mut xe_vram_region) -> c_int;
}
extern "C" {
    pub fn xe_svm_init(vm: *mut xe_vm) -> c_int;
}
extern "C" {
    pub fn xe_svm_fini(vm: *mut xe_vm);
}
extern "C" {
    pub fn xe_svm_close(vm: *mut xe_vm);
}
extern "C" {
    pub fn xe_svm_has_mapping(vm: *mut xe_vm, start: u64, end: u64) -> bool;
}
extern "C" {
    pub fn xe_svm_bo_evict(bo: *mut xe_bo) -> c_int;
}
extern "C" {
    pub fn xe_svm_range_debug(range: *mut xe_svm_range, operation: *const c_char);
}
extern "C" {
    pub fn xe_svm_range_migrate_to_smem(vm: *mut xe_vm, range: *mut xe_svm_range);
}
extern "C" {
    pub fn xe_svm_find_vma_start(vm: *mut xe_vm, addr: u64, end: u64, vma: *mut xe_vma) -> u64;
}
extern "C" {
    pub fn xe_svm_unmap_address_range(vm: *mut xe_vm, start: u64, end: u64);
}
extern "C" {
    pub fn xe_svm_ranges_zap_ptes_in_range(vm: *mut xe_vm, start: u64, end: u64) -> u8;
}
//
// xe_svm_range_has_dma_mapping() - SVM range has DMA mapping
// @range: SVM range
//
// Return: True if SVM range has a DMA mapping, False otherwise
//
// to_xe_range - Convert a drm_gpusvm_range pointer to a xe_svm_range
// @r: Pointer to the drm_gpusvm_range structure
//
// This function takes a pointer to a drm_gpusvm_range structure and
// converts it to a pointer to the containing xe_svm_range structure.
//
// Return: Pointer to the xe_svm_range structure
//
extern "C" {
    pub fn container_of(_arg: r, xe_svm_range: struct, _arg: base) -> return;
}
//
// xe_svm_range_start() - SVM range start address
// @range: SVM range
//
// Return: start address of range.
//
extern "C" {
    pub fn drm_gpusvm_range_start(_arg: &range->base) -> return;
}
//
// xe_svm_range_end() - SVM range end address
// @range: SVM range
//
// Return: end address of range.
//
extern "C" {
    pub fn drm_gpusvm_range_end(_arg: &range->base) -> return;
}
//
// xe_svm_range_size() - SVM range size
// @range: SVM range
//
// Return: Size of range.
//
extern "C" {
    pub fn drm_gpusvm_range_size(_arg: &range->base) -> return;
}
extern "C" {
    pub fn xe_svm_flush(vm: *mut xe_vm);
}
extern "C" {
    pub fn xe_pagemap_shrinker_create(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_pagemap_cache_create(tile: *mut xe_tile) -> c_int;
}

pub const XE_INTERCONNECT_VRAM: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_svm_range {
    pub itree: interval_tree_node,
    pub base: },
    pub dma_addr: *const drm_pagemap_addr,
    pub pages: },
    pub tile_present: u32,
    pub tile_invalidated: u32,
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}

//
// Assert the svm notifier_lock is held. Read mode by default; write mode
// when CONFIG_DRM_XE_USERPTR_INVAL_INJECT is on, because that path forces
// a userptr invalidation that ends in drm_gpusvm_unmap_pages() with
// ctx->in_notifier=true, which requires the lock held for write.
//


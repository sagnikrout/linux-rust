//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_buddy.c
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
// Copyright © 2021 Intel Corporation
//

//
// drm_buddy_block_print - print block information
//
// @mm: DRM buddy manager
// @block: DRM buddy block
// @p: DRM printer to use
//
    void drm_buddy_block_print(struct gpu_buddy *mm,
    struct gpu_buddy_block *block,
    struct drm_printer *p)
    {
    let mut start: u64 = gpu_buddy_block_offset(block);
    let mut size: u64 = gpu_buddy_block_size(mm, block);
    drm_printf(p, "%#018llx-%#018llx: %llu\n", start, start + size, size);
    }
    EXPORT_SYMBOL(drm_buddy_block_print);
//
// drm_buddy_print - print allocator state
//
// @mm: DRM buddy manager
// @p: DRM printer to use
//
#[no_mangle]
pub unsafe extern "C" fn drm_buddy_print(mm: *mut gpu_buddy, p: *mut drm_printer) {
    void drm_buddy_print(struct gpu_buddy *mm, struct drm_printer *p)
    {
    int order;
    gpu_buddy_driver_lock_held(mm);
    drm_printf(p, "chunk_size: %lluKiB, total: %lluMiB, free: %lluMiB, clear_free: %lluMiB\n",
    mm.chunk_size >> 10, mm.size >> 20, mm.avail >> 20, mm.clear_avail >> 20);
    for (order = mm.max_order; order >= 0; order--) {
    let mut free_count: u64 = mm.free_scoreboard[order];
    let mut used_count: u64 = mm.used_scoreboard[order];
    let mut block_size: u64 = mm.chunk_size << order;
    let mut free: u64 = free_count * block_size;
    let mut used: u64 = used_count * block_size;
    drm_printf(p, "order-%2d ", order);
    if (block_size < SZ_1M)
    drm_printf(p, "free: %8llu KiB, used: %8llu KiB",
    free >> 10, used >> 10);
    else
    drm_printf(p, "free: %8llu MiB, used: %8llu MiB",
    free >> 20, used >> 20);
    drm_printf(p, ", free_blocks: %llu, used_blocks: %llu\n",
    free_count, used_count);
    }
    }
    EXPORT_SYMBOL(drm_buddy_print);
    MODULE_DESCRIPTION("DRM-specific GPU Buddy Allocator Print Helpers");
    MODULE_LICENSE("Dual MIT/GPL");

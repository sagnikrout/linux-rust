//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/panthor/panthor_mmu.h
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


// SPDX-License-Identifier: GPL-2.0 or MIT
// Copyright 2019 Linaro, Ltd, Rob Herring <robh@kernel.org>
// Copyright 2023 Collabora ltd.
// Copyright 2025 ARM Limited. All rights reserved.

extern "C" {
    pub fn panthor_mmu_init(ptdev: *mut panthor_device) -> c_int;
}
extern "C" {
    pub fn panthor_mmu_unplug(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_mmu_pre_reset(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_mmu_post_reset(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_mmu_suspend(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_mmu_resume(ptdev: *mut panthor_device);
}
extern "C" {
    pub fn panthor_vm_unmap_range(vm: *mut panthor_vm, va: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn panthor_vm_active(vm: *mut panthor_vm) -> c_int;
}
extern "C" {
    pub fn panthor_vm_idle(vm: *mut panthor_vm);
}
extern "C" {
    pub fn panthor_vm_page_size(vm: *mut panthor_vm) -> u32;
}
extern "C" {
    pub fn panthor_vm_as(vm: *mut panthor_vm) -> c_int;
}
extern "C" {
    pub fn panthor_vm_heaps_sizes(pfile: *mut panthor_file, stats: *mut drm_memory_stats);
}
extern "C" {
    pub fn panthor_vm_put(vm: *mut panthor_vm);
}
extern "C" {
    pub fn panthor_vm_update_bo_reclaim_lru_locked(bo: *mut panthor_gem_object);
}
extern "C" {
    pub fn panthor_vm_evict_bo_mappings_locked(bo: *mut panthor_gem_object) -> c_int;
}
extern "C" {
    pub fn panthor_vm_pool_destroy(pfile: *mut panthor_file);
}
extern "C" {
    pub fn panthor_vm_pool_create(pfile: *mut panthor_file) -> c_int;
}
extern "C" {
    pub fn panthor_vm_pool_destroy_vm(pool: *mut panthor_vm_pool, handle: u32) -> c_int;
}
extern "C" {
    pub fn panthor_vm_has_unhandled_faults(vm: *mut panthor_vm) -> bool;
}
extern "C" {
    pub fn panthor_vm_is_unusable(vm: *mut panthor_vm) -> bool;
}
//
// PANTHOR_VM_KERNEL_AUTO_VA: Use this magic address when you want the GEM
// logic to auto-allocate the virtual address in the reserved kernel VA range.
//

extern "C" {
    pub fn panthor_vm_free_va(vm: *mut panthor_vm, va_node: *mut drm_mm_node);
}
extern "C" {
    pub fn panthor_vm_bind_job_put(job: *mut drm_sched_job);
}
extern "C" {
    pub fn panthor_vm_bind_job_update_resvs(exec: *mut drm_exec, job: *mut drm_sched_job);
}
extern "C" {
    pub fn panthor_mmu_pt_cache_init() -> c_int;
}
extern "C" {
    pub fn panthor_mmu_pt_cache_fini();
}

extern "C" {
    pub fn panthor_mmu_debugfs_init(minor: *mut drm_minor);
}


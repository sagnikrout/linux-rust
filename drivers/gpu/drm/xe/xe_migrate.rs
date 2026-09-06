//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_migrate.h
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
// Copyright © 2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_migrate_copy_dir {
    XE_MIGRATE_COPY_TO_VRAM,
    XE_MIGRATE_COPY_TO_SRAM,
}

//
// struct xe_migrate_pt_update_ops - Callbacks for the
// xe_migrate_update_pgtables() function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_migrate_pt_update_ops {
//
// @populate: Populate a command buffer or page-table with ptes.
// @pt_update: Embeddable callback argument.
// @tile: The tile for the current operation.
// @map: struct iosys_map into the memory to be populated.
// @pos: If @map is NULL, map into the memory to be populated.
// @ofs: qword offset into @map, unused if @map is NULL.
// @num_qwords: Number of qwords to write.
// @update: Information about the PTEs to be inserted.
//
// This interface is intended to be used as a callback into the
// page-table system to populate command buffers or shared
// page-tables with PTEs.
//
    pub update): *const xe_vm_pgtable_update,
//
// @clear: Clear a command buffer or page-table with ptes.
// @pt_update: Embeddable callback argument.
// @tile: The tile for the current operation.
// @map: struct iosys_map into the memory to be populated.
// @pos: If @map is NULL, map into the memory to be populated.
// @ofs: qword offset into @map, unused if @map is NULL.
// @num_qwords: Number of qwords to write.
// @update: Information about the PTEs to be inserted.
//
// This interface is intended to be used as a callback into the
// page-table system to populate command buffers or shared
// page-tables with PTEs.
//
    pub update): *const xe_vm_pgtable_update,
//
// @pre_commit: Callback to be called just before arming the
// sched_job.
// @pt_update: Pointer to embeddable callback argument.
//
// Return: 0 on success, negative error code on error.
//
    pub pt_update): *mut *mut int (pre_commit)(struct xe_migrate_pt_update,
}

//
// struct xe_migrate_pt_update - Argument to the
// struct xe_migrate_pt_update_ops callbacks.
//
// Intended to be subclassed to support additional arguments if necessary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_migrate_pt_update {
// @ops: Pointer to the struct xe_migrate_pt_update_ops callbacks
    pub ops: *const xe_migrate_pt_update_ops,
// @vops: VMA operations
    pub vops: *mut xe_vma_ops,
// @job: The job if a GPU page-table update. NULL otherwise
    pub job: *mut xe_sched_job,
//
// @ijob: The TLB invalidation job for primary GT. NULL otherwise
//
    pub ijob: *mut xe_tlb_inval_job,
//
// @mjob: The TLB invalidation job for media GT. NULL otherwise
//
    pub mjob: *mut xe_tlb_inval_job,
// @tile_id: Tile ID of the update
    pub tile_id: u8,
}

extern "C" {
    pub fn xe_migrate_init(m: *mut xe_migrate) -> c_int;
}

extern "C" {
    pub fn xe_migrate_wait(m: *mut xe_migrate);
}

extern "C" {
    pub fn xe_migrate_job_lock_assert(q: *mut xe_exec_queue);
}

extern "C" {
    pub fn xe_migrate_job_lock(m: *mut xe_migrate, q: *mut xe_exec_queue);
}
extern "C" {
    pub fn xe_migrate_job_unlock(m: *mut xe_migrate, q: *mut xe_exec_queue);
}

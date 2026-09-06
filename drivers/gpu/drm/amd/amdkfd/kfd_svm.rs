//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_svm.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2020-2021 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

// Macro flag: #define SVM_ADEV_PGMAP_OWNER(adev)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svm_range_bo {
    pub bo: amdgpu_bo,
    pub kref: kref,
    pub /: *mut *mut list_head range_list; / all svm ranges shared this bo,
    pub list_lock: spinlock_t,
    pub mm: *mut mm_struct,
    pub evicting: u32,
    pub release_work: work_struct,
    pub node: *mut kfd_node,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum svm_work_list_ops {
    SVM_OP_NULL,
    SVM_OP_UNMAP_RANGE,
    SVM_OP_UPDATE_RANGE_NOTIFIER,
    SVM_OP_UPDATE_RANGE_NOTIFIER_AND_MAP,
    SVM_OP_ADD_RANGE,
    SVM_OP_ADD_RANGE_AND_MAP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct svm_work_list_item {
    pub op: svm_work_list_ops,
    pub mm: *mut mm_struct,
}

//
// struct svm_range - shared virtual memory range
//
// @svms:       list of svm ranges, structure defined in kfd_process
// @migrate_mutex: to serialize range migration, validation and mapping update
// @start:      range start address in pages
// @last:       range last address in pages
// @it_node:    node [start, last] stored in interval tree, start, last are page
// aligned, page size is (last - start + 1)
// @list:       link list node, used to scan all ranges of svms
// @update_list:link list node used to add to update_list
// @mapping:    bo_va mapping structure to create and update GPU page table
// @npages:     number of pages
// @vram_pages: vram pages number in this svm_range
// @dma_addr:   dma mapping address on each GPU for system memory physical page
// @ttm_res:    vram ttm resource map
// @offset:     range start offset within mm_nodes
// @svm_bo:     struct to manage splited amdgpu_bo
// @svm_bo_list:link list node, to scan all ranges which share same svm_bo
// @lock:       protect prange start, last, child_list, svm_bo_list
// @saved_flags:save/restore current PF_MEMALLOC flags
// @flags:      flags defined as KFD_IOCTL_SVM_FLAG_
// @perferred_loc: perferred location, 0 for CPU, or GPU id
// @perfetch_loc: last prefetch location, 0 for CPU, or GPU id
// @actual_loc: this svm_range location. 0: all pages are from sys ram;
// GPU id: this svm_range may include vram pages from GPU with
// id actual_loc.
// @granularity:migration granularity, log2 num pages
// @invalid:    not 0 means cpu page table is invalidated
// @validate_timestamp: system timestamp when range is validated
// @notifier:   register mmu interval notifier
// @work_item:  deferred work item information
// @deferred_list: list header used to add range to deferred list
// @child_list: list header for split ranges which are not added to svms yet
// @bitmap_access: index bitmap of GPUs which can access the range
// @bitmap_aip: index bitmap of GPUs which can access the range in place
// @bitmap_needs_unmap: index bitmap of GPUs which currently set NO_ACCESS
// @bitmap_mapped: index bitmap of GPUs which currently have the range mapped
// @mapping_done: true if range_validate_and_map complete successfully
//
// Data structure for virtual memory range shared by CPU and GPUs, it can be
// allocated from system memory ram or device vram, and migrate from ram to vram
// or from vram to ram.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct svm_range {
    pub svms: *mut svm_range_list,
    pub migrate_mutex: mutex,
    pub start: c_ulong,
    pub last: c_ulong,
    pub it_node: interval_tree_node,
    pub list: list_head,
    pub update_list: list_head,
    pub npages: u64,
    pub vram_pages: u64,
    pub dma_addr: [*mut dma_addr_t; MAX_GPU_INSTANCE],
    pub ttm_res: *mut ttm_resource,
    pub offset: u64,
    pub svm_bo: *mut svm_range_bo,
    pub svm_bo_list: list_head,
    pub lock: mutex,
    pub saved_flags: c_uint,
    pub flags: u32,
    pub preferred_loc: u32,
    pub prefetch_loc: u32,
    pub actual_loc: u32,
    pub granularity: u8,
    pub invalid: core::sync::atomic::AtomicI32,
    pub validate_timestamp: ktime_t,
    pub notifier: mmu_interval_notifier,
    pub work_item: svm_work_list_item,
    pub deferred_list: list_head,
    pub child_list: list_head,
    pub MAX_GPU_INSTANCE): DECLARE_BITMAP(bitmap_access,,
    pub MAX_GPU_INSTANCE): DECLARE_BITMAP(bitmap_aip,,
    pub MAX_GPU_INSTANCE): DECLARE_BITMAP(bitmap_needs_unmap,,
    pub MAX_GPU_INSTANCE): DECLARE_BITMAP(bitmap_mapped,,
    pub mapping_done: bool,
    pub queue_refcount: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn svm_range_list_init(p: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn svm_range_list_fini(p: *mut kfd_process);
}
extern "C" {
    pub fn svm_range_bo_destroy(tbo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn svm_range_vram_node_free(prange: *mut svm_range);
}
extern "C" {
    pub fn svm_range_evict_svm_bo(bo: *mut amdgpu_bo) -> c_int;
}
extern "C" {
    pub fn schedule_deferred_list_work(svms: *mut svm_range_list);
}
extern "C" {
    pub fn svm_range_dma_unmap(prange: *mut svm_range);
}
extern "C" {
    pub fn kfd_criu_resume_svm(p: *mut kfd_process) -> c_int;
}
extern "C" {
    pub fn svm_range_list_lock_and_flush_work(svms: *mut svm_range_list, mm: *mut mm_struct);
}
// SVM API and HMM page migration work together, device memory type
// is initialized to not 0 when page migration register device memory.
//

extern "C" {
    pub fn svm_range_bo_unref_async(svm_bo: *mut svm_range_bo);
}
extern "C" {
    pub fn svm_range_set_max_pages(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn svm_range_switch_xnack_reserve_mem(p: *mut kfd_process, xnack_enabled: bool) -> c_int;
}

// empty
// num_svm_ranges = 0;
// svm_priv_data_size = 0;


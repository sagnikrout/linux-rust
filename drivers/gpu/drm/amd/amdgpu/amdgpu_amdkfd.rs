//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_amdkfd.h
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


//
// Copyright 2014 Advanced Micro Devices, Inc.
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
// amdgpu_amdkfd.h defines the private interface between amdgpu and amdkfd.

// Macro flag: #define AMDGPU_AMDKFD_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TLB_FLUSH_TYPE {
    TLB_FLUSH_LEGACY = 0,
    TLB_FLUSH_LIGHTWEIGHT,
    TLB_FLUSH_HEAVYWEIGHT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfd_mem_attachment_type {
    KFD_MEM_ATT_SHARED,	/* Share kgd_mem->bo or another attachment's */
    KFD_MEM_ATT_USERPTR,	/* SG bo to DMA map pages from a userptr bo */
    KFD_MEM_ATT_DMABUF,	/* DMAbuf to DMA map TTM BOs */
    KFD_MEM_ATT_SG		/* Tag to DMA map SG BOs */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfd_mem_attachment {
    pub list: list_head,
    pub type: kfd_mem_attachment_type,
    pub is_mapped: bool,
    pub bo_va: *mut amdgpu_bo_va,
    pub adev: *mut amdgpu_device,
    pub va: u64,
    pub pte_flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kgd_mem {
    pub lock: mutex,
    pub bo: *mut amdgpu_bo,
    pub dmabuf: *mut dma_buf,
    pub range: *mut amdgpu_hmm_range,
    pub attachments: list_head,
// protected by amdkfd_process_info.lock
    pub validate_list: list_head,
    pub domain: u32,
    pub mapped_to_gpu_memory: c_uint,
    pub va: u64,
    pub alloc_flags: u32,
    pub invalid: u32,
    pub process_info: *mut amdkfd_process_info,
    pub sync: amdgpu_sync,
    pub gem_handle: u32,
    pub aql_queue: bool,
    pub is_imported: bool,
}

// KFD Memory Eviction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_amdkfd_fence {
    pub base: dma_fence,
    pub mm: *mut mm_struct,
    pub lock: spinlock_t,
    pub timeline_name: [c_char; TASK_COMM_LEN],
    pub context_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_kfd_dev {
    pub dev: *mut kfd_dev,
    pub vram_used: [i64; MAX_XCP],
    pub vram_used_aligned: [u64; MAX_XCP],
    pub init_complete: bool,
    pub reset_work: work_struct,
// Client for KFD BO GEM handle allocations
    pub client: drm_client_dev,
// HMM page migration MEMORY_DEVICE_PRIVATE mapping
// Must be last --ends in a flexible-array member.
//
    pub pgmap: dev_pagemap,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kgd_engine_type {
    KGD_ENGINE_PFP = 1,
    KGD_ENGINE_ME,
    KGD_ENGINE_CE,
    KGD_ENGINE_MEC1,
    KGD_ENGINE_MEC2,
    KGD_ENGINE_RLC,
    KGD_ENGINE_SDMA1,
    KGD_ENGINE_SDMA2,
    KGD_ENGINE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdkfd_process_info {
// List head of all VMs that belong to a KFD process
    pub vm_list_head: list_head,
// List head for all KFD BOs that belong to a KFD process.
    pub kfd_bo_list: list_head,
// List of userptr BOs that are valid or invalid
    pub userptr_valid_list: list_head,
    pub userptr_inval_list: list_head,
// Lock to protect kfd_bo_list
    pub lock: mutex,
// Number of VMs
    pub n_vms: c_uint,
// Eviction Fence
    pub eviction_fence: *mut amdgpu_amdkfd_fence,
// MMU-notifier related fields
    pub notifier_lock: mutex,
    pub evicted_bos: u32,
// kfd context id
    pub context_id: u16,
    pub restore_userptr_work: delayed_work,
    pub pid: *mut pid,
    pub block_mmu_notifications: bool,
}

extern "C" {
    pub fn amdgpu_amdkfd_init() -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_fini();
}
extern "C" {
    pub fn amdgpu_amdkfd_teardown_processes(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_suspend(adev: *mut amdgpu_device, suspend_proc: bool);
}
extern "C" {
    pub fn amdgpu_amdkfd_resume(adev: *mut amdgpu_device, resume_proc: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_suspend_process(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_resume_process(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_device_probe(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_device_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_device_fini_sw(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_check_and_lock_kfd(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_unlock_kfd(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_set_compute_idle(adev: *mut amdgpu_device, idle: bool);
}
extern "C" {
    pub fn amdgpu_amdkfd_have_atomics_support(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_amdkfd_is_kfd_vmid(adev: *mut amdgpu_device, vmid: u32) -> bool;
}
extern "C" {
    pub fn amdgpu_amdkfd_post_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_gpu_reset(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_drm_client_create(adev: *mut amdgpu_device) -> c_int;
}

extern "C" {
    pub fn kfd_debugfs_kfd_mem_limits(m: *mut seq_file, data: *mut c_void) -> c_int;
}

extern "C" {
    pub fn amdkfd_fence_check_mm(f: *mut dma_fence, mm: *mut mm_struct) -> bool;
}
extern "C" {
    pub fn amdgpu_amdkfd_remove_all_eviction_fences(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn amdgpu_amdkfd_set_sigbus_delay(task: *mut task_struct, ms: u32) -> c_int;
}

// Shared API
extern "C" {
    pub fn amdgpu_amdkfd_free_kernel_mem(adev: *mut amdgpu_device, mem_obj: *mut c_void);
}
extern "C" {
    pub fn amdgpu_amdkfd_free_gws(adev: *mut amdgpu_device, mem_obj: *mut c_void);
}
extern "C" {
    pub fn amdgpu_amdkfd_add_gws_to_process(info: *mut c_void, gws: *mut c_void, mem: *mut kgd_mem) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_remove_gws_from_process(info: *mut c_void, mem: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_get_gpu_clock_counter(adev: *mut amdgpu_device) -> u64;
}
extern "C" {
    pub fn amdgpu_amdkfd_get_max_engine_clock_in_mhz(adev: *mut amdgpu_device) -> u32;
}
extern "C" {
    pub fn amdgpu_amdkfd_get_pcie_bandwidth_mbytes(adev: *mut amdgpu_device, is_min: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_start_sched(adev: *mut amdgpu_device, node_id: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_stop_sched(adev: *mut amdgpu_device, node_id: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_compute_active(adev: *mut amdgpu_device, node_id: u32) -> bool;
}
// Read user wptr from a specified user address space with page fault
// disabled. The memory must be pinned and mapped to the hardware when
// this is called in hqd_load functions, so it should never fault in
// the first place. This resolves a circular lock dependency involving
// four locks, including the DQM lock and mmap_lock.
//

// GPUVM API

extern "C" {
    pub fn amdgpu_amdkfd_gpuvm_get_process_page_dir(drm_priv: *mut c_void) -> u64;
}
extern "C" {
    pub fn amdgpu_amdkfd_gpuvm_dmaunmap_mem(mem: *mut kgd_mem, drm_priv: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_gpuvm_unmap_bo_from_kernel(mem: *mut kgd_mem);
}
extern "C" {
    pub fn amdgpu_amdkfd_map_gtt_bo_to_gart(bo: *mut amdgpu_bo, bo_gart: *mut amdgpu_bo) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_debug_mem_fence(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_is_fed(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_amdkfd_bo_mapped_to_dev(drm_priv: *mut c_void, mem: *mut kgd_mem) -> bool;
}
extern "C" {
    pub fn amdgpu_amdkfd_block_mmu_notifications(p: *mut c_void);
}
extern "C" {
    pub fn amdgpu_amdkfd_criu_resume(p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_clear_kfd_mapping(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_amdkfd_xcp_memory_size(adev: *mut amdgpu_device, xcp_id: c_int) -> u64;
}

extern "C" {
    pub fn amdgpu_amdkfd_gpuvm_init_mem_limits();
}
//
// @amdgpu_amdkfd_release_notify() - Notify KFD when GEM object is released
//
// Allows KFD to release its resources associated with the GEM object.
//
extern "C" {
    pub fn amdgpu_amdkfd_release_notify(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn amdgpu_amdkfd_reserve_system_mem(size: u64);
}

extern "C" {
    pub fn kgd2kfd_init_zone_device(adev: *mut amdgpu_device) -> c_int;
}

// KGD2KFD callbacks
extern "C" {
    pub fn kgd2kfd_quiesce_mm(mm: *mut mm_struct, trigger: u32) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_resume_mm(mm: *mut mm_struct) -> c_int;
}

extern "C" {
    pub fn kgd2kfd_init() -> c_int;
}
extern "C" {
    pub fn kgd2kfd_exit();
}
extern "C" {
    pub fn kgd2kfd_device_exit(kfd: *mut kfd_dev);
}
extern "C" {
    pub fn kgd2kfd_suspend(kfd: *mut kfd_dev, suspend_proc: bool);
}
extern "C" {
    pub fn kgd2kfd_resume(kfd: *mut kfd_dev, resume_proc: bool) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_suspend_process(kfd: *mut kfd_dev);
}
extern "C" {
    pub fn kgd2kfd_resume_process(kfd: *mut kfd_dev) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_post_reset(kfd: *mut kfd_dev) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_interrupt(kfd: *mut kfd_dev, ih_ring_entry: *const c_void);
}
extern "C" {
    pub fn kgd2kfd_set_sram_ecc_flag(kfd: *mut kfd_dev);
}
extern "C" {
    pub fn kgd2kfd_smi_event_throttle(kfd: *mut kfd_dev, throttle_bitmask: u64);
}
extern "C" {
    pub fn kgd2kfd_check_and_lock_kfd(kfd: *mut kfd_dev) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_unlock_kfd(kfd: *mut kfd_dev);
}
extern "C" {
    pub fn kgd2kfd_start_sched(kfd: *mut kfd_dev, node_id: u32) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_start_sched_all_nodes(kfd: *mut kfd_dev) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_start_sched_all(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_stop_sched(kfd: *mut kfd_dev, node_id: u32) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_stop_sched_all_nodes(kfd: *mut kfd_dev) -> c_int;
}
extern "C" {
    pub fn amdgpu_amdkfd_stop_sched_all(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn kgd2kfd_compute_active(kfd: *mut kfd_dev, node_id: u32) -> bool;
}
extern "C" {
    pub fn kgd2kfd_lock_kfd();
}
extern "C" {
    pub fn kgd2kfd_teardown_processes(adev: *mut amdgpu_device);
}


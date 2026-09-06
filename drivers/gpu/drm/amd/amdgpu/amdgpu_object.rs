//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_object.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//

pub const AMDGPU_BO_MAX_PLACEMENTS: c_int = 3;
// BO flag to indicate a KFD userptr BO

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_bo_param {
    pub size: c_ulong,
    pub byte_align: c_int,
    pub bo_ptr_size: u32,
    pub domain: u32,
    pub preferred_domain: u32,
    pub flags: u64,
    pub type: ttm_bo_type,
    pub no_wait_gpu: bool,
    pub resv: *mut dma_resv,
    pub bo): *mut *mut void (destroy)(struct ttm_buffer_object,
// xcp partition number plus 1, 0 means any partition
    pub xcp_id_plus1: i8,
}

// bo virtual addresses in a vm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_bo_va_mapping {
    pub bo_va: *mut amdgpu_bo_va,
    pub list: list_head,
    pub rb: rb_node,
    pub start: u64,
    pub last: u64,
    pub __subtree_last: u64,
    pub offset: u64,
    pub flags: u32,
}

// User space allocated BO in a VM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_bo_va {
    pub base: amdgpu_vm_bo_base,
// protected by bo being reserved
    pub ref_count: unsigned,
// all other members protected by the VM PD being reserved
    pub last_pt_update: *mut dma_fence,
// mappings for this bo_va
    pub invalids: list_head,
    pub valids: list_head,
// If the mappings are cleared or filled
    pub cleared: bool,
    pub is_xgmi: bool,
//
// protected by vm reservation lock
// if non-zero, cannot unmap from GPU because user queues may still access it
//
    pub queue_refcount: c_uint,
// Indicates if this buffer is mapped for any user queue. Once set, never reset.
    pub userq_va_mapped: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_bo {
// Protected by tbo.reserved
    pub preferred_domains: u32,
    pub allowed_domains: u32,
    pub placements: [ttm_place; AMDGPU_BO_MAX_PLACEMENTS],
    pub placement: ttm_placement,
    pub tbo: ttm_buffer_object,
    pub kmap: ttm_bo_kmap_obj,
    pub flags: u64,
// per VM structure for page tables and with virtual addresses
    pub vm_bo: *mut amdgpu_vm_bo_base,
// Constant after initialization
    pub parent: *mut amdgpu_bo,

    pub notifier: mmu_interval_notifier,

    pub kfd_bo: *mut kgd_mem,
//
// For GPUs with spatial partitioning, xcp partition number, -1 means
// any partition. For other ASICs without spatial partition, always 0
// for memory accounting.
//
    pub xcp_id: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_bo_user {
    pub bo: amdgpu_bo,
    pub tiling_flags: u64,
    pub metadata_flags: u64,
    pub metadata: *mut c_void,
    pub metadata_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_bo_vm {
    pub bo: amdgpu_bo,
    pub entries: [amdgpu_vm_bo_base; ],
}

extern "C" {
    pub fn container_of(_arg: tbo, amdgpu_bo: struct, _arg: tbo) -> return;
}
//
// amdgpu_mem_type_to_domain - return domain corresponding to mem_type
// @mem_type:	ttm memory type
//
// Returns corresponding domain of the ttm mem_type
//
// amdgpu_bo_reserve - reserve bo
// @bo:		bo structure
// @no_intr:	don't return -ERESTARTSYS on pending signal
//
// Returns:
// -ERESTARTSYS: A wait for the buffer to become unreserved was interrupted by
// a signal. Release all buffer reservations and return to user-space.
//
// amdgpu_bo_mmap_offset - return mmap offset of bo
// @bo:	amdgpu object for which we query the offset
//
// Returns mmap offset of the object.
//
extern "C" {
    pub fn drm_vma_node_offset_addr(_arg: &bo->tbo.base.vma_node) -> return;
}
//
// amdgpu_bo_explicit_sync - return whether the bo is explicitly synced
//
// amdgpu_bo_encrypted - test if the BO is encrypted
// @bo: pointer to a buffer object
//
// Return true if the buffer object is encrypted, false otherwise.
//
extern "C" {
    pub fn amdgpu_bo_is_amdgpu_bo(bo: *mut ttm_buffer_object) -> bool;
}
extern "C" {
    pub fn amdgpu_bo_placement_from_domain(abo: *mut amdgpu_bo, domain: u32);
}
extern "C" {
    pub fn amdgpu_bo_free_isp_user(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn amdgpu_bo_kmap(bo: *mut amdgpu_bo, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amdgpu_bo_kunmap(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn amdgpu_bo_unref(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn amdgpu_bo_pin(bo: *mut amdgpu_bo, domain: u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_bo_unpin(bo: *mut amdgpu_bo);
}
extern "C" {
    pub fn amdgpu_bo_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_bo_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_bo_set_tiling_flags(bo: *mut amdgpu_bo, tiling_flags: u64) -> c_int;
}
extern "C" {
    pub fn amdgpu_bo_get_tiling_flags(bo: *mut amdgpu_bo, tiling_flags: *mut u64);
}
extern "C" {
    pub fn amdgpu_bo_release_notify(bo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn amdgpu_bo_fault_reserve_notify(bo: *mut ttm_buffer_object) -> vm_fault_t;
}
extern "C" {
    pub fn amdgpu_bo_sync_wait(bo: *mut amdgpu_bo, owner: *mut c_void, intr: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_bo_gpu_offset(bo: *mut amdgpu_bo) -> u64;
}
extern "C" {
    pub fn amdgpu_bo_fb_aper_addr(bo: *mut amdgpu_bo) -> u64;
}
extern "C" {
    pub fn amdgpu_bo_gpu_offset_no_check(bo: *mut amdgpu_bo) -> u64;
}
extern "C" {
    pub fn amdgpu_bo_mem_stats_placement(bo: *mut amdgpu_bo) -> u32;
}
extern "C" {
    pub fn amdgpu_bo_support_uswc(bo_flags: u64) -> bool;
}

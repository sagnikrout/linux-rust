//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ttm.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gtt_mgr {
    pub manager: ttm_resource_manager,
    pub mm: drm_mm,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ttm_buffer_entity {
    pub base: drm_sched_entity,
    pub lock: mutex,
    pub gart_node: drm_mm_node,
    pub gart_window_offs: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_resv_region_id {
    AMDGPU_RESV_STOLEN_VGA,
    AMDGPU_RESV_STOLEN_EXTENDED,
    AMDGPU_RESV_STOLEN_RESERVED,
    AMDGPU_RESV_FW,
    AMDGPU_RESV_FW_EXTEND,
    AMDGPU_RESV_FW_VRAM_USAGE,
    AMDGPU_RESV_DRV_VRAM_USAGE,
    AMDGPU_RESV_MEM_TRAIN,
    AMDGPU_RESV_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vram_resv {
    pub offset: u64,
    pub size: u64,
    pub bo: *mut amdgpu_bo,
    pub cpu_ptr: *mut c_void,
    pub needs_cpu_map: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mman {
    pub bdev: ttm_device,
    pub ttm_pools: *mut ttm_pool,
    pub initialized: bool,
    pub aper_base_kaddr: *mut void __iomem,
// buffer handling
    pub buffer_funcs: *const amdgpu_buffer_funcs,
    pub buffer_funcs_scheds: [*mut drm_gpu_scheduler; AMDGPU_MAX_RINGS],
    pub num_buffer_funcs_scheds: u32,
    pub buffer_funcs_enabled: bool,
// @default_entity: for workarounds, has no gart windows
    pub default_entity: amdgpu_ttm_buffer_entity,
    pub clear_entities: *mut amdgpu_ttm_buffer_entity,
    pub next_clear_entity: core::sync::atomic::AtomicI32,
    pub num_clear_entities: u32,
    pub move_entities: [amdgpu_ttm_buffer_entity; TTM_NUM_MOVE_FENCES],
    pub next_move_entity: core::sync::atomic::AtomicI32,
    pub num_move_entities: u32,
    pub vram_mgr: amdgpu_vram_mgr,
    pub gtt_mgr: amdgpu_gtt_mgr,
    pub preempt_mgr: ttm_resource_manager,
    pub keep_stolen_vga_memory: bool,
    pub resv_region: [amdgpu_vram_resv; AMDGPU_RESV_MAX],
// PAGE_SIZE'd BO for process memory r/w over SDMA.
    pub sdma_access_bo: *mut amdgpu_bo,
    pub sdma_access_ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_copy_mem {
    pub bo: *mut ttm_buffer_object,
    pub mem: *mut ttm_resource,
    pub offset: c_ulong,
}

pub const AMDGPU_COPY_FLAGS_MAX_COMPRESSED_SHIFT: c_int = 3;
pub const AMDGPU_COPY_FLAGS_MAX_COMPRESSED_MASK: c_uint = 0x03;
pub const AMDGPU_COPY_FLAGS_NUMBER_TYPE_SHIFT: c_int = 5;
pub const AMDGPU_COPY_FLAGS_NUMBER_TYPE_MASK: c_uint = 0x07;
pub const AMDGPU_COPY_FLAGS_DATA_FORMAT_SHIFT: c_int = 8;
pub const AMDGPU_COPY_FLAGS_DATA_FORMAT_MASK: c_uint = 0x3f;
pub const AMDGPU_COPY_FLAGS_WRITE_COMPRESS_DISABLE_SHIFT: c_int = 14;
pub const AMDGPU_COPY_FLAGS_WRITE_COMPRESS_DISABLE_MASK: c_uint = 0x1;

extern "C" {
    pub fn amdgpu_gtt_mgr_init(adev: *mut amdgpu_device, gtt_size: u64) -> c_int;
}
extern "C" {
    pub fn amdgpu_gtt_mgr_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_preempt_mgr_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_preempt_mgr_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_preempt_mgr_sysfs_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vram_mgr_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vram_mgr_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_gtt_mgr_has_gart_addr(mem: *mut ttm_resource) -> bool;
}
extern "C" {
    pub fn amdgpu_gtt_mgr_mark_bo_teardown(tbo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn amdgpu_gtt_mgr_recover(mgr: *mut amdgpu_gtt_mgr);
}
extern "C" {
    pub fn amdgpu_preempt_mgr_usage(man: *mut ttm_resource_manager) -> u64;
}
extern "C" {
    pub fn amdgpu_vram_mgr_bo_visible_size(bo: *mut amdgpu_bo) -> u64;
}
extern "C" {
    pub fn amdgpu_vram_mgr_vis_usage(mgr: *mut amdgpu_vram_mgr) -> u64;
}
extern "C" {
    pub fn amdgpu_vram_mgr_clear_reset_blocks(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ttm_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ttm_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ttm_enable_buffer_funcs(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ttm_disable_buffer_funcs(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_ttm_alloc_gart(bo: *mut ttm_buffer_object) -> c_int;
}
extern "C" {
    pub fn amdgpu_ttm_recover_gart(tbo: *mut ttm_buffer_object);
}
extern "C" {
    pub fn amdgpu_ttm_domain_start(adev: *mut amdgpu_device, type: u32) -> u64;
}

//
// amdgpu_compute_gart_address() - Returns GART address of an entity's window
// @gmc: The &struct amdgpu_gmc instance to use
// @entity: The &struct amdgpu_ttm_buffer_entity owning the GART window
// @index: The window to use (must be 0 or 1)
//
// amdgpu_gtt_node_to_byte_offset() - Returns a byte offset of a gtt node
//
extern "C" {
    pub fn amdgpu_ttm_tt_set_user_pages(ttm: *mut ttm_tt, range: *mut amdgpu_hmm_range);
}
extern "C" {
    pub fn amdgpu_ttm_tt_has_userptr(ttm: *mut ttm_tt) -> bool;
}
extern "C" {
    pub fn amdgpu_ttm_tt_is_userptr(ttm: *mut ttm_tt) -> bool;
}
extern "C" {
    pub fn amdgpu_ttm_tt_is_readonly(ttm: *mut ttm_tt) -> bool;
}
extern "C" {
    pub fn amdgpu_ttm_tt_pde_flags(ttm: *mut ttm_tt, mem: *mut ttm_resource) -> u64;
}
extern "C" {
    pub fn amdgpu_ttm_evict_resources(adev: *mut amdgpu_device, mem_type: c_int) -> c_int;
}
extern "C" {
    pub fn amdgpu_ttm_debugfs_init(adev: *mut amdgpu_device);
}

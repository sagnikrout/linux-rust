//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/etnaviv/etnaviv_gem.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2015-2018 Etnaviv Project
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_gem_userptr {
    pub ptr: uintptr_t,
    pub mm: *mut mm_struct,
    pub ro: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_vram_mapping {
    pub obj_node: list_head,
    pub scan_node: list_head,
    pub mmu_node: list_head,
    pub object: *mut etnaviv_gem_object,
    pub context: *mut etnaviv_iommu_context,
    pub vram_node: drm_mm_node,
    pub use: c_uint,
    pub iova: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_gem_object {
    pub base: drm_gem_object,
    pub ops: *const etnaviv_gem_ops,
    pub lock: mutex,
//
// The actual size that is visible to the GPU, not necessarily
// PAGE_SIZE aligned, but should be aligned to GPU page size.
//
    pub size: u32,
    pub flags: u32,
    pub gem_node: list_head,
    pub gpu_active: core::sync::atomic::AtomicI32,
    pub pages: *mut page,
    pub sgt: *mut sg_table,
    pub vaddr: *mut c_void,
    pub vram_list: list_head,
// cache maintenance
    pub last_cpu_prep_op: u32,
    pub userptr: etnaviv_gem_userptr,
}

extern "C" {
    pub fn container_of(_arg: obj, etnaviv_gem_object: struct, _arg: base) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_gem_ops {
    pub ): *mut *mut int (get_pages)(struct etnaviv_gem_object,
    pub ): *mut *mut void (release)(struct etnaviv_gem_object,
    pub ): *mut *mut *mut void (vmap)(struct etnaviv_gem_object,
    pub ): *mut *mut *mut int (mmap)(struct etnaviv_gem_object , struct vm_area_struct,
}

pub const MAX_CMDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_gem_submit_bo {
    pub flags: u32,
    pub va: u64,
    pub obj: *mut etnaviv_gem_object,
    pub mapping: *mut etnaviv_vram_mapping,
}

// Created per submit-ioctl, to track bo's and cmdstream bufs, etc,
// associated with the cmdstream submission for synchronization (and
// make it easier to unwind when things go wrong, etc).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_gem_submit {
    pub sched_job: drm_sched_job,
    pub refcount: kref,
    pub ctx: *mut etnaviv_file_private,
    pub gpu: *mut etnaviv_gpu,
    pub prev_mmu_context: *mut *mut etnaviv_iommu_context mmu_context,,
    pub out_fence: *mut dma_fence,
    pub out_fence_id: c_int,
    pub /: *mut *mut list_head node; / GPU active submit list,
    pub cmdbuf: etnaviv_cmdbuf,
    pub /: *mut *mut *mut pid pid; / submitting process,
    pub exec_state: u32,
    pub flags: u32,
    pub nr_pmrs: c_uint,
    pub pmrs: *mut etnaviv_perfmon_request,
    pub nr_bos: c_uint,
    pub bos: [etnaviv_gem_submit_bo; ],
// No new members here, the previous one is variable-length!
}

extern "C" {
    pub fn etnaviv_submit_put(submit: *mut *mut etnaviv_gem_submit);
}
extern "C" {
    pub fn etnaviv_gem_obj_add(dev: *mut drm_device, obj: *mut drm_gem_object);
}
extern "C" {
    pub fn etnaviv_gem_put_pages(obj: *mut etnaviv_gem_object);
}
extern "C" {
    pub fn etnaviv_gem_mapping_unreference(mapping: *mut etnaviv_vram_mapping);
}

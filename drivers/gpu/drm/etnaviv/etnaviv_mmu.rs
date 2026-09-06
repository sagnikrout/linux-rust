//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/etnaviv/etnaviv_mmu.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etnaviv_iommu_version {
    ETNAVIV_IOMMU_V1 = 0,
    ETNAVIV_IOMMU_V2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_iommu_ops {
    pub ): *mut *mut *mut etnaviv_iommu_context (init)(etnaviv_iommu_global,
    pub ): *mut *mut void (free)(struct etnaviv_iommu_context,
    pub prot): phys_addr_t paddr, size_t size, int,
    pub size): usize,
    pub ): *mut *mut size_t (dump_size)(struct etnaviv_iommu_context,
    pub ): *mut *mut *mut void (dump)(struct etnaviv_iommu_context , void,
    pub ): *mut *mut *mut void (restore)(struct etnaviv_gpu , struct etnaviv_iommu_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_iommu_global {
    pub dev: *mut device,
    pub version: etnaviv_iommu_version,
    pub ops: *const etnaviv_iommu_ops,
    pub use: c_uint,
    pub lock: mutex,
    pub bad_page_cpu: *mut c_void,
    pub bad_page_dma: dma_addr_t,
    pub memory_base: u32,
//
// This union holds members needed by either MMUv1 or MMUv2, which
// can not exist at the same time.
//
    pub shared_context: *mut etnaviv_iommu_context,
    pub v1: },
// P(age) T(able) A(rray)
    pub pta_cpu: *mut u64,
    pub pta_dma: dma_addr_t,
    pub ETNAVIV_PTA_ENTRIES): DECLARE_BITMAP(pta_alloc,,
    pub v2: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_iommu_context {
    pub refcount: kref,
    pub global: *mut etnaviv_iommu_global,
// memory manager for GPU address area
    pub lock: mutex,
    pub mappings: list_head,
    pub mm: drm_mm,
    pub flush_seq: c_uint,
// Not part of the context, but needs to have the same lifetime
    pub cmdbuf_mapping: etnaviv_vram_mapping,
}

extern "C" {
    pub fn etnaviv_iommu_global_init(gpu: *mut etnaviv_gpu) -> c_int;
}
extern "C" {
    pub fn etnaviv_iommu_global_fini(gpu: *mut etnaviv_gpu);
}
extern "C" {
    pub fn etnaviv_iommu_reap_mapping(mapping: *mut etnaviv_vram_mapping);
}
extern "C" {
    pub fn etnaviv_iommu_dump_size(ctx: *mut etnaviv_iommu_context) -> usize;
}
extern "C" {
    pub fn etnaviv_iommu_dump(ctx: *mut etnaviv_iommu_context, buf: *mut c_void);
}
extern "C" {
    pub fn etnaviv_iommu_context_put(ctx: *mut etnaviv_iommu_context);
}
extern "C" {
    pub fn etnaviv_iommuv2_get_mtlb_addr(context: *mut etnaviv_iommu_context) -> u32;
}
extern "C" {
    pub fn etnaviv_iommuv2_get_pta_id(context: *mut etnaviv_iommu_context) -> c_ushort;
}

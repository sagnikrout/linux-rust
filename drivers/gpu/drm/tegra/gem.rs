//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/gem.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Tegra host1x GEM implementation
//
// Copyright (c) 2012-2013, NVIDIA Corporation.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_bo_tiling_mode {
    TEGRA_BO_TILING_MODE_PITCH,
    TEGRA_BO_TILING_MODE_TILED,
    TEGRA_BO_TILING_MODE_BLOCK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_bo_sector_layout {
    TEGRA_BO_SECTOR_LAYOUT_TEGRA,
    TEGRA_BO_SECTOR_LAYOUT_GPU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bo_tiling {
    pub mode: tegra_bo_tiling_mode,
    pub value: c_ulong,
    pub sector_layout: tegra_bo_sector_layout,
}

//
// How memory is referenced within a tegra_bo:
//
// Buffer source  | Mapping API(*)  | Fields
// ---------------+-----------------+---------------
// Allocated here | DMA API         | iova (IOVA mapped to drm->dev), vaddr (CPU VA)
//
// Allocated here | IOMMU API       | pages/num_pages (Phys. memory), sgt (Mapped to drm->dev),
// | iova/size (Mapped to domain)
//
// Imported       | DMA API         | dma_buf (Imported dma_buf)
//
// Imported       | IOMMU API       | dma_buf (Imported dma_buf),
// | gem->import_attach (Attachment on drm->dev),
// | sgt (Mapped to drm->dev)
// | iova/size (Mapped to domain)
//
// (*) If tegra->domain is set, i.e. TegraDRM IOMMU domain is directly managed through IOMMU API,
// this is IOMMU API. Otherwise DMA API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bo {
    pub gem: drm_gem_object,
    pub base: host1x_bo,
    pub flags: c_ulong,
    pub sgt: *mut sg_table,
    pub iova: dma_addr_t,
    pub vaddr: *mut c_void,
    pub dma_buf: *mut dma_buf,
    pub mm: *mut drm_mm_node,
    pub num_pages: c_ulong,
    pub pages: *mut page,
// size of IOMMU mapping
    pub size: usize,
    pub tiling: tegra_bo_tiling,
}

extern "C" {
    pub fn container_of(_arg: gem, tegra_bo: struct, _arg: gem) -> return;
}
extern "C" {
    pub fn container_of(_arg: bo, tegra_bo: struct, _arg: base) -> return;
}
extern "C" {
    pub fn tegra_bo_free_object(gem: *mut drm_gem_object);
}
extern "C" {
    pub fn __tegra_gem_mmap(gem: *mut drm_gem_object, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn tegra_drm_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
}

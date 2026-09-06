//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_mmu.h
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_mmu_funcs {
    pub mmu): *mut *mut void (detach)(struct msm_mmu,
    pub len): uint64_t iova, size_t,
    pub p): *mut *mut *mut int (prealloc_allocate)(struct msm_mmu mmu, struct msm_mmu_prealloc,
    pub p): *mut *mut *mut void (prealloc_cleanup)(struct msm_mmu mmu, struct msm_mmu_prealloc,
    pub prot): size_t off, size_t len, int,
    pub len): *mut *mut *mut int (unmap)(struct msm_mmu mmu, uint64_t iova, size_t,
    pub mmu): *mut *mut void (destroy)(struct msm_mmu,
    pub enable): *mut *mut *mut void (set_stall)(struct msm_mmu mmu, bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msm_mmu_type {
    MSM_MMU_GPUMMU,
    MSM_MMU_IOMMU,
    MSM_MMU_IOMMU_PAGETABLE,
}

//
// struct msm_mmu_prealloc - Tracking for pre-allocated pages for MMU updates.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_mmu_prealloc {
// @count: Number of pages reserved.
    pub count: u32,
// @ptr: Index of first unused page in @pages
    pub ptr: u32,
//
// @pages: Array of pages preallocated for MMU table updates.
//
// After a VM operation, there might be free pages remaining in this
// array (since the amount allocated is a worst-case).  These are
// returned to the pt_cache at mmu->prealloc_cleanup().
//
    pub pages: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_mmu {
    pub funcs: *const msm_mmu_funcs,
    pub dev: *mut device,
    pub data): *mut *mut *mut int (handler)(void arg, unsigned long iova, int flags, void,
    pub arg: *mut c_void,
    pub type: msm_mmu_type,
//
// @prealloc: pre-allocated pages for pgtable
//
// Set while a VM_BIND job is running, serialized under
// msm_gem_vm::mmu_lock.
//
    pub prealloc: *mut msm_mmu_prealloc,
}

extern "C" {
    pub fn msm_iommu_pagetable_walk(mmu: *mut msm_mmu, iova: c_ulong, ptes[4]: u64) -> c_int;
}

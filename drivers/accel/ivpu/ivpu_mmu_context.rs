//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_mmu_context.h
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
// Copyright (C) 2020-2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_mmu_pgtable {
    pub pte_ptrs: [*mut u64; IVPU_MMU_PGTABLE_ENTRIES],
    pub pmd_ptrs: [*mut u64; IVPU_MMU_PGTABLE_ENTRIES],
    pub pud_ptrs: [*mut u64; IVPU_MMU_PGTABLE_ENTRIES],
    pub pgd_dma_ptr: *mut u64,
    pub pgd_dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_mmu_context {
    pub /: *mut *mut mutex lock; / Protects: mm, pgtable, is_cd_valid,
    pub mm: drm_mm,
    pub pgtable: ivpu_mmu_pgtable,
    pub is_cd_valid: bool,
    pub id: u32,
}

extern "C" {
    pub fn ivpu_mmu_context_init(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context, context_id: u32);
}
extern "C" {
    pub fn ivpu_mmu_context_fini(vdev: *mut ivpu_device, ctx: *mut ivpu_mmu_context);
}
extern "C" {
    pub fn ivpu_mmu_global_context_init(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_global_context_fini(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_reserved_context_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_mmu_reserved_context_fini(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_mmu_context_remove_node(ctx: *mut ivpu_mmu_context, node: *mut drm_mm_node);
}

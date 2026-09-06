//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/v3d/v3d_mmu.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (C) 2017-2018 Broadcom
//
// DOC: Broadcom V3D MMU
//
// The V3D 3.x hardware (compared to VC4) now includes an MMU. It has
// a single level of page tables for the V3D's 4GB address space to
// map to AXI bus addresses, thus it could need up to 4MB of
// physically contiguous memory to store the PTEs.
//
// Because the 4MB of contiguous memory for page tables is precious,
// and switching between them is expensive, we load all BOs into the
// same 4GB address space.
//
// To protect clients from each other, we should use the GMP to
// quickly mask out (at 128kb granularity) what pages are available to
// each client. This is not yet implemented.
//

// Note: All PTEs for the 64KB bigpage or 1MB superpage must be filled
// with the bigpage/superpage bit set.
//

#[no_mangle]
unsafe extern "C" fn v3d_mmu_is_aligned(page: u32, page_address: u32, alignment: usize) -> bool {
    static bool v3d_mmu_is_aligned(u32 page, u32 page_address, size_t alignment)
    {
    return IS_ALIGNED(page, alignment >> V3D_MMU_PAGE_SHIFT) &&
    IS_ALIGNED(page_address, alignment >> V3D_MMU_PAGE_SHIFT);
    }
//
// Issue the MMUC flush and TLB clear unconditionally. The caller must
// already know that V3D is reachable. In particular, this is used from
// the runtime resume callback.
//
#[no_mangle]
unsafe extern "C" fn v3d_mmu_flush_all_locked(v3d: *mut v3d_dev) -> c_int {
    static int v3d_mmu_flush_all_locked(struct v3d_dev *v3d)
    {
    int ret;
    V3D_WRITE(V3D_MMUC_CONTROL, V3D_MMUC_CONTROL_FLUSH |
    V3D_MMUC_CONTROL_ENABLE);
    ret = wait_for(!(V3D_READ(V3D_MMUC_CONTROL) &
    V3D_MMUC_CONTROL_FLUSHING), 100);
    if (ret) {
    dev_err(v3d.drm.dev, "MMUC flush wait idle failed\n");
    return ret;
    }
    V3D_WRITE(V3D_MMU_CTL, V3D_READ(V3D_MMU_CTL) |
    V3D_MMU_CTL_TLB_CLEAR);
    ret = wait_for(!(V3D_READ(V3D_MMU_CTL) &
    V3D_MMU_CTL_TLB_CLEARING), 100);
    if (ret)
    dev_err(v3d.drm.dev, "MMU TLB clear wait idle failed\n");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn v3d_mmu_flush_all(v3d: *mut v3d_dev) -> c_int {
    int v3d_mmu_flush_all(struct v3d_dev *v3d)
    {
    int ret;
// Flush the PTs only if we're already awake
    if (!pm_runtime_get_if_active(v3d.drm.dev))
    return 0;
    ret = v3d_mmu_flush_all_locked(v3d);
    v3d_pm_runtime_put(v3d);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn v3d_mmu_set_page_table(v3d: *mut v3d_dev) -> c_int {
    int v3d_mmu_set_page_table(struct v3d_dev *v3d)
    {
    V3D_WRITE(V3D_MMU_PT_PA_BASE, v3d.pt_paddr >> V3D_MMU_PAGE_SHIFT);
    V3D_WRITE(V3D_MMU_CTL,
    V3D_MMU_CTL_ENABLE |
    V3D_MMU_CTL_PT_INVALID_ENABLE |
    V3D_MMU_CTL_PT_INVALID_ABORT |
    V3D_MMU_CTL_PT_INVALID_INT |
    V3D_MMU_CTL_WRITE_VIOLATION_ABORT |
    V3D_MMU_CTL_WRITE_VIOLATION_INT |
    V3D_MMU_CTL_CAP_EXCEEDED_ABORT |
    V3D_MMU_CTL_CAP_EXCEEDED_INT);
    V3D_WRITE(V3D_MMU_ILLEGAL_ADDR,
    (v3d.mmu_scratch_paddr >> V3D_MMU_PAGE_SHIFT) |
    V3D_MMU_ILLEGAL_ADDR_ENABLE);
    V3D_WRITE(V3D_MMUC_CONTROL, V3D_MMUC_CONTROL_ENABLE);
    return v3d_mmu_flush_all_locked(v3d);
    }
#[no_mangle]
pub unsafe extern "C" fn v3d_mmu_insert_ptes(bo: *mut v3d_bo) {
    void v3d_mmu_insert_ptes(struct v3d_bo *bo)
    {
    struct drm_gem_shmem_object *shmem_obj = &bo.base;
    struct v3d_dev *v3d = to_v3d_dev(shmem_obj.base.dev);
    let mut page: u32 = bo.node.start;
    struct scatterlist *sgl;
    unsigned int count;
    for_each_sgtable_dma_sg(shmem_obj.sgt, sgl, count) {
    let mut dma_addr: dma_addr_t = sg_dma_address(sgl);
    let mut pfn: u32 = dma_addr >> V3D_MMU_PAGE_SHIFT;
    let mut len: c_uint = sg_dma_len(sgl);
    while (len > 0) {
    let mut page_prot: u32 = V3D_PTE_WRITEABLE | V3D_PTE_VALID;
    let mut page_address: u32 = page_prot | pfn;
    unsigned int i, page_size;
    BUG_ON(pfn + V3D_PAGE_FACTOR >= BIT(24));
    if (len >= SZ_1M &&
    v3d_mmu_is_aligned(page, page_address, SZ_1M)) {
    page_size = SZ_1M;
    page_address |= V3D_PTE_SUPERPAGE;
    } else if (len >= SZ_64K &&
    v3d_mmu_is_aligned(page, page_address, SZ_64K)) {
    page_size = SZ_64K;
    page_address |= V3D_PTE_BIGPAGE;
    } else {
    page_size = SZ_4K;
    }
    for (i = 0; i < page_size >> V3D_MMU_PAGE_SHIFT; i++) {
    v3d.pt[page++] = page_address + i;
    pfn++;
    }
    len -= page_size;
    }
    }
    WARN_ON_ONCE(page - bo.node.start !=
    shmem_obj.base.size >> V3D_MMU_PAGE_SHIFT);
    if (v3d_mmu_flush_all(v3d))
    drm_err(&v3d.drm, "MMU flush timeout\n");
    }
#[no_mangle]
pub unsafe extern "C" fn v3d_mmu_remove_ptes(bo: *mut v3d_bo) {
    void v3d_mmu_remove_ptes(struct v3d_bo *bo)
    {
    struct v3d_dev *v3d = to_v3d_dev(bo.base.base.dev);
    let mut npages: u32 = bo.base.base.size >> V3D_MMU_PAGE_SHIFT;
    u32 page;
    for (page = bo.node.start; page < bo.node.start + npages; page++)
    v3d.pt[page] = 0;
    if (v3d_mmu_flush_all(v3d))
    drm_err(&v3d.drm, "MMU flush timeout\n");
    }

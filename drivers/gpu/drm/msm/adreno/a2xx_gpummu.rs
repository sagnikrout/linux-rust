//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/adreno/a2xx_gpummu.c
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
// Copyright (c) 2018 The Linux Foundation. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a2xx_gpummu {
    pub base: msm_mmu,
    pub gpu: *mut msm_gpu,
    pub pt_base: dma_addr_t,
    pub table: *mut u32,
}

#[no_mangle]
unsafe extern "C" fn a2xx_gpummu_detach(mmu: *mut msm_mmu) {
    static void a2xx_gpummu_detach(struct msm_mmu *mmu)
    {
    }
    static int a2xx_gpummu_map(struct msm_mmu *mmu, uint64_t iova,
    struct sg_table *sgt, size_t off, size_t len,
    int prot)
    {
    struct a2xx_gpummu *gpummu = to_a2xx_gpummu(mmu);
    let mut idx: unsigned = (iova - GPUMMU_VA_START) / GPUMMU_PAGE_SIZE;
    struct sg_dma_page_iter dma_iter;
    let mut prot_bits: unsigned = 0;
    WARN_ON(off != 0);
    if (prot & IOMMU_WRITE)
    prot_bits |= 1;
    if (prot & IOMMU_READ)
    prot_bits |= 2;
    for_each_sgtable_dma_page(sgt, &dma_iter, 0) {
    let mut addr: dma_addr_t = sg_page_iter_dma_address(&dma_iter);
    int i;
    for (i = 0; i < PAGE_SIZE; i += GPUMMU_PAGE_SIZE)
    gpummu.table[idx++] = (addr + i) | prot_bits;
    }
// we can improve by deferring flush for multiple map()
    gpu_write(gpummu.gpu, REG_A2XX_MH_MMU_INVALIDATE,
    A2XX_MH_MMU_INVALIDATE_INVALIDATE_ALL |
    A2XX_MH_MMU_INVALIDATE_INVALIDATE_TC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a2xx_gpummu_unmap(mmu: *mut msm_mmu, iova: u64, len: usize) -> c_int {
    static int a2xx_gpummu_unmap(struct msm_mmu *mmu, uint64_t iova, size_t len)
    {
    struct a2xx_gpummu *gpummu = to_a2xx_gpummu(mmu);
    let mut idx: unsigned = (iova - GPUMMU_VA_START) / GPUMMU_PAGE_SIZE;
    unsigned i;
    for (i = 0; i < len / GPUMMU_PAGE_SIZE; i++, idx++)
    gpummu.table[idx] = 0;
    gpu_write(gpummu.gpu, REG_A2XX_MH_MMU_INVALIDATE,
    A2XX_MH_MMU_INVALIDATE_INVALIDATE_ALL |
    A2XX_MH_MMU_INVALIDATE_INVALIDATE_TC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a2xx_gpummu_destroy(mmu: *mut msm_mmu) {
    static void a2xx_gpummu_destroy(struct msm_mmu *mmu)
    {
    struct a2xx_gpummu *gpummu = to_a2xx_gpummu(mmu);
    dma_free_attrs(mmu.dev, TABLE_SIZE + 32, gpummu.table, gpummu.pt_base,
    DMA_ATTR_FORCE_CONTIGUOUS);
    kfree(gpummu);
    }
    static const struct msm_mmu_funcs funcs = {
    .detach = a2xx_gpummu_detach,
    .map = a2xx_gpummu_map,
    .unmap = a2xx_gpummu_unmap,
    .destroy = a2xx_gpummu_destroy,
    };
    struct msm_mmu *a2xx_gpummu_new(struct device *dev, struct msm_gpu *gpu)
    {
    struct a2xx_gpummu *gpummu;
    gpummu = kzalloc_obj(*gpummu);
    if (!gpummu)
    return ERR_PTR(-ENOMEM);
    gpummu.table = dma_alloc_attrs(dev, TABLE_SIZE + 32, &gpummu.pt_base,
    GFP_KERNEL | __GFP_ZERO, DMA_ATTR_FORCE_CONTIGUOUS);
    if (!gpummu.table) {
    kfree(gpummu);
    return ERR_PTR(-ENOMEM);
    }
    gpummu.gpu = gpu;
    msm_mmu_init(&gpummu.base, dev, &funcs, MSM_MMU_GPUMMU);
    return &gpummu.base;
    }
    void a2xx_gpummu_params(struct msm_mmu *mmu, dma_addr_t *pt_base,
    dma_addr_t *tran_error)
    {
    let mut base: dma_addr_t = to_a2xx_gpummu(mmu).pt_base;
// pt_base = base;
// tran_error = base + TABLE_SIZE; /* 32-byte aligned
    }

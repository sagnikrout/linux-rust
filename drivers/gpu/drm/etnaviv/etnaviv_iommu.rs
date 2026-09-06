//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/etnaviv/etnaviv_iommu.c
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
// Copyright (C) 2014-2018 Etnaviv Project
//

pub const GPU_MEM_START: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_iommuv1_context {
    pub base: etnaviv_iommu_context,
    pub pgtable_cpu: *mut u32,
    pub pgtable_dma: dma_addr_t,
}

    static struct etnaviv_iommuv1_context *
    to_v1_context(struct etnaviv_iommu_context *context)
    {
    return container_of(context, struct etnaviv_iommuv1_context, base);
    }
#[no_mangle]
unsafe extern "C" fn etnaviv_iommuv1_free(context: *mut etnaviv_iommu_context) {
    static void etnaviv_iommuv1_free(struct etnaviv_iommu_context *context)
    {
    struct etnaviv_iommuv1_context *v1_context = to_v1_context(context);
    drm_mm_takedown(&context.mm);
    dma_free_wc(context.global.dev, PT_SIZE, v1_context.pgtable_cpu,
    v1_context.pgtable_dma);
    context.global.v1.shared_context = core::ptr::null_mut();
    kfree(v1_context);
    }
    static int etnaviv_iommuv1_map(struct etnaviv_iommu_context *context,
    unsigned long iova, phys_addr_t paddr,
    size_t size, int prot)
    {
    struct etnaviv_iommuv1_context *v1_context = to_v1_context(context);
    let mut index: c_uint = (iova - GPU_MEM_START) / SZ_4K;
    if (size != SZ_4K)
    return -EINVAL;
    v1_context.pgtable_cpu[index] = paddr;
    return 0;
    }
    static size_t etnaviv_iommuv1_unmap(struct etnaviv_iommu_context *context,
    unsigned long iova, size_t size)
    {
    struct etnaviv_iommuv1_context *v1_context = to_v1_context(context);
    let mut index: c_uint = (iova - GPU_MEM_START) / SZ_4K;
    if (size != SZ_4K)
    return -EINVAL;
    v1_context.pgtable_cpu[index] = context.global.bad_page_dma;
    return SZ_4K;
    }
#[no_mangle]
unsafe extern "C" fn etnaviv_iommuv1_dump_size(context: *mut etnaviv_iommu_context) -> usize {
    static size_t etnaviv_iommuv1_dump_size(struct etnaviv_iommu_context *context)
    {
    return PT_SIZE;
    }
    static void etnaviv_iommuv1_dump(struct etnaviv_iommu_context *context,
    void *buf)
    {
    struct etnaviv_iommuv1_context *v1_context = to_v1_context(context);
    memcpy(buf, v1_context.pgtable_cpu, PT_SIZE);
    }
    static void etnaviv_iommuv1_restore(struct etnaviv_gpu *gpu,
    struct etnaviv_iommu_context *context)
    {
    struct etnaviv_iommuv1_context *v1_context = to_v1_context(context);
    u32 pgtable;
    if (gpu.mmu_context)
    etnaviv_iommu_context_put(gpu.mmu_context);
    gpu.mmu_context = etnaviv_iommu_context_get(context);
// set base addresses
    gpu_write(gpu, VIVS_MC_MEMORY_BASE_ADDR_RA, context.global.memory_base);
    gpu_write(gpu, VIVS_MC_MEMORY_BASE_ADDR_FE, context.global.memory_base);
    gpu_write(gpu, VIVS_MC_MEMORY_BASE_ADDR_TX, context.global.memory_base);
    gpu_write(gpu, VIVS_MC_MEMORY_BASE_ADDR_PEZ, context.global.memory_base);
    gpu_write(gpu, VIVS_MC_MEMORY_BASE_ADDR_PE, context.global.memory_base);
// set page table address in MC
    pgtable = (u32)v1_context.pgtable_dma;
    gpu_write(gpu, VIVS_MC_MMU_FE_PAGE_TABLE, pgtable);
    gpu_write(gpu, VIVS_MC_MMU_TX_PAGE_TABLE, pgtable);
    gpu_write(gpu, VIVS_MC_MMU_PE_PAGE_TABLE, pgtable);
    gpu_write(gpu, VIVS_MC_MMU_PEZ_PAGE_TABLE, pgtable);
    gpu_write(gpu, VIVS_MC_MMU_RA_PAGE_TABLE, pgtable);
    }
    const struct etnaviv_iommu_ops etnaviv_iommuv1_ops = {
    .free = etnaviv_iommuv1_free,
    .map = etnaviv_iommuv1_map,
    .unmap = etnaviv_iommuv1_unmap,
    .dump_size = etnaviv_iommuv1_dump_size,
    .dump = etnaviv_iommuv1_dump,
    .restore = etnaviv_iommuv1_restore,
    };
    struct etnaviv_iommu_context *
    etnaviv_iommuv1_context_alloc(struct etnaviv_iommu_global *global)
    {
    struct etnaviv_iommuv1_context *v1_context;
    struct etnaviv_iommu_context *context;
    mutex_lock(&global.lock);
//
// MMUv1 does not support switching between different contexts without
// a stop the world operation, so we only support a single shared
// context with this version.
//
    if (global.v1.shared_context) {
    context = global.v1.shared_context;
    etnaviv_iommu_context_get(context);
    mutex_unlock(&global.lock);
    return context;
    }
    v1_context = kzalloc_obj(*v1_context);
    if (!v1_context) {
    mutex_unlock(&global.lock);
    return core::ptr::null_mut();
    }
    v1_context.pgtable_cpu = dma_alloc_wc(global.dev, PT_SIZE,
    &v1_context.pgtable_dma,
    GFP_KERNEL);
    if (!v1_context.pgtable_cpu)
    goto out_free;
    memset32(v1_context.pgtable_cpu, global.bad_page_dma, PT_ENTRIES);
    context = &v1_context.base;
    context.global = global;
    kref_init(&context.refcount);
    mutex_init(&context.lock);
    INIT_LIST_HEAD(&context.mappings);
    drm_mm_init(&context.mm, GPU_MEM_START, PT_ENTRIES * SZ_4K);
    context.global.v1.shared_context = context;
    mutex_unlock(&global.lock);
    return context;
    out_free:
    mutex_unlock(&global.lock);
    kfree(v1_context);
    return core::ptr::null_mut();
    }

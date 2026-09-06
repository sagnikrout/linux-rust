//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/amdxdna_iommu.c
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
// Copyright (C) 2025, Advanced Micro Devices, Inc.
//

    static bool force_iova;
    module_param(force_iova, bool, 0600);
    MODULE_PARM_DESC(force_iova, "Force use IOVA (Default false)");
    static struct iova *amdxdna_iommu_alloc_iova(struct amdxdna_dev *xdna,
    size_t size,
    dma_addr_t *dma_addr,
    bool size_aligned)
    {
    unsigned long shift, end;
    struct iova *iova;
    end = xdna.domain.geometry.aperture_end;
    shift = iova_shift(&xdna.iovad);
    size = iova_align(&xdna.iovad, size);
    iova = alloc_iova(&xdna.iovad, size >> shift, end >> shift, size_aligned);
    if (!iova)
    return ERR_PTR(-ENOMEM);
// dma_addr = iova_dma_addr(&xdna->iovad, iova);
    return iova;
    }
#[no_mangle]
pub unsafe extern "C" fn amdxdna_dma_map_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj) -> c_int {
    int amdxdna_dma_map_bo(struct amdxdna_dev *xdna, struct amdxdna_gem_obj *abo)
    {
    unsigned long contig_sz;
    struct sg_table *sgt;
    dma_addr_t dma_addr;
    struct iova *iova;
    ssize_t size;
    if (abo.type != AMDXDNA_BO_DEV_HEAP && abo.type != AMDXDNA_BO_SHARE)
    return 0;
    sgt = drm_gem_shmem_get_pages_sgt(&abo.base);
    if (IS_ERR(sgt)) {
    XDNA_ERR(xdna, "Get sgt failed, ret %ld", PTR_ERR(sgt));
    return PTR_ERR(sgt);
    }
    if (!sgt.orig_nents) {
    XDNA_ERR(xdna, "sgl is zero length");
    return -EOPNOTSUPP;
    }
    if (amdxdna_iova_on(xdna)) {
    if (!sg_page(sgt.sgl)) {
    XDNA_ERR(xdna, "sgl is not page backed");
    return -EOPNOTSUPP;
    }
    iova = amdxdna_iommu_alloc_iova(xdna, abo.mem.size, &dma_addr,
    (abo.type == AMDXDNA_BO_DEV_HEAP));
    if (IS_ERR(iova)) {
    XDNA_ERR(xdna, "Alloc iova failed, ret %ld", PTR_ERR(iova));
    return PTR_ERR(iova);
    }
    size = iommu_map_sgtable(xdna.domain, dma_addr, sgt,
    IOMMU_READ | IOMMU_WRITE);
    if (size < 0) {
    XDNA_ERR(xdna, "iommu_map_sgtable failed: %zd", size);
    __free_iova(&xdna.iovad, iova);
    return size;
    }
    if (size < abo.mem.size) {
    iommu_unmap(xdna.domain, dma_addr, size);
    __free_iova(&xdna.iovad, iova);
    return -ENXIO;
    }
    abo.mem.dma_addr = dma_addr;
    } else {
// Device doesn't support scatter/gather list, fail non-contiguous mapping.
    contig_sz = drm_prime_get_contiguous_size(sgt);
    if (contig_sz < abo.mem.size) {
    XDNA_ERR(xdna,
    "noncontiguous dma addr, contig size:%ld, expected size:%ld",
    contig_sz, abo.mem.size);
    return -EINVAL;
    }
    abo.mem.dma_addr = sg_dma_address(sgt.sgl);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn amdxdna_dma_unmap_bo(xdna: *mut amdxdna_dev, abo: *mut amdxdna_gem_obj) {
    void amdxdna_dma_unmap_bo(struct amdxdna_dev *xdna, struct amdxdna_gem_obj *abo)
    {
    size_t size;
    if (abo.mem.dma_addr == AMDXDNA_INVALID_ADDR)
    return;
    if (amdxdna_iova_on(xdna)) {
    size = iova_align(&xdna.iovad, abo.mem.size);
    iommu_unmap(xdna.domain, abo.mem.dma_addr, size);
    free_iova(&xdna.iovad, iova_pfn(&xdna.iovad, abo.mem.dma_addr));
    }
    abo.mem.dma_addr = AMDXDNA_INVALID_ADDR;
    }
    void *amdxdna_iommu_alloc(struct amdxdna_dev *xdna, size_t size, dma_addr_t *dma_addr)
    {
    struct iova *iova;
    void *cpu_addr;
    int ret;
    iova = amdxdna_iommu_alloc_iova(xdna, size, dma_addr, true);
    if (IS_ERR(iova)) {
    XDNA_ERR(xdna, "Alloc iova failed, ret %ld", PTR_ERR(iova));
    return iova;
    }
    cpu_addr = (void *)__get_free_pages(GFP_KERNEL, get_order(size));
    if (!cpu_addr) {
    ret = -ENOMEM;
    goto free_iova;
    }
    ret = iommu_map(xdna.domain, *dma_addr, virt_to_phys(cpu_addr),
    iova_align(&xdna.iovad, size),
    IOMMU_READ | IOMMU_WRITE, GFP_KERNEL);
    if (ret)
    goto free_cpu_addr;
    return cpu_addr;
    free_cpu_addr:
    free_pages((unsigned long)cpu_addr, get_order(size));
    free_iova:
    __free_iova(&xdna.iovad, iova);
    return ERR_PTR(ret);
    }
    void amdxdna_iommu_free(struct amdxdna_dev *xdna, size_t size,
    void *cpu_addr, dma_addr_t dma_addr)
    {
    iommu_unmap(xdna.domain, dma_addr, iova_align(&xdna.iovad, size));
    free_iova(&xdna.iovad, iova_pfn(&xdna.iovad, dma_addr));
    free_pages((unsigned long)cpu_addr, get_order(size));
    }
#[no_mangle]
unsafe extern "C" fn amdxdna_cleanup_force_iova(dev: *mut drm_device, res: *mut c_void) {
    static void amdxdna_cleanup_force_iova(struct drm_device *dev, void *res)
    {
    struct amdxdna_dev *xdna = to_xdna_dev(dev);
    if (xdna.domain) {
    iommu_detach_group(xdna.domain, xdna.group);
    put_iova_domain(&xdna.iovad);
    iova_cache_put();
    iommu_domain_free(xdna.domain);
    }
    iommu_group_put(xdna.group);
    }
#[no_mangle]
pub unsafe extern "C" fn amdxdna_iommu_fini(xdna: *mut amdxdna_dev) {
    void amdxdna_iommu_fini(struct amdxdna_dev *xdna)
    {
    if (xdna.group && !xdna.domain)
    iommu_group_put(xdna.group);
    }
#[no_mangle]
pub unsafe extern "C" fn amdxdna_iommu_init(xdna: *mut amdxdna_dev) -> c_int {
    int amdxdna_iommu_init(struct amdxdna_dev *xdna)
    {
    unsigned long order;
    let mut ret: c_int = 0;
    xdna.group = iommu_group_get(xdna.ddev.dev);
    if (!xdna.group || !force_iova)
    return 0;
    XDNA_WARN(xdna, "Enabled force_iova mode.");
    xdna.domain = iommu_paging_domain_alloc_flags(xdna.ddev.dev,
    IOMMU_HWPT_ALLOC_PASID);
    if (IS_ERR(xdna.domain)) {
    XDNA_ERR(xdna, "Failed to alloc iommu domain");
    ret = PTR_ERR(xdna.domain);
    goto put_group;
    }
    ret = iova_cache_get();
    if (ret)
    goto free_domain;
    order = __ffs(xdna.domain.pgsize_bitmap);
    init_iova_domain(&xdna.iovad, 1UL << order, 0);
    ret = iommu_attach_group(xdna.domain, xdna.group);
    if (ret)
    goto put_iova;
    ret = drmm_add_action(&xdna.ddev, amdxdna_cleanup_force_iova, core::ptr::null_mut());
    if (ret)
    goto detach_group;
    return 0;
    detach_group:
    iommu_detach_group(xdna.domain, xdna.group);
    put_iova:
    put_iova_domain(&xdna.iovad);
    iova_cache_put();
    free_domain:
    iommu_domain_free(xdna.domain);
    put_group:
    iommu_group_put(xdna.group);
    xdna.group = core::ptr::null_mut();
    xdna.domain = core::ptr::null_mut();
    return ret;
    }

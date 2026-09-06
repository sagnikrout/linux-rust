//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/nvidia/tegra-vde/iommu.c
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
//
// NVIDIA Tegra Video decoder driver
//
// Copyright (C) 2016-2019 GRATE-DRIVER project
//

    int tegra_vde_iommu_map(struct tegra_vde *vde,
    struct sg_table *sgt,
    struct iova **iovap,
    size_t size)
    {
    struct iova *iova;
    unsigned long shift;
    unsigned long end;
    dma_addr_t addr;
    end = vde.domain.geometry.aperture_end;
    size = iova_align(&vde.iova, size);
    shift = iova_shift(&vde.iova);
    iova = alloc_iova(&vde.iova, size >> shift, end >> shift, true);
    if (!iova)
    return -ENOMEM;
    addr = iova_dma_addr(&vde.iova, iova);
    size = iommu_map_sgtable(vde.domain, addr, sgt,
    IOMMU_READ | IOMMU_WRITE);
    if (!size) {
    __free_iova(&vde.iova, iova);
    return -ENXIO;
    }
// iovap = iova;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_vde_iommu_unmap(vde: *mut tegra_vde, iova: *mut iova) {
    void tegra_vde_iommu_unmap(struct tegra_vde *vde, struct iova *iova)
    {
    let mut shift: c_ulong = iova_shift(&vde.iova);
    let mut size: c_ulong = iova_size(iova) << shift;
    let mut addr: dma_addr_t = iova_dma_addr(&vde.iova, iova);
    iommu_unmap(vde.domain, addr, size);
    __free_iova(&vde.iova, iova);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_vde_iommu_init(vde: *mut tegra_vde) -> c_int {
    int tegra_vde_iommu_init(struct tegra_vde *vde)
    {
    struct device *dev = vde.dev;
    struct iova *iova;
    unsigned long order;
    unsigned long shift;
    int err;
    vde.group = iommu_group_get(dev);
    if (!vde.group)
    return 0;

    if (dev.archdata.mapping) {
    struct dma_iommu_mapping *mapping = to_dma_iommu_mapping(dev);
    arm_iommu_detach_device(dev);
    arm_iommu_release_mapping(mapping);
    }

    vde.domain = iommu_paging_domain_alloc(dev);
    if (IS_ERR(vde.domain)) {
    err = PTR_ERR(vde.domain);
    vde.domain = core::ptr::null_mut();
    goto put_group;
    }
    err = iova_cache_get();
    if (err)
    goto free_domain;
    order = __ffs(vde.domain.pgsize_bitmap);
    init_iova_domain(&vde.iova, 1UL << order, 0);
    err = iommu_attach_group(vde.domain, vde.group);
    if (err)
    goto put_iova;
//
// We're using some static addresses that are not accessible by VDE
// to trap invalid memory accesses.
//
    shift = iova_shift(&vde.iova);
    iova = reserve_iova(&vde.iova, 0x60000000 >> shift,
    0x70000000 >> shift);
    if (!iova) {
    err = -ENOMEM;
    goto detach_group;
    }
    vde.iova_resv_static_addresses = iova;
//
// BSEV's end-address wraps around due to integer overflow during
// of hardware context preparation if IOVA is allocated at the end
// of address space and VDE can't handle that. Hence simply reserve
// the last page to avoid the problem.
//
    iova = reserve_iova(&vde.iova, 0xffffffff >> shift,
    (0xffffffff >> shift) + 1);
    if (!iova) {
    err = -ENOMEM;
    goto unreserve_iova;
    }
    vde.iova_resv_last_page = iova;
    return 0;
    unreserve_iova:
    __free_iova(&vde.iova, vde.iova_resv_static_addresses);
    detach_group:
    iommu_detach_group(vde.domain, vde.group);
    put_iova:
    put_iova_domain(&vde.iova);
    iova_cache_put();
    free_domain:
    iommu_domain_free(vde.domain);
    put_group:
    iommu_group_put(vde.group);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_vde_iommu_deinit(vde: *mut tegra_vde) {
    void tegra_vde_iommu_deinit(struct tegra_vde *vde)
    {
    if (vde.domain) {
    __free_iova(&vde.iova, vde.iova_resv_last_page);
    __free_iova(&vde.iova, vde.iova_resv_static_addresses);
    iommu_detach_group(vde.domain, vde.group);
    put_iova_domain(&vde.iova);
    iova_cache_put();
    iommu_domain_free(vde.domain);
    iommu_group_put(vde.group);
    vde.domain = core::ptr::null_mut();
    }
    }

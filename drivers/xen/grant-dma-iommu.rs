//! Automatically rewritten from C to Rust
//! Source: drivers/xen/grant-dma-iommu.c
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
// Stub IOMMU driver which does nothing.
// The main purpose of it being present is to reuse generic IOMMU device tree
// bindings by Xen grant DMA-mapping layer.
//
// Copyright (C) 2022 EPAM Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grant_dma_iommu_device {
    pub dev: *mut device,
    pub iommu: iommu_device,
}

    static struct iommu_device *grant_dma_iommu_probe_device(struct device *dev)
    {
    return ERR_PTR(-ENODEV);
    }
// Nothing is really needed here except a dummy probe_device callback
    static const struct iommu_ops grant_dma_iommu_ops = {
    .probe_device = grant_dma_iommu_probe_device,
    };
    static const struct of_device_id grant_dma_iommu_of_match[] = {
    { .compatible = "xen,grant-dma" },
    { },
    };
#[no_mangle]
unsafe extern "C" fn grant_dma_iommu_probe(pdev: *mut platform_device) -> c_int {
    static int grant_dma_iommu_probe(struct platform_device *pdev)
    {
    struct grant_dma_iommu_device *mmu;
    int ret;
    mmu = devm_kzalloc(&pdev.dev, sizeof(*mmu), GFP_KERNEL);
    if (!mmu)
    return -ENOMEM;
    mmu.dev = &pdev.dev;
    ret = iommu_device_register(&mmu.iommu, &grant_dma_iommu_ops, &pdev.dev);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, mmu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn grant_dma_iommu_remove(pdev: *mut platform_device) {
    static void grant_dma_iommu_remove(struct platform_device *pdev)
    {
    struct grant_dma_iommu_device *mmu = platform_get_drvdata(pdev);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    iommu_device_unregister(&mmu.iommu);
    }
    static struct platform_driver grant_dma_iommu_driver = {
    .driver = {
    .name = "grant-dma-iommu",
    .of_match_table = grant_dma_iommu_of_match,
    },
    .probe = grant_dma_iommu_probe,
    .remove = grant_dma_iommu_remove,
    };
#[no_mangle]
unsafe extern "C" fn grant_dma_iommu_init() -> int __init {
    static int __init grant_dma_iommu_init(void)
    {
    struct device_node *iommu_np;
    iommu_np = of_find_matching_node(core::ptr::null_mut(), grant_dma_iommu_of_match);
    if (!iommu_np)
    return 0;
    of_node_put(iommu_np);
    return platform_driver_register(&grant_dma_iommu_driver);
    }
    subsys_initcall(grant_dma_iommu_init);

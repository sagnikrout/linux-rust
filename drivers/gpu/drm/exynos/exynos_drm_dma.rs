//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/exynos/exynos_drm_dma.c
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
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// Author: Inki Dae <inki.dae@samsung.com>
// Author: Andrzej Hajda <a.hajda@samsung.com>

pub const EXYNOS_DEV_ADDR_START: c_uint = 0x20000000;
pub const EXYNOS_DEV_ADDR_SIZE: c_uint = 0x40000000;
//
// drm_iommu_attach_device- attach device to iommu mapping
//
// @drm_dev: DRM device
// @subdrv_dev: device to be attach
//
// This function should be called by sub drivers to attach it to iommu
// mapping.
//
    static int drm_iommu_attach_device(struct drm_device *drm_dev,
    struct device *subdrv_dev, void **dma_priv)
    {
    struct exynos_drm_private *priv = drm_dev.dev_private;
    let mut ret: c_int = 0;
    if (get_dma_ops(drm_dev_dma_dev(drm_dev)) != get_dma_ops(subdrv_dev)) {
    DRM_DEV_ERROR(subdrv_dev, "Device %s lacks support for IOMMU\n",
    dev_name(subdrv_dev));
    return -EINVAL;
    }
    dma_set_max_seg_size(subdrv_dev, DMA_BIT_MASK(32));
    if (IS_ENABLED(CONFIG_ARM_DMA_USE_IOMMU)) {
//
// Keep the original DMA mapping of the sub-device and
// restore it on Exynos DRM detach, otherwise the DMA
// framework considers it as IOMMU-less during the next
// probe (in case of deferred probe or modular build)
//
// dma_priv = to_dma_iommu_mapping(subdrv_dev);
    if (*dma_priv)
    arm_iommu_detach_device(subdrv_dev);
    ret = arm_iommu_attach_device(subdrv_dev, priv.mapping);
    } else if (IS_ENABLED(CONFIG_IOMMU_DMA)) {
    ret = iommu_attach_device(priv.mapping, subdrv_dev);
    }
    return ret;
    }
//
// drm_iommu_detach_device -detach device address space mapping from device
//
// @drm_dev: DRM device
// @subdrv_dev: device to be detached
//
// This function should be called by sub drivers to detach it from iommu
// mapping
//
    static void drm_iommu_detach_device(struct drm_device *drm_dev,
    struct device *subdrv_dev, void **dma_priv)
    {
    struct exynos_drm_private *priv = drm_dev.dev_private;
    if (IS_ENABLED(CONFIG_ARM_DMA_USE_IOMMU)) {
    arm_iommu_detach_device(subdrv_dev);
    arm_iommu_attach_device(subdrv_dev, *dma_priv);
    } else if (IS_ENABLED(CONFIG_IOMMU_DMA))
    iommu_detach_device(priv.mapping, subdrv_dev);
    }
    int exynos_drm_register_dma(struct drm_device *drm, struct device *dev,
    void **dma_priv)
    {
    struct exynos_drm_private *priv = drm.dev_private;
    if (drm_dev_dma_dev(drm) == drm.dev) {
    drm_dev_set_dma_dev(drm, dev);
    DRM_INFO("Exynos DRM: using %s device for DMA mapping operations\n",
    dev_name(dev));
    }
    if (!IS_ENABLED(CONFIG_EXYNOS_IOMMU))
    return 0;
    if (!priv.mapping) {
    void *mapping = core::ptr::null_mut();
    if (IS_ENABLED(CONFIG_ARM_DMA_USE_IOMMU))
    mapping = arm_iommu_create_mapping(dev,
    EXYNOS_DEV_ADDR_START, EXYNOS_DEV_ADDR_SIZE);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_ENABLED(CONFIG_IOMMU_DMA)) -> else {
    else if (IS_ENABLED(CONFIG_IOMMU_DMA))
    mapping = iommu_get_domain_for_dev(dev);
    if (!mapping)
    return -ENODEV;
    priv.mapping = mapping;
    }
    return drm_iommu_attach_device(drm, dev, dma_priv);
    }
    void exynos_drm_unregister_dma(struct drm_device *drm, struct device *dev,
    void **dma_priv)
    {
    if (IS_ENABLED(CONFIG_EXYNOS_IOMMU))
    drm_iommu_detach_device(drm, dev, dma_priv);
    }
#[no_mangle]
pub unsafe extern "C" fn exynos_drm_cleanup_dma(drm: *mut drm_device) {
    void exynos_drm_cleanup_dma(struct drm_device *drm)
    {
    struct exynos_drm_private *priv = drm.dev_private;
    if (!IS_ENABLED(CONFIG_EXYNOS_IOMMU))
    return;
    arm_iommu_release_mapping(priv.mapping);
    priv.mapping = core::ptr::null_mut();
    drm_dev_set_dma_dev(drm, core::ptr::null_mut());
    }

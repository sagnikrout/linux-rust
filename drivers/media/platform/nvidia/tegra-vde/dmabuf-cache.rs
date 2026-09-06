//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/nvidia/tegra-vde/dmabuf-cache.c
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

    MODULE_IMPORT_NS("DMA_BUF");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_vde_cache_entry {
    pub dma_dir: enum dma_data_direction,
    pub a: *mut dma_buf_attachment,
    pub dwork: delayed_work,
    pub vde: *mut tegra_vde,
    pub list: list_head,
    pub sgt: *mut sg_table,
    pub iova: *mut iova,
    pub refcnt: c_uint,
}

#[no_mangle]
unsafe extern "C" fn tegra_vde_release_entry(entry: *mut tegra_vde_cache_entry) {
    static void tegra_vde_release_entry(struct tegra_vde_cache_entry *entry)
    {
    struct dma_buf *dmabuf = entry.a.dmabuf;
    WARN_ON_ONCE(entry.refcnt);
    if (entry.vde.domain)
    tegra_vde_iommu_unmap(entry.vde, entry.iova);
    dma_buf_unmap_attachment_unlocked(entry.a, entry.sgt, entry.dma_dir);
    dma_buf_detach(dmabuf, entry.a);
    dma_buf_put(dmabuf);
    list_del(&entry.list);
    kfree(entry);
    }
#[no_mangle]
unsafe extern "C" fn tegra_vde_delayed_unmap(work: *mut work_struct) {
    static void tegra_vde_delayed_unmap(struct work_struct *work)
    {
    struct tegra_vde_cache_entry *entry;
    struct tegra_vde *vde;
    entry = container_of(work, struct tegra_vde_cache_entry,
    dwork.work);
    vde = entry.vde;
    mutex_lock(&vde.map_lock);
    tegra_vde_release_entry(entry);
    mutex_unlock(&vde.map_lock);
    }
    int tegra_vde_dmabuf_cache_map(struct tegra_vde *vde,
    struct dma_buf *dmabuf,
    enum dma_data_direction dma_dir,
    struct dma_buf_attachment **ap,
    dma_addr_t *addrp)
    {
    struct dma_buf_attachment *attachment;
    struct tegra_vde_cache_entry *entry;
    struct device *dev = vde.dev;
    struct sg_table *sgt;
    struct iova *iova;
    int err;
    mutex_lock(&vde.map_lock);
    list_for_each_entry(entry, &vde.map_list, list) {
    if (entry.a.dmabuf != dmabuf)
    continue;
    if (!cancel_delayed_work(&entry.dwork))
    continue;
    if (entry.dma_dir != dma_dir)
    entry.dma_dir = DMA_BIDIRECTIONAL;
    dma_buf_put(dmabuf);
    if (vde.domain)
// addrp = iova_dma_addr(&vde->iova, entry->iova);
    else
// addrp = sg_dma_address(entry->sgt->sgl);
    goto ref;
    }
    attachment = dma_buf_attach(dmabuf, dev);
    if (IS_ERR(attachment)) {
    dev_err(dev, "Failed to attach dmabuf\n");
    err = PTR_ERR(attachment);
    goto err_unlock;
    }
    sgt = dma_buf_map_attachment_unlocked(attachment, dma_dir);
    if (IS_ERR(sgt)) {
    dev_err(dev, "Failed to get dmabufs sg_table\n");
    err = PTR_ERR(sgt);
    goto err_detach;
    }
    if (!vde.domain && sgt.nents > 1) {
    dev_err(dev, "Sparse DMA region is unsupported, please enable IOMMU\n");
    err = -EINVAL;
    goto err_unmap;
    }
    entry = kzalloc_obj(*entry);
    if (!entry) {
    err = -ENOMEM;
    goto err_unmap;
    }
    if (vde.domain) {
    err = tegra_vde_iommu_map(vde, sgt, &iova, dmabuf.size);
    if (err)
    goto err_free;
// addrp = iova_dma_addr(&vde->iova, iova);
    } else {
// addrp = sg_dma_address(sgt->sgl);
    iova = core::ptr::null_mut();
    }
    INIT_DELAYED_WORK(&entry.dwork, tegra_vde_delayed_unmap);
    list_add(&entry.list, &vde.map_list);
    entry.dma_dir = dma_dir;
    entry.iova = iova;
    entry.vde = vde;
    entry.sgt = sgt;
    entry.a = attachment;
    ref:
    entry.refcnt++;
// ap = entry->a;
    mutex_unlock(&vde.map_lock);
    return 0;
    err_free:
    kfree(entry);
    err_unmap:
    dma_buf_unmap_attachment_unlocked(attachment, sgt, dma_dir);
    err_detach:
    dma_buf_detach(dmabuf, attachment);
    err_unlock:
    mutex_unlock(&vde.map_lock);
    return err;
    }
    void tegra_vde_dmabuf_cache_unmap(struct tegra_vde *vde,
    struct dma_buf_attachment *a,
    bool release)
    {
    struct tegra_vde_cache_entry *entry;
    mutex_lock(&vde.map_lock);
    list_for_each_entry(entry, &vde.map_list, list) {
    if (entry.a != a)
    continue;
    WARN_ON_ONCE(!entry.refcnt);
    if (--entry.refcnt == 0) {
    if (release)
    tegra_vde_release_entry(entry);
    else
    schedule_delayed_work(&entry.dwork, 5 * HZ);
    }
    break;
    }
    mutex_unlock(&vde.map_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_vde_dmabuf_cache_unmap_sync(vde: *mut tegra_vde) {
    void tegra_vde_dmabuf_cache_unmap_sync(struct tegra_vde *vde)
    {
    struct tegra_vde_cache_entry *entry, *tmp;
    mutex_lock(&vde.map_lock);
    list_for_each_entry_safe(entry, tmp, &vde.map_list, list) {
    if (entry.refcnt)
    continue;
    if (!cancel_delayed_work(&entry.dwork))
    continue;
    tegra_vde_release_entry(entry);
    }
    mutex_unlock(&vde.map_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_vde_dmabuf_cache_unmap_all(vde: *mut tegra_vde) {
    void tegra_vde_dmabuf_cache_unmap_all(struct tegra_vde *vde)
    {
    struct tegra_vde_cache_entry *entry, *tmp;
    mutex_lock(&vde.map_lock);
    while (!list_empty(&vde.map_list)) {
    list_for_each_entry_safe(entry, tmp, &vde.map_list, list) {
    if (!cancel_delayed_work(&entry.dwork))
    continue;
    tegra_vde_release_entry(entry);
    }
    mutex_unlock(&vde.map_lock);
    schedule();
    mutex_lock(&vde.map_lock);
    }
    mutex_unlock(&vde.map_lock);
    }

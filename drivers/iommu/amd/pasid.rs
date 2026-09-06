//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/amd/pasid.c
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
// Copyright (C) 2024 Advanced Micro Devices, Inc.
//

#[no_mangle]
pub unsafe extern "C" fn is_pasid_enabled(dev_data: *mut iommu_dev_data) -> bool {
    static inline bool is_pasid_enabled(struct iommu_dev_data *dev_data)
    {
    if (dev_data.pasid_enabled && dev_data.max_pasids &&
    dev_data.gcr3_info.gcr3_tbl != core::ptr::null_mut())
    return true;
    return false;
    }
    static inline bool is_pasid_valid(struct iommu_dev_data *dev_data,
    ioasid_t pasid)
    {
    if (pasid > 0 && pasid < dev_data.max_pasids)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn remove_dev_pasid(pdom_dev_data: *mut pdom_dev_data) {
    static void remove_dev_pasid(struct pdom_dev_data *pdom_dev_data)
    {
// Update GCR3 table and flush IOTLB
    amd_iommu_clear_gcr3(pdom_dev_data.dev_data, pdom_dev_data.pasid);
    list_del(&pdom_dev_data.list);
    kfree(pdom_dev_data);
    }
// Clear PASID from device GCR3 table and remove pdom_dev_data from list
    static void remove_pdom_dev_pasid(struct protection_domain *pdom,
    struct device *dev, ioasid_t pasid)
    {
    struct pdom_dev_data *pdom_dev_data;
    struct iommu_dev_data *dev_data = dev_iommu_priv_get(dev);
    lockdep_assert_held(&pdom.lock);
    for_each_pdom_dev_data(pdom_dev_data, pdom) {
    if (pdom_dev_data.dev_data == dev_data &&
    pdom_dev_data.pasid == pasid) {
    remove_dev_pasid(pdom_dev_data);
    break;
    }
    }
    }
    static void sva_arch_invalidate_secondary_tlbs(struct mmu_notifier *mn,
    struct mm_struct *mm,
    unsigned long start, unsigned long end)
    {
    struct pdom_dev_data *pdom_dev_data;
    struct protection_domain *sva_pdom;
    unsigned long flags;
    sva_pdom = container_of(mn, struct protection_domain, mn);
    spin_lock_irqsave(&sva_pdom.lock, flags);
    for_each_pdom_dev_data(pdom_dev_data, sva_pdom) {
    amd_iommu_dev_flush_pasid_pages(pdom_dev_data.dev_data,
    pdom_dev_data.pasid,
    start, end - 1);
    }
    spin_unlock_irqrestore(&sva_pdom.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sva_mn_release(mn: *mut mmu_notifier, mm: *mut mm_struct) {
    static void sva_mn_release(struct mmu_notifier *mn, struct mm_struct *mm)
    {
    struct pdom_dev_data *pdom_dev_data, *next;
    struct protection_domain *sva_pdom;
    unsigned long flags;
    sva_pdom = container_of(mn, struct protection_domain, mn);
    spin_lock_irqsave(&sva_pdom.lock, flags);
// Assume dev_data_list contains same PASID with different devices
    for_each_pdom_dev_data_safe(pdom_dev_data, next, sva_pdom)
    remove_dev_pasid(pdom_dev_data);
    spin_unlock_irqrestore(&sva_pdom.lock, flags);
    }
    static const struct mmu_notifier_ops sva_mn = {
    .arch_invalidate_secondary_tlbs = sva_arch_invalidate_secondary_tlbs,
    .release = sva_mn_release,
    };
    int iommu_sva_set_dev_pasid(struct iommu_domain *domain,
    struct device *dev, ioasid_t pasid,
    struct iommu_domain *old)
    {
    struct pdom_dev_data *pdom_dev_data;
    struct protection_domain *sva_pdom = to_pdomain(domain);
    struct iommu_dev_data *dev_data = dev_iommu_priv_get(dev);
    unsigned long flags;
    let mut ret: c_int = -EINVAL;
    if (old)
    return -EOPNOTSUPP;
// PASID zero is used for requests from the I/O device without PASID
    if (!is_pasid_valid(dev_data, pasid))
    return ret;
// Make sure PASID is enabled
    if (!is_pasid_enabled(dev_data))
    return ret;
// Add PASID to protection domain pasid list
    pdom_dev_data = kzalloc_obj(*pdom_dev_data);
    if (pdom_dev_data == core::ptr::null_mut())
    return ret;
    pdom_dev_data.pasid = pasid;
    pdom_dev_data.dev_data = dev_data;
    spin_lock_irqsave(&sva_pdom.lock, flags);
// Setup GCR3 table
    ret = amd_iommu_set_gcr3(dev_data, pasid,
    iommu_virt_to_phys(domain.mm.pgd));
    if (ret) {
    kfree(pdom_dev_data);
    goto out_unlock;
    }
    list_add(&pdom_dev_data.list, &sva_pdom.dev_data_list);
    out_unlock:
    spin_unlock_irqrestore(&sva_pdom.lock, flags);
    return ret;
    }
    void amd_iommu_remove_dev_pasid(struct device *dev, ioasid_t pasid,
    struct iommu_domain *domain)
    {
    struct protection_domain *sva_pdom;
    unsigned long flags;
    if (!is_pasid_valid(dev_iommu_priv_get(dev), pasid))
    return;
    sva_pdom = to_pdomain(domain);
    spin_lock_irqsave(&sva_pdom.lock, flags);
// Remove PASID from dev_data_list
    remove_pdom_dev_pasid(sva_pdom, dev, pasid);
    spin_unlock_irqrestore(&sva_pdom.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn iommu_sva_domain_free(domain: *mut iommu_domain) {
    static void iommu_sva_domain_free(struct iommu_domain *domain)
    {
    struct protection_domain *sva_pdom = to_pdomain(domain);
    if (sva_pdom.mn.ops)
    mmu_notifier_unregister(&sva_pdom.mn, domain.mm);
    amd_iommu_domain_free(domain);
    }
    static const struct iommu_domain_ops amd_sva_domain_ops = {
    .set_dev_pasid = iommu_sva_set_dev_pasid,
    .free	       = iommu_sva_domain_free
    };
    struct iommu_domain *amd_iommu_domain_alloc_sva(struct device *dev,
    struct mm_struct *mm)
    {
    struct protection_domain *pdom;
    int ret;
    pdom = protection_domain_alloc();
    if (!pdom)
    return ERR_PTR(-ENOMEM);
    pdom.domain.ops = &amd_sva_domain_ops;
    pdom.mn.ops = &sva_mn;
    pdom.domain.type = IOMMU_DOMAIN_SVA;
    ret = mmu_notifier_register(&pdom.mn, mm);
    if (ret) {
    amd_iommu_domain_free(&pdom.domain);
    return ERR_PTR(ret);
    }
    return &pdom.domain;
    }

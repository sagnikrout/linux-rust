//! Automatically rewritten from C to Rust
//! Source: drivers/iommu/iommu-debug-pagealloc.c
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
// Copyright (C) 2025 - Google Inc
// Author: Mostafa Saleh <smostafa@google.com>
// IOMMU API debug page alloc sanitizer
//

    static bool needed;
    DEFINE_STATIC_KEY_FALSE(iommu_debug_initialized);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_debug_metadata {
    pub ref: core::sync::atomic::AtomicI32,
}

#[no_mangle]
unsafe extern "C" fn need_iommu_debug() -> __init bool {
    static __init bool need_iommu_debug(void)
    {
    return needed;
    }
    struct page_ext_operations page_iommu_debug_ops = {
    .size = sizeof(struct iommu_debug_metadata),
    .need = need_iommu_debug,
    };
    static struct iommu_debug_metadata *get_iommu_data(struct page_ext *page_ext)
    {
    return page_ext_data(page_ext, &page_iommu_debug_ops);
    }
#[no_mangle]
unsafe extern "C" fn iommu_debug_inc_page(phys: phys_addr_t) {
    static void iommu_debug_inc_page(phys_addr_t phys)
    {
    struct page_ext *page_ext = page_ext_from_phys(phys);
    struct iommu_debug_metadata *d;
    if (!page_ext)
    return;
    d = get_iommu_data(page_ext);
    WARN_ON(atomic_inc_return_relaxed(&d.ref) <= 0);
    page_ext_put(page_ext);
    }
#[no_mangle]
unsafe extern "C" fn iommu_debug_dec_page(phys: phys_addr_t) {
    static void iommu_debug_dec_page(phys_addr_t phys)
    {
    struct page_ext *page_ext = page_ext_from_phys(phys);
    struct iommu_debug_metadata *d;
    if (!page_ext)
    return;
    d = get_iommu_data(page_ext);
    WARN_ON(atomic_dec_return_relaxed(&d.ref) < 0);
    page_ext_put(page_ext);
    }
//
// IOMMU page size doesn't have to match the CPU page size. So, we use
// the smallest IOMMU page size to refcount the pages in the vmemmap.
// That is important as both map and unmap has to use the same page size
// to update the refcount to avoid double counting the same page.
// And as we can't know from iommu_unmap() what was the original page size
// used for map, we just use the minimum supported one for both.
//
#[no_mangle]
unsafe extern "C" fn iommu_debug_page_size(domain: *mut iommu_domain) -> usize {
    static size_t iommu_debug_page_size(struct iommu_domain *domain)
    {
    return 1UL << __ffs(domain.pgsize_bitmap);
    }
#[no_mangle]
unsafe extern "C" fn iommu_debug_page_count(page: *const page) -> bool {
    static bool iommu_debug_page_count(const struct page *page)
    {
    unsigned int ref;
    struct page_ext *page_ext = page_ext_get(page);
    struct iommu_debug_metadata *d = get_iommu_data(page_ext);
    ref = atomic_read(&d.ref);
    page_ext_put(page_ext);
    return ref != 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __iommu_debug_check_unmapped(page: *const page, numpages: c_int) {
    void __iommu_debug_check_unmapped(const struct page *page, int numpages)
    {
    while (numpages--) {
    if (WARN_ON(iommu_debug_page_count(page))) {
    pr_warn("iommu: Detected page leak!\n");
    dump_page_owner(page);
    }
    page++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __iommu_debug_map(domain: *mut iommu_domain, phys: phys_addr_t, size: usize) {
    void __iommu_debug_map(struct iommu_domain *domain, phys_addr_t phys, size_t size)
    {
    size_t off, end;
    let mut page_size: usize = iommu_debug_page_size(domain);
    if (WARN_ON(!phys || check_add_overflow(phys, size, &end)))
    return;
    for (off = 0 ; off < size ; off += page_size)
    iommu_debug_inc_page(phys + off);
    }
    static void __iommu_debug_update_iova(struct iommu_domain *domain,
    unsigned long iova, size_t size, bool inc)
    {
    size_t off, end;
    let mut page_size: usize = iommu_debug_page_size(domain);
    if (WARN_ON(check_add_overflow(iova, size, &end)))
    return;
    for (off = 0 ; off < size ; off += page_size) {
    let mut phys: phys_addr_t = iommu_iova_to_phys(domain, iova + off);
    if (!phys)
    continue;
    if (inc)
    iommu_debug_inc_page(phys);
    else
    iommu_debug_dec_page(phys);
    }
    }
    void __iommu_debug_unmap_begin(struct iommu_domain *domain,
    unsigned long iova, size_t size)
    {
    __iommu_debug_update_iova(domain, iova, size, false);
    }
    void __iommu_debug_unmap_end(struct iommu_domain *domain,
    unsigned long iova, size_t size,
    size_t unmapped)
    {
    if ((unmapped == size) || WARN_ON_ONCE(unmapped > size))
    return;
// If unmap failed, re-increment the refcount.
    __iommu_debug_update_iova(domain, iova + unmapped,
    size - unmapped, true);
    }
#[no_mangle]
pub unsafe extern "C" fn iommu_debug_init() {
    void iommu_debug_init(void)
    {
    if (!needed)
    return;
    pr_info("iommu: Debugging page allocations, expect overhead or disable iommu.debug_pagealloc");
    static_branch_enable(&iommu_debug_initialized);
    }
#[no_mangle]
unsafe extern "C" fn iommu_debug_pagealloc(str: *mut c_char) -> int __init {
    static int __init iommu_debug_pagealloc(char *str)
    {
    return kstrtobool(str, &needed);
    }
    early_param("iommu.debug_pagealloc", iommu_debug_pagealloc);

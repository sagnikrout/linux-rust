//! Automatically rewritten from C to Rust
//! Source: drivers/xen/unpopulated-alloc.c
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

    static DEFINE_MUTEX(list_lock);
    static struct page *page_list;
    static unsigned int list_count;
    static struct resource *target_resource;
// Pages to subtract from the memory count when setting balloon target.
    unsigned long xen_unpopulated_pages __initdata;
//
// If arch is not happy with system "iomem_resource" being used for
// the region allocation it can provide it's own view by creating specific
// Xen resource with unused regions of guest physical address space provided
// by the hypervisor.
//
#[no_mangle]
pub unsafe extern "C" fn arch_xen_unpopulated_init(res: *mut resource) -> int __weak __init {
    int __weak __init arch_xen_unpopulated_init(struct resource **res)
    {
// res = &iomem_resource;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fill_list(nr_pages: c_uint) -> c_int {
    static int fill_list(unsigned int nr_pages)
    {
    struct dev_pagemap *pgmap;
    struct resource *res, *tmp_res = core::ptr::null_mut();
    void *vaddr;
    unsigned int i, alloc_pages = round_up(nr_pages, PAGES_PER_SECTION);
    struct range mhp_range;
    int ret;
    res = kzalloc_obj(*res);
    if (!res)
    return -ENOMEM;
    res.name = "Xen scratch";
    res.flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    mhp_range = mhp_get_pluggable_range(true);
    ret = allocate_resource(target_resource, res,
    alloc_pages * PAGE_SIZE, mhp_range.start, mhp_range.end,
    PAGES_PER_SECTION * PAGE_SIZE, core::ptr::null_mut(), core::ptr::null_mut());
    if (ret < 0) {
    pr_err("Cannot allocate new IOMEM resource\n");
    goto err_resource;
    }
//
// Reserve the region previously allocated from Xen resource to avoid
// re-using it by someone else.
//
    if (target_resource != &iomem_resource) {
    tmp_res = kzalloc_obj(*tmp_res);
    if (!tmp_res) {
    ret = -ENOMEM;
    goto err_insert;
    }
    tmp_res.name = res.name;
    tmp_res.start = res.start;
    tmp_res.end = res.end;
    tmp_res.flags = res.flags;
    ret = request_resource(&iomem_resource, tmp_res);
    if (ret < 0) {
    pr_err("Cannot request resource %pR (%d)\n", tmp_res, ret);
    kfree(tmp_res);
    goto err_insert;
    }
    }
    pgmap = kzalloc_obj(*pgmap);
    if (!pgmap) {
    ret = -ENOMEM;
    goto err_pgmap;
    }
    pgmap.type = MEMORY_DEVICE_GENERIC;
    pgmap.range = (struct range) {
    .start = res.start,
    .end = res.end,
    };
    pgmap.nr_range = 1;
    pgmap.owner = res;

//
// memremap will build page tables for the new memory so
// the p2m must contain invalid entries so the correct
// non-present PTEs will be written.
//
// If a failure occurs, the original (identity) p2m entries
// are not restored since this region is now known not to
// conflict with any devices.
//
    if (xen_pv_domain()) {
    let mut pfn: xen_pfn_t = PFN_DOWN(res.start);
    for (i = 0; i < alloc_pages; i++) {
    if (!set_phys_to_machine(pfn + i, INVALID_P2M_ENTRY)) {
    pr_warn("set_phys_to_machine() failed, no memory added\n");
    ret = -ENOMEM;
    goto err_memremap;
    }
    }
    }

    vaddr = memremap_pages(pgmap, NUMA_NO_NODE);
    if (IS_ERR(vaddr)) {
    pr_err("Cannot remap memory range\n");
    ret = PTR_ERR(vaddr);
    goto err_memremap;
    }
    for (i = 0; i < alloc_pages; i++) {
    struct page *pg = virt_to_page(vaddr + PAGE_SIZE * i);
    pg.zone_device_data = page_list;
    page_list = pg;
    list_count++;
    }
    return 0;
    err_memremap:
    kfree(pgmap);
    err_pgmap:
    if (tmp_res) {
    release_resource(tmp_res);
    kfree(tmp_res);
    }
    err_insert:
    release_resource(res);
    err_resource:
    kfree(res);
    return ret;
    }
//
// xen_alloc_unpopulated_pages - alloc unpopulated pages
// @nr_pages: Number of pages
// @pages: pages returned
// @return 0 on success, error otherwise
//
#[no_mangle]
pub unsafe extern "C" fn xen_alloc_unpopulated_pages(nr_pages: c_uint, pages: *mut page) -> c_int {
    int xen_alloc_unpopulated_pages(unsigned int nr_pages, struct page **pages)
    {
    unsigned int i;
    let mut ret: c_int = 0;
//
// Fallback to default behavior if we do not have any suitable resource
// to allocate required region from and as the result we won't be able to
// construct pages.
//
    if (!target_resource)
    return xen_alloc_ballooned_pages(nr_pages, pages);
    mutex_lock(&list_lock);
    if (list_count < nr_pages) {
    ret = fill_list(nr_pages - list_count);
    if (ret)
    goto out;
    }
    for (i = 0; i < nr_pages; i++) {
    struct page *pg = page_list;
    BUG_ON(!pg);
    page_list = pg.zone_device_data;
    list_count--;
    pages[i] = pg;

    if (xen_pv_domain()) {
    ret = xen_alloc_p2m_entry(page_to_pfn(pg));
    if (ret < 0) {
    unsigned int j;
    for (j = 0; j <= i; j++) {
    pages[j].zone_device_data = page_list;
    page_list = pages[j];
    list_count++;
    }
    goto out;
    }
    }

    }
    out:
    mutex_unlock(&list_lock);
    return ret;
    }
    EXPORT_SYMBOL(xen_alloc_unpopulated_pages);
//
// xen_free_unpopulated_pages - return unpopulated pages
// @nr_pages: Number of pages
// @pages: pages to return
//
#[no_mangle]
pub unsafe extern "C" fn xen_free_unpopulated_pages(nr_pages: c_uint, pages: *mut page) {
    void xen_free_unpopulated_pages(unsigned int nr_pages, struct page **pages)
    {
    unsigned int i;
    if (!target_resource) {
    xen_free_ballooned_pages(nr_pages, pages);
    return;
    }
    mutex_lock(&list_lock);
    for (i = 0; i < nr_pages; i++) {
    pages[i].zone_device_data = page_list;
    page_list = pages[i];
    list_count++;
    }
    mutex_unlock(&list_lock);
    }
    EXPORT_SYMBOL(xen_free_unpopulated_pages);
#[no_mangle]
unsafe extern "C" fn unpopulated_init() -> int __init {
    static int __init unpopulated_init(void)
    {
    int ret;
    if (!xen_domain())
    return -ENODEV;
    ret = arch_xen_unpopulated_init(&target_resource);
    if (ret) {
    pr_err("xen:unpopulated: Cannot initialize target resource\n");
    target_resource = core::ptr::null_mut();
    }
    return ret;
    }
    early_initcall(unpopulated_init);

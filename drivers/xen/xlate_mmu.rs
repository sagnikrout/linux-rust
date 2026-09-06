//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xlate_mmu.c
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


//
// MMU operations common to all auto-translated physmap guests.
//
// Copyright (C) 2015 Citrix Systems R&D Ltd.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

    typedef void (*xen_gfn_fn_t)(unsigned long gfn, void *data);
// Break down the pages in 4KB chunk and call fn for each gfn
    static void xen_for_each_gfn(struct page **pages, unsigned nr_gfn,
    xen_gfn_fn_t fn, void *data)
    {
    let mut xen_pfn: c_ulong = 0;
    struct page *page;
    int i;
    for (i = 0; i < nr_gfn; i++) {
    if ((i % XEN_PFN_PER_PAGE) == 0) {
    page = pages[i / XEN_PFN_PER_PAGE];
    xen_pfn = page_to_xen_pfn(page);
    }
    fn(pfn_to_gfn(xen_pfn++), data);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct remap_data {
    pub /: *mut *mut *mut xen_pfn_t fgfn; / foreign domain's gfn,
    pub /: *mut *mut int nr_fgfn; / Number of foreign gfn left to map,
    pub prot: pgprot_t,
    pub domid: domid_t,
    pub vma: *mut vm_area_struct,
    pub index: c_int,
    pub pages: *mut page,
    pub info: *mut xen_remap_gfn_info,
    pub err_ptr: *mut c_int,
    pub mapped: c_int,
// Hypercall parameters
    pub h_errs: [c_int; XEN_PFN_PER_PAGE],
    pub h_idxs: [xen_ulong_t; XEN_PFN_PER_PAGE],
    pub h_gpfns: [xen_pfn_t; XEN_PFN_PER_PAGE],
    pub /: *mut *mut int h_iter; / Iterator,
}

#[no_mangle]
unsafe extern "C" fn setup_hparams(gfn: c_ulong, data: *mut c_void) {
    static void setup_hparams(unsigned long gfn, void *data)
    {
    struct remap_data *info = data;
    info.h_idxs[info.h_iter] = *info.fgfn;
    info.h_gpfns[info.h_iter] = gfn;
    info.h_errs[info.h_iter] = 0;
    info.h_iter++;
    info.fgfn++;
    }
#[no_mangle]
unsafe extern "C" fn remap_pte_fn(ptep: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    static int remap_pte_fn(pte_t *ptep, unsigned long addr, void *data)
    {
    struct remap_data *info = data;
    struct page *page = info.pages[info.index++];
    let mut pte: pte_t = pte_mkspecial(pfn_pte(page_to_pfn(page), info.prot));
    int rc, nr_gfn;
    uint32_t i;
    struct xen_add_to_physmap_range xatp = {
    .domid = DOMID_SELF,
    .foreign_domid = info.domid,
    .space = XENMAPSPACE_gmfn_foreign,
    };
    nr_gfn = min_t(typeof(info.nr_fgfn), XEN_PFN_PER_PAGE, info.nr_fgfn);
    info.nr_fgfn -= nr_gfn;
    info.h_iter = 0;
    xen_for_each_gfn(&page, nr_gfn, setup_hparams, info);
    BUG_ON(info.h_iter != nr_gfn);
    set_xen_guest_handle(xatp.idxs, info.h_idxs);
    set_xen_guest_handle(xatp.gpfns, info.h_gpfns);
    set_xen_guest_handle(xatp.errs, info.h_errs);
    xatp.size = nr_gfn;
    rc = HYPERVISOR_memory_op(XENMEM_add_to_physmap_range, &xatp);
// info->err_ptr expect to have one error status per Xen PFN
    for (i = 0; i < nr_gfn; i++) {
    let mut err: c_int = (rc < 0) ? rc : info.h_errs[i];
// (info->err_ptr++) = err;
    if (!err)
    info.mapped++;
    }
//
// Note: The hypercall will return 0 in most of the case if even if
// all the fgmfn are not mapped. We still have to update the pte
// as the userspace may decide to continue.
//
    if (!rc)
    set_pte_at(info.vma.vm_mm, addr, ptep, pte);
    return 0;
    }
    int xen_xlate_remap_gfn_array(struct vm_area_struct *vma,
    unsigned long addr,
    xen_pfn_t *gfn, int nr,
    int *err_ptr, pgprot_t prot,
    unsigned domid,
    struct page **pages)
    {
    int err;
    struct remap_data data;
    let mut range: c_ulong = DIV_ROUND_UP(nr, XEN_PFN_PER_PAGE) << PAGE_SHIFT;
// Kept here for the purpose of making sure code doesn't break
    x86 PVOPS */
    BUG_ON(!((vma.vm_flags & (VM_PFNMAP | VM_IO)) == (VM_PFNMAP | VM_IO)));
    data.fgfn = gfn;
    data.nr_fgfn = nr;
    data.prot  = prot;
    data.domid = domid;
    data.vma   = vma;
    data.pages = pages;
    data.index = 0;
    data.err_ptr = err_ptr;
    data.mapped = 0;
    err = apply_to_page_range(vma.vm_mm, addr, range,
    remap_pte_fn, &data);
    return err < 0 ? err : data.mapped;
    }
    EXPORT_SYMBOL_GPL(xen_xlate_remap_gfn_array);
#[no_mangle]
unsafe extern "C" fn unmap_gfn(gfn: c_ulong, data: *mut c_void) {
    static void unmap_gfn(unsigned long gfn, void *data)
    {
    struct xen_remove_from_physmap xrp;
    xrp.domid = DOMID_SELF;
    xrp.gpfn = gfn;
    (void)HYPERVISOR_memory_op(XENMEM_remove_from_physmap, &xrp);
    }
    int xen_xlate_unmap_gfn_range(struct vm_area_struct *vma,
    int nr, struct page **pages)
    {
    xen_for_each_gfn(pages, nr, unmap_gfn, core::ptr::null_mut());
    return 0;
    }
    EXPORT_SYMBOL_GPL(xen_xlate_unmap_gfn_range);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_balloon_pages {
    pub pfns: *mut xen_pfn_t,
    pub idx: c_uint,
}

#[no_mangle]
unsafe extern "C" fn setup_balloon_gfn(gfn: c_ulong, data: *mut c_void) {
    static void setup_balloon_gfn(unsigned long gfn, void *data)
    {
    struct map_balloon_pages *info = data;
    info.pfns[info.idx++] = gfn;
    }
//
// xen_xlate_map_ballooned_pages - map a new set of ballooned pages
// @gfns: returns the array of corresponding GFNs
// @virt: returns the virtual address of the mapped region
// @nr_grant_frames: number of GFNs
// @return 0 on success, error otherwise
//
// This allocates a set of ballooned pages and maps them into the
// kernel's address space.
//
    int __init xen_xlate_map_ballooned_pages(xen_pfn_t **gfns, void **virt,
    unsigned long nr_grant_frames)
    {
    struct page **pages;
    xen_pfn_t *pfns;
    void *vaddr;
    struct map_balloon_pages data;
    int rc;
    unsigned long nr_pages;
    BUG_ON(nr_grant_frames == 0);
    nr_pages = DIV_ROUND_UP(nr_grant_frames, XEN_PFN_PER_PAGE);
    pages = kzalloc_objs(pages[0], nr_pages);
    if (!pages)
    return -ENOMEM;
    pfns = kzalloc_objs(pfns[0], nr_grant_frames);
    if (!pfns) {
    kfree(pages);
    return -ENOMEM;
    }
    rc = xen_alloc_unpopulated_pages(nr_pages, pages);
    if (rc) {
    pr_warn("%s Couldn't balloon alloc %ld pages rc:%d\n", __func__,
    nr_pages, rc);
    kfree(pages);
    kfree(pfns);
    return rc;
    }
    data.pfns = pfns;
    data.idx = 0;
    xen_for_each_gfn(pages, nr_grant_frames, setup_balloon_gfn, &data);
    vaddr = vmap(pages, nr_pages, 0, PAGE_KERNEL);
    if (!vaddr) {
    pr_warn("%s Couldn't map %ld pages rc:%d\n", __func__,
    nr_pages, rc);
    xen_free_unpopulated_pages(nr_pages, pages);
    kfree(pages);
    kfree(pfns);
    return -ENOMEM;
    }
    kfree(pages);
// gfns = pfns;
// virt = vaddr;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct remap_pfn {
    pub mm: *mut mm_struct,
    pub pages: *mut page,
    pub prot: pgprot_t,
    pub i: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn remap_pfn_fn(ptep: *mut pte_t, addr: c_ulong, data: *mut c_void) -> c_int {
    static int remap_pfn_fn(pte_t *ptep, unsigned long addr, void *data)
    {
    struct remap_pfn *r = data;
    struct page *page = r.pages[r.i];
    let mut pte: pte_t = pte_mkspecial(pfn_pte(page_to_pfn(page), r.prot));
    set_pte_at(r.mm, addr, ptep, pte);
    r.i++;
    return 0;
    }
// Used by the privcmd module, but has to be built-in on ARM
#[no_mangle]
pub unsafe extern "C" fn xen_remap_vma_range(vma: *mut vm_area_struct, addr: c_ulong, len: c_ulong) -> c_int {
    int xen_remap_vma_range(struct vm_area_struct *vma, unsigned long addr, unsigned long len)
    {
    struct remap_pfn r = {
    .mm = vma.vm_mm,
    .pages = vma.vm_private_data,
    .prot = vma.vm_page_prot,
    };
    return apply_to_page_range(vma.vm_mm, addr, len, remap_pfn_fn, &r);
    }
    EXPORT_SYMBOL_GPL(xen_remap_vma_range);

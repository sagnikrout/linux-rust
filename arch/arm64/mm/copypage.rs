//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/copypage.c
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
// Based on arch/arm/mm/copypage.c
//
// Copyright (C) 2002 Deep Blue Solutions Ltd, All Rights Reserved.
// Copyright (C) 2012 ARM Ltd.
//

#[no_mangle]
pub unsafe extern "C" fn copy_highpage(to: *mut page, from: *mut page) {
    void copy_highpage(struct page *to, struct page *from)
    {
    void *kto = page_address(to);
    void *kfrom = page_address(from);
    struct folio *src = page_folio(from);
    struct folio *dst = page_folio(to);
    unsigned int i, nr_pages;
    copy_page(kto, kfrom);
    if (kasan_hw_tags_enabled())
    page_kasan_tag_reset(to);
    if (!system_supports_mte())
    return;
    if (folio_test_hugetlb(src)) {
    if (!folio_test_hugetlb_mte_tagged(src) ||
    from != folio_page(src, 0))
    return;
    folio_try_hugetlb_mte_tagging(dst);
//
// Populate tags for all subpages.
//
// Don't assume the first page is head page since
// huge page copy may start from any subpage.
//
    nr_pages = folio_nr_pages(src);
    for (i = 0; i < nr_pages; i++) {
    kfrom = page_address(folio_page(src, i));
    kto = page_address(folio_page(dst, i));
    mte_copy_page_tags(kto, kfrom);
    }
    folio_set_hugetlb_mte_tagged(dst);
    } else if (page_mte_tagged(from)) {
//
// Most of the time it's a new page that shouldn't have been
// tagged yet. However, folio migration can end up reusing the
// same page without untagging it. Ignore the warning if the
// page is already tagged.
//
    try_page_mte_tagging(to);
    mte_copy_page_tags(kto, kfrom);
    set_page_mte_tagged(to);
    }
    }
    EXPORT_SYMBOL(copy_highpage);
    void copy_user_highpage(struct page *to, struct page *from,
    unsigned long vaddr, struct vm_area_struct *vma)
    {
    copy_highpage(to, from);
    flush_dcache_page(to);
    }
    EXPORT_SYMBOL_GPL(copy_user_highpage);

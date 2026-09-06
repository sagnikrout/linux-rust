//! Automatically rewritten from C to Rust
//! Source: tools/testing/nvdimm/pmem-dax.c
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
// Copyright (c) 2014-2016, Intel Corporation.
//

    long __pmem_direct_access(struct pmem_device *pmem, pgoff_t pgoff,
    long nr_pages, enum dax_access_mode mode, void **kaddr,
    unsigned long *pfn)
    {
    let mut offset: resource_size_t = PFN_PHYS(pgoff) + pmem.data_offset;
    if (unlikely(is_bad_pmem(&pmem.bb, PFN_PHYS(pgoff) / 512,
    PFN_PHYS(nr_pages))))
    return -EIO;
//
// Limit dax to a single page at a time given vmalloc()-backed
// in the nfit_test case.
//
    if (get_nfit_res(pmem.phys_addr + offset)) {
    struct page *page;
    if (kaddr)
// kaddr = pmem->virt_addr + offset;
    page = vmalloc_to_page(pmem.virt_addr + offset);
    if (pfn)
// pfn = page_to_pfn(page);
    pr_debug_ratelimited("%s: pmem: %p pgoff: %#lx pfn: %#lx\n",
    __func__, pmem, pgoff, page_to_pfn(page));
    return 1;
    }
    if (kaddr)
// kaddr = pmem->virt_addr + offset;
    if (pfn)
// pfn = PHYS_PFN(pmem->phys_addr + offset);
//
// If badblocks are present, limit known good range to the
// requested range.
//
    if (unlikely(pmem.bb.count))
    return nr_pages;
    return PHYS_PFN(pmem.size - pmem.pfn_pad - offset);
    }

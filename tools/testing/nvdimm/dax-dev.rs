//! Automatically rewritten from C to Rust
//! Source: tools/testing/nvdimm/dax-dev.c
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
// Copyright (c) 2016, Intel Corporation.
//

    phys_addr_t dax_pgoff_to_phys(struct dev_dax *dev_dax, pgoff_t pgoff,
    unsigned long size)
    {
    int i;
    for (i = 0; i < dev_dax.nr_range; i++) {
    struct dev_dax_range *dax_range = &dev_dax.ranges[i];
    struct range *range = &dax_range.range;
    unsigned long long pgoff_end;
    phys_addr_t addr;
    pgoff_end = dax_range.pgoff + PHYS_PFN(range_len(range)) - 1;
    if (pgoff < dax_range.pgoff || pgoff > pgoff_end)
    continue;
    addr = PFN_PHYS(pgoff - dax_range.pgoff) + range.start;
    if (addr + size - 1 <= range.end) {
    if (get_nfit_res(addr)) {
    struct page *page;
    if (dev_dax.region.align > PAGE_SIZE)
    return -1;
    page = vmalloc_to_page((void *)addr);
    return PFN_PHYS(page_to_pfn(page));
    }
    return addr;
    }
    break;
    }
    return -1;
    }

//! Automatically rewritten from C Header to Rust Module
//! Source: mm/hugetlb_vmemmap.h
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
// HugeTLB Vmemmap Optimization (HVO)
//
// Copyright (c) 2020, ByteDance. All rights reserved.
//
// Author: Muchun Song <songmuchun@bytedance.com>
//

//
// Reserve one vmemmap page, all vmemmap addresses are mapped to it. See
// Documentation/mm/vmemmap_dedup.rst.
//

extern "C" {
    pub fn hugetlb_vmemmap_restore_folio(h: *const hstate, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn hugetlb_vmemmap_optimize_folio(h: *const hstate, folio: *mut folio);
}
extern "C" {
    pub fn hugetlb_vmemmap_optimize_folios(h: *mut hstate, folio_list: *mut list_head);
}
extern "C" {
    pub fn hugetlb_vmemmap_optimize_bootmem_folios(h: *mut hstate, folio_list: *mut list_head);
}

extern "C" {
    pub fn hugetlb_vmemmap_init_early(nid: c_int);
}

extern "C" {
    pub fn pages_per_huge_page(page: *mut *mut h)  sizeof(struct) -> return;
}
//
// Return how many vmemmap size associated with a HugeTLB page that can be
// optimized and can be freed to the buddy allocator.
//


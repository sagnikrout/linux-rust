//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iova.h
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
// Copyright (c) 2006, Intel Corporation.
//
// Copyright (C) 2006-2008 Intel Corporation
// Author: Anil S Keshavamurthy <anil.s.keshavamurthy@intel.com>
//

// iova structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iova {
    pub node: rb_node,
    pub /: *mut *mut unsigned long pfn_hi; / Highest allocated pfn,
    pub /: *mut *mut unsigned long pfn_lo; / Lowest allocated pfn,
}

// holds all the iova translations for a domain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iova_domain {
    pub /: *mut *mut spinlock_t iova_rbtree_lock; / Lock to protect update of rbtree,
    pub /: *mut *mut rb_root rbroot; / iova domain rbtree root,
    pub /: *mut *mut *mut rb_node cached_node; / Save last alloced node,
    pub /: *mut *mut *mut rb_node cached32_node; / Save last 32-bit alloced node,
    pub /: *mut *mut unsigned long granule; / pfn granularity for this domain,
    pub /: *mut *mut unsigned long start_pfn; / Lower limit for this domain,
    pub dma_32bit_pfn: c_ulong,
    pub /: *mut *mut unsigned long max32_alloc_size; / Size of last failed allocation,
    pub /: *mut *mut iova anchor; / rbtree lookup anchor,
    pub rcaches: *mut iova_rcache,
    pub cpuhp_dead: hlist_node,
}

extern "C" {
    pub fn __ffs(_arg: iovad->granule) -> return;
}
extern "C" {
    pub fn ALIGN(_arg: size, _arg: iovad->granule) -> return;
}
extern "C" {
    pub fn ALIGN_DOWN(_arg: size, _arg: iovad->granule) -> return;
}

extern "C" {
    pub fn iova_cache_get() -> c_int;
}
extern "C" {
    pub fn iova_cache_put();
}
extern "C" {
    pub fn iova_rcache_range() -> c_ulong;
}
extern "C" {
    pub fn free_iova(iovad: *mut iova_domain, pfn: c_ulong);
}
extern "C" {
    pub fn __free_iova(iovad: *mut iova_domain, iova: *mut iova);
}
extern "C" {
    pub fn iova_domain_init_rcaches(iovad: *mut iova_domain) -> c_int;
}
extern "C" {
    pub fn put_iova_domain(iovad: *mut iova_domain);
}


//! Automatically rewritten from C Header to Rust Module
//! Source: mm/cma.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cma_kobject {
    pub kobj: kobject,
    pub cma: *mut cma,
}

//
// Multi-range support. This can be useful if the size of the allocation
// is not expected to be larger than the alignment (like with hugetlb_cma),
// and the total amount of memory requested, while smaller than the total
// amount of memory available, is large enough that it doesn't fit in a
// single physical memory range because of memory holes.
//
// Fields:
// @base_pfn: physical address of range
// @early_pfn: first PFN not reserved through cma_reserve_early
// @count: size of range
// @bitmap: bitmap of allocated (1 << order_per_bit)-sized chunks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cma_memrange {
    pub base_pfn: c_ulong,
    pub count: c_ulong,
    pub early_pfn: c_ulong,
    pub bitmap: *mut c_ulong,
}

pub const CMA_MAX_RANGES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cma {
    pub count: c_ulong,
    pub available_count: c_ulong,
    pub /: *mut *mut unsigned int order_per_bit; / Order of pages represented by one bit,
    pub lock: spinlock_t,
    pub alloc_mutex: mutex,

    pub mem_head: hlist_head,
    pub mem_head_lock: spinlock_t,
    pub name: [c_char; CMA_MAX_NAME],
    pub nranges: c_int,
    pub ranges: [cma_memrange; CMA_MAX_RANGES],
// the number of CMA page successful allocations
    pub nr_pages_succeeded: core::sync::atomic::AtomicI64,
// the number of CMA page allocation failures
    pub nr_pages_failed: core::sync::atomic::AtomicI64,
// the number of CMA page released
    pub nr_pages_released: core::sync::atomic::AtomicI64,
// kobject requires dynamic object
    pub cma_kobj: *mut cma_kobject,

    pub flags: c_ulong,
// NUMA node (NUMA_NO_NODE if unspecified)
    pub nid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cma_flags {
    CMA_RESERVE_PAGES_ON_ERROR,
    CMA_ZONES_VALID,
    CMA_ZONES_INVALID,
    CMA_ACTIVATED,
}

extern "C" {
    pub fn cma_sysfs_account_success_pages(cma: *mut cma, nr_pages: c_ulong);
}
extern "C" {
    pub fn cma_sysfs_account_fail_pages(cma: *mut cma, nr_pages: c_ulong);
}
extern "C" {
    pub fn cma_sysfs_account_release_pages(cma: *mut cma, nr_pages: c_ulong);
}


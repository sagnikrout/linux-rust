//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hugetlb_cgroup.h
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
// Copyright IBM Corporation, 2012
// Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of version 2.1 of the GNU Lesser General Public License
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it would be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hugetlb_memory_event {
    HUGETLB_MAX,
    HUGETLB_NR_MEMORY_EVENTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hugetlb_cgroup_per_node {
// hugetlb usage in pages over all hstates.
    pub usage: [c_ulong; HUGE_MAX_HSTATE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hugetlb_cgroup {
    pub css: cgroup_subsys_state,
//
// the counter to account for hugepages from hugetlb.
//
    pub hugepage: [page_counter; HUGE_MAX_HSTATE],
//
// the counter to account for hugepage reservations from hugetlb.
//
    pub rsvd_hugepage: [page_counter; HUGE_MAX_HSTATE],
    pub events: [atomic_long_t; HUGE_MAX_HSTATE][HUGETLB_NR_MEMORY_EVENTS],
    pub events_local: [atomic_long_t; HUGE_MAX_HSTATE][HUGETLB_NR_MEMORY_EVENTS],
// Handle for "hugetlb.events"
    pub events_file: [cgroup_file; HUGE_MAX_HSTATE],
// Handle for "hugetlb.events.local"
    pub events_local_file: [cgroup_file; HUGE_MAX_HSTATE],
    pub nodeinfo: [*mut hugetlb_cgroup_per_node; ],
}

extern "C" {
    pub fn __hugetlb_cgroup_from_folio(_arg: folio, _arg: false) -> return;
}
extern "C" {
    pub fn __hugetlb_cgroup_from_folio(_arg: folio, _arg: true) -> return;
}


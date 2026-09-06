//! Automatically rewritten from C Header to Rust Module
//! Source: mm/hugetlb_internal.h
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
// Internal HugeTLB definitions.
// (C) Nadia Yvette Chambers, April 2004
//

//
// Check if the hstate represents gigantic pages but gigantic page
// runtime support is not available. This is a common condition used to
// skip operations that cannot be performed on gigantic pages when runtime
// support is disabled.
//
extern "C" {
    pub fn hstate_is_gigantic(!gigantic_page_runtime_supported(: h) &&) -> return;
}
//
// common helper functions for hstate_next_node_to_{alloc|free}.
// We may have allocated or freed a huge page based on a different
// nodes_allowed previously, so h->next_node_to_{alloc|free} might
// be outside of *nodes_allowed.  Ensure that we use an allowed
// node for alloc or free.
//
// returns the previously saved node ["this node"] from which to
// allocate a persistent huge page for the pool and advance the
// next node from which to allocate, handling wrap at end of node
// mask.
//
// next_node = next_node_allowed(nid, nodes_allowed);
//
// helper for remove_pool_hugetlb_folio() - return the previously saved
// node ["this node"] from which to free a huge page.  Advance the
// next node id whether or not we find a free huge page to free so
// that the next attempt to free addresses the next node.
//

extern "C" {
    pub fn init_new_hugetlb_folio(folio: *mut folio);
}

extern "C" {
    pub fn hugetlb_sysctl_init();
}


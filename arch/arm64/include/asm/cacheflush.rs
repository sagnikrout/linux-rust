//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/cacheflush.h
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
// Based on arch/arm/include/asm/cacheflush.h
//
// Copyright (C) 1999-2002 Russell King.
// Copyright (C) 2012 ARM Ltd.
//

//
// This flag is used to indicate that the page pointed to by a pte is clean
// and does not require cleaning before returning it to the user.
//

//
// MM Cache Management
// ===================
//
// The arch/arm64/mm/cache.S implements these methods.
//
// Start addresses are inclusive and end addresses are exclusive; start
// addresses should be rounded down, end addresses up.
//
// See Documentation/core-api/cachetlb.rst for more information. Please note that
// the implementation assumes non-aliasing VIPT D-cache and (aliasing)
// VIPT I-cache.
//
// All functions below apply to the interval [start, end)
// - start  - virtual start address (inclusive)
// - end    - virtual end address (exclusive)
//
// caches_clean_inval_pou(start, end)
//
// Ensure coherency between the I-cache and the D-cache region to
// the Point of Unification.
//
// caches_clean_inval_user_pou(start, end)
//
// Ensure coherency between the I-cache and the D-cache region to
// the Point of Unification.
// Use only if the region might access user memory.
//
// icache_inval_pou(start, end)
//
// Invalidate I-cache region to the Point of Unification.
//
// dcache_clean_inval_poc(start, end)
//
// Clean and invalidate D-cache region to the Point of Coherency.
//
// dcache_inval_poc(start, end)
//
// Invalidate D-cache region to the Point of Coherency.
//
// dcache_clean_poc(start, end)
//
// Clean D-cache region to the Point of Coherency.
//
// dcache_clean_pop(start, end)
//
// Clean D-cache region to the Point of Persistence.
//
// dcache_clean_pou(start, end)
//
// Clean D-cache region to the Point of Unification.
//
extern "C" {
    pub fn caches_clean_inval_pou(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn icache_inval_pou(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn dcache_clean_inval_poc(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn dcache_inval_poc(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn dcache_clean_poc(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn dcache_inval_poc_nosync(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn dcache_clean_poc_nosync(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn dcache_clean_pop(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn dcache_clean_pou(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn caches_clean_inval_user_pou(start: c_ulong, end: c_ulong) -> c_long;
}
extern "C" {
    pub fn sync_icache_aliases(start: c_ulong, end: c_ulong);
}
//
// IPI all online CPUs so that they undergo a context synchronization
// event and are forced to refetch the new instructions.
//
// KGDB performs cache maintenance with interrupts disabled, so we
// will deadlock trying to IPI the secondary CPUs. In theory, we can
// set CACHE_FLUSH_IS_SAFE to 0 to avoid this known issue, but that
// just means that KGDB will elide the maintenance altogether! As it
// turns out, KGDB uses IPIs to round-up the secondary CPUs during
// the patching operation, so we don't need extra IPIs here anyway.
// In which case, add a KGDB-specific bodge and return early.
//

//
// Copy user data from/to a page which is mapped into a different
// processes address space.  Really, we want to allow our "user
// space" model to handle this.
//

//
// flush_dcache_folio is used when the kernel has written to the page
// cache page at virtual address page->virtual.
//
// If this page isn't mapped (ie, folio_mapping == NULL), or it might
// have userspace mappings, then we _must_ always clean + invalidate
// the dcache entries associated with the kernel mapping.
//
// Otherwise we can defer the operation, and clean the cache when we are
// about to change to user space.  This is the same method as used on SPARC64.
// See update_mmu_cache for the user space part.
//
pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: c_int = 1;
extern "C" {
    pub fn flush_dcache_page(: *mut page);
}
extern "C" {
    pub fn flush_dcache_folio(: *mut folio);
}

